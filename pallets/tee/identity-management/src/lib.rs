// Copyright 2020-2022 Litentry Technologies GmbH.
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
// - origin management, only allow calls from TEE (= origin is signed with the ECC key)
// - maybe don't emit events at all, or at least remove sensistive data
// - benchmarking

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
pub mod identity;
pub mod identity_context;

use frame_support::{pallet_prelude::*, traits::StorageVersion};
use frame_system::pallet_prelude::*;
use identity_context::IdentityContext;

pub type UserShieldingKeyOf<T> = BoundedVec<u8, <T as Config>::UserShieldingKeyLength>;
pub type ChallengeCodeOf<T> = <T as Config>::ChallengeCode;
pub type DidOf<T> = BoundedVec<u8, <T as Config>::MaxDidLength>;
pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub(crate) type MetadataOf<T> = BoundedVec<u8, <T as Config>::MaxMetadataLength>;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the event
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// challenge code type
		type ChallengeCode: Member + Parameter + Default + Copy + MaxEncodedLen;
		/// constant for the user shielding key length
		#[pallet::constant]
		type UserShieldingKeyLength: Get<u32>;
		/// maximum did length
		#[pallet::constant]
		type MaxDidLength: Get<u32>;
		/// maximum metadata length
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
		/// maximum delay in block numbers between linking an identity and verifying an identity
		#[pallet::constant]
		type MaxVerificationDelay: Get<BlockNumberOf<Self>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// user shielding key was set
		UserShieldingKeySet { who: T::AccountId, key: UserShieldingKeyOf<T> },
		/// challenge code was set
		ChallengeCodeSet { who: T::AccountId, did: DidOf<T>, code: ChallengeCodeOf<T> },
		/// challenge code was removed
		ChallengeCodeRemoved { who: T::AccountId, did: DidOf<T> },
		/// an identity was linked
		IdentityLinked { who: T::AccountId, did: DidOf<T> },
		/// an identity was removed
		IdentityUnlinked { who: T::AccountId, did: DidOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the shielding key length is invalid
		InvalidUserShieldingKeyLength,
		/// challenge code doesn't exist
		ChallengeCodeNotExist,
		/// the pair (litentry-account, did) already exists
		IdentityAlreadyExist,
		/// the pair (litentry-account, did) doesn't exist
		IdentityNotExist,
		/// a verification reqeust comes too early
		VerificationRequestTooEarly,
		/// a verification reqeust comes too late
		VerificationRequestTooLate,
	}

	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_256, T::AccountId, UserShieldingKeyOf<T>, OptionQuery>;

	/// challenge code is per Litentry account + did
	#[pallet::storage]
	#[pallet::getter(fn challenge_codes)]
	pub type ChallengeCodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_256,
		T::AccountId,
		Blake2_256,
		DidOf<T>,
		ChallengeCodeOf<T>,
		OptionQuery,
	>;

	/// ID graph is per Litentry account + did
	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_256,
		T::AccountId,
		Blake2_256,
		DidOf<T>,
		IdentityContext<T>,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(15_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			who: T::AccountId,
			key: UserShieldingKeyOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(
				key.len() == T::UserShieldingKeyLength::get() as usize,
				Error::<T>::InvalidUserShieldingKeyLength
			);
			// we don't care about the current key
			UserShieldingKeys::<T>::insert(&who, &key);
			Self::deposit_event(Event::UserShieldingKeySet { who, key });
			Ok(())
		}

		#[pallet::weight(15_000_000)]
		pub fn set_challenge_code(
			origin: OriginFor<T>,
			who: T::AccountId,
			did: DidOf<T>,
			code: ChallengeCodeOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// we don't care if it has already associated with any challenge code
			ChallengeCodes::<T>::insert(&who, &did, &code);
			Self::deposit_event(Event::ChallengeCodeSet { who, did, code });
			Ok(())
		}

		#[pallet::weight(15_000_000)]
		pub fn remove_challenge_code(
			origin: OriginFor<T>,
			who: T::AccountId,
			did: DidOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(
				ChallengeCodes::<T>::contains_key(&who, &did),
				Error::<T>::ChallengeCodeNotExist
			);
			ChallengeCodes::<T>::remove(&who, &did);
			Self::deposit_event(Event::ChallengeCodeRemoved { who, did });
			Ok(())
		}

		#[pallet::weight(15_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			did: DidOf<T>,
			metadata: Option<MetadataOf<T>>,
			linking_request_block: BlockNumberOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(!IDGraphs::<T>::contains_key(&who, &did), Error::<T>::IdentityAlreadyExist);
			let context = IdentityContext { metadata, linking_request_block, is_verified: false };
			IDGraphs::<T>::insert(&who, &did, context);
			Self::deposit_event(Event::IdentityLinked { who, did });
			Ok(())
		}

		#[pallet::weight(15_000_000)]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			did: DidOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &did), Error::<T>::IdentityNotExist);
			IDGraphs::<T>::remove(&who, &did);
			Self::deposit_event(Event::IdentityUnlinked { who, did });
			Ok(())
		}

		#[pallet::weight(15_000_000)]
		pub fn verify_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			did: DidOf<T>,
			verification_request_block: BlockNumberOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			IDGraphs::<T>::try_mutate(&who, &did, |context| -> DispatchResult {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				ensure!(
					c.linking_request_block <= verification_request_block,
					Error::<T>::VerificationRequestTooEarly
				);
				ensure!(
					verification_request_block - c.linking_request_block <=
						T::MaxVerificationDelay::get(),
					Error::<T>::VerificationRequestTooLate
				);
				c.is_verified = true;
				*context = Some(c);
				Ok(())
			})
		}
	}
}
