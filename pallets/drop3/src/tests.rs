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

use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, ReservableCurrency},
};

#[test]
fn set_admin_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Drop3::admin().unwrap(), 1);
		assert_ok!(Drop3::set_admin(RuntimeOrigin::signed(1), 2));
		assert_eq!(Drop3::admin().unwrap(), 2);
		System::assert_last_event(RuntimeEvent::Drop3(crate::Event::AdminChanged {
			old_admin: Some(1),
		}));
	});
}

#[test]
fn set_admin_fails_with_unprivileged_origin() {
	new_test_ext().execute_with(|| {
		assert_eq!(Drop3::admin().unwrap(), 1);
		assert_noop!(
			Drop3::set_admin(RuntimeOrigin::signed(2), 2),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_eq!(Drop3::admin().unwrap(), 1);
	});
}

#[test]
fn propose_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_eq!(Drop3::get_sorted_pool_ids(), vec![1]);
		assert!(Drop3::reward_pools(1).is_some());
		let pool = Drop3::reward_pools(1).unwrap();
		assert!(!pool.started);
		assert!(!pool.approved);
		assert_eq!(pool.owner, 3);
		assert_eq!(pool.total, 100);
		assert_eq!(pool.remain, 100);
		assert_eq!(Balances::reserved_balance(3), 100);
		System::assert_last_event(RuntimeEvent::Drop3(crate::Event::RewardPoolProposed {
			id: 1,
			name: b"test".to_vec(),
			owner: 3,
		}));
	});
}

#[test]
fn multiple_propose_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			10,
			2,
			3
		));
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			20,
			4,
			5
		));
		assert_eq!(Drop3::get_sorted_pool_ids(), vec![1, 2]);
		// they should have the same owner
		assert_eq!(Drop3::reward_pool_owners(1), Some(3));
		assert_eq!(Drop3::reward_pool_owners(2), Some(3));
	});
}

#[test]
fn propose_reward_pool_works_with_wrapping_id() {
	new_test_ext().execute_with(|| {
		// manually insert a reward pool with `PoolId::max_value() - 1` as id
		propose_default_reward_pool(PoolId::max_value() - 1, true);

		let _ = Balances::deposit_creating(&3, 1000);

		// create a new proposal, it should have the id PoolId::max_value()
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			10,
			2,
			3
		));
		assert_eq!(
			Drop3::get_sorted_pool_ids(),
			vec![PoolId::max_value() - 1, PoolId::max_value()]
		);
		assert_eq!(Drop3::current_max_pool_id(), PoolId::max_value());

		// create a new proposal, it should have the id PoolId::min_value() + 1
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			10,
			2,
			3
		));
		assert_eq!(
			Drop3::get_sorted_pool_ids(),
			vec![1, PoolId::max_value() - 1, PoolId::max_value()]
		);
		assert_eq!(Drop3::current_max_pool_id(), PoolId::max_value());

		// manually insert reward pools with id = 2 and 4
		propose_default_reward_pool(2, false);
		propose_default_reward_pool(4, false);

		// create a new proposal, it should have the id 3
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			10,
			2,
			3
		));
		System::assert_last_event(RuntimeEvent::Drop3(crate::Event::RewardPoolProposed {
			id: 3,
			name: b"test".to_vec(),
			owner: 3,
		}));
		assert_eq!(
			Drop3::get_sorted_pool_ids(),
			vec![1, 2, 3, 4, PoolId::max_value() - 1, PoolId::max_value()]
		);
	});
}

#[test]
fn propose_reward_pool_fails_with_zero_toal() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_noop!(
			Drop3::propose_reward_pool(RuntimeOrigin::signed(3), b"test".to_vec(), 0, 2, 3),
			Error::<Test>::InvalidTotalBalance
		);
		assert_eq!(Drop3::get_sorted_pool_ids(), Vec::<PoolId>::new());
	});
}

