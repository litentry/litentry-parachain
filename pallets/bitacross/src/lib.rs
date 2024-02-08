// Copyright 2020-2024 Trust Computing GmbH.
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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]
#![allow(clippy::let_unit_value, deprecated)]
use sp_core::H160;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use core_primitives::Identity;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// Index type for distinct relayer
		type RelayerIndex: Default
			+ Copy
			+ PartialEq
			+ core::fmt::Debug
			+ codec::FullCodec
			+ AtLeast32BitUnsigned
			+ From<u64>
			+ TypeInfo;
		// some extrinsics should only be called by origins from TEE
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		// origin to manage Relayer Admin
		type SetAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		// origin that is allowed to call extrinsics
		type ExtrinsicWhitelistOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;
	}

	/// Relayer Index => Relayer Public Address
	#[pallet::storage]
	#[pallet::getter(fn relayer_public)]
	pub type RelayerPublic<T: Config> =
		StorageMap<_, Blake2_128Concat, T::RelayerIndex, Identity, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn admin)]
	pub type Admin<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RelayerAdded { index: T::RelayerIndex, account: Identity },
		RelayerRemoved { index: T::RelayerIndex },
		BitAcrossAdminChanged { account: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		RelayerNotExist,
		RequireAdmin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add an account to the delegatees
		#[pallet::call_index(0)]
		#[pallet::weight(10000000)]
		pub fn placeholder(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			Ok(())
		}

		/// add an account to the relayer storage
		#[pallet::call_index(1)]
		#[pallet::weight(10000000)]
		pub fn add_relayer(
			origin: OriginFor<T>,
			index: T::RelayerIndex,
			account: Identity,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender) == Admin::<T>::get(), Error::<T>::RequireAdmin);
			// we don't care if `account` already exists
			RelayerPublic::<T>::insert(index, account.clone());
			Self::deposit_event(Event::RelayerAdded { index, account });
			Ok(())
		}

		/// remove an account from the delegatees
		#[pallet::call_index(2)]
		#[pallet::weight(10000000)]
		pub fn remove_relayer(origin: OriginFor<T>, index: T::RelayerIndex) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender) == Admin::<T>::get(), Error::<T>::RequireAdmin);
			ensure!(RelayerPublic::<T>::contains_key(&account), Error::<T>::RelayerNotExist);
			RelayerPublic::<T>::remove(index);
			Self::deposit_event(Event::RelayerRemoved { index });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10000000)]
		pub fn set_bitacross_admin(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _ = T::SetAdminOrigin::ensure_origin(origin)?;
			Admin::<T>::set(Some(account.clone()));
			Self::deposit_event(Event::BitAcrossAdminChanged { account });
			Ok(())
		}
	}
}
