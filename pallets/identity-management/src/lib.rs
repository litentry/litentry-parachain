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

#![cfg_attr(not(feature = "std"), no_std)]

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

#[frame_support::pallet]
pub mod pallet {
	use crate::{weights::WeightInfo, AesOutput};
	use frame_support::{
		dispatch::{DispatchErrorWithPostInfo, Dispatchable, PostDispatchInfo},
		pallet_prelude::*,
		traits::IsSubType,
		weights::GetDispatchInfo,
	};
	use frame_system::pallet_prelude::*;
	use sp_core::H256;
	use sp_std::prelude::*;
	use teerex_primitives::Request;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_teerex::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: Parameter
			+ Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ From<pallet_teerex::Call<Self>>
			+ IsSubType<Call<Self>>;
		type WeightInfo: WeightInfo;
		// the origin allowed to call event-triggering extrinsics, normally TEE
		type EventTriggerOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LinkIdentityRequested,
		UnlinkIdentityRequested,
		VerifyIdentityRequested,
		SetUserShieldingKeyRequested,
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
		/// Link an identity
		#[pallet::weight(<T as Config>::WeightInfo::link_identity())]
		pub fn link_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin.clone())?;

			// Forward encrypted call to worker via teerex
			let request = Request { shard, cyphertext: encrypted_data };
			let call: <T as Config>::Call = pallet_teerex::Call::call_worker { request }.into();
			let result = call.dispatch(origin);

			Self::deposit_event(Event::LinkIdentityRequested);

			// Parse dispatch result and update weight and error information
			result
				.map(|post_dispatch_info| {
					post_dispatch_info
						.actual_weight
						.map(|actual_weight| {
							<T as Config>::WeightInfo::link_identity().saturating_add(actual_weight)
						})
						.into()
				})
				.map_err(|err| match err.post_info.actual_weight {
					Some(actual_weight) => {
						let weight_used = <T as Config>::WeightInfo::link_identity()
							.saturating_add(actual_weight);
						let post_info = Some(weight_used).into();
						DispatchErrorWithPostInfo { post_info, error: err.error }
					},
					None => err,
				})
		}

		/// Unlink an identity
		#[pallet::weight(<T as Config>::WeightInfo::unlink_identity())]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin.clone())?;

			// Forward encrypted call to worker via teerex
			let request = Request { shard, cyphertext: encrypted_data };
			let call: <T as Config>::Call = pallet_teerex::Call::call_worker { request }.into();
			let result = call.dispatch(origin);

			Self::deposit_event(Event::UnlinkIdentityRequested);

			// Parse dispatch result and update weight and error information
			result
				.map(|post_dispatch_info| {
					post_dispatch_info
						.actual_weight
						.map(|actual_weight| {
							<T as Config>::WeightInfo::unlink_identity()
								.saturating_add(actual_weight)
						})
						.into()
				})
				.map_err(|err| match err.post_info.actual_weight {
					Some(actual_weight) => {
						let weight_used = <T as Config>::WeightInfo::unlink_identity()
							.saturating_add(actual_weight);
						let post_info = Some(weight_used).into();
						DispatchErrorWithPostInfo { post_info, error: err.error }
					},
					None => err,
				})
		}

		/// Verify a linked identity
		#[pallet::weight(<T as Config>::WeightInfo::verify_identity())]
		pub fn verify_identity(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin.clone())?;

			// Forward encrypted call to worker via teerex
			let request = Request { shard, cyphertext: encrypted_data };
			let call: <T as Config>::Call = pallet_teerex::Call::call_worker { request }.into();
			let result = call.dispatch(origin);

			Self::deposit_event(Event::VerifyIdentityRequested);

			// Parse dispatch result and update weight and error information
			result
				.map(|post_dispatch_info| {
					post_dispatch_info
						.actual_weight
						.map(|actual_weight| {
							<T as Config>::WeightInfo::verify_identity()
								.saturating_add(actual_weight)
						})
						.into()
				})
				.map_err(|err| match err.post_info.actual_weight {
					Some(actual_weight) => {
						let weight_used = <T as Config>::WeightInfo::verify_identity()
							.saturating_add(actual_weight);
						let post_info = Some(weight_used).into();
						DispatchErrorWithPostInfo { post_info, error: err.error }
					},
					None => err,
				})
		}

		/// Set or update user's shielding key
		#[pallet::weight(<T as Config>::WeightInfo::set_user_shielding_key())]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			shard: H256,
			encrypted_data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin.clone())?;

			// Forward encrypted call to worker via teerex
			let request = Request { shard, cyphertext: encrypted_data };
			let call: <T as Config>::Call = pallet_teerex::Call::call_worker { request }.into();
			let result = call.dispatch(origin);

			Self::deposit_event(Event::SetUserShieldingKeyRequested);

			// Parse dispatch result and update weight and error information
			result
				.map(|post_dispatch_info| {
					post_dispatch_info
						.actual_weight
						.map(|actual_weight| {
							<T as Config>::WeightInfo::set_user_shielding_key()
								.saturating_add(actual_weight)
						})
						.into()
				})
				.map_err(|err| match err.post_info.actual_weight {
					Some(actual_weight) => {
						let weight_used = <T as Config>::WeightInfo::set_user_shielding_key()
							.saturating_add(actual_weight);
						let post_info = Some(weight_used).into();
						DispatchErrorWithPostInfo { post_info, error: err.error }
					},
					None => err,
				})
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
}
