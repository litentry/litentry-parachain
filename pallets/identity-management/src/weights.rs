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

//! Autogenerated weights for pallet_identity_management
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-30, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tee-dev`, CPU: `Intel(R) Xeon(R) E-2274G CPU @ 4.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_identity_management
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/identity-management/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_identity_management.
pub trait WeightInfo {
	fn add_delegatee() -> Weight;
	fn remove_delegatee() -> Weight;
	fn link_identity() -> Weight;
	fn deactivate_identity() -> Weight;
	fn activate_identity() -> Weight;
	fn update_id_graph_hash() -> Weight;
	fn identity_linked() -> Weight;
	fn identity_deactivated() -> Weight;
	fn identity_activated() -> Weight;
	fn some_error() -> Weight;
}

/// Weights for pallet_identity_management using the Substrate node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	/// Storage: IdentityManagement Delegatee (r:0 w:1)
	/// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_476_000 picoseconds.
		Weight::from_parts(12_793_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: IdentityManagement Delegatee (r:1 w:1)
	/// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `3513`
		// Minimum execution time: 18_305_000 picoseconds.
		Weight::from_parts(18_709_000, 3513)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 17_266_000 picoseconds.
		Weight::from_parts(17_608_000, 1561)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn deactivate_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 15_515_000 picoseconds.
		Weight::from_parts(15_775_000, 1561)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 15_495_000 picoseconds.
		Weight::from_parts(15_770_000, 1561)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: IdentityManagement IDGraphHash (r:0 w:1)
	/// Proof: IdentityManagement IDGraphHash (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn update_id_graph_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 20_641_000 picoseconds.
		Weight::from_parts(20_986_000, 3720)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_linked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_140_000 picoseconds.
		Weight::from_parts(18_549_000, 3720)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_deactivated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_157_000 picoseconds.
		Weight::from_parts(18_510_000, 3720)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_activated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_225_000 picoseconds.
		Weight::from_parts(18_547_000, 3720)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn some_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_094_000 picoseconds.
		Weight::from_parts(18_477_000, 3720)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: IdentityManagement Delegatee (r:0 w:1)
	/// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_476_000 picoseconds.
		Weight::from_parts(12_793_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: IdentityManagement Delegatee (r:1 w:1)
	/// Proof: IdentityManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `3513`
		// Minimum execution time: 18_305_000 picoseconds.
		Weight::from_parts(18_709_000, 3513)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn link_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 17_266_000 picoseconds.
		Weight::from_parts(17_608_000, 1561)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn deactivate_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 15_515_000 picoseconds.
		Weight::from_parts(15_775_000, 1561)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: IMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: IMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1561`
		// Minimum execution time: 15_495_000 picoseconds.
		Weight::from_parts(15_770_000, 1561)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: IdentityManagement IDGraphHash (r:0 w:1)
	/// Proof: IdentityManagement IDGraphHash (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn update_id_graph_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 20_641_000 picoseconds.
		Weight::from_parts(20_986_000, 3720)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_linked() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_140_000 picoseconds.
		Weight::from_parts(18_549_000, 3720)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_deactivated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_157_000 picoseconds.
		Weight::from_parts(18_510_000, 3720)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn identity_activated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_225_000 picoseconds.
		Weight::from_parts(18_547_000, 3720)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn some_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `3720`
		// Minimum execution time: 18_094_000 picoseconds.
		Weight::from_parts(18_477_000, 3720)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
