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

// Tests for AssetManager Pallet
use crate::*;
use mock::*;

use frame_support::{assert_noop, assert_ok};

#[test]
fn registering_foreign_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(1));
		assert_eq!(AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(), 0);
		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
		])
	});
}

#[test]
fn registering_foreign_errors() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_noop!(
			AssetManager::register_foreign_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(1),
				asset_metadata_1.clone()
			),
			Error::<Test>::AssetAlreadyExists
		);

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
		]);
	});
}

#[test]
fn test_relocated_asset_id_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(1));

		crate::ForeignAssetTracker::<Test>::put(10);
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_1.clone()
		));
		assert_eq!(AssetManager::asset_id_type(10).unwrap(), MockAssetType::MockAsset(2));

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated {
				asset_id: 0,
				metadata: asset_metadata_1.clone(),
			},
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 10,
				asset_type: MockAssetType::MockAsset(2),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 10, metadata: asset_metadata_1 },
		]);
	});
}

#[test]
fn test_update_foreign_asset_metadata_works() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		let asset_metadata_2 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "change".into(),
			symbol: "CHG".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));
		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(1));
		assert_noop!(
			AssetManager::update_foreign_asset_metadata(
				RuntimeOrigin::root(),
				2,
				asset_metadata_2.clone()
			),
			Error::<Test>::AssetIdDoesNotExist
		);
		assert_eq!(AssetManager::asset_metadatas(0).unwrap(), asset_metadata_1);

		assert_ok!(AssetManager::update_foreign_asset_metadata(
			RuntimeOrigin::root(),
			0,
			asset_metadata_2.clone()
		));
		assert_eq!(AssetManager::asset_metadatas(0).unwrap(), asset_metadata_2);

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_2 },
		]);
	});
}

#[test]
fn test_root_can_change_units_per_second() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));
		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(1));
		assert_eq!(AssetManager::asset_metadatas(0).unwrap(), asset_metadata_1);

		assert!(!AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), None);

		assert_ok!(AssetManager::set_asset_units_per_second(RuntimeOrigin::root(), 0, 200u128));

		assert!(AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::asset_id_units_per_second(0), 200);
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), Some(200));

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
			crate::Event::UnitsPerSecondChanged { asset_id: 0, units_per_second: 200 },
		]);
	});
}

#[test]
fn test_regular_user_cannot_call_extrinsics() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_noop!(
			AssetManager::register_foreign_asset_type(
				RuntimeOrigin::signed(1),
				MockAssetType::MockAsset(1),
				asset_metadata_1.clone()
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::update_foreign_asset_metadata(
				RuntimeOrigin::signed(1),
				0,
				asset_metadata_1
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::set_asset_units_per_second(RuntimeOrigin::signed(1), 0, 200u128),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::add_asset_type(RuntimeOrigin::signed(1), 1, MockAssetType::MockAsset(2)),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::signed(1),
				MockAssetType::MockAsset(1),
				Some(MockAssetType::MockAsset(2))
			),
			sp_runtime::DispatchError::BadOrigin
		);

		expect_events(vec![]);
	});
}

#[test]
fn test_root_can_add_asset_type() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		let asset_metadata_2 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "change".into(),
			symbol: "CHG".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_2.clone()
		));

		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			0,
			MockAssetType::MockAsset(3)
		));

		assert_noop!(
			AssetManager::add_asset_type(RuntimeOrigin::root(), 0, MockAssetType::MockAsset(2)),
			Error::<Test>::AssetAlreadyExists
		);

		assert_noop!(
			AssetManager::add_asset_type(RuntimeOrigin::root(), 2, MockAssetType::MockAsset(4)),
			Error::<Test>::AssetIdDoesNotExist
		);

		// New associations are stablished
		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(3));
		assert_eq!(AssetManager::asset_type_id(MockAssetType::MockAsset(3)).unwrap(), 0);
		// Old associations are remained, but not default
		assert_eq!(AssetManager::asset_type_id(MockAssetType::MockAsset(1)).unwrap(), 0);

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 1,
				asset_type: MockAssetType::MockAsset(2),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 1, metadata: asset_metadata_2 },
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(3),
			},
		])
	});
}

#[test]
fn test_change_units_per_second_after_setting_it_once() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert!(!AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), None);

		assert_ok!(AssetManager::set_asset_units_per_second(RuntimeOrigin::root(), 0, 200u128));
		assert!(AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::asset_id_units_per_second(0), 200);
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), Some(200));

		assert_ok!(AssetManager::set_asset_units_per_second(RuntimeOrigin::root(), 0, 100u128));

		assert!(AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::asset_id_units_per_second(0), 100);
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), Some(100));

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
			crate::Event::UnitsPerSecondChanged { asset_id: 0, units_per_second: 200 },
			crate::Event::UnitsPerSecondChanged { asset_id: 0, units_per_second: 100 },
		]);
	});
}

