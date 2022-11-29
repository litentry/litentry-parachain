// Copyright 2020-2021 Litentry Technologies GmbH.
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
#![allow(deprecated)]
use frame_support::{
	inherent::Vec,
	storage,
	traits::{Get, OnRuntimeUpgrade},
};
use pallet_parachain_staking::{
	AtStake, BalanceOf, BondWithAutoCompound, CollatorSnapshot, Round, RoundIndex,
};
use sp_runtime::Percent;
use sp_std::marker::PhantomData;

mod deprecated {
	use codec::{Decode, Encode};
	use frame_support::{inherent::Vec, traits::OnRuntimeUpgrade, RuntimeDebug};
	use pallet_parachain_staking::Bond;
	use scale_info::TypeInfo;
	// CollatorSnapshot
	#[deprecated(note = "use CollatorSnapshot with BondWithAutoCompound delegations")]
	#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
	/// Snapshot of collator state at the start of the round for which they are selected
	pub struct CollatorSnapshot<AccountId, Balance> {
		/// The total value locked by the collator.
		pub bond: Balance,

		/// The rewardable delegations. This list is a subset of total delegators, where certain
		/// delegators are adjusted based on their scheduled
		/// [DelegationChange::Revoke] or [DelegationChange::Decrease] action.
		pub delegations: Vec<Bond<AccountId, Balance>>,

		/// The total counted value locked for the collator, including the self bond + total staked
		/// by top delegators.
		pub total: Balance,
	}

	impl<A: PartialEq, B: PartialEq> PartialEq for CollatorSnapshot<A, B> {
		fn eq(&self, other: &Self) -> bool {
			let must_be_true = self.bond == other.bond && self.total == other.total;
			if !must_be_true {
				return false
			}
			for (Bond { owner: o1, amount: a1 }, Bond { owner: o2, amount: a2 }) in
				self.delegations.iter().zip(other.delegations.iter())
			{
				if o1 != o2 || a1 != a2 {
					return false
				}
			}
			true
		}
	}

	impl<A, B: Default> Default for CollatorSnapshot<A, B> {
		fn default() -> CollatorSnapshot<A, B> {
			CollatorSnapshot { bond: B::default(), delegations: Vec::new(), total: B::default() }
		}
	}

	/// Prefix to be used (optionally) for implementing [`OnRuntimeUpgradeHelpersExt::storage_key`].
	const ON_RUNTIME_UPGRADE_PREFIX: &[u8] = b"__ON_RUNTIME_UPGRADE__";

	/// Some helper functions for [`OnRuntimeUpgrade`] during `try-runtime` testing.
	pub trait OnRuntimeUpgradeHelpersExt {
		/// Generate a storage key unique to this runtime upgrade.
		///
		/// This can be used to communicate data from pre-upgrade to post-upgrade state and check
		/// them. See [`Self::set_temp_storage`] and [`Self::get_temp_storage`].
		fn storage_key(ident: &str) -> [u8; 32] {
			frame_support::storage::storage_prefix(ON_RUNTIME_UPGRADE_PREFIX, ident.as_bytes())
		}

		/// Get temporary storage data written by [`Self::set_temp_storage`].
		///
		/// Returns `None` if either the data is unavailable or un-decodable.
		///
		/// A `at` storage identifier must be provided to indicate where the storage is being read
		/// from.
		fn get_temp_storage<T: codec::Decode>(at: &str) -> Option<T> {
			sp_io::storage::get(&Self::storage_key(at))
				.and_then(|bytes| codec::Decode::decode(&mut &*bytes).ok())
		}

		/// Write some temporary data to a specific storage that can be read (potentially in
		/// post-upgrade hook) via [`Self::get_temp_storage`].
		///
		/// A `at` storage identifier must be provided to indicate where the storage is being
		/// written to.
		fn set_temp_storage<T: codec::Encode>(data: T, at: &str) {
			sp_io::storage::set(&Self::storage_key(at), &data.encode());
		}
	}

