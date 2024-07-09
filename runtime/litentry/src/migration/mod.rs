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
use frame_support::{
	traits::{Get, OnRuntimeUpgrade},
	StorageHasher, Twox128,
};
use sp_runtime::Saturating;
use sp_std::marker::PhantomData;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use pallet_parachain_staking::{BalanceOf, CandidateMetadata, Delegator};
pub const DECIMAL_CONVERTOR: Balance = 1_000_000;

// Replace Parachain Staking Storage for Decimal Change from 12 to 18
pub struct ReplaceParachainStakingStorage<T>(PhantomData<T>);
impl<T: pallet_parachain_staking::Config> ReplaceParachainStakingStorage<T> {
	pub fn replace_delegator_state_storage() -> frame_support::weights::Weight {
		// DelegatorState
		let pallet_prefix: &[u8] = b"ParachainStaking";
		let storage_item_prefix: &[u8] = b"DelegatorState";
		// Read all the data into memory.
		// https://crates.parity.io/frame_support/storage/migration/fn.storage_key_iter.html
		let stored_data: Vec<_> = storage_key_iter::<
			T::AccountId,
			Delegator<T::AccountId, BalanceOf<T>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.collect();
		let migrated_count: frame_support::weights::Weight = stored_data
			.len()
			.try_into()
			.expect("There are between 0 and 2**64 mappings stored.");
		// Now remove the old storage
		// https://crates.parity.io/frame_support/storage/migration/fn.remove_storage_prefix.html
		remove_storage_prefix(pallet_prefix, storage_item_prefix, &[]);
		// Assert that old storage is empty
		assert!(storage_key_iter::<
			T::AccountId,
			Delegator<T::AccountId, BalanceOf<T>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.next()
		.is_none());
		for (account, state) in stored_data {
			let mut new_delegator: Delegator<T::AccountId, BalanceOf<T>> = state;
			new_delegator.total = new_delegator.total.saturating_mul(DECIMAL_CONVERTOR);
			new_delegator.less_total = new_delegator.less_total.saturating_mul(DECIMAL_CONVERTOR);
			let mut sorted_inner_vector = new_delegator.delegations.0;
			for elem in sorted_inner_vector.iter_mut() {
				*elem.amount = elem.amount.saturating_mul(DECIMAL_CONVERTOR);
			}
			new_delegator.delegations = OrderedSet::from(sorted_inner_vector);

			<DelegatorState<T>>::insert(&account, new_delegator)
		}
		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}

	pub fn replace_candidate_info_storage() -> frame_support::weights::Weight {
		// CandidateInfo
		let pallet_prefix: &[u8] = b"ParachainStaking";
		let storage_item_prefix: &[u8] = b"CandidateInfo";
		// Read all the data into memory.
		// https://crates.parity.io/frame_support/storage/migration/fn.storage_key_iter.html
		let stored_data: Vec<_> = storage_key_iter::<
			T::AccountId,
			CandidateMetadata<BalanceOf<T>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.collect();
		let migrated_count: frame_support::weights::Weight = stored_data
			.len()
			.try_into()
			.expect("There are between 0 and 2**64 mappings stored.");
		// Now remove the old storage
		// https://crates.parity.io/frame_support/storage/migration/fn.remove_storage_prefix.html
		remove_storage_prefix(pallet_prefix, storage_item_prefix, &[]);
		// Assert that old storage is empty
		assert!(storage_key_iter::<T::AccountId, CandidateMetadata<BalanceOf<T>>, Twox64Concat>(
			pallet_prefix,
			storage_item_prefix
		)
		.next()
		.is_none());
		for (account, state) in stored_data {
			let mut new_metadata: CandidateMetadata<BalanceOf<T>> = state;
			new_metadata.bond = new_metadata.bond.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.total_counted =
				new_metadata.total_counted.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.lowest_top_delegation_amount =
				new_metadata.lowest_top_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.highest_bottom_delegation_amount =
				new_metadata.highest_bottom_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.lowest_bottom_delegation_amount =
				new_metadata.lowest_bottom_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);

			if let Some(i) = new_metadata.request {
				new_metadata.request.amount = i.amount.saturating_mul(DECIMAL_CONVERTOR);
			}
			<CandidateInfo<T>>::insert(&account, new_metadata)
		}
		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}
}

