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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use crate::weights::WeightInfo;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
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
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LinkIdentityRequested,
		UnlinkIdentityRequested,
		VerifyIdentityRequested,
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
	}
}