	impl<U: OnRuntimeUpgrade> OnRuntimeUpgradeHelpersExt for U {}
}
use deprecated::CollatorSnapshot as OldCollatorSnapshot;
pub struct MigrateAtStakeAutoCompound<T>(PhantomData<T>);
impl<T: pallet_parachain_staking::Config> MigrateAtStakeAutoCompound<T> {
	/// Get keys for the `AtStake` storage for the rounds up to `RewardPaymentDelay` rounds ago.
	/// We migrate only the last unpaid rounds due to the presence of stale entries in `AtStake`
	/// which significantly increase the PoV size.
	fn unpaid_rounds_keys() -> impl Iterator<Item = (RoundIndex, T::AccountId, Vec<u8>)> {
		let current_round = <Round<T>>::get().current;
		let max_unpaid_round = current_round.saturating_sub(T::RewardPaymentDelay::get());
		(max_unpaid_round..=current_round).into_iter().flat_map(|round| {
			<AtStake<T>>::iter_key_prefix(round).map(move |candidate| {
				let key = <AtStake<T>>::hashed_key_for(round, candidate.clone());
				(round, candidate, key)
			})
		})
	}
}
impl<T> OnRuntimeUpgrade for MigrateAtStakeAutoCompound<T>
where
	T: pallet_parachain_staking::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		use deprecated::OnRuntimeUpgradeHelpersExt;
		let mut num_to_update = 0u32;
		let mut rounds_candidates = vec![];
		for (round, candidate, key) in Self::unpaid_rounds_keys() {
			let state: OldCollatorSnapshot<T::AccountId, BalanceOf<T>> =
				storage::unhashed::get(&key).expect("unable to decode value");

			num_to_update = num_to_update.saturating_add(1);
			rounds_candidates.push((round, candidate.clone()));
			let mut delegation_str = vec![];
			for d in state.delegations {
				delegation_str
					.push(format!("owner={:?}_amount={:?}_autoCompound=0%", d.owner, d.amount));
			}
			Self::set_temp_storage(
				format!(
					"bond={:?}_total={:?}_delegations={:?}",
					state.bond, state.total, delegation_str
				),
				&format!("round_{:?}_candidate_{:?}", round, candidate),
			);
		}

		rounds_candidates.sort();
		Self::set_temp_storage(format!("{:?}", rounds_candidates), "rounds_candidates");
		Self::set_temp_storage(num_to_update, "num_to_update");
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		log::info!(
			target: "MigrateAtStakeAutoCompound",
			"running migration to add auto-compound values"
		);
		let mut reads = 0u64;
		let mut writes = 0u64;
		for (round, candidate, key) in Self::unpaid_rounds_keys() {
			let old_state: OldCollatorSnapshot<T::AccountId, BalanceOf<T>> =
				storage::unhashed::get(&key).expect("unable to decode value");
			reads = reads.saturating_add(1);
			writes = writes.saturating_add(1);
			log::info!(
				target: "MigrateAtStakeAutoCompound",
				"migration from old format round {:?}, candidate {:?}", round, candidate
			);
			let new_state = CollatorSnapshot {
				bond: old_state.bond,
				delegations: old_state
					.delegations
					.into_iter()
					.map(|d| BondWithAutoCompound {
						owner: d.owner,
						amount: d.amount,
						auto_compound: Percent::zero(),
					})
					.collect(),
				total: old_state.total,
			};
			storage::unhashed::put(&key, &new_state);
		}

		T::DbWeight::get().reads_writes(reads, writes)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		use deprecated::OnRuntimeUpgradeHelpersExt;
		let mut num_updated = 0u32;
		let mut rounds_candidates = vec![];
		for (round, candidate, _) in Self::unpaid_rounds_keys() {
			let state = <AtStake<T>>::get(round, &candidate);
			num_updated = num_updated.saturating_add(1);
			rounds_candidates.push((round, candidate.clone()));
			let mut delegation_str = vec![];
			for d in state.delegations {
				delegation_str.push(format!(
					"owner={:?}_amount={:?}_autoCompound={:?}",
					d.owner, d.amount, d.auto_compound
				));
			}
			assert_eq!(
				Some(format!(
					"bond={:?}_total={:?}_delegations={:?}",
					state.bond, state.total, delegation_str
				)),
				Self::get_temp_storage(&format!("round_{:?}_candidate_{:?}", round, candidate)),
				"incorrect delegations migration for round_{:?}_candidate_{:?}",
				round,
				candidate,
			);
		}

		rounds_candidates.sort();
		assert_eq!(
			Some(format!("{:?}", rounds_candidates)),
			Self::get_temp_storage("rounds_candidates")
		);
		assert_eq!(Some(num_updated), Self::get_temp_storage("num_to_update"));
		Ok(())
	}
}
