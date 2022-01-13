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
use frame_support::{inherent::Vec, traits::{Currency, Get, OnRuntimeUpgrade}, StorageHasher, Twox128};
use frame_support::storage::storage_prefix;
use sp_std::marker::PhantomData;
use frame_support::pallet_prelude::*;
use sp_runtime::{Perbill};
use parachain_staking::BalanceOf;
use sp_runtime::traits::Zero;


pub struct MigrateCollatorSelectionIntoParachainStaking<T>(PhantomData<T>);
impl <T> OnRuntimeUpgrade for MigrateCollatorSelectionIntoParachainStaking<T> 
where 
	T: parachain_staking::Config,
	<T as frame_system::Config>::Event: From<parachain_staking::Event<T>>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		// use primitives::AccountId;

		// log::info!("Pre check pallet CollatorSelection exists");
        // // Get Invulnerables address from CollatorSelection
		// // WARN: We do not care about any Candidates storage, as we forbid any general transaction by sudo and so no info there in practice
		// let invulnerables = frame_support::storage::migration::get_storage_value::<Vec<AccountId>>(
		// 	b"CollatorSelection",
		// 	b"Invulnerables",
		// 	b"",
		// ).expect("Storage query fails: CollatorSelection Invulnerables");

		// if invulnerables.len() == 0 {
        //     Err("CollatorSelection empty")
        // } else {
        //     Ok(())
        // }
		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// let invulnerables = frame_support::storage::migration::get_storage_value::<Vec<<T as frame_system::Config>::AccountId>>(
		// 	b"CollatorSelection",
		// 	b"Invulnerables",
		// 	b"",
		// ).expect("Storage query fails: CollatorSelection Invulnerables");		
		
		// let mut candidate_count = 0u32;
		// // Get the minimum collator stake amount
		// let min_collator_stk = <T as parachain_staking::Config>::MinCollatorStk::get();
		// // Initialize the candidates
		// for candidate in invulnerables {
		// 	assert!(
		// 		<T as parachain_staking::Config>::Currency::free_balance(&candidate) >= min_collator_stk,
		// 		"Account does not have enough balance to bond as a candidate."
		// 	);
		// 	candidate_count += 1u32;
		// 	if let Err(error) = <parachain_staking::Pallet<T>>::join_candidates(
		// 		<T as frame_system::Config>::Origin::from(Some(candidate.clone()).into()),
		// 		min_collator_stk,
		// 		candidate_count,
		// 	) {
		// 		log::warn!("Join candidates failed in genesis with error {:?}", error);
		// 	} else {
		// 		candidate_count += 1u32;
		// 	}
		// }

		// // Reproduce the genesis build
		// // Initialize the rest setup of parachain-staking
		// // After runtimeUpgrade, we should:
		// // (1) Set the inflation by                              set_inflation
		// // (2) Set collator commission by                        set_collator_commission
		// // (3) Set parachain bond config by                      parachset_parachain_bond_account
		// // (4) Set total selected candidates to minimum config   set_total_selected
		// // (5) Choose top TotalSelected collator candidates
		// // (6) Refresh round if it is during runtimeUpgrade
		// // (7) Snapshot total staked (through select_top_candidates)
		// // (8) NewRound Event Deposit

		// // Inflation config as Default, so do nothing
		// // ...
		// // Set collator commission to default config
		// frame_support::storage::migration::put_storage_value::<Perbill>(
		// 	b"ParachainStaking",
		// 	b"CollatorCommission",
		// 	b"",
		// 	<T as parachain_staking::Config>::DefaultCollatorCommission::get(),
		// );
		// // Set parachain bond config to default config
		// frame_support::storage::migration::put_storage_value::<parachain_staking::ParachainBondConfig<<T as frame_system::Config>::AccountId>>(
		// 	b"ParachainStaking",
		// 	b"ParachainBondInfo",
		// 	b"",
		// 	parachain_staking::ParachainBondConfig {
		// 		// must be set soon; if not => due inflation will be sent to collators/delegators
		// 		account: <T as frame_system::Config>::AccountId::default(),
		// 		percent: <T as parachain_staking::Config>::DefaultParachainBondReservePercent::get(),
		// 	},
		// );

		// // // Set total selected candidates to minimum config
		// frame_support::storage::migration::put_storage_value::<u32>(
		// 	b"ParachainStaking",
		// 	b"TotalSelected",
		// 	b"",
		// 	<T as parachain_staking::Config>::MinSelectedCandidates::get(),
		// );
		// // Choose top TotalSelected collator candidates
		// // WARNING/TODO: We change the private into public of select_top_candidates function in pallet. We should change it back in next runtime upgrade for safety.
		// let (v_count, _, total_staked) = <parachain_staking::Pallet<T>>::select_top_candidates(1u32);

		
		// // Start Round 1 at Block 0
		// let round: parachain_staking::RoundInfo<<T as frame_system::Config>::BlockNumber> =
		// 	parachain_staking::RoundInfo::new(1u32, 0u32.into(), <T as parachain_staking::Config>::DefaultBlocksPerRound::get());
		// frame_support::storage::migration::put_storage_value::<parachain_staking::RoundInfo<<T as frame_system::Config>::BlockNumber>>(
		// 	b"ParachainStaking",
		// 	b"Round",
		// 	b"",
		// 	round,
		// );

		// // // Snapshot total stake
		// // The code below is supposed to behave same as:
		// // <Staked<T>>::insert(1u32, <Total<T>>::get())
		// let val = frame_support::storage::migration::get_storage_value::<BalanceOf<T>>(
		// 	b"ParachainStaking",
		// 	b"Total",
		// 	b"",
		// );
		// let storage_prefix = storage_prefix(b"ParachainStaking", b"Staked");
		// let key_hashed = 1u32.using_encoded(Twox64Concat::hash);
		// let mut final_key = Vec::with_capacity(storage_prefix.len() + &key_hashed.len());
		// final_key.extend_from_slice(&storage_prefix);
		// final_key.extend_from_slice(key_hashed.as_ref());
		// frame_support::storage::unhashed::put(final_key.as_ref(), &val);
		

		// // Deposit NewRound event at RuntimeUpgrade
		// <frame_system::Pallet<T>>::deposit_event(parachain_staking::Event::NewRound(
		// 	<T as frame_system::Config>::BlockNumber::zero(),
		// 	1u32,
		// 	v_count,
		// 	total_staked,
		// ));
		


		// // Remove CollatorSelection Storage
		// // TODO: Very Weak safety
		let entries: u64 = 4 + 6142;
		// frame_support::storage::unhashed::kill_prefix(&Twox128::hash(b"CollatorSelection"), Some(entries));
		<T as frame_system::Config>::DbWeight::get().writes(entries.into())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		// use sp_io::KillStorageResult;

		// log::info!("Post check CollatorSelection");
		// let res = frame_support::storage::unhashed::kill_prefix(&Twox128::hash(b"CollatorSelection"), Some(0));

		// match res {
		// 	KillStorageResult::AllRemoved(0) | KillStorageResult::SomeRemaining(0) => Ok(()),
		// 	KillStorageResult::AllRemoved(n) | KillStorageResult::SomeRemaining(n) => {
		// 		log::error!("Remaining entries: {:?}", n);
		// 		Err("CollatorSelection not removed")
		// 	}
		// }
		Ok(())
	}
}
