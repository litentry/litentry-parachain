// Copyright 2020-2021 Litentry Technologies GmbH.
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
	use frame_support::{
		fail,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, StorageVersion},
	};
	use frame_system::pallet_prelude::*;
	pub use pallet_bridge as bridge;

	type ResourceId = bridge::ResourceId;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + bridge::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Specifies the origin check provided by the bridge for calls that can only be called by the bridge pallet
		type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

		/// The currency mechanism.
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type NativeTokenResourceId: Get<ResourceId>;
	}

	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		InvalidCommand,
		InvalidResourceId,
	}

	#[pallet::storage]
	#[pallet::getter(fn bridge_balances)]
	pub type BridgeBalances<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		bridge::ResourceId,
		Twox64Concat,
		T::AccountId,
		BalanceOf<T>,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Executes a simple currency transfer using the bridge account as the source
		#[pallet::weight(195_000_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			rid: ResourceId,
		) -> DispatchResult {
			let source = T::BridgeOrigin::ensure_origin(origin)?;
			// transfer to bridge account from external accounts is not allowed.
			if source == to {
				fail!(Error::<T>::InvalidCommand);
			}

			if rid == T::NativeTokenResourceId::get() {
				// ERC20 LIT transfer
				<T as Config>::Currency::transfer(
					&source,
					&to,
					amount,
					ExistenceRequirement::AllowDeath,
				)?;
			} else {
				return Err(Error::<T>::InvalidResourceId.into())
			}

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {}
}
