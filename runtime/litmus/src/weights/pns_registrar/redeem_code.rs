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

//! Autogenerated weights for `pns_registrar::redeem_code`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-11, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pns_registrar::redeem_code
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pns_registrar/redeem_code.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pns_registrar::redeem_code`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pns_registrar::redeem_code::WeightInfo for WeightInfo<T> {
	// Storage: PnsOrigin Origins (r:2 w:0)
	// Storage: PnsRedeemCode Redeems (r:0 w:2)
	fn mint_redeem(l: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((815_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(l as Weight)))
	}
	// Storage: PnsRedeemCode Redeems (r:1 w:1)
	// Storage: PnsRegistry Official (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsNft Tokens (r:2 w:2)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:1)
	// Storage: PnsNft Classes (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:1)
	// Storage: PnsNft TokensByOwner (r:0 w:1)
	fn name_redeem_min() -> Weight {
		(83_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: PnsRedeemCode Redeems (r:1 w:1)
	// Storage: PnsRegistry Official (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsNft Tokens (r:2 w:2)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:1)
	// Storage: PnsNft Classes (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:1)
	// Storage: PnsNft TokensByOwner (r:0 w:1)
	fn name_redeem_any_min() -> Weight {
		(82_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn create_label(l: u32, ) -> Weight {
		(770_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(l as Weight))
	}
	// Storage: PnsRegistry Official (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsNft Tokens (r:2 w:2)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:1)
	// Storage: PnsNft Classes (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:1)
	// Storage: PnsNft TokensByOwner (r:0 w:1)
	fn for_redeem_code(l: u32, ) -> Weight {
		(33_702_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((5_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}