#[cfg(feature = "try-runtime")]
impl<T: pallet_parachain_staking::Config> ReplaceParachainStakingStorage<T> {
	pub fn pre_upgrade_delegator_state_storage() -> Result<(), &'static str> {
		// get DelegatorState to check consistency
		for (account, state) in <DelegatorState<T>>::iter() {
			let mut new_delegator: Delegator<T::AccountId, BalanceOf<T>> = state;
			new_delegator.total = new_delegator.total.saturating_mul(DECIMAL_CONVERTOR);
			new_delegator.less_total = new_delegator.less_total.saturating_mul(DECIMAL_CONVERTOR);
			let mut sorted_inner_vector = new_delegator.delegations.0;
			for elem in sorted_inner_vector.iter_mut() {
				*elem.amount = elem.amount.saturating_mul(DECIMAL_CONVERTOR);
			}
			new_delegator.delegations = OrderedSet::from(sorted_inner_vector);

			Self::set_temp_storage(
				new_delegator,
				&format!("Delegator{}DelegatorState", account)[..],
			);
		}
		Ok(())
	}
	pub fn post_upgrade_delegator_state_storage() -> Result<(), &'static str> {
		// check DelegatorState are the same as the expected
		for (account, state) in <DelegatorState<T>>::iter() {
			let expected_result: Delegator<T::AccountId, BalanceOf<T>> =
				Self::get_temp_storage(&format!("Delegator{}DelegatorState", account)[..])
					.expect("qed");
			let actual_result = state;
			assert_eq!(expected_result, actual_result);
		}
		Ok(())
	}
	pub fn pre_upgrade_candidate_info_storage() -> Result<(), &'static str> {
		// get DelegatorState to check consistency
		for (account, state) in <CandidateInfo<T>>::iter() {
			let mut new_metadata: CandidateMetadata<BalanceOf<T>> = state;
			new_metadata.bond = new_metadata.bond.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.total_counted =
				new_metadata.total_counted.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.lowest_top_delegation_amount =
				new_metadata.lowest_top_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.highest_bottom_delegation_amount =
				new_metadata.highest_bottom_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);
			new_metadata.lowest_bottom_delegation_amount =
				new_metadata.lowest_bottom_delegation_amount.saturating_mul(DECIMAL_CONVERTOR);

			if let Some(i) = new_metadata.request {
				new_metadata.request.amount = i.amount.saturating_mul(DECIMAL_CONVERTOR);
			}

			Self::set_temp_storage(
				new_delegator,
				&format!("Candidate{}CandidateInfo", account)[..],
			);
		}
		Ok(())
	}
	pub fn post_upgrade_candidate_info_storage() -> Result<(), &'static str> {
		// check CandidateInfo are the same as the expected
		for (account, state) in <CandidateInfo<T>>::iter() {
			let expected_result: CandidateMetadata<BalanceOf<T>> =
				Self::get_temp_storage(&format!("Candidate{}CandidateInfo", account)[..])
					.expect("qed");
			let actual_result = state;
			assert_eq!(expected_result, actual_result);
		}
		Ok(())
	}
}

impl<T> OnRuntimeUpgrade for ReplaceParachainStakingStorage<T>
where
	T: frame_system::Config + pallet_parachain_staking::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		let _ = pre_upgrade_delegator_state_storage()?;
		let _ = pre_upgrade_candidate_info_storage()?;
		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let mut weight = frame_support::weights::Weight;
		weight += replace_delegator_state_storage();
		weight += replace_candidate_info_storage();

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		let _ = post_upgrade_delegator_state_storage()?;
		let _ = post_upgrade_candidate_info_storage()?;
		Ok(())
	}
}