#[test]
fn propose_reward_pool_fails_with_insufficient_balance() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_noop!(
			Drop3::propose_reward_pool(RuntimeOrigin::signed(3), b"test".to_vec(), 200, 2, 3),
			pallet_balances::Error::<Test, _>::InsufficientBalance
		);
		assert_eq!(Drop3::get_sorted_pool_ids(), Vec::<PoolId>::new());
	});
}

#[test]
fn approve_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert!(Drop3::reward_pools(1).unwrap().approved);
		System::assert_last_event(RuntimeEvent::Drop3(crate::Event::RewardPoolApproved { id: 1 }));
	});
}

#[test]
fn approve_reward_pool_fails_with_non_admin() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_noop!(
			Drop3::approve_reward_pool(RuntimeOrigin::signed(3), 1),
			Error::<Test>::RequireAdmin
		);
		assert!(!Drop3::reward_pools(1).unwrap().approved);
	});
}

#[test]
fn approve_reward_pool_fails_with_non_existent_pool() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_noop!(
			Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 2),
			Error::<Test>::NoSuchRewardPool
		);
		assert!(!Drop3::reward_pools(1).unwrap().approved);
	});
}

#[test]
fn reject_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::reject_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_eq!(Balances::reserved_balance(3), 0);
		assert_eq!(Balances::free_balance(3), 80);
		assert!(!crate::RewardPools::<Test>::contains_key(1));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolRejected { id: 1 }));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::BalanceSlashed {
			who: 3,
			amount: 20,
		}));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolRemoved {
			id: 1,
			name: b"test".to_vec(),
			owner: 3,
		}));
		assert_noop!(
			Drop3::reject_reward_pool(RuntimeOrigin::signed(1), 5),
			Error::<Test>::NoSuchRewardPool
		);
	});
}

#[test]
fn reject_reward_pool_works_with_unexpected_unreserve() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		// intentionally unreserve 90 out of 100
		<Balances as ReservableCurrency<_>>::unreserve(&3, 90);
		// should be handled gracefully and no error happens
		assert_ok!(Drop3::reject_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_eq!(Balances::reserved_balance(3), 0);
		assert_eq!(Balances::free_balance(3), 90);
		assert!(!crate::RewardPools::<Test>::contains_key(1));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolRejected { id: 1 }));
		// only 10 was slashed
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::BalanceSlashed {
			who: 3,
			amount: 10,
		}));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolRemoved {
			id: 1,
			name: b"test".to_vec(),
			owner: 3,
		}));
	});
}

#[test]
fn reject_reward_pool_fails_with_already_approved() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert!(Drop3::reward_pools(1).unwrap().approved);
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolApproved { id: 1 }));
		// reject an approved proposal should fail
		assert_noop!(
			Drop3::reject_reward_pool(RuntimeOrigin::signed(1), 1),
			Error::<Test>::RewardPoolAlreadyApproved
		);
		// the pool shouldn't be deleted and no change of reserved balance
		assert!(crate::RewardPools::<Test>::contains_key(1));
		assert!(Drop3::reward_pools(1).unwrap().approved);
		assert_eq!(Drop3::reward_pools(1).unwrap().remain, 100);
		assert_eq!(Balances::reserved_balance(3), 100);
	});
}

#[test]
fn start_stop_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		// pool owner starts the reward pool
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));
		assert!(Drop3::reward_pools(1).unwrap().started);
		// admin stops the reward pool
		assert_ok!(Drop3::stop_reward_pool(RuntimeOrigin::signed(1), 1));
		assert!(!Drop3::reward_pools(1).unwrap().started);
		assert_noop!(
			Drop3::start_reward_pool(RuntimeOrigin::signed(2), 1),
			Error::<Test>::RequireAdminOrRewardPoolOwner
		);
		assert_noop!(
			Drop3::start_reward_pool(RuntimeOrigin::signed(1), 5),
			Error::<Test>::NoSuchRewardPool
		);
		assert_noop!(
			Drop3::stop_reward_pool(RuntimeOrigin::signed(2), 1),
			Error::<Test>::RequireAdminOrRewardPoolOwner
		);
		assert_noop!(
			Drop3::stop_reward_pool(RuntimeOrigin::signed(1), 5),
			Error::<Test>::NoSuchRewardPool
		);
	});
}

