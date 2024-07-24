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

//! A pallet for handling bridge transfer lgoic.
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
use frame_support::{
	pallet_prelude::*,
	traits::{
		tokens::{
			fungible::Mutate as FMutate, fungibles::Mutate as FsMutate, Fortitude, Precision,
		},
		StorageVersion,
	},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_bridge_transfer::BridgeHandler;
use sp_runtime::{traits::CheckedSub, ArithmeticError, DispatchError, FixedPointOperand};
use sp_std::{fmt::Debug, prelude::*};
type ResourceId = pallet_bridge::ResourceId;

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct AssetInfo<AssetId, Balance> {
	fee: Balance,
	// None for native token
	asset: Option<AssetId>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);
	type BalanceOf<T> = <T as pallet_bridge::Config>::Balance;
	type AssetId<T> = <T as pallet_assets::Config>::AssetId;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_assets::Config
		+ pallet_bridge::Config
	{
		/// Overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Treasury account to receive assets fee
		type TreasuryAccount: Get<Self::AccountId>;

		/// The privileged origin to call update_maximum_issuance
		type SetMaximumIssuanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		
		#[pallet::constant]
		type DefaultMaximumIssuance: Get<bridge::BalanceOf<Self>>;

		#[pallet::constant]
		// In parachain local decimal format
		type ExternalTotalIssuance: Get<bridge::BalanceOf<Self>>;

	}

	// Resource Id of pallet assets token
	#[pallet::storage]
	#[pallet::getter(fn resource_to_asset_info)]
	pub type ResourceToAssetInfo<T: Config> =
		StorageMap<_, Twox64Concat, ResourceId, AssetInfo<AssetId<T>, BalanceOf<T>>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// asset id = None means native token
		ResourceUpdated {
			resource_id: ResourceId,
			asset: AssetInfo<AssetId<T>, BalanceOf<T>>,
		},
		ResourceRemoved {
			resource_id: ResourceId,
		},
		/// A certain amount of asset tokens was minted
		TokenBridgeIn {
			asset_id: Option<AssetId<T>>,
			to: T::AccountId,
			amount: BalanceOf<T>,
		},
		TokenBridgeOut {
			asset_id: Option<AssetId<T>>,
			to: T::AccountId,
			// Before Fee
			amount: BalanceOf<T>,
			fee: BalanceOf<T>,
		},
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// MaximumIssuance was changed
		MaximumIssuanceChanged { old_value: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidResourceId,
		CannotPayAsFee,
		ReachMaximumSupply,
		OverFlow,
	}

	#[pallet::type_value]
	pub fn DefaultExternalBalances<T: Config>() -> bridge::BalanceOf<T> {
		T::ExternalTotalIssuance::get()
			.checked_sub(&<<T as bridge::Config>::Currency as Currency<
				<T as frame_system::Config>::AccountId,
			>>::total_issuance())
			.map_or_else(|| 0u32.into(), |v| v)
	}

	// Native Token External Balance
	#[pallet::storage]
	#[pallet::getter(fn external_balances)]
	pub type ExternalBalances<T: Config> =
		StorageValue<_, bridge::BalanceOf<T>, ValueQuery, DefaultExternalBalances<T>>;
	
	// Native Token Maximum Issuance
	#[pallet::storage]
	#[pallet::getter(fn maximum_issuance)]
	pub type MaximumIssuance<T: Config> =
		StorageValue<_, bridge::BalanceOf<T>, ValueQuery, T::DefaultMaximumIssuance>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stores an asset id on chain under an associated resource ID.
		#[pallet::call_index(0)]
		#[pallet::weight({1000})]
		pub fn set_resource(
			origin: OriginFor<T>,
			resource_id: ResourceId,
			asset: AssetInfo<AssetId<T>, BalanceOf<T>>,
		) -> DispatchResult {
			<T as pallet_bridge::Config>::BridgeCommitteeOrigin::ensure_origin(origin)?;
			ResourceToAssetInfo::<T>::insert(resource_id, asset.clone());
			Self::deposit_event(Event::ResourceUpdated { resource_id, asset });
			Ok(())
		}

		/// Removes a resource ID from the resource mapping.
		///
		/// After this call, bridge transfers with the associated resource ID will
		/// be rejected.
		#[pallet::call_index(1)]
		#[pallet::weight({1000})]
		pub fn remove_resource(origin: OriginFor<T>, resource_id: ResourceId) -> DispatchResult {
			<T as pallet_bridge::Config>::BridgeCommitteeOrigin::ensure_origin(origin)?;
			ResourceToAssetInfo::<T>::remove(resource_id);
			Self::deposit_event(Event::ResourceRemoved { resource_id });
			Ok(())
		}
	}
}

