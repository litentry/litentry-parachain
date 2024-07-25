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
use pallet_democracy::{
	AccountVote, BoundedCallOf, Conviction, Delegations, DepositOf, PropIndex, ReferendumIndex,
	ReferendumInfo, ReferendumInfoOf, ReferendumStatus, Tally, VotingOf,
};
use sp_std::marker::PhantomData;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::migration::clear_storage_prefix;
use frame_support::{
	migration::{get_storage_value, storage_key_iter},
	pallet_prelude::*,
	traits::Currency,
	Twox64Concat,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_bounties::{Bounties, BountyIndex, BountyStatus};
use parity_scale_codec::EncodeLike;
use sp_runtime::Saturating;
use sp_std::collections::btree_map::BTreeMap;

type BalanceOf<T> = <<T as pallet_democracy::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

use crate::migration::DECIMAL_CONVERTOR;

/// A "prior" lock, i.e. a lock for some now-forgotten reason.
#[derive(
	Encode,
	MaxEncodedLen,
	Decode,
	Default,
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	RuntimeDebug,
	TypeInfo,
)]
pub struct PriorLock<BlockNumber, Balance>(BlockNumber, Balance);

#[derive(Clone, Encode, Decode, Eq, MaxEncodedLen, PartialEq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound(skip_type_params(MaxVotes)))]
#[scale_info(skip_type_params(MaxVotes))]
pub enum Voting<Balance, AccountId, BlockNumber, MaxVotes: Get<u32>> {
	/// The account is voting directly. `delegations` is the total amount of post-conviction voting
	/// weight that it controls from those that have delegated to it.
	Direct {
		/// The current votes of the account.
		votes: BoundedVec<(ReferendumIndex, AccountVote<Balance>), MaxVotes>,
		/// The total amount of delegations that this account has received.
		delegations: Delegations<Balance>,
		/// Any pre-existing locks from past voting/delegating activity.
		prior: PriorLock<BlockNumber, Balance>,
	},
	/// The account is delegating `balance` of its balance to a `target` account with `conviction`.
	Delegating {
		balance: Balance,
		target: AccountId,
		conviction: Conviction,
		/// The total amount of delegations that this account has received.
		delegations: Delegations<Balance>,
		/// Any pre-existing locks from past voting/delegating activity.
		prior: PriorLock<BlockNumber, Balance>,
	},
}

// This is important when we want to insert into the storage item
impl<AccountId, Balance, BlockNumber, MaxVotes>
	EncodeLike<pallet_democracy::Voting<AccountId, Balance, BlockNumber, MaxVotes>>
	for Voting<AccountId, Balance, BlockNumber, MaxVotes>
where
	AccountId: EncodeLike<AccountId>,
	Balance: EncodeLike<Balance>,
	BlockNumber: EncodeLike<BlockNumber>,
	MaxVotes: Get<u32>,
{
}

