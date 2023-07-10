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
	all_substrate_web3networks, BoundedWeb3Network, Identity, ParentchainBlockNumber,
	UserShieldingKeyType, Web3Network,
};
use sp_std::vec::Vec;

pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type IDGraph<T> = Vec<(Identity, IdentityContext<T>)>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use litentry_primitives::LitentryMultiAddress;
	use log::{debug, warn};

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
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
		UserShieldingKeySet { who: LitentryMultiAddress, key: UserShieldingKeyType },
		/// an identity was linked
		IdentityLinked { who: LitentryMultiAddress, identity: Identity },
		/// an identity was removed
		IdentityRemoved { who: LitentryMultiAddress, identity: Identity },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the pair (address, identity) already linked
		IdentityAlreadyLinked,
		/// the pair (address, identity) doesn't exist
		IdentityNotExist,
		/// creating the prime identity manually is disallowed
		LinkPrimeIdentityDisallowed,
		/// remove prime identity should be disallowed
		RemovePrimeIdentityDisallowed,
		/// IDGraph len limit reached
		IDGraphLenLimitReached,
		/// Web3Network len limit reached
		Web3NetworkLenLimitReached,
		/// identity doesn't match the network types
		WrongWeb3NetworkTypes,
	}

	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, LitentryMultiAddress, UserShieldingKeyType, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		LitentryMultiAddress,
		Blake2_128Concat,
		Identity,
		IdentityContext<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type IDGraphLens<T: Config> =
		StorageMap<_, Blake2_128Concat, LitentryMultiAddress, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(15_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			who: LitentryMultiAddress,
			key: UserShieldingKeyType,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			let prime_id = Self::build_prime_identity(&who)?;
			if IDGraphs::<T>::get(&who, &prime_id).is_none() {
				// TODO: shall we activate all available networks for the prime id?
				let web3networks = all_substrate_web3networks()
					.try_into()
					.map_err(|_| Error::<T>::Web3NetworkLenLimitReached)?;
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
			who: LitentryMultiAddress,
			identity: Identity,
			web3networks: BoundedWeb3Network,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			ensure!(
				!IDGraphs::<T>::contains_key(&who, &identity),
				Error::<T>::IdentityAlreadyLinked
			);
			let prime_id = Self::build_prime_identity(&who)?;
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
		pub fn remove_identity(
			origin: OriginFor<T>,
			who: LitentryMultiAddress,
			identity: Identity,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			let prime_id = Self::build_prime_identity(&who)?;
			ensure!(identity != prime_id, Error::<T>::RemovePrimeIdentityDisallowed);

			Self::remove_identity_with_limit(&who, &identity);
			Self::deposit_event(Event::IdentityRemoved { who, identity });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// build the prime identity which is always a substrate address32-based identity
		fn build_prime_identity(address: &LitentryMultiAddress) -> Result<Identity, DispatchError> {
			match address {
				LitentryMultiAddress::Substrate(address) => Ok(Identity::Substrate(*address)),
				LitentryMultiAddress::Evm(address) => Ok(Identity::Evm(*address)),
			}
		}

		fn insert_identity_with_limit(
			owner: &LitentryMultiAddress,
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
			IDGraphs::<T>::insert(owner, identity, context);
			Ok(())
		}

		fn remove_identity_with_limit(owner: &LitentryMultiAddress, identity: &Identity) {
			IDGraphLens::<T>::mutate_exists(owner, |maybe_value| {
				if let Some(graph_len) = maybe_value {
					if *graph_len == 0 {
						warn!(
						"Detected IDGraphLens inconsistency, found len 0 while removing identity"
					);
						*maybe_value = None
					} else {
						let new_graph_len = *graph_len - 1;
						if new_graph_len == 0 {
							*maybe_value = None
						} else {
							*maybe_value = Some(new_graph_len)
						}
					}
				} else {
					warn!("Detected IDGraphLens inconsistency, missing IdGraphLen while removing identity");
				}
			});
			IDGraphs::<T>::remove(owner, identity);
		}

		// get the most recent `max_len` elements in IDGraph
		pub fn get_id_graph(who: &LitentryMultiAddress, max_len: usize) -> IDGraph<T> {
			let mut id_graph = IDGraphs::iter_prefix(who).collect::<IDGraph<T>>();
			id_graph.sort_by(|a, b| Ord::cmp(&b.1.link_block, &a.1.link_block));
			id_graph.truncate(max_len);
			id_graph
		}

		// get count of all keys account + identity in the IDGraphs
		pub fn id_graph_stats() -> Option<Vec<(LitentryMultiAddress, u32)>> {
			let stats = IDGraphLens::<T>::iter().collect();
			debug!("IDGraph stats: {:?}", stats);
			Some(stats)
		}
	}
}