impl<T, B, A> BridgeHandler<B, A, ResourceId> for Pallet<T>
where
	T: Config
		+ frame_system::Config<AccountId = A>
		+ pallet_bridge::Config<Balance = B>
		+ pallet_assets::Config<Balance = B>
		+ pallet_balances::Config<Balance = B>,
	B: Copy + FixedPointOperand + CheckedSub,
	A: Clone,
{
	fn prepare_token_bridge_in(
		resource_id: ResourceId,
		who: A,
		amount: B,
	) -> Result<B, DispatchError> {
		let asset_info = Self::resource_to_asset_info(resource_id);
		match asset_info {
			None => Err(Error::<T>::InvalidResourceId.into()),
			// Native token
			Some(AssetInfo { fee: _, asset: None }) => {
				Self::deposit_event(Event::TokenBridgeIn {
					asset_id: None,
					to: who.clone(),
					amount,
				});
				pallet_balances::Pallet::<T>::mint_into(&who, amount)
			},
			// pallet assets
			Some(AssetInfo { fee: _, asset: Some(asset) }) => {
				Self::deposit_event(Event::TokenBridgeIn {
					asset_id: Some(asset.clone()),
					to: who.clone(),
					amount,
				});
				pallet_assets::Pallet::<T>::mint_into(asset, &who, amount)
			},
		}
	}
	// Return actual amount to target chain after deduction e.g fee
	fn prepare_token_bridge_out(
		resource_id: ResourceId,
		who: A,
		amount: B,
	) -> Result<B, DispatchError> {
		let asset_info = Self::resource_to_asset_info(resource_id);
		match asset_info {
			None => Err(Error::<T>::InvalidResourceId.into()),
			// Native token
			Some(AssetInfo { fee, asset: None }) => {
				Self::deposit_event(Event::TokenBridgeOut {
					asset_id: None,
					to: who.clone(),
					amount,
					fee,
				});
				let burn_amount = pallet_balances::Pallet::<T>::burn_from(
					&who,
					amount,
					Precision::Exact,
					Fortitude::Polite,
				)?;
				ensure!(burn_amount > fee, Error::<T>::CannotPayAsFee);
				pallet_balances::Pallet::<T>::mint_into(&T::TreasuryAccount::get(), fee)?;
				Ok(burn_amount.checked_sub(&fee).ok_or(ArithmeticError::Overflow)?)
			},
			// pallet assets
			Some(AssetInfo { fee, asset: Some(asset) }) => {
				Self::deposit_event(Event::TokenBridgeOut {
					asset_id: Some(asset.clone()),
					to: who.clone(),
					amount,
					fee,
				});
				// Since we use Exact approach
				// Burn amount will always be amount exactly
				// Otherwise
				let burn_amount = pallet_assets::Pallet::<T>::burn_from(
					asset.clone(),
					&who,
					amount,
					Precision::Exact,
					Fortitude::Polite,
				)?;
				ensure!(burn_amount > fee, Error::<T>::CannotPayAsFee);
				pallet_assets::Pallet::<T>::mint_into(asset, &T::TreasuryAccount::get(), fee)?;
				Ok(burn_amount.checked_sub(&fee).ok_or(ArithmeticError::Overflow)?)
			},
		}
	}
}
