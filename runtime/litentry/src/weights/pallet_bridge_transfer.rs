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

//! Autogenerated weights for `pallet_bridge_transfer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-19, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_bridge_transfer
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_bridge_transfer.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_transfer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_transfer::WeightInfo for WeightInfo<T> {
	/// Storage: BridgeTransfer ExternalBalances (r:1 w:1)
	/// Proof Skipped: BridgeTransfer ExternalBalances (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge ChainNonces (r:1 w:1)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge BridgeFee (r:1 w:0)
	/// Proof Skipped: ChainBridge BridgeFee (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ChainBridge BridgeEvents (r:1 w:1)
	/// Proof Skipped: ChainBridge BridgeEvents (max_values: Some(1), max_size: None, mode: Measured)
	fn transfer_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `598`
		//  Estimated: `13538`
		// Minimum execution time: 74_707 nanoseconds.
		Weight::from_ref_time(75_550_000)
			.saturating_add(Weight::from_proof_size(13538))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: BridgeTransfer MaximumIssuance (r:1 w:0)
	/// Proof Skipped: BridgeTransfer MaximumIssuance (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: BridgeTransfer ExternalBalances (r:1 w:1)
	/// Proof Skipped: BridgeTransfer ExternalBalances (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `244`
		//  Estimated: `4081`
		// Minimum execution time: 36_453 nanoseconds.
		Weight::from_ref_time(38_481_000)
			.saturating_add(Weight::from_proof_size(4081))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: BridgeTransfer MaximumIssuance (r:1 w:1)
	/// Proof Skipped: BridgeTransfer MaximumIssuance (max_values: Some(1), max_size: None, mode: Measured)
	fn set_maximum_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 14_821 nanoseconds.
		Weight::from_ref_time(15_395_000)
			.saturating_add(Weight::from_proof_size(604))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeTransfer ExternalBalances (r:0 w:1)
	/// Proof Skipped: BridgeTransfer ExternalBalances (max_values: Some(1), max_size: None, mode: Measured)
	fn set_external_balances() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_432 nanoseconds.
		Weight::from_ref_time(3_676_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
