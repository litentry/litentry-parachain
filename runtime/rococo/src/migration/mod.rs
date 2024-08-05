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

pub mod migrate_identity;
pub use migrate_identity::ReplacePalletIdentityStorage;
pub mod migrate_multisig;
pub use migrate_multisig::ReplacePalletMultisigStorage;
pub mod migrate_proxy;
pub use migrate_proxy::ReplacePalletProxyStorage;
pub mod migrate_vesting;
pub use migrate_vesting::ReplacePalletVestingStorage;
pub mod migrate_bounty;
pub use migrate_bounty::ReplacePalletBountyStorage;
pub mod migrate_democracy;
pub use migrate_democracy::ReplaceDemocracyStorage;
pub mod migrate_preimage;
pub use migrate_preimage::ReplacePreImageStorage;
pub mod migrate_treasury;
pub use migrate_treasury::ReplaceTreasuryStorage;

use frame_support::{
	migration::storage_key_iter,
	pallet_prelude::*,
	traits::{Get, OnRuntimeUpgrade},
};
use frame_system::{Account, AccountInfo};
use pallet_balances::AccountData;
use sp_std::{marker::PhantomData, vec::Vec};

#[cfg(feature = "try-runtime")]
use sp_std::collections::btree_map::BTreeMap;

// Force System Balances Storage frozen amount to 0
pub struct ForceFixAccountFrozenStorage<T>(PhantomData<T>);
impl<T> OnRuntimeUpgrade for ForceFixAccountFrozenStorage<T>
where
	T: frame_system::Config<AccountData = AccountData<u128>>
		+ pallet_balances::Config<Balance = u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let result: BTreeMap<T::AccountId, AccountInfo<T::Index, AccountData<u128>>> =
			<Account<T>>::iter()
				.map(|(account, state)| {
					let mut new_account: AccountInfo<T::Index, AccountData<u128>> = state;
					new_account.data.frozen = 0u128;

					(account, new_account)
				})
				.collect();
		Ok(result.encode())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// The storage of Account for pallet balances is in frame_system pallet
		log::info!(
			target: "ReplaceBalancesRelatedStorage",
			"running migration to Frame System Account"
		);
		let pallet_prefix: &[u8] = b"System";
		let storage_item_prefix: &[u8] = b"Account";
		let mut weight: Weight = frame_support::weights::Weight::zero();

		for (account, mut account_info) in storage_key_iter::<
			T::AccountId,
			AccountInfo<T::Index, T::AccountData>,
			Blake2_128Concat,
		>(pallet_prefix, storage_item_prefix)
		.drain()
		{
			account_info.data.frozen = 0u128;
			<Account<T>>::insert(&account, account_info);
			weight += T::DbWeight::get().reads_writes(1, 1);
		}
		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let expected_state =
			BTreeMap::<T::AccountId, AccountInfo<T::Index, AccountData<u128>>>::decode(
				&mut &state[..],
			)
			.map_err(|_| "Failed to decode AccountInfo")?;
		for (account, actual_result) in <Account<T>>::iter() {
			let expected_result: AccountInfo<T::Index, AccountData<u128>> =
				expected_state.get(&account).ok_or("Not Expected AccountInfo")?.clone();
			assert_eq!(expected_result, actual_result);
		}
		Ok(())
	}
}
