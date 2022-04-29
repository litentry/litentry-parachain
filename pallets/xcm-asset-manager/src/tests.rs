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

// Tests for AssetManager Pallet
use crate::*;
use mock::*;

use frame_support::{
	assert_noop, assert_ok,
};

#[test]
fn registering_foreign_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(1)
		);
		assert_eq!(
			AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(),
			0
		);
		expect_events(vec![
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1.clone(),
			},
		])
	});
}

#[test]
fn registering_foreign_errors() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_noop!(
			AssetManager::register_foreign_asset(
				Origin::root(),
				MockAssetType::MockAsset(1),
				asset_metadata_1.clone()
			),
			Error::<Test>::AssetAlreadyExists
		);
	});
}

#[test]
fn test_relocated_asset_id_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(1)
		);
		assert_ok!(AssetManager::relocate_foreign_asset_id(
			Origin::root(),
			10u32.into()
		));
		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_1.clone()
		));
		assert_eq!(
			AssetManager::asset_id_type(10).unwrap(),
			MockAssetType::MockAsset(2)
		);
	});
}

#[test]
fn test_update_foreign_asset_metadata_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		let asset_metadata_2 = crate::AssetMetadata::<BalanceOf<Test>> { name: "change".into(), symbol: "CHG".into(), decimals: 12, minimal_balance: 0, is_frozen: false };

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(1)
		);

		assert_noop!(
			AssetManager::update_foreign_asset_metadata(
				Origin::root(),
				MockAssetType::MockAsset(2),
				asset_metadata_2.clone()
			),
			Error::<Test>::AssetTypeDoesNotExist
		);

		assert_eq!(
			AssetManager::asset_metadatas(0).unwrap(),
			asset_metadata_1.clone()
		);


		assert_ok!(AssetManager::update_foreign_asset_metadata(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_2.clone()
		));
		
		assert_eq!(
			AssetManager::asset_metadatas(0).unwrap(),
			asset_metadata_2.clone()
		);
	});
}


#[test]
fn test_root_can_change_units_per_second() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
			0
		));

		assert_eq!(
			AssetManager::asset_id_units_per_second(0).unwrap(),
			200
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&0u32));

		expect_events(vec![
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1,
			},
			crate::Event::UnitsPerSecondChanged {
				asset_id: 0,
				units_per_second: 200,
			},
		])
	});
}

#[test]
fn test_regular_user_cannot_call_extrinsics() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		assert_noop!(
			AssetManager::register_foreign_asset(
				Origin::signed(1),
				MockAssetType::MockAsset(1),
				asset_metadata_1.clone()
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::relocate_foreign_asset_id(
				Origin::signed(1),
				10u32.into()
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::update_foreign_asset_metadata(
				Origin::signed(1),
				MockAssetType::MockAsset(1),
				asset_metadata_1.clone()
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::set_asset_units_per_second(
				Origin::signed(1),
				MockAssetType::MockAsset(1),
				200u128.into(),
				0
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::add_asset_type(
				Origin::signed(1),
				1,
				MockAssetType::MockAsset(2)
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::remove_asset_type(
				Origin::signed(1),
				MockAssetType::MockAsset(1),
				Some(MockAssetType::MockAsset(2)),
				0
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_root_can_add_asset_type() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		let asset_metadata_2 = crate::AssetMetadata::<BalanceOf<Test>> { name: "change".into(), symbol: "CHG".into(), decimals: 12, minimal_balance: 0, is_frozen: false };

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_2.clone()
		));

		assert_ok!(AssetManager::add_asset_type(
			Origin::root(),
			0,
			MockAssetType::MockAsset(3)
		));

		assert_noop!(
			AssetManager::add_asset_type(
				Origin::root(),
				0,
				MockAssetType::MockAsset(2)
			),
			Error::<Test>::AssetAlreadyExists
		);

		assert_noop!(
			AssetManager::add_asset_type(
				Origin::root(),
				2,
				MockAssetType::MockAsset(4)
			),
			Error::<Test>::AssetIdDoesNotExist
		);


		// New associations are stablished
		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(3)
		);
		assert_eq!(
			AssetManager::asset_type_id(MockAssetType::MockAsset(3)).unwrap(),
			0
		);
		// Old associations are remained, but not default
		assert_eq!(
			AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(),
			0
		);

		expect_events(vec![
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1.clone(),
			},
			crate::Event::ForeignAssetRegistered {
				asset_id: 1,
				asset_type: MockAssetType::MockAsset(2),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 1,
				metadata: asset_metadata_2.clone(),
			},
			crate::Event::ForeignAssetTypeAdded {
				asset_id: 0,
				new_asset_type: MockAssetType::MockAsset(3),
			},
		])
	});
}

