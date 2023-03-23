// Copyright 2020-2023 Litentry Technologies GmbH.
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

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::{pallet_prelude::*, traits::StorageVersion, transactional, PalletId, Parameter};
	use frame_system::{
		pallet_prelude::*,
		{self as system},
	};
	use sp_std::prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Origin used to administer the pallet
		type WhitelistManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Relayer added to set
		WhitelistAdded(T::AccountId),
		/// Relayer removed from set
		WhitelistRemoved(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Whitelist already in set
		WhitelistAlreadyExists,
		/// Provided accountId is not a Whitelist
		WhitelistInvalid,
	}

	#[pallet::storage]
	#[pallet::getter(fn whitelist_on)]
	pub type WhitelistOn<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn whitelists)]
	pub type Whitelists<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a new whitelist
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn add_whitelist(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
            ensure!(!Self::is_whitelist(&v), Error::<T>::WhitelistAlreadyExists);
			Whitelists::<T>::insert(&v, true);
            Self::deposit_event(Event::WhitelistAdded(v));
            Ok(())
		}

		/// Batch adding of new whitelists
		#[pallet::call_index(1)]
		#[pallet::weight(100_000)]
		#[transactional]
		pub fn batch_add_whitelists(origin: OriginFor<T>, vs: Vec<T::AccountId>) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
			for v in vs {
				ensure!(!Self::is_whitelist(&v), Error::<T>::WhitelistAlreadyExists);
				Whitelists::<T>::insert(&v, true);
				Self::deposit_event(Event::WhitelistAdded(v));
			}
            Ok(())
		}

		/// Removes an existing whitelist
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn remove_whitelist(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
            ensure!(Self::is_whitelist(&v), Error::<T>::WhitelistInvalid);
			Whitelists::<T>::remove(&v);
            Self::deposit_event(Event::WhitelistRemoved(v));
            Ok(())
		}

		/// Batch Removing existing whitelists
		#[pallet::call_index(3)]
		#[pallet::weight(100_000)]
		#[transactional]
		pub fn batch_remove_whitelists(origin: OriginFor<T>, vs: Vec<T::AccountId>) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
			for v in vs {
				ensure!(Self::is_whitelist(&v), Error::<T>::WhitelistInvalid);
				Whitelists::<T>::remove(&v);
				Self::deposit_event(Event::WhitelistRemoved(v));
			}
			Ok(())
		}

		/// Swith WhitelistOn on
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn swtich_whitelist_on(origin: OriginFor<T>) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
			<WhitelistOn<T>>::put(true);
			Ok(())
		}

		/// Swith WhitelistOn off
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn swtich_whitelist_off(origin: OriginFor<T>) -> DispatchResult {
			T::WhitelistManagerOrigin::ensure_origin(origin)?;
			<WhitelistOn<T>>::put(false);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Checks if who is a whitelist
		pub fn is_whitelist(who: &T::AccountId) -> bool {
			Self::whitelists(who)
		}
	}

	/// Simple ensure origin for the whitelist account
	pub struct EnsureWhitelist<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> EnsureOrigin<T::RuntimeOrigin> for EnsureWhitelist<T> {
		type Success = T::AccountId;
		fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
			// If function off, then pass everything as long as signed 
			if !Pallet::<T>::whitelist_on() {
				o.into().and_then(|o| match o {
					system::RawOrigin::Signed(who) => Ok(who),
					r => Err(T::RuntimeOrigin::from(r)),
				})
			} else {
				o.into().and_then(|o| match o {
					system::RawOrigin::Signed(ref who) 
                    	if Pallet::<T>::is_whitelist(who) => Ok(who.clone()),
					r => Err(T::RuntimeOrigin::from(r)),
				})
			}
		}
	}
}
