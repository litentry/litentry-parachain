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

//! Autogenerated weights for pallet_extrinsic_filter
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-07, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// --chain=dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_extrinsic_filter
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/extrinsic-filter/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_extrinsic_filter.
pub trait WeightInfo {
	fn block_extrinsics(p: u32, f: u32, ) -> Weight;
	fn unblock_extrinsics(p: u32, f: u32, ) -> Weight;
}

/// Weights for pallet_extrinsic_filter using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn block_extrinsics(_p: u32, f: u32, ) -> Weight {
		Weight::from_parts(30_218_000 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_000 as u64, 0).saturating_mul(f as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn unblock_extrinsics(p: u32, f: u32, ) -> Weight {
		Weight::from_parts(29_586_000 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(4_000 as u64, 0).saturating_mul(p as u64))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(3_000 as u64, 0).saturating_mul(f as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn block_extrinsics(_p: u32, f: u32, ) -> Weight {
		Weight::from_parts(30_218_000 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(1_000 as u64, 0).saturating_mul(f as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	fn unblock_extrinsics(p: u32, f: u32, ) -> Weight {
		Weight::from_parts(29_586_000 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(4_000 as u64, 0).saturating_mul(p as u64))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(3_000 as u64, 0).saturating_mul(f as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}