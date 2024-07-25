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
use sp_std::marker::PhantomData;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::migration::clear_storage_prefix;
use frame_support::{
	migration::{get_storage_value, storage_key_iter},
	pallet_prelude::*,
	Twox64Concat,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_bounties::{Bounties, BountyIndex, BountyStatus};
use pallet_treasury::{BalanceOf, Deactivated, ProposalIndex, Proposals};
use parity_scale_codec::EncodeLike;
use sp_runtime::Saturating;
use sp_std::collections::btree_map::BTreeMap;

use crate::migration::DECIMAL_CONVERTOR;

// We are recreating the proposal struct with public fields
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct Proposal<AccountId, Balance> {
	/// The account proposing it.
	pub proposer: AccountId,
	/// The (total) amount that should be paid if the proposal is accepted.
	pub value: Balance,
	/// The account to whom the payment should be made if the proposal is accepted.
	pub beneficiary: AccountId,
	/// The amount held on deposit (reserved) for making this proposal.
	pub bond: Balance,
}

// This is important when we want to insert into the storage item
impl<AccountId, Balance> EncodeLike<pallet_treasury::Proposal<AccountId, Balance>>
	for Proposal<AccountId, Balance>
where
	AccountId: EncodeLike<AccountId>,
	Balance: EncodeLike<Balance>,
{
}

pub struct ReplaceTreasuryStorage<T, I = ()>(PhantomData<(T, I)>);

impl<T, I: 'static> ReplaceTreasuryStorage<T, I>
where
	T: pallet_treasury::Config<I>,
	BalanceOf<T, I>: EncodeLike<BalanceOf<T, I>> + From<u128>,
{
	fn replace_proposals_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceTreasuryStorage",
			"running migration to Treasury Proposals Storage Item"
		);
		let pallet_prefix: &[u8] = b"Treasury";
		let storage_item_prefix: &[u8] = b"Proposals";
		let stored_data: Vec<_> = storage_key_iter::<
			ProposalIndex,
			Proposal<T::AccountId, BalanceOf<T, I>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.collect();

		let migrated_count = frame_support::weights::Weight::from_parts(
			0,
			stored_data
				.len()
				.try_into()
				.expect("There are between 0 and 2**64 mappings stored."),
		);

		log::info!(
			target: "ReplaceTreasuryStorage",
			"obtained state of existing treasury data"
		);

		// Now clear previos storage
		let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

		// Assert that old storage is empty
		assert!(storage_key_iter::<
			ProposalIndex,
			Proposal<T::AccountId, BalanceOf<T, I>>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.next()
		.is_none());

		for (proposal_index, proposal) in stored_data {
			let mut new_proposal = proposal;
			new_proposal.value = new_proposal.value.saturating_mul(DECIMAL_CONVERTOR.into());
			new_proposal.bond = new_proposal.bond.saturating_mul(DECIMAL_CONVERTOR.into());

			<Proposals<T, I>>::insert(proposal_index, new_proposal);
		}

		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}

	fn replace_deactivated_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceTreasuryStorage",
			"running migration to Treasury Deactivated Storage Item"
		);
		let pallet_prefix: &[u8] = b"Treasury";
		let storage_item_prefix: &[u8] = b"Deactivated";
		let stored_data =
			get_storage_value::<BalanceOf<T, I>>(pallet_prefix, storage_item_prefix, b"")
				.expect("Storage query fails: Treasury Deactivated");

		<Deactivated<T, I>>::put(stored_data.saturating_mul(DECIMAL_CONVERTOR.into()));

		let weight = T::DbWeight::get();
		frame_support::weights::Weight::from_parts(0, weight.write + weight.read)
	}
}

impl<T, I: 'static> OnRuntimeUpgrade for ReplaceTreasuryStorage<T, I>
where
	T: pallet_treasury::Config<I>,
	BalanceOf<T, I>: EncodeLike<BalanceOf<T, I>> + From<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let weight = T::DbWeight::get();
		T::DbWeight::get().reads_writes(1, 1)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		log::info!(target: "ReplaceTreasuryStorage", "Finished performing storage migrations");
		Ok(())
	}
}
