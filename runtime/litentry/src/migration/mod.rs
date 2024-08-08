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
#![allow(clippy::type_complexity)]

use frame_support::{
	migration::storage_key_iter,
	pallet_prelude::*,
	traits::{Get, OnRuntimeUpgrade},
	Blake2_128Concat, Twox64Concat,
};
use sp_runtime::Saturating;
use sp_std::{convert::From, marker::PhantomData, vec::Vec};

use pallet_parachain_staking::{
	set::OrderedSet, BalanceOf, Bond, BottomDelegations, CandidateInfo, CandidateMetadata,
	CandidatePool, DelayedPayout, DelayedPayouts, DelegationAction, DelegationScheduledRequests,
	Delegations, Delegator, DelegatorState, ScheduledRequest, Staked, TopDelegations, Total,
};
pub const DECIMAL_CONVERTOR: u128 = 1_000_000u128;

#[cfg(feature = "try-runtime")]
use parity_scale_codec::Encode;
#[cfg(feature = "try-runtime")]
use sp_std::collections::btree_map::BTreeMap;
use storage::migration::get_storage_value;

// Replace Parachain Staking Storage for Decimal Change from 12 to 18
pub struct ReplaceParachainStakingStorage<T>(PhantomData<T>);
impl<T: pallet_parachain_staking::Config> ReplaceParachainStakingStorage<T>
where
	BalanceOf<T>: From<u128>,
{
	// TopDelegations require fix
	// DelegatorState No fix
	// CandidateInfo require fix ; must after topDelegations fix
	// DelegationScheduledRequests No fix
	// BottomDelegations No fix since 0
	// CandidatePool still require fix, although update_active will correct it;
	// Total ?? Rough number looks correct ; TB check

	// Staked, better fix it ; must after topDelegations fix, CandidateInfo fix and CandidatePool
	// fix.

	// AtStake, will not fix, let's wait extra two round with reward = 0 to make this data clean
	// DelayedPayouts, will not fix, let's wait extra two round with reward = 0 to make this data
	// clean

	pub fn replace_top_delegations_storage() -> frame_support::weights::Weight {}

	pub fn replace_candidate_info_storage() -> frame_support::weights::Weight {}

	pub fn replace_candidate_pool_storage() -> frame_support::weights::Weight {}

	pub fn replace_total_storage() -> frame_support::weights::Weight {}

	pub fn replace_staked_storage() -> frame_support::weights::Weight {}
}

#[cfg(feature = "try-runtime")]
impl<T: pallet_parachain_staking::Config> ReplaceParachainStakingStorage<T> where
	BalanceOf<T>: From<u128>
{
}

impl<T> OnRuntimeUpgrade for ReplaceParachainStakingStorage<T>
where
	T: frame_system::Config + pallet_parachain_staking::Config,
	BalanceOf<T>: From<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let mut weight = frame_support::weights::Weight::from_parts(0, 0);
		weight += Self::replace_top_delegations_storage();
		weight += Self::replace_candidate_info_storage();
		weight += Self::replace_candidate_pool_storage();
		weight += Self::replace_total_storage();
		weight += Self::replace_staked_storage();

		// No need since all balance related config is Zero
		// InflationConfig

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {}
}
