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
pub use litentry_primitives::{ParentchainBlockNumber, Status, VCSchema};
pub use parentchain_primitives::{
	SchemaContentString, SchemaIdString, SchemaIndex, SCHEMA_CONTENT_LEN, SCHEMA_ID_LEN,
};
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type MetadataOf<T> = BoundedVec<u8, <T as Config>::MaxMetadataLength>;

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
		SchemaAdd { who: T::AccountId, index: SchemaIndex },
		SchemaDisabled { who: T::AccountId, index: SchemaIndex },
		SchemaActivated { who: T::AccountId, index: SchemaIndex },
		SchemaRevoked { who: T::AccountId, index: SchemaIndex },
	}

	#[pallet::error]
	pub enum Error<T> {
		SchemaNotExist,
		SchemaAlreadyDisabled,
		SchemaAlreadyActivated,
		SchemaIndexOverFlow,
	}

	/// Number of schemas
	#[pallet::storage]
	#[pallet::getter(fn schema_count)]
	pub type SchemaCount<T: Config> = StorageValue<_, SchemaIndex, ValueQuery>;

	/// schema: key is SchemaIndex, vaule is VCSchema
	#[pallet::storage]
	#[pallet::getter(fn schema_registry)]
	pub type SchemaRegistry<T: Config> = StorageMap<_, Blake2_256, SchemaIndex, VCSchema>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(15_000_000)]
		pub fn add_schema(
			origin: OriginFor<T>,
			who: T::AccountId,
			id: SchemaIdString,
			content: SchemaContentString,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			let index = Self::schema_count();
			let new_index = index.checked_add(1u64).ok_or(Error::<T>::SchemaIndexOverFlow);
			if new_index.is_ok() {
				<SchemaCount<T>>::put(new_index.as_ref().unwrap());
			}

			SchemaRegistry::<T>::insert(index, VCSchema::new(id.clone(), content.clone()));
			Self::deposit_event(Event::SchemaAdd { who, index });
			Ok(())
		}

		#[pallet::weight(195_000_000)]
		pub fn disable_schema(
			origin: OriginFor<T>,
			who: T::AccountId,
			index: SchemaIndex,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			SchemaRegistry::<T>::try_mutate(index, |context| {
				let mut c = context.take().ok_or(Error::<T>::SchemaNotExist)?;
				ensure!(c.status == Status::Active, Error::<T>::SchemaAlreadyDisabled);
				c.status = Status::Disabled;
				*context = Some(c);
				Self::deposit_event(Event::SchemaDisabled { who, index });
				Ok(())
			})
		}

		#[pallet::weight(195_000_000)]
		pub fn activate_schema(
			origin: OriginFor<T>,
			who: T::AccountId,
			index: SchemaIndex,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			SchemaRegistry::<T>::try_mutate(index, |context| {
				let mut c = context.take().ok_or(Error::<T>::SchemaNotExist)?;
				ensure!(c.status == Status::Disabled, Error::<T>::SchemaAlreadyActivated);
				c.status = Status::Active;
				*context = Some(c);
				Self::deposit_event(Event::SchemaActivated { who, index });
				Ok(())
			})
		}

		#[pallet::weight(195_000_000)]
		pub fn revoke_schema(
			origin: OriginFor<T>,
			who: T::AccountId,
			index: SchemaIndex,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;

			let _ = SchemaRegistry::<T>::get(index).ok_or(Error::<T>::SchemaNotExist)?;
			SchemaRegistry::<T>::remove(index);
			Self::deposit_event(Event::SchemaRevoked { who, index });
			Ok(())
		}
	}
}
