/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_sidechain.
pub trait WeightInfo {
	fn confirm_imported_sidechain_block() -> Weight;
}

/// Weights for pallet_sidechain using the Integritee parachain node and recommended hardware.
pub struct IntegriteeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for IntegriteeWeight<T> {
	fn confirm_imported_sidechain_block() -> Weight {
		Weight::from_ref_time(46_200_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For tests
impl WeightInfo for () {
	fn confirm_imported_sidechain_block() -> Weight {
		Weight::from_ref_time(46_200_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}