#[test]
fn test_change_units_per_second_after_setting_it_once() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
			0
		));

		assert_eq!(
			AssetManager::asset_id_units_per_second(0).unwrap(),
			200
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&0u32));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			100u128.into(),
			1
		));

		assert_eq!(
			AssetManager::asset_id_units_per_second(0).unwrap(),
			100
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&0u32));

		expect_events(vec![
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1.clone(),
			},
			crate::Event::UnitsPerSecondChanged {
				asset_id: 0,
				units_per_second: 200,
			},
			crate::Event::UnitsPerSecondChanged {
				asset_id: 0,
				units_per_second: 100,
			},
		]);
	});
}

#[test]
// remove asset_type will remove all related fee setting.
fn test_root_can_remove_asset_type() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			100u128.into(),
			0
		));

		assert_ok!(AssetManager::add_asset_type(
			Origin::root(),
			0,
			MockAssetType::MockAsset(2)
		));

		assert_ok!(AssetManager::add_asset_type(
			Origin::root(),
			0,
			MockAssetType::MockAsset(3)
		));

		assert_eq!(
			AssetManager::asset_id_units_per_second(0).unwrap(),
			100
		);
		assert!(AssetManager::supported_fee_payment_assets().contains(&0u32));

		assert_ok!(AssetManager::remove_asset_type(
			Origin::root(),
			MockAssetType::MockAsset(1),
			None,
			0
		));

		assert!(AssetManager::asset_id_units_per_second(0).is_none());
		assert!(AssetManager::supported_fee_payment_assets().contains(&0u32)==false);

		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(3)
		);

		assert_ok!(AssetManager::remove_asset_type(
			Origin::root(),
			MockAssetType::MockAsset(3),
			Some(MockAssetType::MockAsset(2)),
			0
		));

		assert_eq!(
			AssetManager::asset_id_type(0).unwrap(),
			MockAssetType::MockAsset(2)
		);

		expect_events(vec![
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1.clone(),
			},
			crate::Event::UnitsPerSecondChanged {
				asset_id: 0,
				units_per_second: 100,
			},
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(2),
			},
			crate::Event::ForeignAssetRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(3),
			},
			crate::Event::SupportedAssetTypeRemoved {
				asset_id: 0, 
				removed_asset_type: MockAssetType::MockAsset(1), 
				default_asset_type: MockAssetType::MockAsset(3),
			},
			crate::Event::SupportedAssetTypeRemoved {
				asset_id: 0, 
				removed_asset_type: MockAssetType::MockAsset(1), 
				default_asset_type: MockAssetType::MockAsset(2),
			},
		]);
	});
}

#[test]
fn test_weight_hint_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> { name: "test".into(), symbol: "TST".into(), decimals: 12, minimal_balance: 0, is_frozen: false };
		assert_ok!(AssetManager::register_foreign_asset(
			Origin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::set_asset_units_per_second(
			Origin::root(),
			MockAssetType::MockAsset(1),
			200u128.into(),
			0
		));

		assert_noop!(
			AssetManager::set_asset_units_per_second(
				Origin::root(),
				MockAssetType::MockAsset(1),
				100u128.into(),
				0
			),
			Error::<Test>::TooLowNumAssetsWeightHint
		);
	});
}

#[test]
fn test_asset_id_non_existent_error() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AssetManager::set_asset_units_per_second(
				Origin::root(),
				MockAssetType::MockAsset(1),
				200u128.into(),
				0
			),
			Error::<Test>::AssetTypeDoesNotExist
		);
		assert_noop!(
			AssetManager::add_asset_type(
				Origin::root(),
				1,
				MockAssetType::MockAsset(2)
			),
			Error::<Test>::AssetIdDoesNotExist
		);
	});
}
