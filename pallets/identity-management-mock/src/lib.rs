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

// Ensure we're `no_std` when compiling for Wasm.

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

pub type Mrenclave = [u8; 32];
pub type UserShieldingKey = BoundedVec<u8, ConstU32<1024>>;
pub type ChallengeCode = [u8; 6];
pub type Did = BoundedVec<u8, ConstU32<1024>>;
pub(crate) type Metadata = BoundedVec<u8, ConstU32<2048>>;
pub(crate) use primitives::BlockNumber;

mod key;
use key::{encrypt_with_public_key, get_mock_tee_shielding_key, PaddingScheme};

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
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Events from this pallet
		LinkIdentityRequested,
		UnlinkIdentityRequested,
		VerifyIdentityRequested,
		SetShieldingKeyRequested,
		// =============================================
		// Mocked events that should have come from TEE
		// we have both the "plain" version and "encrypted" version for debugging
		// =============================================
		// set user's shielding key
		UserShieldingKeySetPlain { account: AccountId, key: UserShieldingKey },
		UserShieldingKeySetEnc { account: Vec<u8>, key: Vec<u8> },
		// link identity
		ChallengeCodeGeneratedPlain { account: AccountId, code: ChallengeCode },
		ChallengeCodeGeneratedEnc { account: Vec<u8>, code: Vec<u8> },
		IdentityLinkedPlain { account: AccountId, identity: Did },
		IdentityLinkedEnc { account: Vec<u8>, identity: Vec<u8> },
		// unlink identity
		IdentityUnlinkedPlain { account: AccountId, identity: Did },
		IdentityUnlinkedEnc { account: Vec<u8>, identity: Vec<u8> },
		// verify identity
		IdentityVerifiedPlain { account: AccountId, identity: Did },
		IdentityVerifiedEnc { account: Vec<u8>, identity: Vec<u8> },
	}

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
			let caller = ensure_signed(origin.clone())?;
			ensure!(
				WhitelistedCallers::<T>::contains_key(&caller),
				Error::<T>::CallerNotWhitelisted
			);
			Self::deposit_event(Event::<T>::SetShieldingKeyRequested);

			let trusted_call = Self::handle_call_worker_payload(&shard, &encrypted_data)?;
			match trusted_call {
				TrustedCall::set_shielding_key(_root, who, key) => {
					UserShieldingKeys::<T>::insert(&who, &key);
					Self::deposit_event(Event::<T>::UserShieldingKeySetPlain {
						account: who.clone(),
						key: key.clone(),
					});
					Self::deposit_event(Event::<T>::UserShieldingKeySetEnc {
						account: encrypt_with_public_key(&key, who.encode().as_slice()),
						key: encrypt_with_public_key(&key, key.as_slice()),
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
			let caller = ensure_signed(origin.clone())?;
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
					let code: ChallengeCode = [1, 2, 3, 4, 5, 6].into();
					Self::deposit_event(Event::<T>::ChallengeCodeGeneratedPlain {
						account: who.clone(),
						code,
					});
					Self::deposit_event(Event::<T>::ChallengeCodeGeneratedEnc {
						account: encrypt_with_public_key(&key, who.encode().as_slice()),
						code: encrypt_with_public_key(&key, code.as_ref()),
					});

					// emit the IdentityLinked event
					let context =
						IdentityContext { metadata, linking_request_block: bn, is_verified: false };
					IDGraphs::<T>::insert(&who, &did, context);
					Self::deposit_event(Event::<T>::IdentityLinkedPlain {
						account: who.clone(),
						identity: did.clone(),
					});
					Self::deposit_event(Event::<T>::IdentityLinkedEnc {
						account: encrypt_with_public_key(&key, who.encode().as_slice()),
						identity: encrypt_with_public_key(&key, did.as_slice()),
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
			let caller = ensure_signed(origin.clone())?;
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
					Self::deposit_event(Event::<T>::IdentityUnlinkedEnc {
						account: encrypt_with_public_key(&key, who.encode().as_slice()),
						identity: encrypt_with_public_key(&key, did.as_slice()),
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
			let caller = ensure_signed(origin.clone())?;
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
						Self::deposit_event(Event::<T>::IdentityVerifiedEnc {
							account: encrypt_with_public_key(&key, who.encode().as_slice()),
							identity: encrypt_with_public_key(&key, did.as_slice()),
						});
						Ok(().into())
					})
				},
				_ => return Err(Error::<T>::WrongTrustedCallType.into()),
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn handle_call_worker_payload(
			shard: &ShardIdentifier,
			payload: &Vec<u8>,
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
