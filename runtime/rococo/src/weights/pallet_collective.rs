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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-09, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_collective
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 29_051 nanoseconds.
		Weight::from_ref_time(29_463_000 as u64)
			// Standard Error: 129_093
			.saturating_add(Weight::from_ref_time(9_125_591 as u64).saturating_mul(m as u64))
			// Standard Error: 129_093
			.saturating_add(Weight::from_ref_time(14_400_404 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 36_142 nanoseconds.
		Weight::from_ref_time(35_603_280 as u64)
			// Standard Error: 475
			.saturating_add(Weight::from_ref_time(2_782 as u64).saturating_mul(b as u64))
			// Standard Error: 4_894
			.saturating_add(Weight::from_ref_time(49_645 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, _m: u32, ) -> Weight {
		// Minimum execution time: 38_979 nanoseconds.
		Weight::from_ref_time(49_687_916 as u64)
			// Standard Error: 806
			.saturating_add(Weight::from_ref_time(530 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 51_246 nanoseconds.
		Weight::from_ref_time(34_616_371 as u64)
			// Standard Error: 2_698
			.saturating_add(Weight::from_ref_time(6_779 as u64).saturating_mul(b as u64))
			// Standard Error: 28_040
			.saturating_add(Weight::from_ref_time(193_072 as u64).saturating_mul(m as u64))
			// Standard Error: 27_761
			.saturating_add(Weight::from_ref_time(665_384 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(_m: u32, ) -> Weight {
		// Minimum execution time: 66_290 nanoseconds.
		Weight::from_ref_time(90_068_070 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 57_046 nanoseconds.
		Weight::from_ref_time(69_909_081 as u64)
			// Standard Error: 31_972
			.saturating_add(Weight::from_ref_time(152_500 as u64).saturating_mul(m as u64))
			// Standard Error: 30_930
			.saturating_add(Weight::from_ref_time(256_463 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 77_560 nanoseconds.
		Weight::from_ref_time(92_043_303 as u64)
			// Standard Error: 2_744
			.saturating_add(Weight::from_ref_time(1_456 as u64).saturating_mul(b as u64))
			// Standard Error: 29_185
			.saturating_add(Weight::from_ref_time(94_386 as u64).saturating_mul(m as u64))
			// Standard Error: 28_238
			.saturating_add(Weight::from_ref_time(443_082 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 61_543 nanoseconds.
		Weight::from_ref_time(39_069_935 as u64)
			// Standard Error: 58_164
			.saturating_add(Weight::from_ref_time(396_346 as u64).saturating_mul(m as u64))
			// Standard Error: 56_268
			.saturating_add(Weight::from_ref_time(549_127 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 81_209 nanoseconds.
		Weight::from_ref_time(70_743_362 as u64)
			// Standard Error: 2_800
			.saturating_add(Weight::from_ref_time(2_345 as u64).saturating_mul(b as u64))
			// Standard Error: 29_783
			.saturating_add(Weight::from_ref_time(153_161 as u64).saturating_mul(m as u64))
			// Standard Error: 28_817
			.saturating_add(Weight::from_ref_time(671_800 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 36_316 nanoseconds.
		Weight::from_ref_time(41_087_182 as u64)
			// Standard Error: 24_758
			.saturating_add(Weight::from_ref_time(565_325 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
