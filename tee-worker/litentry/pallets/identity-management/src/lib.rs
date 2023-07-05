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
use log::debug;

pub use litentry_primitives::{
	Identity, ParentchainBlockNumber, SubstrateNetwork, UserShieldingKeyType,
};
use sp_std::vec::Vec;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type IDGraph<T> = Vec<(Identity, IdentityContext<T>)>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use litentry_primitives::Address32;
	use log::warn;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// the manager origin for extrincis
		type ManageOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// maximum number of identities an account can have, if you change this value to lower some accounts may exceed this limit
		#[pallet::constant]
		type MaxIDGraphLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// user shielding key was set
		UserShieldingKeySet { who: T::AccountId, key: UserShieldingKeyType },
		/// an identity was linked
		IdentityLinked { who: T::AccountId, identity: Identity },
		/// an identity was removed
		IdentityRemoved { who: T::AccountId, identity: Identity },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the pair (litentry-account, identity) already linked
		IdentityAlreadyLinked,
		/// the pair (litentry-account, identity) doesn't exist
		IdentityNotExist,
		/// creating the prime identity manually is disallowed
		LinkPrimeIdentityDisallowed,
		/// remove prime identiy should be disallowed
		RemovePrimeIdentityDisallowed,
		/// IDGraph len limit reached
		IDGraphLenLimitReached,
	}

	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserShieldingKeyType, OptionQuery>;

	/// ID graph is per Litentry account + identity
	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Identity,
		IdentityContext<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type IDGraphLens<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(15_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			who: T::AccountId,
			key: UserShieldingKeyType,
			parent_ss58_prefix: u16,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			// we don't care about the current key
			UserShieldingKeys::<T>::insert(&who, key);

			let prime_id = Self::build_prime_identity(&who, parent_ss58_prefix)?;
			if IDGraphs::<T>::get(&who, &prime_id).is_none() {
				let context = <IdentityContext<T>>::new(<frame_system::Pallet<T>>::block_number());
				Self::insert_identity_with_limit(&who, &prime_id, context)?;
			}

			Self::deposit_event(Event::UserShieldingKeySet { who, key });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(15_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
			parent_ss58_prefix: u16,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			ensure!(
				!IDGraphs::<T>::contains_key(&who, &identity),
				Error::<T>::IdentityAlreadyLinked
			);
			let prime_id = Self::build_prime_identity(&who, parent_ss58_prefix)?;
			ensure!(identity != prime_id, Error::<T>::LinkPrimeIdentityDisallowed);

			let context = <IdentityContext<T>>::new(<frame_system::Pallet<T>>::block_number());
			Self::insert_identity_with_limit(&who, &identity, context)?;
			Self::deposit_event(Event::IdentityLinked { who, identity });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(15_000_000)]
		pub fn remove_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
			parent_ss58_prefix: u16,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			let prime_id = Self::build_prime_identity(&who, parent_ss58_prefix)?;
			ensure!(identity != prime_id, Error::<T>::RemovePrimeIdentityDisallowed);

			Self::remove_identity_with_limit(&who, &identity);
			Self::deposit_event(Event::IdentityRemoved { who, identity });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// build the prime identity which is always a substrate address32-based identity
		fn build_prime_identity(
			who: &T::AccountId,
			parent_ss58_prefix: u16,
		) -> Result<Identity, DispatchError> {
			let address_raw: [u8; 32] =
				who.encode().try_into().map_err(|_| DispatchError::Other("Invalid AccountId"))?;
			let address: Address32 = address_raw.into();

			Ok(Identity::Substrate {
				network: SubstrateNetwork::from_ss58_prefix(parent_ss58_prefix),
				address,
			})
		}

		fn insert_identity_with_limit(
			owner: &T::AccountId,
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

		fn remove_identity_with_limit(owner: &T::AccountId, identity: &Identity) {
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
		pub fn get_id_graph(who: &T::AccountId, max_len: usize) -> IDGraph<T> {
			let mut id_graph = IDGraphs::<T>::iter_prefix(who).collect::<IDGraph<T>>();
			id_graph.sort_by(|a, b| Ord::cmp(&b.1.link_block, &a.1.link_block));
			id_graph.truncate(max_len);
			id_graph
		}

		// get count of all keys account + identity in the IDGraphs
		pub fn id_graph_stats() -> Option<Vec<(T::AccountId, u32)>> {
			let stats = IDGraphLens::<T>::iter().collect();
			debug!("IDGraph stats: {:?}", stats);
			Some(stats)
		}
	}
}
