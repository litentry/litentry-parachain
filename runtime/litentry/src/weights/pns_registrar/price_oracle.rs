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

//! Autogenerated weights for `pns_registrar::price_oracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-15, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pns_registrar::price_oracle
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pns_registrar/price_oracle.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pns_registrar::price_oracle`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pns_registrar::price_oracle::WeightInfo for WeightInfo<T> {
	// Storage: PnsOrigin Origins (r:2 w:0)
	// Storage: PnsPriceOracle ExchangeRate (r:0 w:1)
	fn set_exchange_rate() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PnsOrigin Origins (r:2 w:0)
	// Storage: PnsPriceOracle BasePrice (r:0 w:1)
	fn set_base_price() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PnsOrigin Origins (r:2 w:0)
	// Storage: PnsPriceOracle RentPrice (r:0 w:1)
	fn set_rent_price() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
