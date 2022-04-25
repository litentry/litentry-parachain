// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

// Litentry: Code has been simplified by Litentry.
// (1) The local asset support is removed. This pallet is for managing Xcm foreign asset
// specifically. (2) The mapping of AssetId and AssetType is now capable of manual choosen
// (3) Compatibility to orml_token instead of pallet_assets
// (4) Code for destroy asset is removed

//! TODO Doc comments for the pallet
//! # Asset Manager Pallet
//!
//! This pallet allows to register new assets if certain conditions are met
//! The main goal of this pallet is to allow moonbeam to register XCM assets
//! and control the creation of local assets
//! The assumption is we work with AssetTypes, which can then be comperted to AssetIds
//!
//! This pallet has five storage items: AssetIdType, which holds a mapping from AssetId->AssetType
//! AssetIdUnitsPerSecond: an AssetId->u128 mapping that holds how much each AssetId should
//! be charged per unit of second, in the case such an Asset is received as a XCM asset. Finally,
//! AssetTypeId holds a mapping from AssetType -> AssetId. LocalAssetCounter
//! which holds the counter of local assets that have been created so far.
//!
//! This pallet has eight extrinsics: register_foreign_asset, which registers a foreign
//! asset in this pallet and creates the asset as dictated by the AssetRegistrar trait.
//! set_asset_units_per_second: which sets the unit per second that should be charged for
//! a particular asset.
//! change_existing_asset_type: which allows to update the correspondence between AssetId and
//! AssetType
//! remove_supported_asset: which removes an asset from the supported assets for fee payment
//! remove_existing_asset_type: which removes a mapping from a foreign asset to an assetId

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::clone_on_copy)]

// TODO implement
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
pub mod weights;

use frame_support::pallet;
pub use pallet::*;

