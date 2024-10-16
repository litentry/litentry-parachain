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

//! Autogenerated weights for `pallet_teebag`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-08-28, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `litentry-benchmark-server`, CPU: `Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_teebag
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_teebag.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_teebag`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teebag::WeightInfo for WeightInfo<T> {
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:1)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_add_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `213`
		//  Estimated: `3678`
		// Minimum execution time: 14_943_000 picoseconds.
		Weight::from_parts(15_537_000, 0)
			.saturating_add(Weight::from_parts(0, 3678))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:1)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_remove_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `541`
		//  Estimated: `4006`
		// Minimum execution time: 18_467_000 picoseconds.
		Weight::from_parts(19_230_000, 0)
			.saturating_add(Weight::from_parts(0, 4006))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:4 w:3)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_remove_enclave_by_mrenclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `720`
		//  Estimated: `11610`
		// Minimum execution time: 46_203_000 picoseconds.
		Weight::from_parts(47_696_000, 0)
			.saturating_add(Weight::from_parts(0, 11610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:4 w:3)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_remove_enclave_by_worker_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `720`
		//  Estimated: `11610`
		// Minimum execution time: 46_312_000 picoseconds.
		Weight::from_parts(50_255_000, 0)
			.saturating_add(Weight::from_parts(0, 11610))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Teebag::AuthorizedEnclave` (r:1 w:1)
	/// Proof: `Teebag::AuthorizedEnclave` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_add_authorized_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `3848`
		// Minimum execution time: 12_701_000 picoseconds.
		Weight::from_parts(13_163_000, 0)
			.saturating_add(Weight::from_parts(0, 3848))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Teebag::AuthorizedEnclave` (r:1 w:1)
	/// Proof: `Teebag::AuthorizedEnclave` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:0)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_remove_authorized_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `415`
		//  Estimated: `3880`
		// Minimum execution time: 17_704_000 picoseconds.
		Weight::from_parts(18_234_000, 0)
			.saturating_add(Weight::from_parts(0, 3880))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Teebag::Mode` (r:1 w:0)
	/// Proof: `Teebag::Mode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::AuthorizedEnclave` (r:1 w:1)
	/// Proof: `Teebag::AuthorizedEnclave` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:1)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_enclave_with_ias_attestation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3892`
		// Minimum execution time: 1_571_568_000 picoseconds.
		Weight::from_parts(1_649_222_000, 0)
			.saturating_add(Weight::from_parts(0, 3892))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Teebag::QuotingEnclaveRegistry` (r:1 w:0)
	/// Proof: `Teebag::QuotingEnclaveRegistry` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::TcbInfo` (r:1 w:0)
	/// Proof: `Teebag::TcbInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::Mode` (r:1 w:0)
	/// Proof: `Teebag::Mode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::AuthorizedEnclave` (r:1 w:1)
	/// Proof: `Teebag::AuthorizedEnclave` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:1)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_enclave_with_dcap_attestation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `644`
		//  Estimated: `4109`
		// Minimum execution time: 3_397_776_000 picoseconds.
		Weight::from_parts(4_248_064_000, 0)
			.saturating_add(Weight::from_parts(0, 4109))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:1)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:1)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unregister_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `458`
		//  Estimated: `3923`
		// Minimum execution time: 19_744_000 picoseconds.
		Weight::from_parts(22_575_000, 0)
			.saturating_add(Weight::from_parts(0, 3923))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Teebag::QuotingEnclaveRegistry` (r:0 w:1)
	/// Proof: `Teebag::QuotingEnclaveRegistry` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn register_quoting_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `1493`
		// Minimum execution time: 1_654_144_000 picoseconds.
		Weight::from_parts(1_720_747_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Teebag::TcbInfo` (r:0 w:1)
	/// Proof: `Teebag::TcbInfo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_tcb_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `1493`
		// Minimum execution time: 1_811_473_000 picoseconds.
		Weight::from_parts(2_399_065_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn post_opaque_task() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_031_000 picoseconds.
		Weight::from_parts(7_338_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:0)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn parentchain_block_processed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `3872`
		// Minimum execution time: 15_448_000 picoseconds.
		Weight::from_parts(15_950_000, 0)
			.saturating_add(Weight::from_parts(0, 3872))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `Teebag::EnclaveRegistry` (r:1 w:0)
	/// Proof: `Teebag::EnclaveRegistry` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Teebag::EnclaveIdentifier` (r:1 w:0)
	/// Proof: `Teebag::EnclaveIdentifier` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::SidechainBlockFinalizationCandidate` (r:1 w:1)
	/// Proof: `Teebag::SidechainBlockFinalizationCandidate` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Teebag::LatestSidechainBlockConfirmation` (r:0 w:1)
	/// Proof: `Teebag::LatestSidechainBlockConfirmation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sidechain_block_imported() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `528`
		//  Estimated: `3993`
		// Minimum execution time: 24_642_000 picoseconds.
		Weight::from_parts(25_438_000, 0)
			.saturating_add(Weight::from_parts(0, 3993))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
