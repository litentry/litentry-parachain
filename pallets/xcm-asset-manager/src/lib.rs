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

//! Inspired by:
//! - Moonbeam `pallet_asset_manager`
//! implementations.

// (1) The local asset support is removed. This pallet is for managing Xcm foreign asset
// specifically. (2) The mapping of AssetId and AssetType is now capable of manual choosen
// (3) Compatibility to orml_token instead of pallet_assets
// (4) Code for destroy asset is removed

//! TODO Doc comments for the pallet
//! # Asset Manager Pallet
//!
//! This pallet allows to register new assets if certain conditions are met
//! The main goal of this pallet is to allow Litmus to register XCM assets
//! and control the creation of foreign assets
//! The assumption is we work with AssetTypes, which can then be comverted to AssetIds
//!
//! This pallet has six storage items:
//! - AssetIdType: A mapping from AssetId->AssetType.
//! - AssetIdUnitsPerSecond: An AssetId->u128 mapping that holds how much each AssetId should be
//!   charged per unit of second, in the case such an Asset is received as a XCM asset.
//! - AssetTypeId: A mapping from AssetType -> AssetId.
//! - ForeignAssetTracker: The counter of foreign assets that have been created so far.
//! - SupportedFeePaymentAssets: The assets supported to serve as weight payment in XCM.
//! - AssetIdMetadatas: An AssetId->AssetMetadata mapping that holds the metadata token info.
//!
//! This pallet has six extrinsics:
//! - register_foreign_asset: Register a foreign asset in this pallet.
//! - set_asset_units_per_second: Set the unit per second that should be charged for a particular
//!   asset.
//! - relocate_foreign_asset_id: Allow to manipulate ForeignAssetTracker value.
//! - update_foreign_asset_metadata: Update the metadata of asset.
//! - add_asset_type: Add the correspondence between AssetId and AssetType.
//! - remove_asset_type: Remove the correspondence between AssetId and AssetType. At least one
//!   relation must exist.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::clone_on_copy)]

// TODO implement
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub mod tests;
pub mod weights;

