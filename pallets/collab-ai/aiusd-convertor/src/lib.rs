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
use frame_support::{
	pallet_prelude::*,
	traits::{
		tokens::{
			fungibles::{Inspect as FsInspect, Mutate as FsMutate},
			Fortitude, Precision,
		},
		StorageVersion,
	},
};



#[frame_support::pallet]
pub mod pallet {
	use super::*;
	/// Balance type alias for balances of assets that implement the `fungibles` trait.
	pub(crate) type InspectFungibles<T> = pallet_assets::Pallet<T> as FsInspect<T as frame_system::Config>::AccountId>;
	pub(crate) type AssetBalanceOf<T> =
		InspectFungibles<T>::Balance;
	/// Type alias for Asset IDs.
	pub(crate) type AssetIdOf<T> =
		InspectFungibles<T>::AssetId;

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
        // Balance of all types in this account record the 
		// available stable coin for AIUSD to switch back
        type ConvertingFeeAccount: Get<Self::AccountId>;

		// Declare the asset id of AIUSD
        type AIUSDAssetId: Get<AssetIdOf<Self>>;
	}

	// Asset Id => ratio of system exchange rate for AIUSD to other token in 10^18
	// e.g.
	// (1 USDT) = 10^6 in system
	// (1 AIUSD) = 10^18 in system
	// Value of StorageMap n = 10^(-12) * 10^(18) = 10^(6), 
	// which means (1 AIUSD) * n = (10^18) * (1 USDT) in system balance when converting.
    #[pallet::storage]
	#[pallet::getter(fn enabled_tokens)]
	pub type EnabledTokens<T: Config> =
		StorageMap<_, Twox64Concat, AssetIdOf<T>, AssetBalanceOf<T>, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		InvalidAssetId,
		AssetNotEnabled,
		CannotPayAsFee,
		ReachMaximumSupply,
		Overflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// AIUSD minted with other eligible stable token locked
		AIUSDCreated {
			beneficiary: T::AccountId,
			asset_id: AssetIdOf<T>,
			amount: AssetBalanceOf<T>,
		},
		/// AIUSD burned with other eligible stable token released
		AIUSDDestroyed {
			beneficiary: T::AccountId,
			asset_id: AssetIdOf<T>,
			amount: AssetBalanceOf<T>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// Lock target_asset_id token and mint AIUSD
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn mint_aiusd(
			origin: OriginFor<T>,
            target_asset_id: AssetIdOf<T>,
			amount: AssetBalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let beneficiary = T::IncConsumerOrigin::ensure_origin(origin)?;
			ensure!(EnabledTokens::<T>::contains_key(&target_asset_id),  Error::<T>::AssetNotEnabled);
			let aiusd_id = AIUSDAssetId::get();
			ensure!(InspectFungibles<T>::asset_exists(&aiusd_id) && InspectFungibles<T>::asset_exists(&target_asset_id), Error::<T>::InvalidAssetId);
			// It will fail if insufficient fund
			let actual_amount: AssetBalanceOf<T> = InspectFungibles<T>::transfer(target_asset_id, beneficiary, ConvertingFeeAccount::get(), amount, Expendable)?;
			
			
			let minted_amount: AssetBalanceOf<T> = InspectFungibles<T>::mint_into(aiusd_id, beneficiary, actual_amount)?;

			Self::deposit_event(Event::AIUSDCreated { beneficiary, asset_id: target_asset_id, amount });

			Ok(())
		}

		// Burn aiusd and get target_asset_id token released
		// Failed if pool does not have enough token of one type
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn burn_aiusd(
			origin: OriginFor<T>,
            target_asset_id: AssetIdOf<T>,
			amount: AssetBalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let beneficiary = T::IncConsumerOrigin::ensure_origin(origin)?;
			if let Some(ratio) = EnabledTokens::<T>::get(&target_asset_id) {
				let aiusd_id = AIUSDAssetId::get();
				ensure!(InspectFungibles<T>::asset_exists(&aiusd_id) && InspectFungibles<T>::asset_exists(&target_asset_id), Error::<T>::InvalidAssetId);
				// It will fail if insufficient fund
				let actual_amount: AssetBalanceOf<T> = InspectFungibles<T>::transfer(aiusd_id, beneficiary, ConvertingFeeAccount::get(), amount, Expendable)?;
				
			
				// Maybe it is better to save decimal of AIUSD somewhere
				let minted_amount: AssetBalanceOf<T> = InspectFungibles<T>::mint_into(target_asset_id, beneficiary, actual_amount * ratio / 10^18)?;

				Self::deposit_event(Event::AIUSDDestroyed { beneficiary, asset_id: target_asset_id, amount });
				Ok(())
			} else {
				Err(Error::<T>::AssetNotEnabled.into())
			}
		}



        /// Enable a specific type of token available for switching
        #[pallet::call_index(1)]
		#[pallet::weight({195_000_000})]
		pub fn enable_token(
			origin: OriginFor<T>,
			target_asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			
		}

		/// disable a specific type of token available for switching
		#[pallet::call_index(2)]
		#[pallet::weight({195_000_000})]
		pub fn disable_token(
			origin: OriginFor<T>,
			target_asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			
		}
	}
}
