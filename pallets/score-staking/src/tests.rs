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

#![allow(dead_code, unused_imports)]

use crate::{mock::*, Error, Event, PoolState, RoundInfo, RoundSetting};
use core_primitives::{DAYS, YEARS};
use frame_support::{assert_err, assert_ok};
use pallet_parachain_staking::Delegator;
use sp_runtime::Perbill;

fn round_reward() -> Balance {
	(Perbill::from_perthousand(5) * 100_000_000 * UNIT / (YEARS as u128)) * 5
}

#[test]
fn default_state_works() {
	new_test_ext(false).execute_with(|| {
		assert_eq!(ScoreStaking::state(), PoolState::Stopped);
		assert_eq!(
			ScoreStaking::round_config(),
			RoundSetting {
				interval: 7 * DAYS,
				score_coefficient: Perbill::from_percent(80),
				stake_coefficient: Perbill::from_percent(20),
			}
		);
		assert_eq!(ScoreStaking::score_feeder().unwrap(), alice());
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 0, start_block: 0 });
	})
}

#[test]
fn start_stop_pool_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));
		assert_eq!(ScoreStaking::state(), PoolState::Running);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::PoolStarted {
			start_block: 2,
		}));
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 1, start_block: 2 });

		run_to_block(6);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 1, start_block: 2 });

		run_to_block(7);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 7 });

		run_to_block(8);
		assert_ok!(ScoreStaking::stop_pool(RuntimeOrigin::root()));
		assert_eq!(ScoreStaking::state(), PoolState::Stopped);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::PoolStopped {}));
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 7 });

		run_to_block(20);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));
		assert_eq!(ScoreStaking::state(), PoolState::Running);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::PoolStarted {
			start_block: 20,
		}));
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 3, start_block: 20 });
	})
}

#[test]
fn set_round_config_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 1, start_block: 2 });

		run_to_block(3);
		assert_ok!(ScoreStaking::set_round_config(
			RuntimeOrigin::root(),
			RoundSetting {
				interval: 3,
				score_coefficient: Perbill::from_percent(80),
				stake_coefficient: Perbill::from_percent(20),
			}
		));

		assert_eq!(ScoreStaking::round(), RoundInfo { index: 1, start_block: 2 });
		assert_eq!(ScoreStaking::round_config().interval, 3);

		run_to_block(5);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 5 });

		run_to_block(6);
		assert_ok!(ScoreStaking::set_round_config(
			RuntimeOrigin::root(),
			RoundSetting {
				interval: 5,
				score_coefficient: Perbill::from_percent(80),
				stake_coefficient: Perbill::from_percent(20),
			}
		));
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 5 });

		run_to_block(9);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 5 });

		run_to_block(10);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 3, start_block: 10 });
	});
}

#[test]
fn default_mint_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		// run to next reward distribution round
		run_to_block(7);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: 0,
		}));
	});
}

#[test]
fn score_update_checks_staking() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		assert_err!(
			ScoreStaking::update_score(RuntimeOrigin::signed(alice()), alice().into(), 2000),
			Error::<Test>::UserNotStaked
		);
	});
}

#[test]
fn score_staking_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			alice(),
			Delegator::new(bob(), bob(), 1000 * UNIT),
		);
		pallet_parachain_staking::Total::<Test>::put(1000 * UNIT);

		assert_ok!(ScoreStaking::update_score(
			RuntimeOrigin::signed(alice()),
			alice().into(),
			2000
		));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().score, 2000);
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, 0);

		// run to next reward distribution round, alice should win all rewards
		run_to_block(7);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: round_reward(),
		}));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, round_reward());

		// alice's winning should accumulate
		run_to_block(12);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: round_reward(),
		}));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, 2 * round_reward());

		// increase the total staked amount, alice should win less
		run_to_block(13);
		pallet_parachain_staking::Total::<Test>::put(2000 * UNIT);

		run_to_block(17);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: round_reward() * (1600 + 200) / (1600 + 400),
		}));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap().unpaid,
			2 * round_reward() + round_reward() * (1600 + 200) / (1600 + 400)
		);

		// add bob's score
		run_to_block(18);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			bob(),
			Delegator::new(alice(), alice(), 7000 * UNIT),
		);
		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), bob().into(), 1000));
		pallet_parachain_staking::Total::<Test>::put(8000 * UNIT);

		run_to_block(22);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: round_reward(),
		}));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap().unpaid,
			2 * round_reward() +
				round_reward() * (1600 + 200) / (1600 + 400) +
				round_reward() * (1600 + 200) / (2400 + 1600)
		);

		assert_eq!(
			ScoreStaking::scores(bob()).unwrap().unpaid,
			round_reward() * (800 + 1400) / (2400 + 1600)
		);
	});
}

#[test]
fn claim_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			alice(),
			Delegator::new(bob(), bob(), 1000),
		);
		pallet_parachain_staking::Total::<Test>::put(1000);

		assert_ok!(ScoreStaking::update_score(
			RuntimeOrigin::signed(alice()),
			alice().into(),
			2000
		));

		// run to next reward distribution round, alice should win all rewards
		run_to_block(7);
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardCalculated {
			total: round_reward(),
			distributed: round_reward(),
		}));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, round_reward());

		assert_ok!(ScoreStaking::claim(RuntimeOrigin::signed(alice()), 200));
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardClaimed {
			who: alice(),
			amount: 200,
		}));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, round_reward() - 200);

		assert_ok!(ScoreStaking::claim_all(RuntimeOrigin::signed(alice())));
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardClaimed {
			who: alice(),
			amount: round_reward() - 200,
		}));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().unpaid, 0);

		// continue to claim will error out
		assert_err!(
			ScoreStaking::claim(RuntimeOrigin::signed(alice()), 100),
			Error::<Test>::InsufficientBalance
		);
	});
}