use crate::weights::WeightInfo;
use codec::HasCompact;
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ReservableCurrency},
	transactional, PalletId,
};
use frame_system::pallet_prelude::*;
use orml_traits::GetByKey;
pub use pallet::*;
use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, One};
use sp_std::{convert::*, vec::Vec};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// The AssetManagers's pallet id
	pub const PALLET_ID: PalletId = PalletId(*b"asstmngr");

	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Default, Eq, Debug, PartialEq, Ord, PartialOrd, Encode, Decode, TypeInfo)]
	pub struct AssetMetadata<Balance> {
		pub name: Vec<u8>,
		pub symbol: Vec<u8>,
		pub decimals: u8,
		pub minimal_balance: Balance,
		pub is_frozen: bool,
	}

	// Defines the trait to obtain a generic AssetType from a generic AssetId and viceversa
	pub trait AssetTypeGetter<AssetId, AssetType> {
		// Get asset type from assetId
		fn get_asset_type(asset_id: AssetId) -> Option<AssetType>;

		// Get assetId from assetType
		fn get_asset_id(asset_type: AssetType) -> Option<AssetId>;
	}

	// We implement this trait to be able to get the AssetType and units per second registered
	impl<T: Config> AssetTypeGetter<T::AssetId, T::ForeignAssetType> for Pallet<T> {
		fn get_asset_type(asset_id: T::AssetId) -> Option<T::ForeignAssetType> {
			AssetIdType::<T>::get(asset_id)
		}

		fn get_asset_id(asset_type: T::ForeignAssetType) -> Option<T::AssetId> {
			AssetTypeId::<T>::get(asset_type)
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Asset Id. This will be used to register the asset in Assets
		type AssetId: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Ord
			+ PartialOrd
			+ Copy
			+ HasCompact
			+ MaxEncodedLen;

		/// The Foreign Asset Kind.
		type ForeignAssetType: Parameter + Member + Ord + PartialOrd;

		/// The units in which we record balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		/// Origin that is allowed to create and modify asset information for foreign assets
		type ForeignAssetModifierOrigin: EnsureOrigin<Self::Origin>;

		/// The currency mechanism in which we reserve deposits for local assets.
		type Currency: ReservableCurrency<Self::AccountId>;

		type WeightInfo: WeightInfo;
	}

	/// An error that can occur while executing the mapping pallet's logic.
	#[pallet::error]
	pub enum Error<T> {
		AssetAlreadyExists,
		AssetTypeDoesNotExist,
		AssetIdDoesNotExist,
		AssetLimitReached,
		DefaultAssetTypeRemoved,
		TooLowNumAssetsWeightHint,
		AssetIdOverflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The foreign asset updated.
		ForeignAssetMetadataUpdated { asset_id: T::AssetId, metadata: AssetMetadata<BalanceOf<T>> },
		/// New asset with the asset manager is registered
		ForeignAssetRegistered { asset_id: T::AssetId, asset_type: T::ForeignAssetType },
		/// Removed all information related to an assetId
		ForeignAssetRemoved { asset_id: T::AssetId, asset_type: T::ForeignAssetType },
		/// AssetTracker manipulated
		ForeignAssetTrackerUpdated { old_asset_tracker: T::AssetId, new_asset_tracker: T::AssetId },
		/// New Event gives the info about involved asset_id, removed asset_type, and the new
		/// default asset_id and asset_type mapping after removal
		SupportedAssetTypeRemoved {
			asset_id: T::AssetId,
			removed_asset_type: T::ForeignAssetType,
			default_asset_type: T::ForeignAssetType,
		},
		/// Changed the amount of units we
		/// are charging per execution second for a given asset
		UnitsPerSecondChanged { asset_id: T::AssetId, units_per_second: u128 },
	}

	/// Mapping from an asset id to asset type.
	/// This is mostly used when receiving transaction specifying an asset directly,
	/// like transferring an asset from this chain to another.
	#[pallet::storage]
	#[pallet::getter(fn asset_id_type)]
	pub type AssetIdType<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, T::ForeignAssetType>;

	/// Reverse mapping of AssetIdType. Mapping from an asset type to an asset id.
	/// This is mostly used when receiving a multilocation XCM message to retrieve
	/// the corresponding asset in which tokens should me minted.
	#[pallet::storage]
	#[pallet::getter(fn asset_type_id)]
	pub type AssetTypeId<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ForeignAssetType, T::AssetId>;

	/// Stores the units per second for local execution for a AssetType.
	/// This is used to know how to charge for XCM execution in a particular asset
	/// Not all assets might contain units per second, hence the different storage
	#[pallet::storage]
	#[pallet::getter(fn asset_id_units_per_second)]
	pub type AssetIdUnitsPerSecond<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, u128>;

	/// Stores the tracker of foreign assets id that have been
	/// created so far
	#[pallet::storage]
	#[pallet::getter(fn foreign_asset_tracker)]
	pub type ForeignAssetTracker<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

	// Supported fee asset payments
	#[pallet::storage]
	#[pallet::getter(fn supported_fee_payment_assets)]
	pub type SupportedFeePaymentAssets<T: Config> = StorageValue<_, Vec<T::AssetId>, ValueQuery>;

	/// The storages for AssetIdMetadatas.
	///
	/// AssetIdMetadatas: map AssetId => Option<AssetMetadata>
	#[pallet::storage]
	#[pallet::getter(fn asset_metadatas)]
	pub type AssetIdMetadatas<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, AssetMetadata<BalanceOf<T>>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register new asset with the asset manager
		/// TODO::Reserve native token multilocation through GenesisBuild/RuntimeUpgrade
		#[pallet::weight(T::WeightInfo::register_foreign_asset())]
		pub fn register_foreign_asset(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			metadata: AssetMetadata<BalanceOf<T>>,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let mut asset_id = ForeignAssetTracker::<T>::get();

			ensure!(!AssetTypeId::<T>::contains_key(&asset_type), Error::<T>::AssetAlreadyExists);

			AssetIdMetadatas::<T>::insert(&asset_id, &metadata);
			AssetIdType::<T>::insert(&asset_id, &asset_type);
			AssetTypeId::<T>::insert(&asset_type, &asset_id);

			Self::deposit_event(Event::<T>::ForeignAssetRegistered { asset_id, asset_type });
			Self::deposit_event(Event::<T>::ForeignAssetMetadataUpdated { asset_id, metadata });

			asset_id = asset_id.checked_add(&One::one()).ok_or(Error::<T>::AssetIdOverflow)?;
			// Auto increment for Asset counter
			ForeignAssetTracker::<T>::put(asset_id);

			Ok(())
		}

		/// Relocate asset id.
		/// Can only be larger than current assignment.
		#[pallet::weight(T::WeightInfo::relocate_foreign_asset_id())]
		pub fn relocate_foreign_asset_id(
			origin: OriginFor<T>,
			next_asset_id: T::AssetId,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;
			let old_next_asset_id = ForeignAssetTracker::<T>::get();
			if next_asset_id > old_next_asset_id {
				Self::deposit_event(Event::<T>::ForeignAssetTrackerUpdated {
					old_asset_tracker: old_next_asset_id,
					new_asset_tracker: next_asset_id,
				});
				ForeignAssetTracker::<T>::put(next_asset_id);
			}
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_foreign_asset_metadata())]
		pub fn update_foreign_asset_metadata(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			metadata: AssetMetadata<BalanceOf<T>>,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetTypeDoesNotExist)?;
			AssetIdMetadatas::<T>::insert(&asset_id, &metadata);

			Self::deposit_event(Event::<T>::ForeignAssetMetadataUpdated { asset_id, metadata });
			Ok(())
		}

		/// Change the amount of units we are charging per execution second
		/// for a given ForeignAssetType
		#[pallet::weight(T::WeightInfo::set_asset_units_per_second(*num_assets_weight_hint))]
		pub fn set_asset_units_per_second(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			units_per_second: u128,
			num_assets_weight_hint: u32,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetTypeDoesNotExist)?;

			// Grab supported assets
			let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

			ensure!(
				num_assets_weight_hint >= (supported_assets.len() as u32),
				Error::<T>::TooLowNumAssetsWeightHint
			);

			// Only if the asset is not supported we need to push it
			if let Err(index) = supported_assets.binary_search(&asset_id) {
				supported_assets.insert(index, asset_id.clone());
				SupportedFeePaymentAssets::<T>::put(supported_assets);
			}

			AssetIdUnitsPerSecond::<T>::insert(&asset_id, &units_per_second);

			Self::deposit_event(Event::UnitsPerSecondChanged { asset_id, units_per_second });
			Ok(())
		}

		/// Add the xcm type mapping for a existing assetId, other assetType still exists if any.
		/// TODO: Weight is no longer related to num_assets_weight_hint; changes here needed
		#[pallet::weight(T::WeightInfo::add_asset_type())]
		pub fn add_asset_type(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			new_asset_type: T::ForeignAssetType,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			ensure!(AssetIdType::<T>::contains_key(&asset_id), Error::<T>::AssetIdDoesNotExist);
			ensure!(
				!AssetTypeId::<T>::contains_key(&new_asset_type),
				Error::<T>::AssetAlreadyExists
			);

			// Insert new asset type info
			AssetIdType::<T>::insert(&asset_id, &new_asset_type);
			AssetTypeId::<T>::insert(&new_asset_type, &asset_id);

			// Remove previous asset type info
			// AssetTypeId::<T>::remove(&previous_asset_type);

			Self::deposit_event(Event::ForeignAssetRegistered {
				asset_id,
				asset_type: new_asset_type,
			});
			Ok(())
		}

		/// TODO: Weight is no longer related to num_assets_weight_hint; changes here needed
		/// We do not allow the destroy of asset id so far
		/// But we will remove all existing related fee settings. So will need re-implement by
		/// extrinsic call. New default asset type is needed for the assignment of asset_id's
		/// default asset type mapping
		#[pallet::weight(T::WeightInfo::remove_asset_type(*num_assets_weight_hint))]
		#[transactional]
		pub fn remove_asset_type(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			new_default_asset_type: Option<T::ForeignAssetType>,
			num_assets_weight_hint: u32,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetTypeDoesNotExist)?;

			if let Some(i) = new_default_asset_type {
				AssetIdType::<T>::insert(&asset_id, i);
			}

			let default_asset_type: T::ForeignAssetType =
				AssetIdType::<T>::get(&asset_id).ok_or(Error::<T>::AssetIdDoesNotExist)?;
			if default_asset_type == asset_type {
				return Err(Error::<T>::DefaultAssetTypeRemoved.into())
			}

			// Grab supported assets
			let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

			ensure!(
				num_assets_weight_hint >= (supported_assets.len() as u32),
				Error::<T>::TooLowNumAssetsWeightHint
			);

			// Only if the old asset is supported we need to remove it
			if let Ok(index) = supported_assets.binary_search(&asset_id) {
				supported_assets.remove(index);
			}

			// Insert
			SupportedFeePaymentAssets::<T>::put(supported_assets);

			// Remove from AssetTypeId
			AssetTypeId::<T>::remove(&asset_type);

			// Remove
			AssetIdUnitsPerSecond::<T>::remove(&asset_id);

			Self::deposit_event(Event::SupportedAssetTypeRemoved {
				asset_id,
				removed_asset_type: asset_type,
				default_asset_type,
			});
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID of AssetManager
		pub fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}
	}

	impl<T: Config> GetByKey<T::AssetId, BalanceOf<T>> for Pallet<T> {
		fn get(asset_id: &T::AssetId) -> BalanceOf<T> {
			let metadata: AssetMetadata<BalanceOf<T>> =
				AssetIdMetadatas::<T>::get(asset_id).unwrap_or_default();
			metadata.minimal_balance
		}
	}

	// AssetManager or other FeeToWeight source should implement this trait
	// Defines the trait to obtain the units per second of a give asset_type for local execution
	// This parameter will be used to charge for fees upon asset_type deposit
	pub trait UnitsToWeightRatio<AssetType> {
		// Whether payment in a particular asset_type is suppotrted
		fn payment_is_supported(asset_type: AssetType) -> bool;
		// Get units per second from asset type
		fn get_units_per_second(asset_type: AssetType) -> Option<u128>;
	}

	impl<T: Config> UnitsToWeightRatio<T::ForeignAssetType> for Pallet<T> {
		fn payment_is_supported(asset_type: T::ForeignAssetType) -> bool {
			if let Some(asset_id) = AssetTypeId::<T>::get(&asset_type) {
				SupportedFeePaymentAssets::<T>::get().binary_search(&asset_id).is_ok()
			} else {
				false
			}
		}
		fn get_units_per_second(asset_type: T::ForeignAssetType) -> Option<u128> {
			if let Some(asset_id) = AssetTypeId::<T>::get(&asset_type) {
				AssetIdUnitsPerSecond::<T>::get(asset_id)
			} else {
				None
			}
		}
	}
}
