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

//! Autogenerated weights for `pallet_chain_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-09-16, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `litentry-benchmark-server`, CPU: `Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("paseo-dev")`, DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=paseo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_chain_bridge
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/paseo/src/weights/pallet_chain_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_chain_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_chain_bridge::WeightInfo for WeightInfo<T> {
	/// Storage: `ChainBridge::RelayerThreshold` (r:0 w:1)
	/// Proof: `ChainBridge::RelayerThreshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_549_000 picoseconds.
		Weight::from_parts(15_004_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ChainBridge::ChainNonces` (r:1 w:1)
	/// Proof: `ChainBridge::ChainNonces` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn whitelist_chain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3607`
		// Minimum execution time: 20_130_000 picoseconds.
		Weight::from_parts(20_575_000, 0)
			.saturating_add(Weight::from_parts(0, 3607))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ChainBridge::Relayers` (r:1 w:1)
	/// Proof: `ChainBridge::Relayers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerCount` (r:1 w:1)
	/// Proof: `ChainBridge::RelayerCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3607`
		// Minimum execution time: 24_303_000 picoseconds.
		Weight::from_parts(24_817_000, 0)
			.saturating_add(Weight::from_parts(0, 3607))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ChainBridge::Relayers` (r:1 w:1)
	/// Proof: `ChainBridge::Relayers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerCount` (r:1 w:1)
	/// Proof: `ChainBridge::RelayerCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `223`
		//  Estimated: `3688`
		// Minimum execution time: 26_908_000 picoseconds.
		Weight::from_parts(27_510_000, 0)
			.saturating_add(Weight::from_parts(0, 3688))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ChainBridge::Relayers` (r:1 w:0)
	/// Proof: `ChainBridge::Relayers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::ChainNonces` (r:1 w:0)
	/// Proof: `ChainBridge::ChainNonces` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::Votes` (r:1 w:1)
	/// Proof: `ChainBridge::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerThreshold` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerThreshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerCount` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn acknowledge_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `3750`
		// Minimum execution time: 60_811_000 picoseconds.
		Weight::from_parts(62_026_000, 0)
			.saturating_add(Weight::from_parts(0, 3750))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ChainBridge::Relayers` (r:1 w:0)
	/// Proof: `ChainBridge::Relayers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::ChainNonces` (r:1 w:0)
	/// Proof: `ChainBridge::ChainNonces` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::Votes` (r:1 w:1)
	/// Proof: `ChainBridge::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerThreshold` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerThreshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerCount` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `3750`
		// Minimum execution time: 51_496_000 picoseconds.
		Weight::from_parts(52_238_000, 0)
			.saturating_add(Weight::from_parts(0, 3750))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ChainBridge::Votes` (r:1 w:1)
	/// Proof: `ChainBridge::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerThreshold` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerThreshold` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ChainBridge::RelayerCount` (r:1 w:0)
	/// Proof: `ChainBridge::RelayerCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn eval_vote_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `451`
		//  Estimated: `3916`
		// Minimum execution time: 26_973_000 picoseconds.
		Weight::from_parts(27_761_000, 0)
			.saturating_add(Weight::from_parts(0, 3916))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
