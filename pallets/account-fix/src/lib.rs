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

//! A pallet for temporary fix of onchain accountInfo.
//! No storage for this pallet and it should be removed right after fixing.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ReservableCurrency, StorageVersion},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::traits::StaticLookup;

use sp_std::vec::Vec;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Change the admin account
		/// similar to sudo.set_key, the old account will be supplied in event
		#[pallet::call_index(0)]
		#[pallet::weight({20_000_000 * who.len()})]
		pub fn upgrade_accounts(
			origin: OriginFor<T>,
			who: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			for i in &who {
				frame_system::Pallet::<T>::inc_consumers(i)?;
			}
			// Do not pay a fee
			Ok(Pays::No.into())
		}

		/// add some balance of an existing account
		#[pallet::call_index(1)]
		#[pallet::weight({10_000})]
		pub fn set_balance(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			#[pallet::compact] add_free: BalanceOf<T>,
			#[pallet::compact] add_reserved: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let who = T::Lookup::lookup(who)?;

			let add_total = add_free + add_reserved;

			// First we try to modify the account's balance to the forced balance.
			T::Currency::deposit_into_existing(&who, add_total)?;
			// Then do the reservation
			T::Currency::reserve(&who, add_reserved)?;

			Ok(().into())
		}
	}
}
