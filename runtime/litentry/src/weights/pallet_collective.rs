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

//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: Council Members (r:1 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:100 w:100)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `32761 + m * (1968 ±23) + p * (4247 ±23)`
		// Minimum execution time: 32_099_000 picoseconds.
		Weight::from_parts(32_610_000, 0)
			.saturating_add(Weight::from_parts(0, 32761))
			// Standard Error: 124_918
			.saturating_add(Weight::from_parts(9_593_736, 0).saturating_mul(m.into()))
			// Standard Error: 124_918
			.saturating_add(Weight::from_parts(13_852_408, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1968).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4247).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103 + m * (32 ±0)`
		//  Estimated: `1588 + m * (32 ±0)`
		// Minimum execution time: 29_668_000 picoseconds.
		Weight::from_parts(30_405_707, 0)
			.saturating_add(Weight::from_parts(0, 1588))
			// Standard Error: 75
			.saturating_add(Weight::from_parts(1_900, 0).saturating_mul(b.into()))
			// Standard Error: 781
			.saturating_add(Weight::from_parts(32_487, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:0)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103 + m * (32 ±0)`
		//  Estimated: `3568 + m * (32 ±0)`
		// Minimum execution time: 36_591_000 picoseconds.
		Weight::from_parts(37_417_220, 0)
			.saturating_add(Weight::from_parts(0, 3568))
			// Standard Error: 98
			.saturating_add(Weight::from_parts(1_773, 0).saturating_mul(b.into()))
			// Standard Error: 1_015
			.saturating_add(Weight::from_parts(50_693, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `3821 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 49_491_000 picoseconds.
		Weight::from_parts(49_422_597, 0)
			.saturating_add(Weight::from_parts(0, 3821))
			// Standard Error: 176
			.saturating_add(Weight::from_parts(4_458, 0).saturating_mul(b.into()))
			// Standard Error: 1_831
			.saturating_add(Weight::from_parts(36_135, 0).saturating_mul(m.into()))
			// Standard Error: 1_813
			.saturating_add(Weight::from_parts(260_744, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841 + m * (64 ±0)`
		//  Estimated: `4306 + m * (64 ±0)`
		// Minimum execution time: 43_051_000 picoseconds.
		Weight::from_parts(44_763_728, 0)
			.saturating_add(Weight::from_parts(0, 4306))
			// Standard Error: 970
			.saturating_add(Weight::from_parts(58_889, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `468 + m * (64 ±0) + p * (35 ±0)`
		//  Estimated: `3856 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 52_412_000 picoseconds.
		Weight::from_parts(52_828_284, 0)
			.saturating_add(Weight::from_parts(0, 3856))
			// Standard Error: 1_712
			.saturating_add(Weight::from_parts(48_100, 0).saturating_mul(m.into()))
			// Standard Error: 1_656
			.saturating_add(Weight::from_parts(243_618, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `778 + m * (64 ±0) + p * (39 ±0)`
		//  Estimated: `4117 + b * (1 ±0) + m * (65 ±0) + p * (40 ±0)`
		// Minimum execution time: 71_762_000 picoseconds.
		Weight::from_parts(76_978_821, 0)
			.saturating_add(Weight::from_parts(0, 4117))
			// Standard Error: 203
			.saturating_add(Weight::from_parts(1_679, 0).saturating_mul(b.into()))
			// Standard Error: 2_161
			.saturating_add(Weight::from_parts(31_752, 0).saturating_mul(m.into()))
			// Standard Error: 2_091
			.saturating_add(Weight::from_parts(273_337, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `488 + m * (64 ±0) + p * (35 ±0)`
		//  Estimated: `3876 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 57_212_000 picoseconds.
		Weight::from_parts(56_540_891, 0)
			.saturating_add(Weight::from_parts(0, 3876))
			// Standard Error: 1_630
			.saturating_add(Weight::from_parts(51_900, 0).saturating_mul(m.into()))
			// Standard Error: 1_577
			.saturating_add(Weight::from_parts(254_014, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `798 + m * (64 ±0) + p * (39 ±0)`
		//  Estimated: `4137 + b * (1 ±0) + m * (65 ±0) + p * (40 ±0)`
		// Minimum execution time: 77_494_000 picoseconds.
		Weight::from_parts(76_063_007, 0)
			.saturating_add(Weight::from_parts(0, 4137))
			// Standard Error: 416
			.saturating_add(Weight::from_parts(7_233, 0).saturating_mul(b.into()))
			// Standard Error: 4_423
			.saturating_add(Weight::from_parts(9_006, 0).saturating_mul(m.into()))
			// Standard Error: 4_280
			.saturating_add(Weight::from_parts(351_269, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (32 ±0)`
		//  Estimated: `1745 + p * (32 ±0)`
		// Minimum execution time: 30_514_000 picoseconds.
		Weight::from_parts(33_865_913, 0)
			.saturating_add(Weight::from_parts(0, 1745))
			// Standard Error: 1_299
			.saturating_add(Weight::from_parts(216_100, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}
