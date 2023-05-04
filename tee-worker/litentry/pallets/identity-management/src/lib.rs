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

use frame_support::{pallet_prelude::*, traits::StorageVersion};
use frame_system::pallet_prelude::*;
use log::debug;

pub use identity_context::IdentityContext;
pub use litentry_primitives::{
	ChallengeCode, Identity, ParentchainBlockNumber, SubstrateNetwork, UserShieldingKeyType,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type MetadataOf<T> = BoundedVec<u8, <T as Config>::MaxMetadataLength>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use litentry_primitives::Address32;

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
		/// maximum metadata length
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
		/// maximum delay in block numbers between creating an identity and verifying an identity
		#[pallet::constant]
		type MaxVerificationDelay: Get<ParentchainBlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// user shielding key was set
		UserShieldingKeySet { who: T::AccountId, key: UserShieldingKeyType },
		/// challenge code was set
		ChallengeCodeSet { who: T::AccountId, identity: Identity, code: ChallengeCode },
		/// challenge code was removed
		ChallengeCodeRemoved { who: T::AccountId, identity: Identity },
		/// an identity was created
		IdentityCreated { who: T::AccountId, identity: Identity },
		/// an identity was removed
		IdentityRemoved { who: T::AccountId, identity: Identity },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// challenge code doesn't exist
		ChallengeCodeNotExist,
		/// the pair (litentry-account, identity) already verified when creating an identity
		IdentityAlreadyVerified,
		/// the pair (litentry-account, identity) doesn't exist
		IdentityNotExist,
		/// the identity was not created before verification
		IdentityNotCreated,
		/// creating the prime identity manually is disallowed
		CreatePrimeIdentityNotAllowed,
		/// a verification reqeust comes too early
		VerificationRequestTooEarly,
		/// a verification reqeust comes too late
		VerificationRequestTooLate,
		/// remove prime identiy should be disallowed
		RemovePrimeIdentityDisallowed,
	}

	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserShieldingKeyType, OptionQuery>;

	/// challenge code is per Litentry account + identity
	#[pallet::storage]
	#[pallet::getter(fn challenge_codes)]
	pub type ChallengeCodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Identity,
		ChallengeCode,
		OptionQuery,
	>;

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

			let prime_address_raw: [u8; 32] = who
				.encode()
				.try_into()
				.map_err(|_| DispatchError::Other("invalid account id"))?;
			let prime_user_address: Address32 = prime_address_raw.into();

			let prime_id = Identity::Substrate {
				network: SubstrateNetwork::from_ss58_prefix(parent_ss58_prefix),
				address: prime_user_address,
			};
			if IDGraphs::<T>::get(&who, &prime_id).is_none() {
				// Not existed, so create the prime entry.
				let context = IdentityContext::<T> {
					metadata: None,
					creation_request_block: Some(0),
					verification_request_block: Some(0),
					is_verified: true,
				};
				IDGraphs::<T>::insert(&who, &prime_id, context);
			}

			Self::deposit_event(Event::UserShieldingKeySet { who, key });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(15_000_000)]
		pub fn set_challenge_code(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
			code: ChallengeCode,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			// we don't care if it has already associated with any challenge code
			ChallengeCodes::<T>::insert(&who, &identity, code);
			Self::deposit_event(Event::ChallengeCodeSet { who, identity, code });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(15_000_000)]
		pub fn remove_challenge_code(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(
				ChallengeCodes::<T>::contains_key(&who, &identity),
				Error::<T>::ChallengeCodeNotExist
			);
			ChallengeCodes::<T>::remove(&who, &identity);
			Self::deposit_event(Event::ChallengeCodeRemoved { who, identity });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(15_000_000)]
		pub fn create_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
			metadata: Option<MetadataOf<T>>,
			creation_request_block: ParentchainBlockNumber,
			parent_ss58_prefix: u16,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			if let Some(c) = IDGraphs::<T>::get(&who, &identity) {
				ensure!(
					!(c.is_verified && c.creation_request_block != Some(0)),
					Error::<T>::IdentityAlreadyVerified
				);
			}
			if let Identity::Substrate { network, address } = identity {
				if network.ss58_prefix() == parent_ss58_prefix {
					let address_raw: [u8; 32] = who
						.encode()
						.try_into()
						.map_err(|_| DispatchError::Other("invalid account id"))?;
					let user_address: Address32 = address_raw.into();
					ensure!(user_address != address, Error::<T>::CreatePrimeIdentityNotAllowed);
				}
			}

			let context = IdentityContext {
				metadata,
				creation_request_block: Some(creation_request_block),
				..Default::default()
			};
			IDGraphs::<T>::insert(&who, &identity, context);
			Self::deposit_event(Event::IdentityCreated { who, identity });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(15_000_000)]
		pub fn remove_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);

			if let Some(IdentityContext::<T> {
				metadata,
				creation_request_block,
				verification_request_block,
				is_verified,
			}) = IDGraphs::<T>::get(&who, &identity)
			{
				if metadata.is_none()
					&& creation_request_block == Some(0)
					&& verification_request_block == Some(0)
					&& is_verified
				{
					ensure!(false, Error::<T>::RemovePrimeIdentityDisallowed);
				}
			}

			IDGraphs::<T>::remove(&who, &identity);
			Self::deposit_event(Event::IdentityRemoved { who, identity });
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(15_000_000)]
		pub fn verify_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity: Identity,
			verification_request_block: ParentchainBlockNumber,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			IDGraphs::<T>::try_mutate(&who, &identity, |context| -> DispatchResult {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				ensure!(!c.is_verified, Error::<T>::IdentityAlreadyVerified);

				if let Some(b) = c.creation_request_block {
					ensure!(
						b <= verification_request_block,
						Error::<T>::VerificationRequestTooEarly
					);
					ensure!(
						verification_request_block - b <= T::MaxVerificationDelay::get(),
						Error::<T>::VerificationRequestTooLate
					);
					c.is_verified = true;
					c.verification_request_block = Some(verification_request_block);
					*context = Some(c);
					Ok(())
				} else {
					Err(Error::<T>::IdentityNotCreated.into())
				}
			})
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_id_graph(who: &T::AccountId) -> Vec<(Identity, IdentityContext<T>)> {
			IDGraphs::iter_prefix(who).collect::<Vec<_>>()
		}

		// get the most recent `max_len` elements in IDGraph, sorted by `creation_request_block`
		pub fn get_id_graph_with_max_len(
			who: &T::AccountId,
			max_len: usize,
		) -> Vec<(Identity, IdentityContext<T>)> {
			let mut id_graph = Self::get_id_graph(who);
			id_graph.sort_by(|a, b| {
				Ord::cmp(
					&b.1.creation_request_block.unwrap_or_default(),
					&a.1.creation_request_block.unwrap_or_default(),
				)
			});
			id_graph.truncate(max_len);
			id_graph
		}

		// get count of all keys account + identity in the IDGraphs
		pub fn id_graph_stats() -> Option<Vec<(T::AccountId, u32)>> {
			let mut stats: BTreeMap<T::AccountId, u32> = BTreeMap::new();
			IDGraphs::<T>::iter().for_each(|item| {
				let account = item.0;
				let value = {
					let mut default_value = 0_u32;
					let value = stats.get_mut(&account).unwrap_or(&mut default_value);
					*value += 1;

					*value
				};

				stats.insert(account, value);
			});

			let stats = stats.into_iter().map(|item| (item.0, item.1)).collect::<Vec<_>>();
			debug!("IDGraph stats: {:?}", stats);
			Some(stats)
		}
	}
}
