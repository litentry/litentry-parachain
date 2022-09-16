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

//! This is a mock to pallet-identity-management on parachain (IMP).
//! It hides/mocks all things happened on TEE side and returns the
//! result immediately.
//!
//! The idea is to give F/E an idea how the interface(extrinsic) would
//! look like and what kind of events can be expected.
//!
//! TODO: event/error handling
//! Currently the errors are synchronously emitted from this pallet itself,
//! meanwhile we have the `SomeError` **Event** which is callable from TEE
//! to represent any generic "error".
//! However, there are so many error cases in TEE that I'm not even sure
//! if it's a good idea to have a matching extrinsic for error propagation.

#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{
	dispatch::{Dispatchable, PostDispatchInfo},
	pallet_prelude::*,
	traits::{ConstU32, IsSubType},
	weights::GetDispatchInfo,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

mod stf_primitives;
use stf_primitives::{AccountId, ShardIdentifier, TrustedCall, TrustedOperation};

mod identity_context;
use identity_context::IdentityContext;

mod key;
use key::{
	aes_encrypt_default, get_mock_tee_shielding_key, AesOutput, PaddingScheme,
	USER_SHIELDING_KEY_LEN,
};

pub type Mrenclave = [u8; 32];
pub type UserShieldingKey = [u8; USER_SHIELDING_KEY_LEN];
pub type ChallengeCode = [u8; 6];
pub type Did = BoundedVec<u8, ConstU32<1024>>;
pub(crate) type Metadata = BoundedVec<u8, ConstU32<2048>>;
pub(crate) use primitives::BlockNumber;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Event
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Call
		type Call: Parameter
			+ Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ IsSubType<Call<Self>>;
		/// origin to manage caller whitelist
		type ManageWhitelistOrigin: EnsureOrigin<Self::Origin>;
		/// basically the mocked enclave hash
		#[pallet::constant]
		type Mrenclave: Get<Mrenclave>;
		// maximum delay in block numbers between linking an identity and verifying an identity
		#[pallet::constant]
		type MaxVerificationDelay: Get<BlockNumber>;
		// the origin allowed to call event-triggering extrinsics, normally TEE
		type EventTriggerOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Events from this pallet
		LinkIdentityRequested,
		UnlinkIdentityRequested,
		VerifyIdentityRequested,
		SetUserShieldingKeyRequested,
		// =============================================
		// Mocked events that should have come from TEE
		// we have both the "plain" version and "encrypted" version for debugging
		// =============================================
		// set user's shielding key
		UserShieldingKeySetPlain { account: AccountId },
		UserShieldingKeySet { account: AesOutput },
		// link identity
		ChallengeCodeGeneratedPlain { account: AccountId, identity: Did, code: ChallengeCode },
		ChallengeCodeGenerated { account: AesOutput, identity: AesOutput, code: AesOutput },
		IdentityLinkedPlain { account: AccountId, identity: Did },
		IdentityLinked { account: AesOutput, identity: AesOutput },
		// unlink identity
		IdentityUnlinkedPlain { account: AccountId, identity: Did },
		IdentityUnlinked { account: AesOutput, identity: AesOutput },
		// verify identity
		IdentityVerifiedPlain { account: AccountId, identity: Did },
		IdentityVerified { account: AesOutput, identity: AesOutput },
		// some error happened during processing in TEE, we use string-like
		// parameters for more "generic" error event reporting
		// TODO: maybe use concrete errors instead of events when we are more sure
		// see also the comment at the beginning
		SomeError { func: Vec<u8>, error: Vec<u8> },
	}

	/// These are the errors that are immediately emitted from this mock pallet
	#[pallet::error]
	pub enum Error<T> {
		/// caller is not in whitelist (therefore disallowed to call some extrinsics)
		CallerNotWhitelisted,
		/// Error when decrypting using TEE'shielding key
		ShieldingKeyDecryptionFailed,
		/// Error when decoding trusted operation
		TrustedOperationDecodingFailed,
		/// Error when converting to trusted call
		TrustedCallConversionFailed,
		/// unexpected TrustedOperation type
		WrongTrustedOperationType,
		/// unexpected TrustedCall type
		WrongTrustedCallType,
		/// error when verifying trusted call signature
		TrustedCallBadSignature,
		/// identity already exists when linking an identity
		IdentityAlreadyExist,
		/// identity not exist when unlinking an identity
		IdentityNotExist,
		/// no shielding key for a given AccountId
		ShieldingKeyNotExist,
		/// a verification reqeust comes too early
		VerificationRequestTooEarly,
		/// a verification reqeust comes too late
		VerificationRequestTooLate,
	}

	#[pallet::storage]
	#[pallet::getter(fn whitelisted_callers)]
	pub type WhitelistedCallers<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

	/// We use stf_primitives::AccountId instead of T::AcoundId as key for the TEE-releated storages
	/// - for simplicity
	/// - fo
	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountId, UserShieldingKey, OptionQuery>;

	/// challenge code is per Litentry account + did
	#[pallet::storage]
	#[pallet::getter(fn challenge_codes)]
	pub type ChallengeCodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountId,
		Blake2_128Concat,
		Did,
		ChallengeCode,
		OptionQuery,
	>;

	/// ID graph is per Litentry account + did
	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountId,
		Blake2_128Concat,
		Did,
		IdentityContext,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add an account to the whitelist
		#[pallet::weight(195_000_000)]
		pub fn add_to_whitelist(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _ = T::ManageWhitelistOrigin::ensure_origin(origin)?;
			WhitelistedCallers::<T>::insert(account, ());
			Ok(().into())
		}

		/// remove an account from the whitelist
		#[pallet::weight(195_000_000)]
		pub fn remove_from_whitelist(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _ = T::ManageWhitelistOrigin::ensure_origin(origin)?;
			WhitelistedCallers::<T>::remove(account);
			Ok(().into())
		}

		/// Set or update user's shielding key
		#[pallet::weight(195_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				WhitelistedCallers::<T>::contains_key(&caller),
				Error::<T>::CallerNotWhitelisted
			);
			Self::deposit_event(Event::<T>::SetUserShieldingKeyRequested);

			let trusted_call = Self::handle_call_worker_payload(&shard, &encrypted_data)?;
			match trusted_call {
				TrustedCall::set_user_shielding_key(_root, who, key) => {
					UserShieldingKeys::<T>::insert(&who, &key);
					Self::deposit_event(Event::<T>::UserShieldingKeySetPlain {
						account: who.clone(),
					});
					Self::deposit_event(Event::<T>::UserShieldingKeySet {
						account: aes_encrypt_default(&key, who.encode().as_slice()),
					});
				},
				_ => return Err(Error::<T>::WrongTrustedCallType.into()),
			};
			Ok(().into())
		}

		/// Link an identity
		#[pallet::weight(195_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				WhitelistedCallers::<T>::contains_key(&caller),
				Error::<T>::CallerNotWhitelisted
			);
			Self::deposit_event(Event::LinkIdentityRequested);
			let trusted_call = Self::handle_call_worker_payload(&shard, &encrypted_data)?;
			match trusted_call {
				TrustedCall::link_identity(_root, who, did, metadata, bn) => {
					ensure!(
						!IDGraphs::<T>::contains_key(&who, &did),
						Error::<T>::IdentityAlreadyExist
					);
					let key = UserShieldingKeys::<T>::get(&who)
						.ok_or(Error::<T>::ShieldingKeyNotExist)?;

					// emit the challenge code event
					let code: ChallengeCode = [1, 2, 3, 4, 5, 6];
					ChallengeCodes::<T>::insert(&who, &did, &code);
					Self::deposit_event(Event::<T>::ChallengeCodeGeneratedPlain {
						account: who.clone(),
						identity: did.clone(),
						code,
					});
					Self::deposit_event(Event::<T>::ChallengeCodeGenerated {
						account: aes_encrypt_default(&key, who.encode().as_slice()),
						identity: aes_encrypt_default(&key, did.as_slice()),
						code: aes_encrypt_default(&key, code.as_ref()),
					});

					// emit the IdentityLinked event
					let context =
						IdentityContext { metadata, linking_request_block: bn, is_verified: false };
					IDGraphs::<T>::insert(&who, &did, context);
					Self::deposit_event(Event::<T>::IdentityLinkedPlain {
						account: who.clone(),
						identity: did.clone(),
					});
					Self::deposit_event(Event::<T>::IdentityLinked {
						account: aes_encrypt_default(&key, who.encode().as_slice()),
						identity: aes_encrypt_default(&key, did.as_slice()),
					});
				},
				_ => return Err(Error::<T>::WrongTrustedCallType.into()),
			};
			Ok(().into())
		}

		/// Unlink an identity
		#[pallet::weight(195_000_000)]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				WhitelistedCallers::<T>::contains_key(&caller),
				Error::<T>::CallerNotWhitelisted
			);
			Self::deposit_event(Event::UnlinkIdentityRequested);
			let trusted_call = Self::handle_call_worker_payload(&shard, &encrypted_data)?;
			match trusted_call {
				TrustedCall::unlink_identity(_root, who, did) => {
					ensure!(IDGraphs::<T>::contains_key(&who, &did), Error::<T>::IdentityNotExist);
					let key = UserShieldingKeys::<T>::get(&who)
						.ok_or(Error::<T>::ShieldingKeyNotExist)?;

					// emit the IdentityUnlinked event
					IDGraphs::<T>::remove(&who, &did);
					Self::deposit_event(Event::<T>::IdentityUnlinkedPlain {
						account: who.clone(),
						identity: did.clone(),
					});
					Self::deposit_event(Event::<T>::IdentityUnlinked {
						account: aes_encrypt_default(&key, who.encode().as_slice()),
						identity: aes_encrypt_default(&key, did.as_slice()),
					});
				},
				_ => return Err(Error::<T>::WrongTrustedCallType.into()),
			};
			Ok(().into())
		}

		/// Verify a linked identity
		#[pallet::weight(195_000_000)]
		pub fn verify_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				WhitelistedCallers::<T>::contains_key(&caller),
				Error::<T>::CallerNotWhitelisted
			);
			Self::deposit_event(Event::VerifyIdentityRequested);
			let trusted_call = Self::handle_call_worker_payload(&shard, &encrypted_data)?;
			match trusted_call {
				TrustedCall::verify_identity(_root, who, did, verification_request_block) => {
					ensure!(IDGraphs::<T>::contains_key(&who, &did), Error::<T>::IdentityNotExist);
					let key = UserShieldingKeys::<T>::get(&who)
						.ok_or(Error::<T>::ShieldingKeyNotExist)?;

					IDGraphs::<T>::try_mutate(&who, &did, |context| -> DispatchResultWithPostInfo {
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
						// emit the IdentityVerified event
						Self::deposit_event(Event::<T>::IdentityVerifiedPlain {
							account: who.clone(),
							identity: did.clone(),
						});
						Self::deposit_event(Event::<T>::IdentityVerified {
							account: aes_encrypt_default(&key, who.encode().as_slice()),
							identity: aes_encrypt_default(&key, did.as_slice()),
						});
						Ok(().into())
					})
				},
				_ => Err(Error::<T>::WrongTrustedCallType.into()),
			}
		}

		// The following extrinsics are supposed to be called by TEE only
		#[pallet::weight(195_000_000)]
		pub fn user_shielding_key_set(
			origin: OriginFor<T>,
			account: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::UserShieldingKeySet { account });
			Ok(().into())
		}

		#[pallet::weight(195_000_000)]
		pub fn challenge_code_generated(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
			code: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::ChallengeCodeGenerated { account, identity, code });
			Ok(().into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_linked(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityLinked { account, identity });
			Ok(().into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_unlinked(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityUnlinked { account, identity });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_verified(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityVerified { account, identity });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn some_error(
			origin: OriginFor<T>,
			func: Vec<u8>,
			error: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::SomeError { func, error });
			Ok(Pays::No.into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn handle_call_worker_payload(
			shard: &ShardIdentifier,
			payload: &[u8],
		) -> Result<TrustedCall, DispatchError> {
			// decrypt using TEE's shielding key
			let (_, private_key) = get_mock_tee_shielding_key();
			let decrypted_extrinsic = private_key
				.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), payload)
				.map_err(|_| Error::<T>::ShieldingKeyDecryptionFailed)?;
			// decode
			let decoded_operation = TrustedOperation::decode(&mut decrypted_extrinsic.as_slice())
				.map_err(|_| Error::<T>::TrustedOperationDecodingFailed)?;
			// convert to TrustedCallSigned
			let trusted_call_signed = match decoded_operation {
				TrustedOperation::indirect_call(call) => call,
				_ => return Err(Error::<T>::WrongTrustedOperationType.into()),
			};
			// double-check the signature
			ensure!(
				trusted_call_signed.verify_signature(&<T as Config>::Mrenclave::get(), shard),
				Error::<T>::TrustedCallBadSignature
			);
			Ok(trusted_call_signed.call)
		}
	}
}
