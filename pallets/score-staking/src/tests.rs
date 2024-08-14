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

use crate::{mock::*, Error, Event, PoolState, RoundInfo, RoundSetting, ScorePayment};
use core_primitives::{DAYS, YEARS};
use frame_support::{assert_err, assert_noop, assert_ok};
use pallet_parachain_staking::Delegator;
use pallet_teebag::{Enclave, WorkerType};
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
			RoundSetting { interval: 7 * DAYS, stake_coef_n: 1, stake_coef_m: 2 }
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
			RoundSetting { interval: 3, stake_coef_n: 1, stake_coef_m: 2 }
		));

		assert_eq!(ScoreStaking::round(), RoundInfo { index: 1, start_block: 2 });
		assert_eq!(ScoreStaking::round_config().interval, 3);

		run_to_block(5);
		assert_eq!(ScoreStaking::round(), RoundInfo { index: 2, start_block: 5 });

		run_to_block(6);
		assert_ok!(ScoreStaking::set_round_config(
			RuntimeOrigin::root(),
			RoundSetting { interval: 5, stake_coef_n: 1, stake_coef_m: 2 }
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
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { total: round_reward(), distributed: 0 },
		));
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
#[allow(clippy::identity_op)]
fn score_staking_works() {
	new_test_ext(true).execute_with(|| {
		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			alice(),
			Delegator::new(bob(), bob(), 900),
		);
		pallet_parachain_staking::Total::<Test>::put(900);

		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), alice().into(), 500));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 500,
				total_reward: 0,
				last_round_reward: 0,
				unpaid_reward: 0,
				token_staking_amount: 0,
			}
		);
		assert_eq!(ScoreStaking::total_score(), 500);
		assert_eq!(ScoreStaking::score_user_count(), 1);

		assert_ok!(ScoreStaking::update_score(
			RuntimeOrigin::signed(alice()),
			alice().into(),
			2000
		));
		assert_eq!(ScoreStaking::scores(alice()).unwrap().score, 2000);
		assert_eq!(ScoreStaking::total_score(), 2000);
		assert_eq!(ScoreStaking::score_user_count(), 1);

		// run to next reward distribution round, alice should win all rewards
		run_to_block(7);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted {
				total: round_reward(),
				distributed: round_reward(),
			},
		));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: round_reward(),
				last_round_reward: round_reward(),
				unpaid_reward: round_reward(),
				token_staking_amount: 0,
			}
		);

		// alice's winning should accumulate
		run_to_block(12);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted {
				total: round_reward(),
				distributed: round_reward(),
			},
		));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: 2 * round_reward(),
				last_round_reward: round_reward(),
				unpaid_reward: 2 * round_reward(),
				token_staking_amount: 0,
			}
		);

		// increase the total staked amount, alice should win less
		run_to_block(13);
		pallet_parachain_staking::Total::<Test>::put(1600);

		run_to_block(17);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted {
				total: round_reward(),
				distributed: round_reward() * 3 / 4,
			},
		));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: 2 * round_reward() + round_reward() * 3 / 4,
				last_round_reward: round_reward() * 3 / 4,
				unpaid_reward: 2 * round_reward() + round_reward() * 3 / 4,
				token_staking_amount: 0,
			}
		);

		// add bob's score
		run_to_block(18);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			bob(),
			Delegator::new(alice(), alice(), 1600),
		);
		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), bob().into(), 1000));
		pallet_parachain_staking::Total::<Test>::put(2500);
		assert_eq!(ScoreStaking::total_score(), 3000);
		assert_eq!(ScoreStaking::score_user_count(), 2);

		run_to_block(22);
		// System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardDistributionStarted {
		// 	total: round_reward(),
		// 	distributed: round_reward() * 2 * 3 / (3 * 5) + round_reward() * 1 * 4 / (3 * 5),
		// }));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: 2 * round_reward() +
					round_reward() * 3 / 4 +
					round_reward() * 2 * 3 / (3 * 5),
				last_round_reward: round_reward() * 2 * 3 / (3 * 5),
				unpaid_reward: 2 * round_reward() +
					round_reward() * 3 / 4 +
					round_reward() * 2 * 3 / (3 * 5),
				token_staking_amount: 0,
			}
		);

		assert_eq!(
			ScoreStaking::scores(bob()).unwrap(),
			ScorePayment {
				score: 1000,
				total_reward: round_reward() * 1 * 4 / (3 * 5),
				last_round_reward: round_reward() * 1 * 4 / (3 * 5),
				unpaid_reward: round_reward() * 1 * 4 / (3 * 5),
				token_staking_amount: 0,
			}
		);

		// update more scores will error out
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			charlie(),
			Delegator::new(alice(), alice(), 7000),
		);
		assert_err!(
			ScoreStaking::update_score(RuntimeOrigin::signed(alice()), charlie().into(), 1000),
			Error::<Test>::MaxScoreUserCountReached
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
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted {
				total: round_reward(),
				distributed: round_reward(),
			},
		));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: round_reward(),
				last_round_reward: round_reward(),
				unpaid_reward: round_reward(),
				token_staking_amount: 0,
			}
		);

		assert_ok!(ScoreStaking::claim(RuntimeOrigin::signed(alice()), 200));
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardClaimed {
			who: alice(),
			amount: 200,
		}));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: round_reward(),
				last_round_reward: round_reward(),
				unpaid_reward: round_reward() - 200,
				token_staking_amount: 0,
			}
		);

		assert_ok!(ScoreStaking::claim_all(RuntimeOrigin::signed(alice())));
		System::assert_last_event(RuntimeEvent::ScoreStaking(Event::<Test>::RewardClaimed {
			who: alice(),
			amount: round_reward() - 200,
		}));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: round_reward(),
				last_round_reward: round_reward(),
				unpaid_reward: 0,
				token_staking_amount: 0,
			}
		);

		// continue to claim will error out
		assert_err!(
			ScoreStaking::claim(RuntimeOrigin::signed(alice()), 100),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn update_token_staking_amount_works() {
	new_test_ext(false).execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			alice(),
			Delegator::new(bob(), bob(), 900),
		);

		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), alice().into(), 500));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 500,
				total_reward: 0,
				last_round_reward: 0,
				unpaid_reward: 0,
				token_staking_amount: 0,
			}
		);

		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice().into(),
			1000
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 500,
				total_reward: 0,
				last_round_reward: 0,
				unpaid_reward: 0,
				token_staking_amount: 1000,
			}
		);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::TokenStakingAmountUpdated { account: alice(), amount: 1000 },
		));
	})
}

#[test]
fn update_token_staking_amount_origin_check_works() {
	new_test_ext(false).execute_with(|| {
		assert_noop!(
			ScoreStaking::update_token_staking_amount(
				RuntimeOrigin::signed(alice()),
				alice().into(),
				1000
			),
			Error::<Test>::UnauthorizedOrigin
		);
	})
}

#[test]
fn update_token_staking_amount_existing_user_check_works() {
	new_test_ext(false).execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

		assert_noop!(
			ScoreStaking::update_token_staking_amount(
				RuntimeOrigin::signed(alice()),
				alice().into(),
				1000
			),
			Error::<Test>::UserNotExist
		);
	})
}

#[test]
fn complete_reward_distribution_works() {
	new_test_ext(false).execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

		assert_ok!(ScoreStaking::complete_reward_distribution(RuntimeOrigin::signed(alice()),));

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionCompleted {},
		));
	});
}

#[test]
fn complete_reward_distribution_origin_check_works() {
	new_test_ext(false).execute_with(|| {
		assert_noop!(
			ScoreStaking::complete_reward_distribution(RuntimeOrigin::signed(alice()),),
			Error::<Test>::UnauthorizedOrigin
		);
	});
}
