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

//! TODO: event/error handling
//! Currently the errors are synchronously emitted from this pallet itself,
//! meanwhile we have the `SomeError` **Event** which is callable from TEE
//! to represent any generic "error".
//! However, there are so many error cases in TEE that I'm not even sure
//! if it's a good idea to have a matching extrinsic for error propagation.
//!
//! The reasons that we don't use pallet_teerex::call_worker directly are:
//! - call teerex::call_worker inside IMP won't trigger the handler, because it's not called as
//!   extrinsics so won't be scraped
//! - the origin is discarded in call_worker but we need it
//! - to simplify the F/E usage, we only need to encrypt the needed parameters (see e.g.
//!   shield_funds)

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;

#[cfg(all(test, feature = "skip-ias-check"))]
mod tests;

pub mod weights;

pub use crate::weights::WeightInfo;
pub use pallet::*;

pub use core_primitives::{AesOutput, ShardIdentifier};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::{AesOutput, ShardIdentifier, Vec, WeightInfo};
	use core_primitives::{ErrorString, IMPError};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		// some extrinsics should only be called by origins from TEE
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// origin to manage authorised delegatee list
		type DelegateeAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		DelegateeAdded {
			account: T::AccountId,
		},
		DelegateeRemoved {
			account: T::AccountId,
		},
		// TODO: do we need account as event parameter? This needs to be decided by F/E
		CreateIdentityRequested {
			shard: ShardIdentifier,
		},
		RemoveIdentityRequested {
			shard: ShardIdentifier,
		},
		VerifyIdentityRequested {
			shard: ShardIdentifier,
		},
		SetUserShieldingKeyRequested {
			shard: ShardIdentifier,
		},
		// event that should be triggered by TEECallOrigin
		// these events keep the `account` as public to be consistent with VCMP and better
		// indexing see https://github.com/litentry/litentry-parachain/issues/1313
		UserShieldingKeySet {
			account: T::AccountId,
		},
		IdentityCreated {
			account: T::AccountId,
			identity: AesOutput,
			code: AesOutput,
			id_graph: AesOutput,
		},
		IdentityRemoved {
			account: T::AccountId,
			identity: AesOutput,
			id_graph: AesOutput,
		},
		IdentityVerified {
			account: T::AccountId,
			identity: AesOutput,
			id_graph: AesOutput,
		},
		// event errors caused by processing in TEE
		// copied from core_primitives::IMPError, we use events instead of pallet::errors,
		// see https://github.com/litentry/litentry-parachain/issues/1275
		DecodeHexFailed {
			reason: ErrorString,
		},
		HttpRequestFailed {
			reason: ErrorString,
		},
		CreateIdentityHandlingFailed,
		RemoveIdentityHandlingFailed,
		VerifyIdentityHandlingFailed,
		SetUserShieldingKeyHandlingFailed,
		InvalidIdentity,
		WrongWeb2Handle,
		UnexpectedMessage,
		WrongIdentityHandleType,
		WrongSignatureType,
		VerifySubstrateSignatureFailed,
		RecoverSubstratePubkeyFailed,
		VerifyEvmSignatureFailed,
		RecoverEvmAddressFailed,
	}

	/// delegatees who are authorised to send extrinsics(currently only `create_identity`)
	/// on behalf of the users
	#[pallet::storage]
	#[pallet::getter(fn delegatee)]
	pub type Delegatee<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// a delegatee doesn't exist
		DelegateeNotExist,
		/// a `create_identity` request from unauthorised user
		UnauthorisedUser,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add an account to the delegatees
		#[pallet::call_index(0)]
		#[pallet::weight(195_000_000)]
		pub fn add_delegatee(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _ = T::DelegateeAdminOrigin::ensure_origin(origin)?;
			// we don't care if `account` already exists
			Delegatee::<T>::insert(account.clone(), ());
			Self::deposit_event(Event::DelegateeAdded { account });
			Ok(())
		}

		/// remove an account from the delegatees
		#[pallet::call_index(1)]
		#[pallet::weight(195_000_000)]
		pub fn remove_delegatee(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _ = T::DelegateeAdminOrigin::ensure_origin(origin)?;
			ensure!(Delegatee::<T>::contains_key(&account), Error::<T>::DelegateeNotExist);
			Delegatee::<T>::remove(account.clone());
			Self::deposit_event(Event::DelegateeRemoved { account });
			Ok(())
		}

		/// Set or update user's shielding key
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::set_user_shielding_key())]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_key: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::SetUserShieldingKeyRequested { shard });
			Ok(().into())
		}

		/// Create an identity
		/// We do the origin check for this extrinsic, it has to be
		/// - either the caller him/herself, i.e. ensure_signed(origin)? == who
		/// - or from a delegatee in the list
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::create_identity())]
		pub fn create_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			user: T::AccountId,
			encrypted_identity: Vec<u8>,
			encrypted_metadata: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(
				who == user || Delegatee::<T>::contains_key(&who),
				Error::<T>::UnauthorisedUser
			);
			Self::deposit_event(Event::CreateIdentityRequested { shard });
			Ok(().into())
		}

		/// Remove an identity
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::remove_identity())]
		pub fn remove_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::RemoveIdentityRequested { shard });
			Ok(().into())
		}

		/// Verify an identity
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::verify_identity())]
		pub fn verify_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
			encrypted_validation_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::VerifyIdentityRequested { shard });
			Ok(().into())
		}

		/// ---------------------------------------------------
		/// The following extrinsics are supposed to be called by TEE only
		/// ---------------------------------------------------
		#[pallet::call_index(6)]
		#[pallet::weight(195_000_000)]
		pub fn user_shielding_key_set(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::UserShieldingKeySet { account });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(195_000_000)]
		pub fn identity_created(
			origin: OriginFor<T>,
			account: T::AccountId,
			identity: AesOutput,
			code: AesOutput,
			id_graph: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityCreated { account, identity, code, id_graph });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(195_000_000)]
		pub fn identity_removed(
			origin: OriginFor<T>,
			account: T::AccountId,
			identity: AesOutput,
			id_graph: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityRemoved { account, identity, id_graph });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(9)]
		#[pallet::weight(195_000_000)]
		pub fn identity_verified(
			origin: OriginFor<T>,
			account: T::AccountId,
			identity: AesOutput,
			id_graph: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityVerified { account, identity, id_graph });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(195_000_000)]
		pub fn some_error(origin: OriginFor<T>, error: IMPError) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			match error {
				IMPError::DecodeHexFailed(s) =>
					Self::deposit_event(Event::DecodeHexFailed { reason: s }),
				IMPError::HttpRequestFailed(s) =>
					Self::deposit_event(Event::HttpRequestFailed { reason: s }),
				IMPError::InvalidIdentity => Self::deposit_event(Event::InvalidIdentity),
				IMPError::CreateIdentityHandlingFailed =>
					Self::deposit_event(Event::CreateIdentityHandlingFailed),
				IMPError::RemoveIdentityHandlingFailed =>
					Self::deposit_event(Event::RemoveIdentityHandlingFailed),
				IMPError::VerifyIdentityHandlingFailed =>
					Self::deposit_event(Event::VerifyIdentityHandlingFailed),
				IMPError::SetUserShieldingKeyHandlingFailed =>
					Self::deposit_event(Event::SetUserShieldingKeyHandlingFailed),
				IMPError::WrongWeb2Handle => Self::deposit_event(Event::WrongWeb2Handle),
				IMPError::UnexpectedMessage => Self::deposit_event(Event::UnexpectedMessage),
				IMPError::WrongIdentityHandleType =>
					Self::deposit_event(Event::WrongIdentityHandleType),
				IMPError::WrongSignatureType => Self::deposit_event(Event::WrongSignatureType),
				IMPError::VerifySubstrateSignatureFailed =>
					Self::deposit_event(Event::VerifySubstrateSignatureFailed),
				IMPError::RecoverSubstratePubkeyFailed =>
					Self::deposit_event(Event::RecoverSubstratePubkeyFailed),
				IMPError::VerifyEvmSignatureFailed =>
					Self::deposit_event(Event::VerifyEvmSignatureFailed),
				IMPError::RecoverEvmAddressFailed =>
					Self::deposit_event(Event::RecoverEvmAddressFailed),
			}
			Ok(Pays::No.into())
		}
	}
}
