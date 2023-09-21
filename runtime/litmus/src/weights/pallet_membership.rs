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

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-27, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_membership
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + m * (64 ±0)`
		//  Estimated: `4915 + m * (192 ±0)`
		// Minimum execution time: 24_310 nanoseconds.
		Weight::from_ref_time(25_667_878)
			.saturating_add(Weight::from_proof_size(4915))
			// Standard Error: 781
			.saturating_add(Weight::from_ref_time(44_323).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(192).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:0)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346 + m * (64 ±0)`
		//  Estimated: `5754 + m * (192 ±0)`
		// Minimum execution time: 27_971 nanoseconds.
		Weight::from_ref_time(30_088_906)
			.saturating_add(Weight::from_proof_size(5754))
			// Standard Error: 2_570
			.saturating_add(Weight::from_ref_time(60_906).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(192).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:0)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346 + m * (64 ±0)`
		//  Estimated: `5754 + m * (192 ±0)`
		// Minimum execution time: 28_606 nanoseconds.
		Weight::from_ref_time(30_510_440)
			.saturating_add(Weight::from_proof_size(5754))
			// Standard Error: 1_668
			.saturating_add(Weight::from_ref_time(56_655).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(192).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:0)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345 + m * (64 ±0)`
		//  Estimated: `5754 + m * (192 ±0)`
		// Minimum execution time: 27_481 nanoseconds.
		Weight::from_ref_time(29_691_023)
			.saturating_add(Weight::from_proof_size(5754))
			// Standard Error: 2_610
			.saturating_add(Weight::from_ref_time(249_422).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(192).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:1)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345 + m * (64 ±0)`
		//  Estimated: `5754 + m * (192 ±0)`
		// Minimum execution time: 28_533 nanoseconds.
		Weight::from_ref_time(31_225_564)
			.saturating_add(Weight::from_proof_size(5754))
			// Standard Error: 2_171
			.saturating_add(Weight::from_ref_time(60_574).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(192).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Members (r:1 w:0)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: CouncilMembership Prime (r:0 w:1)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139 + m * (32 ±0)`
		//  Estimated: `3837 + m * (32 ±0)`
		// Minimum execution time: 12_176 nanoseconds.
		Weight::from_ref_time(13_080_140)
			.saturating_add(Weight::from_proof_size(3837))
			// Standard Error: 625
			.saturating_add(Weight::from_ref_time(12_586).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(m.into()))
	}
	/// Storage: CouncilMembership Prime (r:0 w:1)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_435 nanoseconds.
		Weight::from_ref_time(4_941_146)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
