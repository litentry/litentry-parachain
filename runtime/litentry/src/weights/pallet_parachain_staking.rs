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

//! Autogenerated weights for `pallet_parachain_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-01, STEPS: `25`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_parachain_staking
// --extrinsic=*
// --heap-pages=4096
// --steps=25
// --repeat=20
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_parachain_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking Candidates (r:1 w:1)
	/// The range of component `x` is `[1, 100]`.
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		// Minimum execution time: 26_529 nanoseconds.
		Weight::from_ref_time(32_521_314 as u64)
			// Standard Error: 12_824
			.saturating_add(Weight::from_ref_time(477_908 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:1)
	/// The range of component `x` is `[1, 100]`.
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		// Minimum execution time: 26_596 nanoseconds.
		Weight::from_ref_time(34_159_653 as u64)
			// Standard Error: 30_523
			.saturating_add(Weight::from_ref_time(603_003 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		// Minimum execution time: 27_835 nanoseconds.
		Weight::from_ref_time(29_116_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		// Minimum execution time: 80_423 nanoseconds.
		Weight::from_ref_time(81_345_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		// Minimum execution time: 27_251 nanoseconds.
		Weight::from_ref_time(27_945_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		// Minimum execution time: 27_176 nanoseconds.
		Weight::from_ref_time(29_471_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_total_selected() -> Weight {
		// Minimum execution time: 27_837 nanoseconds.
		Weight::from_ref_time(33_005_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		// Minimum execution time: 25_845 nanoseconds.
		Weight::from_ref_time(29_125_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		// Minimum execution time: 35_381 nanoseconds.
		Weight::from_ref_time(39_849_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn join_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 71_167 nanoseconds.
		Weight::from_ref_time(79_765_356 as u64)
			// Standard Error: 6_063
			.saturating_add(Weight::from_ref_time(457_584 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 45_416 nanoseconds.
		Weight::from_ref_time(56_383_158 as u64)
			// Standard Error: 5_883
			.saturating_add(Weight::from_ref_time(422_725 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	/// The range of component `x` is `[2, 1200]`.
	fn execute_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 150_184 nanoseconds.
		Weight::from_ref_time(153_450_000 as u64)
			// Standard Error: 513_230
			.saturating_add(Weight::from_ref_time(57_593_999 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 42_253 nanoseconds.
		Weight::from_ref_time(64_837_872 as u64)
			// Standard Error: 9_303
			.saturating_add(Weight::from_ref_time(441_221 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_offline() -> Weight {
		// Minimum execution time: 38_656 nanoseconds.
		Weight::from_ref_time(39_417_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_online() -> Weight {
		// Minimum execution time: 37_690 nanoseconds.
		Weight::from_ref_time(38_552_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		// Minimum execution time: 61_776 nanoseconds.
		Weight::from_ref_time(63_087_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		// Minimum execution time: 36_915 nanoseconds.
		Weight::from_ref_time(39_651_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		// Minimum execution time: 102_721 nanoseconds.
		Weight::from_ref_time(108_633_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		// Minimum execution time: 33_166 nanoseconds.
		Weight::from_ref_time(33_849_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	/// The range of component `x` is `[3, 100]`.
	/// The range of component `y` is `[2, 1000]`.
	fn delegate(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 200_331 nanoseconds.
		Weight::from_ref_time(223_297_293 as u64)
			// Standard Error: 120_796
			.saturating_add(Weight::from_ref_time(216_743 as u64).saturating_mul(x as u64))
			// Standard Error: 11_760
			.saturating_add(Weight::from_ref_time(314_199 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn schedule_leave_delegators() -> Weight {
		// Minimum execution time: 42_738 nanoseconds.
		Weight::from_ref_time(43_981_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	/// The range of component `x` is `[2, 100]`.
	fn execute_leave_delegators(x: u32, ) -> Weight {
		// Minimum execution time: 141_067 nanoseconds.
		Weight::from_ref_time(19_012_009 as u64)
			// Standard Error: 207_775
			.saturating_add(Weight::from_ref_time(41_797_561 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		// Minimum execution time: 44_120 nanoseconds.
		Weight::from_ref_time(45_119_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		// Minimum execution time: 43_010 nanoseconds.
		Weight::from_ref_time(43_710_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegator_bond_more() -> Weight {
		// Minimum execution time: 80_341 nanoseconds.
		Weight::from_ref_time(81_460_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		// Minimum execution time: 43_038 nanoseconds.
		Weight::from_ref_time(44_020_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		// Minimum execution time: 136_748 nanoseconds.
		Weight::from_ref_time(144_518_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		// Minimum execution time: 132_225 nanoseconds.
		Weight::from_ref_time(138_210_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		// Minimum execution time: 42_371 nanoseconds.
		Weight::from_ref_time(43_549_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		// Minimum execution time: 80_193 nanoseconds.
		Weight::from_ref_time(85_175_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: BridgeTransfer ExternalBalances (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:8 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:8 w:0)
	// Storage: ParachainStaking TopDelegations (r:8 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:9)
	// Storage: System Account (r:1001 w:1001)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	/// The range of component `x` is `[8, 100]`.
	/// The range of component `y` is `[0, 5000]`.
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 1_302_415 nanoseconds.
		Weight::from_ref_time(3_713_418_493 as u64)
			// Standard Error: 81_760
			.saturating_add(Weight::from_ref_time(12_174 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(212 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(206 as u64))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `y` is `[0, 1000]`.
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		// Minimum execution time: 65_894 nanoseconds.
		Weight::from_ref_time(310_076_725 as u64)
			// Standard Error: 260_364
			.saturating_add(Weight::from_ref_time(18_883_015 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(y as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(y as u64)))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn base_on_initialize() -> Weight {
		// Minimum execution time: 9_268 nanoseconds.
		Weight::from_ref_time(9_640_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}