#[test]
// remove asset_type will remove all related fee setting.
fn test_root_can_remove_asset_type() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));

		assert_ok!(AssetManager::set_asset_units_per_second(RuntimeOrigin::root(), 0, 100u128));

		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			0,
			MockAssetType::MockAsset(2)
		));
		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			0,
			MockAssetType::MockAsset(3)
		));

		assert!(AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::asset_id_units_per_second(0), 100);
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), Some(100));

		assert_ok!(AssetManager::remove_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			None
		));

		assert!(!AssetManager::payment_is_supported(MockAssetType::MockAsset(1)));
		assert_eq!(AssetManager::asset_id_units_per_second(0), 100);
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(1)), None);
		assert!(AssetManager::payment_is_supported(MockAssetType::MockAsset(2)));
		assert_eq!(AssetManager::get_units_per_second(MockAssetType::MockAsset(2)), Some(100));

		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(3));

		assert_ok!(AssetManager::remove_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(3),
			Some(MockAssetType::MockAsset(2))
		));

		assert_eq!(AssetManager::asset_id_type(0).unwrap(), MockAssetType::MockAsset(2));

		expect_events(vec![
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(1),
			},
			crate::Event::ForeignAssetMetadataUpdated { asset_id: 0, metadata: asset_metadata_1 },
			crate::Event::UnitsPerSecondChanged { asset_id: 0, units_per_second: 100 },
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(2),
			},
			crate::Event::ForeignAssetTypeRegistered {
				asset_id: 0,
				asset_type: MockAssetType::MockAsset(3),
			},
			crate::Event::ForeignAssetTypeRemoved {
				asset_id: 0,
				removed_asset_type: MockAssetType::MockAsset(1),
				default_asset_type: MockAssetType::MockAsset(3),
			},
			crate::Event::ForeignAssetTypeRemoved {
				asset_id: 0,
				removed_asset_type: MockAssetType::MockAsset(3),
				default_asset_type: MockAssetType::MockAsset(2),
			},
		]);
	});
}

#[test]
fn test_malicious_remove_asset_type_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_1
		));
		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			0,
			MockAssetType::MockAsset(3)
		));
		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			1,
			MockAssetType::MockAsset(4)
		));

		// try assign asset_type=4 (which belongs to asset_id=1) to asset_id=0
		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(1),
				Some(MockAssetType::MockAsset(4))
			),
			Error::<Test>::AssetAlreadyExists
		);
	})
}

#[test]
fn test_asset_id_non_existent_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};

		assert_noop!(
			AssetManager::update_foreign_asset_metadata(RuntimeOrigin::root(), 0, asset_metadata_1),
			Error::<Test>::AssetIdDoesNotExist
		);
		assert_noop!(
			AssetManager::set_asset_units_per_second(RuntimeOrigin::root(), 0, 200u128),
			Error::<Test>::AssetIdDoesNotExist
		);
		assert_noop!(
			AssetManager::add_asset_type(RuntimeOrigin::root(), 1, MockAssetType::MockAsset(2)),
			Error::<Test>::AssetIdDoesNotExist
		);
	});
}

#[test]
fn test_asset_already_exists_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));
		assert_noop!(
			AssetManager::register_foreign_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(1),
				asset_metadata_1
			),
			Error::<Test>::AssetAlreadyExists
		);
		assert_noop!(
			AssetManager::add_asset_type(RuntimeOrigin::root(), 0, MockAssetType::MockAsset(1)),
			Error::<Test>::AssetAlreadyExists
		);

		//remove_asset_type's error is tested in test_malicious_remove_asset_type_fail
	});
}

#[test]
fn test_asset_type_does_not_exist_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1
		));
		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(2),
				None
			),
			Error::<Test>::AssetTypeDoesNotExist
		);
		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(1),
				Some(MockAssetType::MockAsset(2))
			),
			Error::<Test>::AssetTypeDoesNotExist
		);
	});
}

#[test]
fn test_default_asset_type_removed_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1
		));
		assert_ok!(AssetManager::add_asset_type(
			RuntimeOrigin::root(),
			0,
			MockAssetType::MockAsset(2)
		));

		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(2),
				None
			),
			Error::<Test>::DefaultAssetTypeRemoved
		);
		assert_noop!(
			AssetManager::remove_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(1),
				Some(MockAssetType::MockAsset(1))
			),
			Error::<Test>::DefaultAssetTypeRemoved
		);
	});
}

#[test]
fn test_asset_id_over_flow_error() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_metadata_1 = crate::AssetMetadata::<BalanceOf<Test>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: 0,
			is_frozen: false,
		};
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(1),
			asset_metadata_1.clone()
		));
		crate::ForeignAssetTracker::<Test>::put(10);
		assert_ok!(AssetManager::register_foreign_asset_type(
			RuntimeOrigin::root(),
			MockAssetType::MockAsset(2),
			asset_metadata_1.clone()
		));
		crate::ForeignAssetTracker::<Test>::put(u32::MAX);
		assert_noop!(
			AssetManager::register_foreign_asset_type(
				RuntimeOrigin::root(),
				MockAssetType::MockAsset(3),
				asset_metadata_1
			),
			Error::<Test>::AssetIdLimitReached
		);
	});
}
