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
use hex_literal::hex;
pub use pallet::*;
use rsa::{
	pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
	pkcs1v15::{SigningKey, VerifyingKey},
	Hash, PaddingScheme, PublicKey, PublicKeyParts, RsaPrivateKey, RsaPublicKey,
};
use sp_core::H256;
use sp_std::prelude::*;

mod mock_types;
use mock_types::*;

mod identity_context;
use identity_context::IdentityContext;

pub type Mrenclave = [u8; 32];

pub type UserShieldingKey = BoundedVec<u8, ConstU32<1024>>;
pub type ChallengeCode = [u8; 6];
pub type Did = BoundedVec<u8, ConstU32<1024>>;
pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub(crate) type Metadata = BoundedVec<u8, ConstU32<3072>>;

pub(crate) const RSA_2048_PRIV_PEM: &str = include_str!("rsa_key_examples/pkcs1/2048-priv.pem");
pub(crate) const RSA_2048_PUB_PEM: &str = include_str!("rsa_key_examples/pkcs1/2048-pub.pem");

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
		/// enc
		#[pallet::constant]
		type Mrenclave: Get<Mrenclave>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Events from this pallet
		LinkIdentityRequested,
		UnlinkIdentityRequested,
		VerifyIdentityRequested,
		SetShieldingKeyRequested,
		// Mocked events that should have come from TEE
		UserShieldingKeyUpdated { id: Vec<u8>, key: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// caller is not in whitelist (therefore disallowed to call some extrinsics)
		CallerNotWhitelisted,
	}

	#[pallet::storage]
	#[pallet::getter(fn whitelisted_callers)]
	pub type WhitelistedCallers<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserShieldingKey, OptionQuery>;

	/// challenge code is per Litentry account + did
	#[pallet::storage]
	#[pallet::getter(fn challenge_codes)]
	pub type ChallengeCodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
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
		T::AccountId,
		Blake2_128Concat,
		Did,
		IdentityContext<T>,
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
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			ensure!(WhitelistedCallers::<T>::contains_key(who), Error::<T>::CallerNotWhitelisted);

			Self::deposit_event(Event::SetShieldingKeyRequested);
			Ok(().into())
		}

		/// Link an identity
		#[pallet::weight(195_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			ensure!(WhitelistedCallers::<T>::contains_key(who), Error::<T>::CallerNotWhitelisted);

			Self::deposit_event(Event::LinkIdentityRequested);
			Ok(().into())
		}

		/// Unlink an identity
		#[pallet::weight(195_000_000)]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			ensure!(WhitelistedCallers::<T>::contains_key(who), Error::<T>::CallerNotWhitelisted);

			Self::deposit_event(Event::UnlinkIdentityRequested);
			Ok(().into())
		}

		/// Verify a linked identity
		#[pallet::weight(195_000_000)]
		pub fn verify_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			ensure!(WhitelistedCallers::<T>::contains_key(who), Error::<T>::CallerNotWhitelisted);

			Self::deposit_event(Event::VerifyIdentityRequested);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn handle_call_worker_payload(payload: Vec<u8>) {
			/*
			// decrypt
			let decrypted_extrinsic = shielding_key.decrypt(&payload).unwrap();
			// code
			let decoded_operation =
				TrustedOperation::decode(&mut decrypted_extrinsic.as_slice()).unwrap();
			// conver to TrustedCallSigned
			let trusted_call_signed = decoded_operation.to_call().unwrap();
			assert!(trusted_call_signed.verify_signature(&mr_enclave, &shard_id()));
			*/
		}
	}
}
