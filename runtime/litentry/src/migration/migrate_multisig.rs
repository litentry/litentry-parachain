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
	migration::{clear_storage_prefix, storage_key_iter},
	pallet_prelude::*,
	traits::{Currency, Get, OnRuntimeUpgrade},
	Blake2_128Concat, Twox64Concat,
};
use parity_scale_codec::EncodeLike;
use sp_runtime::{traits::Hash, Saturating};
use sp_std::{
	convert::{From, TryInto},
	marker::PhantomData,
	vec::Vec,
};

pub const DECIMAL_CONVERTOR: u32 = 1_000_000;

#[cfg(feature = "try-runtime")]
use parity_scale_codec::Encode;
#[cfg(feature = "try-runtime")]
use sp_std::collections::btree_map::BTreeMap;
use storage::migration::get_storage_value;

use pallet_multisig::{Multisigs, Timepoint};
// use pallet_multisig::Multisig;
type BalanceOf<T> = <<T as pallet_multisig::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(MaxApprovals))]
pub struct Multisig<BlockNumber, Balance, AccountId, MaxApprovals>
where
	MaxApprovals: Get<u32>,
{
	/// The extrinsic when the multisig operation was opened.
	pub when: Timepoint<BlockNumber>,
	/// The amount held in reserve of the `depositor`, to be returned once the operation ends.
	pub deposit: Balance,
	/// The account who opened it (i.e. the first to approve it).
	pub depositor: AccountId,
	/// The approvals achieved so far, including the depositor. Always sorted.
	pub approvals: BoundedVec<AccountId, MaxApprovals>,
}

impl<BlockNumber, Balance, AccountId, MaxApprovals>
	EncodeLike<pallet_multisig::Multisig<BlockNumber, Balance, AccountId, MaxApprovals>>
	for Multisig<BlockNumber, Balance, AccountId, MaxApprovals>
where
	AccountId: EncodeLike<AccountId>,
	Balance: EncodeLike<Balance>,
	BlockNumber: EncodeLike<BlockNumber>,
	MaxApprovals: Get<u32>,
{
}

// Replace Parachain Staking Storage for Decimal Change from 12 to 18
pub struct ReplacePalletMultisigStorage<T>(PhantomData<T>);

impl<T> ReplacePalletMultisigStorage<T>
where
	T: pallet_multisig::Config,
	BalanceOf<T>: EncodeLike<BalanceOf<T>> + From<u128>,
{
	// pallet_multisig
	pub fn replace_multisig_multisigs_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplacePalletMultisigStorage",
			"Running migration for Multisig - Multisigs"
		);

		let mut migrated_count: u64 = 0;
		for (account, account2, mut multisig) in Multisigs::<T>::drain() {
			multisig.deposit = multisig.deposit.saturating_mul(DECIMAL_CONVERTOR.into());
			Multisigs::<T>::insert(account, account2, multisig);
			migrated_count += 1;
		}

		let weight = T::DbWeight::get();
		weight.reads_writes(migrated_count, migrated_count)
	}
}

#[cfg(feature = "try-runtime")]
impl<T> ReplacePalletMultisigStorage<T>
where
	T: pallet_multisig::Config,
	BalanceOf<T>: EncodeLike<BalanceOf<T>> + From<u128>,
{
	pub fn pre_upgrade_multisig_multisigs_storage() -> Result<Vec<u8>, &'static str> {
		let result: BTreeMap<
			(T::AccountId, [u8; 32]),
			Multisig<T::BlockNumber, BalanceOf<T>, T::AccountId, T::MaxSignatories>,
		> = <Multisigs<T>>::iter()
			.map(|(account, account2, mut multisig)| {
				// let new_deposit = multisig.deposit().saturating_mul(DECIMAL_CONVERTOR.into());
				// multisig.set_deposit(new_deposit);

				multisig.deposit = multisig.deposit.saturating_mul(DECIMAL_CONVERTOR.into());
				((&account, &account2), multisig)

				// ((&account, &hash), Multisig {
				//     when: multisig.when(),
				//     deposit: multisig.deposit().saturating_mul(DECIMAL_CONVERTOR.into()),
				//     depositor: multisig.deposit(),
				//     approvals: multisig.approvals(),
				// }
				// )
			})
			.collect();
		Ok(result.encode())
	}

	pub fn post_upgrade_multisig_multisigs_storage(state: Vec<u8>) -> Result<(), &'static str> {
		let expected_result = BTreeMap::<
			(T::AccountId, [u8; 32]),
			Multisig<T::BlockNumber, BalanceOf<T>, T::AccountId, T::MaxSignatories>,
		>::decode(&mut &state[..])
		.map_err(|_| "Failed to decode Vec<ScheduledRequest>")?;
		for (account, account2, actual_result) in <Multisigs<T>>::iter() {
			let expected_result = expected_result
				.get(&(account, account2))
				.ok_or("Not Expected Vec<ScheduledRequest>")?
				.clone();
			assert_eq!(expected_result.encode(), actual_result.encode());
		}
		Ok(())
	}
}

impl<T> OnRuntimeUpgrade for ReplacePalletMultisigStorage<T>
where
	T: frame_system::Config + pallet_multisig::Config,
	BalanceOf<T>: EncodeLike<BalanceOf<T>> + From<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		// pallet_multisig
		let multisigs_vec = Self::pre_upgrade_multisig_multisigs_storage()?;

		Ok((multisigs_vec,).encode())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let mut weight = frame_support::weights::Weight::from_parts(0, 0);
		// pallet_multisig
		weight += Self::replace_multisig_multisigs_storage();

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let pre_vec: (Vec<u8>, Vec<u8>, Vec<u8>) =
			Decode::decode(&mut &state[..]).map_err(|_| "Failed to decode Tuple")?;

		// pallet_multisig
		Self::post_upgrade_multisig_multisigs_storage(pre_vec.0)?;
		Ok(())
	}
}