#[test]
fn close_reward_pool_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		// pool owner should be able to close the pool even before the admin approves or rejects it
		assert_ok!(Drop3::close_reward_pool(RuntimeOrigin::signed(3), 1));
		assert!(!crate::RewardPools::<Test>::contains_key(1));
		assert_eq!(Balances::reserved_balance(3), 0);
		assert_eq!(Balances::free_balance(3), 100);
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardPoolRemoved {
			id: 1,
			name: b"test".to_vec(),
			owner: 3,
		}));

		// try to close it the second time would bring about NoSuchRewardPool error
		assert_noop!(
			Drop3::close_reward_pool(RuntimeOrigin::signed(3), 1),
			Error::<Test>::NoSuchRewardPool
		);
	});
}

#[test]
fn send_reward_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		let _ = Balances::deposit_creating(&4, 5);
		let _ = Balances::deposit_creating(&5, 10);

		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			1,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));

		// only the reward pool owner can send reward, not even the admin
		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(1), 1, 4, 10),
			Error::<Test>::RequireRewardPoolOwner
		);
		assert_ok!(Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 10));
		assert_ok!(Drop3::send_reward(RuntimeOrigin::signed(3), 1, 5, 20));

		assert_eq!(Balances::reserved_balance(3), 70);
		assert_eq!(Balances::free_balance(4), 15);
		assert_eq!(Balances::free_balance(5), 30);
		assert_eq!(Drop3::reward_pools(1).unwrap().remain, 70);
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardSent {
			to: 4,
			amount: 10,
		}));
		System::assert_has_event(RuntimeEvent::Drop3(crate::Event::RewardSent {
			to: 5,
			amount: 20,
		}));
	});
}

#[test]
fn send_reward_fails_with_unapproved_pool() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		let _ = Balances::deposit_creating(&4, 5);
		let _ = Balances::deposit_creating(&5, 10);

		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			1,
			3
		));
		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 10),
			Error::<Test>::RewardPoolUnapproved
		);
	});
}

#[test]
fn send_reward_fails_with_stopped_pool() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		let _ = Balances::deposit_creating(&4, 5);
		let _ = Balances::deposit_creating(&5, 10);

		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			1,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 10),
			Error::<Test>::RewardPoolStopped
		);
	});
}

#[test]
fn send_reward_fails_with_too_early() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		System::set_block_number(1);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));

		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 10),
			Error::<Test>::RewardPoolRanTooEarly
		);
	});
}

#[test]
fn send_reward_fails_with_too_late() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		System::set_block_number(4);
		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			2,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));

		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 10),
			Error::<Test>::RewardPoolRanTooLate
		);
	});
}

#[test]
fn send_reward_fails_with_insufficient_reserved_balance() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		let _ = Balances::deposit_creating(&4, 5);
		let _ = Balances::deposit_creating(&5, 10);

		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			1,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));

		// intentionally unreserve 90 out of 100
		<Balances as ReservableCurrency<_>>::unreserve(&3, 90);
		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 20),
			Error::<Test>::InsufficientReservedBalance
		);
	});
}

#[test]
fn send_reward_fails_with_insufficient_remain() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&3, 100);
		let _ = Balances::deposit_creating(&4, 5);
		let _ = Balances::deposit_creating(&5, 10);

		assert_ok!(Drop3::propose_reward_pool(
			RuntimeOrigin::signed(3),
			b"test".to_vec(),
			100,
			1,
			3
		));
		assert_ok!(Drop3::approve_reward_pool(RuntimeOrigin::signed(1), 1));
		assert_ok!(Drop3::start_reward_pool(RuntimeOrigin::signed(3), 1));

		assert_noop!(
			Drop3::send_reward(RuntimeOrigin::signed(3), 1, 4, 120),
			Error::<Test>::InsufficientRemain
		);
	});
}
