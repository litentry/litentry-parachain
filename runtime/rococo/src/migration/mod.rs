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
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, OnRuntimeUpgrade},
	Blake2_128Concat, StorageHasher,
};

use sp_std::marker::PhantomData;

use pallet_asset_manager::AssetMetadata;
use primitives::AssetId;
use runtime_common::BaseRuntimeRequirements;

pub struct AddNativeTokenForeignAssetTypeForAssetManager<T>(PhantomData<T>);
impl<T> OnRuntimeUpgrade for AddNativeTokenForeignAssetTypeForAssetManager<T>
where
	T: BaseRuntimeRequirements + pallet_asset_manager::Config,
	<T as frame_system::Config>::Event: From<pallet_asset_manager::Event<T>>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		log::info!("Pre check pallet asset manager exists and missing genesis");
		// This runtime uprade is for the fix of missing genesis build in old pallet asset manager
		// code Check the non-existence of AssetIdType
		let old_native_token_foreign_asset_type =
			frame_support::storage::migration::get_storage_value::<
				<T as pallet_asset_manager::Config>::ForeignAssetType,
			>(b"AssetManager", b"AssetIdType", &Blake2_128Concat::hash(&0u128.encode()));
		assert!(
			old_native_token_foreign_asset_type.is_none(),
			"NativeTokenForeignAssetType occupied already"
		);

		// Check the non-existence of AssetTypeId
		let old_native_token_asset_id = frame_support::storage::migration::get_storage_value::<
			AssetId,
		>(
			b"AssetManager",
			b"AssetTypeId",
			&Blake2_128Concat::hash(
				&<T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get().encode(),
			),
		);
		assert!(old_native_token_asset_id.is_none(), "NativeTokenAssetId occupied already");

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		type BalanceOf<T> = <<T as pallet_asset_manager::Config>::Currency as Currency<
			<T as frame_system::Config>::AccountId,
		>>::Balance;

		frame_support::storage::migration::put_storage_value::<
			<T as pallet_asset_manager::Config>::ForeignAssetType,
		>(
			b"AssetManager",
			b"AssetIdType",
			&Blake2_128Concat::hash(&0u128.encode()),
			<T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get(),
		);

		frame_support::storage::migration::put_storage_value::<AssetId>(
			b"AssetManager",
			b"AssetTypeId",
			&Blake2_128Concat::hash(
				&<T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get().encode(),
			),
			0u128,
		);

		frame_support::storage::migration::put_storage_value::<AssetMetadata<BalanceOf<T>>>(
			b"AssetManager",
			b"AssetIdMetadata",
			&Blake2_128Concat::hash(&0u128.encode()),
			Default::default(),
		);

		// Deposit event at RuntimeUpgrade
		<frame_system::Pallet<T>>::deposit_event(
			pallet_asset_manager::Event::ForeignAssetTypeRegistered {
				asset_id: Default::default(),
				asset_type: <T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get(),
			},
		);

		<frame_system::Pallet<T>>::deposit_event(
			pallet_asset_manager::Event::ForeignAssetMetadataUpdated {
				asset_id: Default::default(),
				metadata: Default::default(),
			},
		);

		// TODO: Very Weak safety
		let entries: u64 = 4 + 100;
		<T as frame_system::Config>::DbWeight::get().writes(entries)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		log::info!("Post check pallet asset manager and genesis existence");
		// This runtime uprade is for the fix of missing genesis build in old pallet asset manager
		// code Check the existence of AssetIdType
		let new_native_token_foreign_asset_type =
			frame_support::storage::migration::get_storage_value::<
				<T as pallet_asset_manager::Config>::ForeignAssetType,
			>(b"AssetManager", b"AssetIdType", &Blake2_128Concat::hash(&0u128.encode()))
			.expect("Storage query fails: AssetIdType genesis not added");
		assert!(
			new_native_token_foreign_asset_type ==
				<T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get(),
			"Wrong NativeTokenForeignAssetType added"
		);

		// Check the existence of AssetTypeId
		let new_native_token_asset_id = frame_support::storage::migration::get_storage_value::<
			AssetId,
		>(
			b"AssetManager",
			b"AssetTypeId",
			&Blake2_128Concat::hash(
				&<T as pallet_asset_manager::Config>::NativeTokenForeignAssetType::get().encode(),
			),
		)
		.expect("Storage query fails: AssetTypeId genesis not added");
		assert!(new_native_token_asset_id == 0u128, "Wrong NativeTokenAssetId added");

		Ok(())
	}
}
