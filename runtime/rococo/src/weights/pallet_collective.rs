// Copyright 2020-2023 Litentry Technologies GmbH.
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
//! DATE: 2023-07-19, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
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
		//  Measured:  `0 + m * (3233 ±0) + p * (3223 ±0)`
		//  Estimated: `33287 + m * (7730 ±23) + p * (10156 ±23)`
		// Minimum execution time: 26_284 nanoseconds.
		Weight::from_ref_time(26_554_000)
			.saturating_add(Weight::from_proof_size(33287))
			// Standard Error: 121_280
			.saturating_add(Weight::from_ref_time(8_742_277).saturating_mul(m.into()))
			// Standard Error: 121_280
			.saturating_add(Weight::from_ref_time(13_206_275).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(7730).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(10156).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(_b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135 + m * (32 ±0)`
		//  Estimated: `630 + m * (32 ±0)`
		// Minimum execution time: 24_460 nanoseconds.
		Weight::from_ref_time(28_086_180)
			.saturating_add(Weight::from_proof_size(630))
			// Standard Error: 3_683
			.saturating_add(Weight::from_ref_time(17_350).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:0)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135 + m * (32 ±0)`
		//  Estimated: `3240 + m * (64 ±0)`
		// Minimum execution time: 28_008 nanoseconds.
		Weight::from_ref_time(29_996_884)
			.saturating_add(Weight::from_proof_size(3240))
			// Standard Error: 461
			.saturating_add(Weight::from_ref_time(1_622).saturating_mul(b.into()))
			// Standard Error: 4_743
			.saturating_add(Weight::from_ref_time(22_641).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(m.into()))
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
		//  Measured:  `527 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `6020 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 37_514 nanoseconds.
		Weight::from_ref_time(41_961_725)
			.saturating_add(Weight::from_proof_size(6020))
			// Standard Error: 671
			.saturating_add(Weight::from_ref_time(2_033).saturating_mul(b.into()))
			// Standard Error: 6_971
			.saturating_add(Weight::from_ref_time(42_850).saturating_mul(m.into()))
			// Standard Error: 6_902
			.saturating_add(Weight::from_ref_time(312_832).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(165).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `906 + m * (64 ±0)`
		//  Estimated: `4782 + m * (128 ±0)`
		// Minimum execution time: 40_357 nanoseconds.
		Weight::from_ref_time(44_174_013)
			.saturating_add(Weight::from_proof_size(4782))
			// Standard Error: 3_602
			.saturating_add(Weight::from_ref_time(95_664).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(m.into()))
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
		//  Measured:  `565 + m * (64 ±0) + p * (35 ±0)`
		//  Estimated: `5413 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 40_106 nanoseconds.
		Weight::from_ref_time(40_160_635)
			.saturating_add(Weight::from_proof_size(5413))
			// Standard Error: 6_808
			.saturating_add(Weight::from_ref_time(75_068).saturating_mul(m.into()))
			// Standard Error: 6_586
			.saturating_add(Weight::from_ref_time(317_428).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(260).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
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
		//  Measured:  `908 + m * (64 ±0) + p * (39 ±0)`
		//  Estimated: `9032 + b * (4 ±0) + m * (260 ±0) + p * (160 ±0)`
		// Minimum execution time: 57_403 nanoseconds.
		Weight::from_ref_time(51_757_452)
			.saturating_add(Weight::from_proof_size(9032))
			// Standard Error: 1_025
			.saturating_add(Weight::from_ref_time(12_443).saturating_mul(b.into()))
			// Standard Error: 10_895
			.saturating_add(Weight::from_ref_time(35_962).saturating_mul(m.into()))
			// Standard Error: 10_541
			.saturating_add(Weight::from_ref_time(452_998).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(4).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(260).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(160).saturating_mul(p.into()))
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
		//  Measured:  `585 + m * (64 ±0) + p * (35 ±0)`
		//  Estimated: `6495 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 44_643 nanoseconds.
		Weight::from_ref_time(48_258_834)
			.saturating_add(Weight::from_proof_size(6495))
			// Standard Error: 8_637
			.saturating_add(Weight::from_ref_time(50_994).saturating_mul(m.into()))
			// Standard Error: 8_355
			.saturating_add(Weight::from_ref_time(299_048).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(325).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
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
		//  Measured:  `928 + m * (64 ±0) + p * (39 ±0)`
		//  Estimated: `10400 + b * (5 ±0) + m * (325 ±0) + p * (200 ±0)`
		// Minimum execution time: 63_786 nanoseconds.
		Weight::from_ref_time(55_982_574)
			.saturating_add(Weight::from_proof_size(10400))
			// Standard Error: 1_039
			.saturating_add(Weight::from_ref_time(11_639).saturating_mul(b.into()))
			// Standard Error: 11_040
			.saturating_add(Weight::from_ref_time(41_443).saturating_mul(m.into()))
			// Standard Error: 10_682
			.saturating_add(Weight::from_ref_time(467_214).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(5).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(325).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(200).saturating_mul(p.into()))
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
		//  Measured:  `291 + p * (32 ±0)`
		//  Estimated: `1371 + p * (96 ±0)`
		// Minimum execution time: 22_685 nanoseconds.
		Weight::from_ref_time(25_436_032)
			.saturating_add(Weight::from_proof_size(1371))
			// Standard Error: 5_724
			.saturating_add(Weight::from_ref_time(331_663).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(96).saturating_mul(p.into()))
	}
}
