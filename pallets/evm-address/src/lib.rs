// Copyright 2020-2023 Trust Computing GmbH.
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

//! A pallet for managing evm address and parachain AccountId
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use scale_info::TypeInfo;
	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// EVM address type, usually H160
		type EVMId: Default + Copy + PartialEq + core::fmt::Debug + codec::FullCodec + TypeInfo;
	}

	/// Map for existing evm address and substrate address relation
	/// We store them since the reverting proccess is not always achievable
	/// without storage.
	#[pallet::storage]
	#[pallet::getter(fn reward_pools)]
	pub type AddressMapping<T: Config> =
		StorageMap<_, Blake2_128Concat, T::EVMId, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// AddressMapping added
		AddressMappingAdded { evm: T::EVMId, account_id: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// AddressMapping existed and conflicted
		AddressMappingConflicted,
	}

	impl<T: Config> Pallet<T> {
		pub fn is_address_mapped(evm: T::EVMId) -> bool {
			AddressMapping::<T>::get(evm).is_some()
		}

		pub fn get_address_mapped(evm: T::EVMId) -> Option<T::AccountId> {
			AddressMapping::<T>::get(evm)
		}
		/// add address mapping, error if other already added
		/// Nothing happened if adding the same existing
		pub fn add_address_mapping(
			evm: T::EVMId,
			address: T::AccountId,
		) -> Result<T::AccountId, Error<T>> {
			AddressMapping::<T>::try_mutate(evm, |old_address| match old_address {
				Some(a) if a.clone() == address => Ok(()),
				Some(_) => Err(Error::<T>::AddressMappingConflicted),
				None => {
					Self::deposit_event(Event::AddressMappingAdded {
						evm,
						account_id: address.clone(),
					});
					*old_address = Some(address.clone());
					Ok(())
				},
			})
			.map(|_| address)
		}
	}
}
