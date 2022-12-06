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

// VC management pallet run within TEE(enclave) -- aka VCMT
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

pub use pallet::*;

use frame_support::{pallet_prelude::*, traits::StorageVersion};
use frame_system::pallet_prelude::*;
pub use litentry_primitives::{
	ChallengeCode, Identity, ParentchainBlockNumber, UserShieldingKeyType,
};
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type MetadataOf<T> = BoundedVec<u8, <T as Config>::MaxMetadataLength>;

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the event
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// the manager origin for extrincis
		type ManageOrigin: EnsureOrigin<Self::Origin>;
		/// maximum metadata length
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;
		/// maximum delay in block numbers between linking an identity and verifying an identity
		#[pallet::constant]
		type MaxVerificationDelay: Get<ParentchainBlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VCSchemaAdd { who: T::AccountId, value: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		VCSchemaNotExist,
	}

	#[pallet::storage]
	#[pallet::getter(fn vc_schema)]
	pub type VCSchemas<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(15_000_000)]
		pub fn update_schema_storage(
			origin: OriginFor<T>,
			who: T::AccountId,
			value: u64,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
			<VCSchemas<T>>::insert(who.clone(), value.clone());
			Self::deposit_event(Event::VCSchemaAdd { who, value });
			Ok(())
		}
	}
}
