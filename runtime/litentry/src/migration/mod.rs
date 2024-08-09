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
	set::OrderedSet, BalanceOf, Bond, CandidateInfo, CandidateMetadata, CandidatePool, Delegations,
	TopDelegations, Total,
};
pub const DECIMAL_CONVERTOR: u128 = 1_000_000u128;

#[cfg(feature = "try-runtime")]
use parity_scale_codec::Encode;
#[cfg(feature = "try-runtime")]
use sp_std::collections::btree_map::BTreeMap;
use storage::migration::get_storage_value;

// Fix Parachain Staking Storage for missing migrating TopDelegations total
pub struct FixParachainStakingStorage<T>(PhantomData<T>);

impl<T> OnRuntimeUpgrade for FixParachainStakingStorage<T>
where
	T: frame_system::Config + pallet_parachain_staking::Config,
	BalanceOf<T>: From<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		// Does not matter
		Ok(Vec::<u8>::new())
	}

	// Staked is the snapshot of Total, and remove after used, so no fix
	// AtStake, will not be fixed, let's wait extra two round with reward = 0 to make this data
	// clean DelayedPayouts, will not fix, let's wait extra two round with reward = 0 to make this
	// data clean

	// TopDelegations require fix
	// CandidateInfo require fix
	// CandidatePool require fix
	// Total fix
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceParachainStakingStorage",
			"running migration to ParachainStaking TopDelegations"
		);
		let pallet_prefix: &[u8] = b"ParachainStaking";
		let storage_item_prefix: &[u8] = b"TopDelegations";
		let mut weight: Weight = frame_support::weights::Weight::zero();

		let mut candidates = <CandidatePool<T>>::get();
		// remove all bonds
		candidates.clear();

		// intitialized total
		let mut total: BalanceOf<T> = 0u128.into();

		for (account, mut delegations) in storage_key_iter::<
			T::AccountId,
			Delegations<T::AccountId, BalanceOf<T>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.drain()
		{
			// Patching the missed total value converting
			delegations.total = delegations.total.saturating_mul(DECIMAL_CONVERTOR.into());

			// Fix TopDelegations
			<TopDelegations<T>>::insert(&account, delegations);

			// Get CandidateInfo of the same collator key
			let mut metadata = <CandidateInfo<T>>::get(&collator).unwrap();
			// Self + delegation total
			metadata.total_counted = metadata.bond + delegations.total;

			// Bond use its owner value to determine if equal without checking its amount
			// We need to check amount later
			candidates.insert(Bond { owner: candidate, amount: metadata.total_counted });
			// Add total
			total = total.saturating_add(metadata.total_counted);

			// Fix CandidateInfo
			<CandidateInfo<T>>::insert(&account, metadata);

			weight += T::DbWeight::get().reads_writes(2, 2);
		}

		// Fix CandidatePool
		<CandidatePool<T>>::put(candidates);
		// Fix Total
		<Total<T>>::put(total);

		weight
	}

	// Check Top Delegation total = sum, collator wise
	// Check CandidateInfo total count = self bond + sum of delegation, collator wise
	// Check Total = sum CandidateInfo total count
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		// Check Top Delegation total = sum
		// Check CandidateInfo total count = self bond + sum of delegation
		// Check Total = sum CandidateInfo total count
		let mut total: BalanceOf<T> = 0u128.into();
		for (account, mut delegations) in <TopDelegations<T>>::iter() {
			log::info!("Checking Top Delegations Collator: {}", account);
			// Start calculating collator delegation sum
			let mut collator_delegations_sum: BalanceOf<T> = 0u128.into();

			for delegation_bond in delegations.delegations.iter() {
				collator_delegations_sum += delegation_bond.amount;
			}

			// Check Top Delegation total = sum, collator wise
			assert_eq!(collator_delegations_sum, delegations.total);

			let metadata = <CandidateInfo<T>>::get(account).unwrap();
			// Check CandidateInfo total count = self bond + sum of delegation
			assert_eq!(metadata.bond + metadata.total_counted, collator_delegations_sum);

			total += collator_delegations_sum;
		}
		// Check Total = sum CandidateInfo total count
		assert_eq!(total, <Total<T>>::get());
		Ok(())
	}
}
