// Copyright 2020-2023 Trust Computing GmbH.
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

#![cfg(feature = "runtime-benchmarks")]

use crate::{AssetMetadata, BalanceOf, Call, Config, Pallet};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use xcm::latest::prelude::*;

benchmarks! {
	// This where clause allows us to create ForeignAssetTypes
	where_clause { where T::ForeignAssetType: From<Option<MultiLocation>> }
	register_foreign_asset_type {
		// does not really matter what we register
		let asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			Here)).into();
		let metadata = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};
		let next_asset_id = Pallet::<T>::foreign_asset_tracker();

	}: _(RawOrigin::Root, asset_type.clone(), metadata)
	verify {
		assert_eq!(Pallet::<T>::asset_id_type(next_asset_id), Some(asset_type));
	}

	update_foreign_asset_metadata {
		// does not really matter what we register
		let asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			Here)).into();
		let metadata = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};

		let asset_id = Pallet::<T>::foreign_asset_tracker();

		Pallet::<T>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			asset_type,
			metadata
		)?;

		let metadata_new = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 18,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};

	}: _(RawOrigin::Root, asset_id.clone(), metadata_new.clone())
	verify {
		assert_eq!(Pallet::<T>::asset_metadatas(asset_id), Some(metadata_new));
	}

	set_asset_units_per_second {
		// does not really matter what we register
		let asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			Here)).into();
		let metadata = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};
		let asset_id = Pallet::<T>::foreign_asset_tracker();
		Pallet::<T>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			asset_type,
			metadata
		)?;

	}: _(RawOrigin::Root, asset_id.clone(), 1)
	verify {
		assert_eq!(Pallet::<T>::asset_id_units_per_second(asset_id), 1);
	}

	add_asset_type {
		// We make it dependent on the number of existing assets already
		// does not really matter what we register, as long as it is different than the previous
		let asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			Here)).into();
		let metadata = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};

		let asset_id = Pallet::<T>::foreign_asset_tracker();

		Pallet::<T>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			asset_type,
			metadata
		)?;

		let new_asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			X1(GeneralIndex(1u128))
		)).into();
	}: _(RawOrigin::Root, asset_id.clone(), new_asset_type.clone())
	verify {
		assert_eq!(Pallet::<T>::asset_id_type(asset_id.clone()), Some(new_asset_type.clone()));
		assert_eq!(Pallet::<T>::asset_type_id(new_asset_type), Some(asset_id));
	}

	remove_asset_type {
		let x = 5;
		let asset_type: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			Here)).into();
		let metadata = AssetMetadata::<BalanceOf<T>> {
			name: "test".into(),
			symbol: "TST".into(),
			decimals: 12,
			minimal_balance: BalanceOf::<T>::default(),
			is_frozen: false,
		};

		let asset_id = Pallet::<T>::foreign_asset_tracker();

		Pallet::<T>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			asset_type,
			metadata
		)?;

		for i in 0..x {
			let asset_type:  T::ForeignAssetType = Some(MultiLocation::new(0, X1(GeneralIndex(i as u128)))).into();
			Pallet::<T>::add_asset_type(
				RawOrigin::Root.into(),
				asset_id.clone(),
				asset_type
			)?;
		}

		let asset_type_to_be_removed: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			X1(GeneralIndex((x-1) as u128))
		)).into();
		let asset_type_new_default: T::ForeignAssetType = Some(MultiLocation::new(
			0,
			X1(GeneralIndex((x-2) as u128))
		)).into();
	}: _(RawOrigin::Root, asset_type_to_be_removed.clone(), Some(asset_type_new_default.clone()))
	verify {
		assert_eq!(Pallet::<T>::asset_id_type(asset_id), Some(asset_type_new_default));
		assert!(Pallet::<T>::asset_type_id(asset_type_to_be_removed).is_none());
	}
}

#[cfg(test)]
mod tests {
	use crate::mock::Test;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		TestExternalities::new(t)
	}
}

impl_benchmark_test_suite!(Pallet, crate::benchmarking::tests::new_test_ext(), crate::mock::Test);
