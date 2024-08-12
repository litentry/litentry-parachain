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

//! Autogenerated weights for `pallet_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-08-12, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `litentry-benchmark-server`, CPU: `Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_bridge
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge::WeightInfo for WeightInfo<T> {
	/// Storage: ChainBridge RelayerThreshold (r:0 w:1)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	fn set_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_564_000 picoseconds.
		Weight::from_parts(16_026_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Resources (r:0 w:1)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	fn set_resource() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_208_000 picoseconds.
		Weight::from_parts(9_561_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Resources (r:0 w:1)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	fn remove_resource() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_047_000 picoseconds.
		Weight::from_parts(8_445_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge ChainNonces (r:1 w:1)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	fn whitelist_chain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 21_346_000 picoseconds.
		Weight::from_parts(22_363_000, 0)
			.saturating_add(Weight::from_parts(0, 3574))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:1)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:1)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn add_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 25_312_000 picoseconds.
		Weight::from_parts(25_841_000, 0)
			.saturating_add(Weight::from_parts(0, 3574))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ChainBridge Relayers (r:1 w:1)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:1)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `190`
		//  Estimated: `3655`
		// Minimum execution time: 29_400_000 picoseconds.
		Weight::from_parts(30_129_000, 0)
			.saturating_add(Weight::from_parts(0, 3655))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ChainBridge BridgeFee (r:0 w:1)
	/// Proof Skipped: ChainBridge BridgeFee (max_values: None, max_size: None, mode: Measured)
	fn update_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_073_000 picoseconds.
		Weight::from_parts(17_532_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:0)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge ChainNonces (r:1 w:0)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Resources (r:1 w:0)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn acknowledge_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `307`
		//  Estimated: `3772`
		// Minimum execution time: 69_949_000 picoseconds.
		Weight::from_parts(71_229_000, 0)
			.saturating_add(Weight::from_parts(0, 3772))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:0)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge ChainNonces (r:1 w:0)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Resources (r:1 w:0)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `307`
		//  Estimated: `3772`
		// Minimum execution time: 60_408_000 picoseconds.
		Weight::from_parts(61_402_000, 0)
			.saturating_add(Weight::from_parts(0, 3772))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn eval_vote_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `451`
		//  Estimated: `3916`
		// Minimum execution time: 29_804_000 picoseconds.
		Weight::from_parts(30_741_000, 0)
			.saturating_add(Weight::from_parts(0, 3916))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
