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

//! Benchmarking setup for pallet-drop3

use super::*;

#[allow(unused)]
use crate::Pallet as Drop3;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{Currency, EnsureOrigin};
use frame_system::RawOrigin;
use sp_runtime::traits::One;
use sp_std::vec;

const SEED: u32 = 0;
const DEFAULT_ED_MULTIPLIER: u32 = 10;
const TRANSFER_ED_MULTIPLIER: u32 = 2;

// TODO:
// - I don't see much difference between whitelisted_caller() and account(), why?
// - it seems the event assertion can only be applied to the event which is emitted by the tested
//   extrinsic, is it true? e.g. assert_event in setup() or assert_event which is emitted by setup
//   would always fail: Error: Input("Error executing and verifying runtime benchmark: Other(\"Wasm
//   execution trapped...
// - maybe have to make more careful thoughts of benchmarking the worst case scenario

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

fn run_to_block<T: Config>(n: T::BlockNumber) {
	while frame_system::Pallet::<T>::block_number() < n {
		crate::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		frame_system::Pallet::<T>::set_block_number(
			frame_system::Pallet::<T>::block_number() + One::one(),
		);
		frame_system::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		crate::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
	}
}

// create a default caller with `DEFAULT_ED_MULTIPLIER` times ED as free balance
// return the caller account
fn create_default_caller<T: Config>() -> T::AccountId {
	let caller: T::AccountId = account("caller", 0, SEED);
	let default_balance =
		T::Currency::minimum_balance().saturating_mul(DEFAULT_ED_MULTIPLIER.into());
	let _ = T::Currency::deposit_creating(&caller, default_balance);
	caller
}

// create a default proposal with the given `caller`
// the proposal `name`, `total`, `start_at`, `end_at` are mostly hardcoded
// return the pool id and pool name
fn create_default_proposal<T: Config>(caller: T::AccountId) -> (T::PoolId, Vec<u8>) {
	let name = vec![0u8; 8];
	assert!(Drop3::<T>::propose_reward_pool(
		RawOrigin::Signed(caller).into(),
		name.clone(),
		T::Currency::minimum_balance().saturating_mul(DEFAULT_ED_MULTIPLIER.into()),
		1u32.into(),
		5u32.into(),
	)
	.is_ok());
	let expected_id: T::PoolId = 1u64.into();
	assert_eq!(Drop3::<T>::get_sorted_pool_ids(), vec![expected_id]);
	(expected_id, name)
}

// setup for most of the benchmarks, including
// - create and set admin using whitelisted_caller
// - create a default caller
// - create a default proposal with the caller
// - admin approves the proposal
// - run to block 1
// - start the reward pool depending on `should_start`
// returns the caller (i.e. owner of the reward pool), pool id and pool name
fn setup<T: Config>(should_start: bool) -> (T::AccountId, T::PoolId, Vec<u8>) {
	let caller = create_default_caller::<T>();
	let admin: T::AccountId = whitelisted_caller();
	let origin = T::SetAdminOrigin::successful_origin();
	assert!(Drop3::<T>::set_admin(origin, admin.clone()).is_ok());
	let (id, name) = create_default_proposal::<T>(caller.clone());
	assert!(Drop3::<T>::approve_reward_pool(RawOrigin::Signed(admin.clone()).into(), id).is_ok());
	run_to_block::<T>(1u32.into());
	if should_start {
		assert!(Drop3::<T>::start_reward_pool(RawOrigin::Signed(admin).into(), id).is_ok());
	}
	(caller, id, name)
}