pub struct ReplaceDemocracyStorage<T>(PhantomData<T>);
impl<T: pallet_democracy::Config> ReplaceDemocracyStorage<T>
where
	BalanceOf<T>: From<u128>,
{
	fn replace_deposit_of_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceDemocracyStorage",
			"running migration to Democracy DepositOf Storage Item"
		);
		let pallet_prefix: &[u8] = b"Democracy";
		let storage_item_prefix: &[u8] = b"DepositOf";
		let stored_data: Vec<_> = storage_key_iter::<
			PropIndex,
			(BoundedVec<T::AccountId, T::MaxDeposits>, BalanceOf<T>),
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
			target: "ReplaceDemocracyStorage",
			"obtained state of existing treasury data"
		);

		// Now clear previos storage
		let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

		// Assert that old storage is empty
		assert!(storage_key_iter::<
			PropIndex,
			(BoundedVec<T::AccountId, T::MaxDeposits>, BalanceOf<T>),
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.next()
		.is_none());

		for (prop_index, value) in stored_data {
			let mut new_value = value;
			new_value.1 = new_value.1.saturating_mul(DECIMAL_CONVERTOR.into());

			<DepositOf<T>>::insert(prop_index, new_value);
		}

		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}

	fn replace_referendum_info_of_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceDemocracyStorage",
			"running migration to Democracy ReferendumInfoOf Storage Item"
		);
		let pallet_prefix: &[u8] = b"Democracy";
		let storage_item_prefix: &[u8] = b"ReferenceInfoOf";
		let stored_data: Vec<_> = storage_key_iter::<
			ReferendumIndex,
			ReferendumInfo<T::BlockNumber, BoundedCallOf<T>, BalanceOf<T>>,
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

		// Now clear previos storage
		let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

		// Assert that old storage is empty
		assert!(storage_key_iter::<
			PropIndex,
			(BoundedVec<T::AccountId, T::MaxDeposits>, BalanceOf<T>),
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.next()
		.is_none());

		for (ref_index, ref_info) in stored_data {
			let mut new_ref_info = match ref_info {
				ReferendumInfo::Finished { approved, end } =>
					ReferendumInfo::Finished { approved, end },
				ReferendumInfo::Ongoing(ref_status) => ReferendumInfo::Ongoing(ReferendumStatus {
					end: ref_status.end,
					proposal: ref_status.proposal,
					threshold: ref_status.threshold,
					delay: ref_status.delay,
					tally: Tally {
						ayes: ref_status.tally.ayes.saturating_mul(DECIMAL_CONVERTOR.into()),
						nays: ref_status.tally.nays.saturating_mul(DECIMAL_CONVERTOR.into()),
						turnout: ref_status.tally.turnout.saturating_mul(DECIMAL_CONVERTOR.into()),
					},
				}),
			};

			<ReferendumInfoOf<T>>::insert(ref_index, new_ref_info)
		}

		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}

	fn replace_voting_of_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplaceDemocracyStorage",
			"running migration to Democracy VotingOf Storage Item"
		);
		let pallet_prefix: &[u8] = b"Democracy";
		let storage_item_prefix: &[u8] = b"VotingOf";
		let stored_data: Vec<_> = storage_key_iter::<
			T::AccountId,
			Voting<BalanceOf<T>, T::AccountId, BlockNumberFor<T>, T::MaxVotes>,
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

		// Now clear previos storage
		let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

		// Assert that old storage is empty
		assert!(storage_key_iter::<
			T::AccountId,
			Voting<BalanceOf<T>, T::AccountId, BlockNumberFor<T>, T::MaxVotes>,
			Twox64Concat,
		>(pallet_prefix, storage_item_prefix)
		.next()
		.is_none());

		for (who, voting) in stored_data {
			let mut new_voting = match voting {
				Voting::Delegating { balance, target, conviction, delegations, prior } => {
					let new_balance = balance.saturating_mul(DECIMAL_CONVERTOR.into());
					let new_delegation = Delegations {
						votes: delegations.votes.saturating_mul(DECIMAL_CONVERTOR.into()),
						capital: delegations.capital.saturating_mul(DECIMAL_CONVERTOR.into()),
					};
					let new_prior_locks =
						PriorLock(prior.0, prior.1.saturating_mul(DECIMAL_CONVERTOR.into()));
					Voting::Delegating {
						balance: new_balance,
						target,
						conviction,
						delegations: new_delegation,
						prior: new_prior_locks,
					}
				},
				Voting::Direct { votes, delegations, prior } => {
					let new_votes: Vec<_> = votes
						.into_iter()
						.map(|(id, vote)| {
							let mut new_vote = match vote {
								AccountVote::Split { aye, nay } => AccountVote::Split {
									aye: aye.saturating_mul(DECIMAL_CONVERTOR.into()),
									nay: nay.saturating_mul(DECIMAL_CONVERTOR.into()),
								},
								AccountVote::Standard { vote, balance } => AccountVote::Standard {
									vote,
									balance: balance.saturating_mul(DECIMAL_CONVERTOR.into()),
								},
							};
							(id, new_vote)
						})
						.collect();

					let bounded_new_votes: BoundedVec<
						(u32, AccountVote<BalanceOf<T>>),
						T::MaxVotes,
					> = new_votes.try_into().unwrap();

					let new_delegation = Delegations {
						votes: delegations.votes.saturating_mul(DECIMAL_CONVERTOR.into()),
						capital: delegations.capital.saturating_mul(DECIMAL_CONVERTOR.into()),
					};
					let new_prior_locks =
						PriorLock(prior.0, prior.1.saturating_mul(DECIMAL_CONVERTOR.into()));

					Voting::Direct {
						votes: bounded_new_votes,
						delegations: new_delegation,
						prior: new_prior_locks,
					}
				},
			};
			<VotingOf<T>>::insert(who, new_voting);
		}

		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}
}
