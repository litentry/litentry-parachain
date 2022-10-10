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

//! Autogenerated weights for pallet_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-09, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
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
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/bridge/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge.
pub trait WeightInfo {
	fn set_threshold() -> Weight;
	fn set_resource() -> Weight;
	fn remove_resource() -> Weight;
	fn whitelist_chain() -> Weight;
	fn add_relayer() -> Weight;
	fn remove_relayer() -> Weight;
	fn update_fee() -> Weight;
	fn acknowledge_proposal() -> Weight;
	fn reject_proposal() -> Weight;
	fn eval_vote_state() -> Weight;
}

/// Weights for pallet_bridge using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: ChainBridge RelayerThreshold (r:0 w:1)
	fn set_threshold() -> Weight {
		(12_574_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Resources (r:0 w:1)
	fn set_resource() -> Weight {
		(5_120_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Resources (r:0 w:1)
	fn remove_resource() -> Weight {
		(4_819_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge ChainNonces (r:1 w:1)
	fn whitelist_chain() -> Weight {
		(15_179_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:1)
	// Storage: ChainBridge RelayerCount (r:1 w:1)
	fn add_relayer() -> Weight {
		(17_723_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:1)
	// Storage: ChainBridge RelayerCount (r:1 w:1)
	fn remove_relayer() -> Weight {
		(18_956_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ChainBridge BridgeFee (r:0 w:1)
	fn update_fee() -> Weight {
		(13_085_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:0)
	// Storage: ChainBridge ChainNonces (r:1 w:0)
	// Storage: ChainBridge Resources (r:1 w:0)
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn acknowledge_proposal() -> Weight {
		(45_447_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:0)
	// Storage: ChainBridge ChainNonces (r:1 w:0)
	// Storage: ChainBridge Resources (r:1 w:0)
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn reject_proposal() -> Weight {
		(39_255_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn eval_vote_state() -> Weight {
		(15_891_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ChainBridge RelayerThreshold (r:0 w:1)
	fn set_threshold() -> Weight {
		(12_574_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Resources (r:0 w:1)
	fn set_resource() -> Weight {
		(5_120_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Resources (r:0 w:1)
	fn remove_resource() -> Weight {
		(4_819_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge ChainNonces (r:1 w:1)
	fn whitelist_chain() -> Weight {
		(15_179_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:1)
	// Storage: ChainBridge RelayerCount (r:1 w:1)
	fn add_relayer() -> Weight {
		(17_723_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:1)
	// Storage: ChainBridge RelayerCount (r:1 w:1)
	fn remove_relayer() -> Weight {
		(18_956_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ChainBridge BridgeFee (r:0 w:1)
	fn update_fee() -> Weight {
		(13_085_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:0)
	// Storage: ChainBridge ChainNonces (r:1 w:0)
	// Storage: ChainBridge Resources (r:1 w:0)
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn acknowledge_proposal() -> Weight {
		(45_447_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Relayers (r:1 w:0)
	// Storage: ChainBridge ChainNonces (r:1 w:0)
	// Storage: ChainBridge Resources (r:1 w:0)
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn reject_proposal() -> Weight {
		(39_255_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ChainBridge Votes (r:1 w:1)
	// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	// Storage: ChainBridge RelayerCount (r:1 w:0)
	fn eval_vote_state() -> Weight {
		(15_891_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
