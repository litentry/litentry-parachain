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

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-18, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		Weight::from_ref_time(31_995_000 as u64)
			// Standard Error: 699
			.saturating_add(Weight::from_ref_time(85_129 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		Weight::from_ref_time(36_230_000 as u64)
			// Standard Error: 1_406
			.saturating_add(Weight::from_ref_time(90_262 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		Weight::from_ref_time(37_624_000 as u64)
			// Standard Error: 4_380
			.saturating_add(Weight::from_ref_time(107_265 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		Weight::from_ref_time(35_851_000 as u64)
			// Standard Error: 1_600
			.saturating_add(Weight::from_ref_time(268_625 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		Weight::from_ref_time(37_491_000 as u64)
			// Standard Error: 1_242
			.saturating_add(Weight::from_ref_time(99_301 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:0)
	// Storage: CouncilMembership Prime (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(14_927_000 as u64)
			// Standard Error: 2_421
			.saturating_add(Weight::from_ref_time(71_091 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: CouncilMembership Prime (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		Weight::from_ref_time(8_063_000 as u64)
			// Standard Error: 561
			.saturating_add(Weight::from_ref_time(10_563 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