#[pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::HasCompact;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use orml_traits::GetByKey;
	use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
	use sp_std::vec::Vec;

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
		ErrorCreatingAsset,
		AssetAlreadyExists,
		AssetDoesNotExist,
		TooLowNumAssetsWeightHint,
		// LocalAssetLimitReached,
		// ErrorDestroyingAsset,
		// NotSufficientDeposit,
		// NonExistentLocalAsset,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New asset with the asset manager is registered
		ForeignAssetRegistered {
			asset_id: T::AssetId,
			asset_type: T::ForeignAssetType,
			metadata: AssetMetadata<BalanceOf<T>>,
		},
		/// The foreign asset updated.
		ForeignAssetUpdated {
			asset_id: T::AssetId,
			asset_type: T::ForeignAssetType,
			metadata: AssetMetadata<BalanceOf<T>>,
		},
		/// Litentry: Purestake use AssetType as index. This might lead to security issue if
		/// MultiLocation Hash changes. Here we use AssetId instead. Changed the amount of units we
		/// are charging per execution second for a given asset
		UnitsPerSecondChanged { asset_id: T::AssetId, units_per_second: u128 },
		/// Changed the xcm type mapping for a given asset id
		ForeignAssetTypeChanged { asset_id: T::AssetId, new_asset_type: T::ForeignAssetType },
		/// Removed all information related to an assetId
		ForeignAssetRemoved { asset_id: T::AssetId, asset_type: T::ForeignAssetType },
		/// Litentry: Purestake use AssetType as index. This might lead to security issue if
		/// MultiLocation Hash changes. Here we use AssetId instead. Supported asset type for fee
		/// payment removed
		SupportedAssetRemoved { asset_id: T::AssetId },
		// /// Local asset was created
		// LocalAssetRegistered { asset_id: T::AssetId, creator: T::AccountId, owner: T::AccountId
		// }, /// Removed all information related to an assetId and destroyed asset
		// ForeignAssetDestroyed { asset_id: T::AssetId, asset_type: T::ForeignAssetType },
		// /// Removed all information related to an assetId and destroyed asset
		// LocalAssetDestroyed { asset_id: T::AssetId },
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

	/// Litentry: Purestake use AssetType as index. This might lead to security issue if
	/// MultiLocation Hash changes. Here we use AssetId instead. Stores the units per second for
	/// local execution for a AssetType. This is used to know how to charge for XCM execution in a
	/// particular asset
	/// Not all assets might contain units per second, hence the different storage
	#[pallet::storage]
	#[pallet::getter(fn asset_id_units_per_second)]
	pub type AssetIdUnitsPerSecond<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, u128>;

	/// Stores the counter of the number of local assets that have been
	/// created so far
	/// This value can be used to salt the creation of an assetId, e.g.,
	/// by hashing it. This is particularly useful for cases like moonbeam
	/// where letting users choose their assetId would result in collision
	/// in the evm side.
	#[pallet::storage]
	#[pallet::getter(fn local_asset_counter)]
	pub type LocalAssetCounter<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Litentry: Purestake use AssetType as index. This might lead to security issue if
	/// MultiLocation Hash changes. Here we use AssetId instead.
	// Supported fee asset payments
	#[pallet::storage]
	#[pallet::getter(fn supported_fee_payment_assets)]
	pub type SupportedFeePaymentAssets<T: Config> = StorageValue<_, Vec<T::AssetId>, ValueQuery>;

	/// The storages for AssetMetadatas.
	///
	/// AssetMetadatas: map AssetId => Option<AssetMetadata>
	#[pallet::storage]
	#[pallet::getter(fn asset_metadatas)]
	pub type AssetMetadatas<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, AssetMetadata<BalanceOf<T>>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register new asset with the asset manager
		#[pallet::weight(T::WeightInfo::register_foreign_asset())]
		pub fn register_foreign_asset(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			asset_id: T::AssetId,
			metadata: AssetMetadata<BalanceOf<T>>,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			ensure!(AssetIdType::<T>::get(&asset_id).is_none(), Error::<T>::AssetAlreadyExists);

			AssetMetadatas::<T>::insert(&asset_id, &metadata);
			AssetIdType::<T>::insert(&asset_id, &asset_type);
			AssetTypeId::<T>::insert(&asset_type, &asset_id);

			Self::deposit_event(Event::ForeignAssetRegistered { asset_id, asset_type, metadata });
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_foreign_asset())]
		pub fn update_foreign_asset(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			metadata: AssetMetadata<BalanceOf<T>>,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetDoesNotExist)?;
			AssetMetadatas::<T>::insert(&asset_id, &metadata);

			Self::deposit_event(Event::<T>::ForeignAssetUpdated { asset_id, asset_type, metadata });
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

			ensure!(AssetTypeId::<T>::get(&asset_type).is_some(), Error::<T>::AssetDoesNotExist);

			// Grab supported assets
			let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

			ensure!(
				num_assets_weight_hint >= (supported_assets.len() as u32),
				Error::<T>::TooLowNumAssetsWeightHint
			);

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetDoesNotExist)?;
			// Only if the asset is not supported we need to push it
			if let Err(index) = supported_assets.binary_search(&asset_id) {
				supported_assets.insert(index, asset_id.clone());
				SupportedFeePaymentAssets::<T>::put(supported_assets);
			}

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetDoesNotExist)?;
			AssetIdUnitsPerSecond::<T>::insert(&asset_id, &units_per_second);

			Self::deposit_event(Event::UnitsPerSecondChanged { asset_id, units_per_second });
			Ok(())
		}

		/// Change the xcm type mapping for a given assetId
		/// We also change this if the previous units per second where pointing at the old
		/// assetType
		/// TODO: Weight is no longer related to num_assets_weight_hint; changes here needed
		#[pallet::weight(T::WeightInfo::change_existing_asset_type(0))]
		pub fn change_existing_asset_type(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			new_asset_type: T::ForeignAssetType,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let previous_asset_type =
				AssetIdType::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

			// Insert new asset type info
			AssetIdType::<T>::insert(&asset_id, &new_asset_type);
			AssetTypeId::<T>::insert(&new_asset_type, &asset_id);

			// Remove previous asset type info
			AssetTypeId::<T>::remove(&previous_asset_type);

			Self::deposit_event(Event::ForeignAssetTypeChanged { asset_id, new_asset_type });
			Ok(())
		}

		/// TODO: Weight is no longer related to num_assets_weight_hint; changes here needed
		#[pallet::weight(T::WeightInfo::remove_supported_asset(*num_assets_weight_hint))]
		pub fn remove_supported_asset(
			origin: OriginFor<T>,
			asset_type: T::ForeignAssetType,
			num_assets_weight_hint: u32,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let asset_id: T::AssetId =
				AssetTypeId::<T>::get(&asset_type).ok_or(Error::<T>::AssetDoesNotExist)?;

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

			// Remove
			AssetIdUnitsPerSecond::<T>::remove(&asset_id);

			Self::deposit_event(Event::SupportedAssetRemoved { asset_id });
			Ok(())
		}

		/// Remove a given assetId -> assetType association
		#[pallet::weight(T::WeightInfo::remove_existing_asset_type(*num_assets_weight_hint))]
		pub fn remove_existing_asset_type(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			num_assets_weight_hint: u32,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			// Grab supported assets
			let mut supported_assets = SupportedFeePaymentAssets::<T>::get();

			ensure!(
				num_assets_weight_hint >= (supported_assets.len() as u32),
				Error::<T>::TooLowNumAssetsWeightHint
			);

			let asset_type =
				AssetIdType::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

			// Remove from AssetIdType
			AssetIdType::<T>::remove(&asset_id);
			// Remove from AssetTypeId
			AssetTypeId::<T>::remove(&asset_type);
			// Remove previous asset type units per second
			AssetIdUnitsPerSecond::<T>::remove(&asset_id);

			// Only if the old asset is supported we need to remove it
			if let Ok(index) = supported_assets.binary_search(&asset_id) {
				supported_assets.remove(index);
				// Insert
				SupportedFeePaymentAssets::<T>::put(supported_assets);
			}

			Self::deposit_event(Event::ForeignAssetRemoved { asset_id, asset_type });
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
				AssetMetadatas::<T>::get(asset_id).unwrap_or_default();
			metadata.minimal_balance
		}
	}
}
