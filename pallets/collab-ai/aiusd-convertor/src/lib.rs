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

//! Pallet for converting among AIUSD and other stable token.
//! 
#![cfg_attr(not(feature = "std"), no_std)]




#[frame_support::pallet]
pub mod pallet {
	use super::*;
    type AssetId<T> = <T as pallet_assets::Config>::AssetId;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // This is not a treasury account
        // Balance of all types in this account will be distributed to user 
        // who help even the distribution of stable token
        type ConvertingFeeAccount = Get<Self::AccountId>;

        // Declare the asset id of AIUSD
        type AIUSDAssetId = Get<AssetId<Self>;
	}

    #[pallet::storage]
	#[pallet::getter(fn reward_pools)]
	pub type CumulatedFee<T: Config> =
		StorageMap<_, Blake2_128Concat, T::EVMId, T::AccountId, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn switch_token(
			origin: OriginFor<T>,
			consumed_asset_id: AssetId,
            target_asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			let _ = T::IncConsumerOrigin::ensure_origin(origin)?;
			for i in &who {
				frame_system::Pallet::<T>::inc_consumers(i)?;
			}
			Ok(())
		}

        /// Enable a specific type of token available for switching
        #[pallet::call_index(1)]
		#[pallet::weight({195_000_000})]
		pub fn enable_token(
			origin: OriginFor<T>,
			
		) -> DispatchResultWithPostInfo {
			
		}

		/// pause the switch of a specific type of token
		#[pallet::call_index(2)]
		#[pallet::weight({195_000_000})]
		pub fn pause_token(
			origin: OriginFor<T>,
			
		) -> DispatchResultWithPostInfo {
			
		}

        /// unpause the switch of a specific type of token
		#[pallet::call_index(3)]
		#[pallet::weight({195_000_000})]
		pub fn unpause_token(
			origin: OriginFor<T>,
			
		) -> DispatchResultWithPostInfo {
			
		}
	}
}