benchmarks! {
	set_admin {
		let origin = T::SetAdminOrigin::successful_origin();
		/*
		If the setup function is used here then old_admin uses the configuration in setup func, i.e whitelisted_caller()
		But here we are not using setup, so when using the `cargo test -p pallet-drop3 --features runtime-benchmarks`  command,
		go back to using the environment configured by the pallet mock.rs
		i.e let _ = Drop3::set_admin(Origin::signed(1), 1);  mock.rs 147
		*/
		let old_admin= <Admin<T>>::get();
		let admin: T::AccountId = account("admin", 0, SEED);
	}: _<T::RuntimeOrigin>(origin, admin)
	verify {
		assert_last_event::<T>(Event::AdminChanged { old_admin }.into());
	}

	approve_reward_pool {
		let caller = create_default_caller::<T>();
		let admin: T::AccountId = whitelisted_caller();
		let origin = T::SetAdminOrigin::successful_origin();
		assert!(Drop3::<T>::set_admin(origin, admin.clone()).is_ok());
		let (id, _) = create_default_proposal::<T>(caller);
	}: _(RawOrigin::Signed(admin), id)
	verify {
		assert_last_event::<T>(Event::RewardPoolApproved { id }.into())
	}

	reject_reward_pool {
		let caller = create_default_caller::<T>();
		let admin: T::AccountId = whitelisted_caller();
		let origin = T::SetAdminOrigin::successful_origin();
		assert!(Drop3::<T>::set_admin(origin, admin.clone()).is_ok());
		let (id, name) = create_default_proposal::<T>(caller.clone());
	}: _(RawOrigin::Signed(admin), id)
	verify {
		assert!(Drop3::<T>::get_sorted_pool_ids().is_empty());
		assert_event::<T>(Event::RewardPoolRejected { id }.into());
		assert_event::<T>(Event::BalanceSlashed {
			who: caller.clone(),
			amount: T::SlashPercent::get() * T::Currency::minimum_balance().saturating_mul(DEFAULT_ED_MULTIPLIER.into()),
		}.into());
		assert_event::<T>(Event::RewardPoolRemoved { id, name, owner: caller }.into());
	}

	start_reward_pool {
		let (caller, id, _) = setup::<T>(false);
	}: _(RawOrigin::Signed(caller), id)
	verify {
		assert_last_event::<T>(Event::RewardPoolStarted { id }.into())
	}

	stop_reward_pool {
		let (caller, id, _) = setup::<T>(true);
	}: _(RawOrigin::Signed(caller), id)
	verify {
		assert_last_event::<T>(Event::RewardPoolStopped { id }.into())
	}

	close_reward_pool {
		let (caller, id, name) = setup::<T>(true);
	}: _(RawOrigin::Signed(caller.clone()), id)
	verify {
		assert!(Drop3::<T>::get_sorted_pool_ids().is_empty());
		assert_event::<T>(Event::RewardPoolRemoved { id, name, owner: caller }.into())
	}

	propose_reward_pool {
		let n in 0 .. T::MaximumNameLength::get();
		let caller: T::AccountId = create_default_caller::<T>();
		let name = vec![0u8; n as usize];
		run_to_block::<T>(1u32.into());
	}: _(
		RawOrigin::Signed(caller.clone()),
		name.clone(),
		T::Currency::minimum_balance().saturating_mul(DEFAULT_ED_MULTIPLIER.into()),
		1u32.into(),
		5u32.into()
	)
	verify {
		assert_eq!(Drop3::<T>::get_sorted_pool_ids(), vec![ 1u64.into() ]);
		assert_event::<T>(Event::RewardPoolProposed { id: 1u64.into(), name, owner: caller }.into())
	}

	send_reward {
		let (caller, id, _) = setup::<T>(true);
		let to: T::AccountId = account("to", 0, SEED);
		// account must be active, otherwise you'll get DeadAccount error
		let _ = T::Currency::deposit_creating(&to, T::Currency::minimum_balance());
		let amount = T::Currency::minimum_balance().saturating_mul(TRANSFER_ED_MULTIPLIER.into());
	}: _(RawOrigin::Signed(caller), id, to.clone(), amount)
	verify {
		assert_event::<T>(Event::RewardSent { to: to.clone(), amount }.into());
		assert_eq!(T::Currency::free_balance(&to), amount + T::Currency::minimum_balance());
	}
}

impl_benchmark_test_suite!(Drop3, crate::mock::new_test_ext(), crate::mock::Test,);
