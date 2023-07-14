// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

// Identity management pallet run within TEE(enclave) -- aka IMT
// The pallet is integrated in SGX-runtime, the extrinsics are supposed
// to be called only by enclave
//
// TODO:
// - origin management, only allow calls from TEE (= origin is signed with the ECC key), or root?
//   otherwise we'd always require the origin has some fund
// - maybe don't emit events at all, or at least remove sensistive data
// - benchmarking

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod migrations;

pub use pallet::*;
pub mod identity_context;
pub use identity_context::*;

use frame_support::{pallet_prelude::*, traits::StorageVersion};
use frame_system::pallet_prelude::*;

pub use litentry_primitives::{
	all_substrate_web3networks, Identity, ParentchainBlockNumber, UserShieldingKeyType, Web3Network,
};
use sp_std::vec::Vec;

pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type IDGraph<T> = Vec<(Identity, IdentityContext<T>)>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use litentry_primitives::all_evm_web3networks;
	use log::debug;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// the manager origin for extrinsics
		type ManageOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// maximum number of identities an account can have, if you change this value to lower some accounts may exceed this limit
		#[pallet::constant]
		type MaxIDGraphLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// user shielding key was set
		UserShieldingKeySet { who: Identity, key: UserShieldingKeyType },
		/// an identity was linked
		IdentityLinked { who: Identity, identity: Identity },
		/// an identity was deactivated
		IdentityDeactivated { who: Identity, identity: Identity },
		/// an identity was activated
		IdentityActivated { who: Identity, identity: Identity },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the identity is already linked
		IdentityAlreadyLinked,
		/// the pair (Identity, Identity) doesn't exist
		IdentityNotExist,
		/// creating the prime identity manually is disallowed
		LinkPrimeIdentityDisallowed,
		/// deactivate prime identity should be disallowed
		DeactivatePrimeIdentityDisallowed,
		/// IDGraph len limit reached
		IDGraphLenLimitReached,
		/// identity doesn't match the network types
		WrongWeb3NetworkTypes,
		/// identity cannot be used to build prime identity
		NotSupportedIdentity,
	}

	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, Identity, UserShieldingKeyType, OptionQuery>;

	#[pallet::storage]
	pub type LinkedIdentities<T: Config> =
		StorageMap<_, Blake2_128Concat, Identity, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Identity,
		Blake2_128Concat,
		Identity,
		IdentityContext<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type IDGraphLens<T: Config> = StorageMap<_, Blake2_128Concat, Identity, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(15_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			who: Identity,
			key: UserShieldingKeyType,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			let (prime_id, web3networks) = Self::build_prime_identity_with_networks(&who)?;
			if IDGraphs::<T>::get(&who, &prime_id).is_none() {
				let context = <IdentityContext<T>>::new(
					<frame_system::Pallet<T>>::block_number(),
					web3networks,
				);
				Self::insert_identity_with_limit(&who, &prime_id, context)?;
			}
			// we don't care about the current key
			UserShieldingKeys::<T>::insert(&who, key);

			Self::deposit_event(Event::UserShieldingKeySet { who, key });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(15_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: Identity,
			web3networks: Vec<Web3Network>,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			ensure!(
				!LinkedIdentities::<T>::contains_key(&identity),
				Error::<T>::IdentityAlreadyLinked
			);
			let (prime_id, _) = Self::build_prime_identity_with_networks(&who)?;
			ensure!(identity != prime_id, Error::<T>::LinkPrimeIdentityDisallowed);

			ensure!(
				identity.matches_web3networks(web3networks.as_ref()),
				Error::<T>::WrongWeb3NetworkTypes
			);

			let context =
				<IdentityContext<T>>::new(<frame_system::Pallet<T>>::block_number(), web3networks);
			Self::insert_identity_with_limit(&who, &identity, context)?;
			Self::deposit_event(Event::IdentityLinked { who, identity });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(15_000_000)]
		pub fn deactivate_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: Identity,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			let (prime_id, _) = Self::build_prime_identity_with_networks(&who)?;
			ensure!(identity != prime_id, Error::<T>::DeactivatePrimeIdentityDisallowed);

			IDGraphs::<T>::try_mutate(&who, &identity, |context| {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				c.deactivate();
				*context = Some(c);
				Result::<(), Error<T>>::Ok(())
			})?;
			Self::deposit_event(Event::IdentityDeactivated { who, identity });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(15_000_000)]
		pub fn activate_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: Identity,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			IDGraphs::<T>::try_mutate(&who, &identity, |context| {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				c.activate();
				*context = Some(c);
				Result::<(), Error<T>>::Ok(())
			})?;
			Self::deposit_event(Event::IdentityActivated { who, identity });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(15_000_000)]
		pub fn set_identity_networks(
			origin: OriginFor<T>,
			who: Identity,
			identity: Identity,
			web3networks: Vec<Web3Network>,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			IDGraphs::<T>::try_mutate(&who, &identity, |context| {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				ensure!(
					identity.matches_web3networks(web3networks.as_ref()),
					Error::<T>::WrongWeb3NetworkTypes
				);
				c.set_web3networks(web3networks);
				*context = Some(c);
				Ok(())
			})
		}
	}

	impl<T: Config> Pallet<T> {
		// Build the prime identity which might be substrate address32-based or evm address20-based and related networks vec
		// For the moment, return all available (matching) networks for the prime id
		fn build_prime_identity_with_networks(
			identity: &Identity,
		) -> Result<(Identity, Vec<Web3Network>), DispatchError> {
			match identity {
				Identity::Substrate(address) =>
					Ok((Identity::Substrate(*address), all_substrate_web3networks())),
				Identity::Evm(address) => Ok((Identity::Evm(*address), all_evm_web3networks())),
				_ => Err(Error::<T>::NotSupportedIdentity.into()),
			}
		}

		fn insert_identity_with_limit(
			owner: &Identity,
			identity: &Identity,
			context: IdentityContext<T>,
		) -> Result<(), DispatchError> {
			IDGraphLens::<T>::try_mutate(owner, |len| {
				let new_len = len.checked_add(1).ok_or(Error::<T>::IDGraphLenLimitReached)?;
				if new_len > T::MaxIDGraphLength::get() {
					return Err(Error::<T>::IDGraphLenLimitReached.into())
				}
				*len = new_len;
				Result::<(), DispatchError>::Ok(())
			})?;
			LinkedIdentities::<T>::insert(identity, ());
			IDGraphs::<T>::insert(owner, identity, context);
			Ok(())
		}

		// get the most recent `max_len` active elements in IDGraph
		pub fn get_id_graph(who: &Identity, max_len: usize) -> IDGraph<T> {
			let mut id_graph = IDGraphs::iter_prefix(who)
				.filter(|(_, c)| c.is_active())
				.collect::<IDGraph<T>>();
			id_graph.sort_by(|a, b| Ord::cmp(&b.1.link_block, &a.1.link_block));
			id_graph.truncate(max_len);
			id_graph
		}

		// get count of all keys account + identity in the IDGraphs
		pub fn id_graph_stats() -> Option<Vec<(Identity, u32)>> {
			let stats = IDGraphLens::<T>::iter().collect();
			debug!("IDGraph stats: {:?}", stats);
			Some(stats)
		}
	}
}
