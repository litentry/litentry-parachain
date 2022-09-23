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

#[cfg(test)]
mod tests;

pub mod weights;

pub use crate::weights::WeightInfo;
pub use pallet::*;

mod key;
pub use key::*;

use sp_core::H256;
use sp_std::vec::Vec;

// enclave related types
pub type ShardIdentifier = H256;
pub type MrenclaveType = [u8; 32];

// fn types for handling inside tee-worker
pub type SetUserShieldingKeyFn = ([u8; 2], ShardIdentifier, Vec<u8>);
pub type LinkIdentityFn = ([u8; 2], ShardIdentifier, Vec<u8>, Option<Vec<u8>>);
pub type UnlinkIdentityFn = ([u8; 2], ShardIdentifier, Vec<u8>);
pub type VerifyIdentityFn = ([u8; 2], ShardIdentifier, Vec<u8>, Vec<u8>);

#[frame_support::pallet]
pub mod pallet {
	use super::{AesOutput, ShardIdentifier, Vec, WeightInfo};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
		// the origin allowed to call event-triggering extrinsics, normally TEE
		type EventTriggerOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO: do we need account as event parameter? This needs to be decided by F/E
		LinkIdentityRequested { shard: ShardIdentifier },
		UnlinkIdentityRequested { shard: ShardIdentifier },
		VerifyIdentityRequested { shard: ShardIdentifier },
		SetUserShieldingKeyRequested { shard: ShardIdentifier },
		// event that should be triggered by TriggerEventOrigin
		UserShieldingKeySet { account: AesOutput },
		ChallengeCodeGenerated { account: AesOutput, identity: AesOutput, code: AesOutput },
		IdentityLinked { account: AesOutput, identity: AesOutput },
		IdentityUnlinked { account: AesOutput, identity: AesOutput },
		IdentityVerified { account: AesOutput, identity: AesOutput },
		// some error happened during processing in TEE, we use string-like
		// parameters for more "generic" error event reporting
		// TODO: maybe use concrete errors instead of events when we are more sure
		// see also the comment at the beginning
		SomeError { func: Vec<u8>, error: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The extrinsic used to verify the response from TEE side
		/// Keep it until HTTPS test on tee-worker refactored
		#[pallet::weight(1_000)]
		pub fn dummy(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			log::error!(
				target: "identity management",
				"dummy called."
			);
			Ok(())
		}

		/// Set or update user's shielding key
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

		/// Link an identity
		#[pallet::weight(<T as Config>::WeightInfo::link_identity())]
		pub fn link_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
			encrypted_metadata: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::LinkIdentityRequested { shard });
			Ok(().into())
		}

		/// Unlink an identity
		#[pallet::weight(<T as Config>::WeightInfo::unlink_identity())]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::UnlinkIdentityRequested { shard });
			Ok(().into())
		}

		/// Verify a linked identity
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
		#[pallet::weight(195_000_000)]
		pub fn user_shielding_key_set(
			origin: OriginFor<T>,
			account: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::UserShieldingKeySet { account });
			Ok(Pays::No.into())
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
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_linked(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::EventTriggerOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityLinked { account, identity });
			Ok(Pays::No.into())
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
}
