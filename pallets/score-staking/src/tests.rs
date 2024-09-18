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

use crate::{mock::*, Error, Event, PoolState, RoundInfo, RoundSetting, ScorePayment, Scores};
use core_primitives::{Identity, DAYS, YEARS};
use frame_support::{assert_err, assert_noop, assert_ok};
use pallet_parachain_staking::{Delegator, OnAllDelegationRemoved};
use pallet_teebag::{Enclave, WorkerType};
use sp_runtime::Perbill;

fn round_reward() -> Balance {
	(Perbill::from_perthousand(5) * 100_000_000 * UNIT / (YEARS as u128)) * 5
}

fn calculate_round_reward(
	user_score: u128,
	total_score: u128,
	user_stake: Balance,
	total_stake: Balance,
) -> Balance {
	round_reward()
		.saturating_mul(user_score)
		.saturating_div(total_score)
		.saturating_mul(num_integer::Roots::sqrt(&user_stake))
		.saturating_div(num_integer::Roots::sqrt(&total_stake))
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
			Event::<Test>::RewardDistributionStarted { round_index: 2 },
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
	new_test_ext_with_parachain_staking().execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

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
				last_token_distributed_round: 0
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
			Event::<Test>::RewardDistributionStarted { round_index: 2 },
		));
		// total reward first distribution
		let mut alice_total_reward = round_reward();

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			2,
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: alice_total_reward,
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 2,
			}
		);

		// alice's winning should accumulate
		run_to_block(12);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 3 },
		));
		// total reward second distribution
		alice_total_reward += round_reward();

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			3,
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: round_reward(),
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 3,
			}
		);

		// increase the total staked amount, alice should win less
		run_to_block(13);
		pallet_parachain_staking::Total::<Test>::put(1600);

		run_to_block(17);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 4 },
		));
		// total reward third distribution
		alice_total_reward += calculate_round_reward(2000, 2000, 900, 1600);

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			4,
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: calculate_round_reward(2000, 2000, 900, 1600),
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 4,
			}
		);

		// add bob's score
		run_to_block(18);
		assert_ok!(ParachainStaking::delegate(RuntimeOrigin::signed(bob()), alice(), 1600));
		assert_eq!(pallet_parachain_staking::Total::<Test>::get(), 3200);
		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), bob().into(), 1000));
		assert_eq!(ScoreStaking::total_score(), 3000);
		assert_eq!(ScoreStaking::score_user_count(), 2);

		run_to_block(22);
		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 5 },
		));

		// total rewards fourth distribution
		alice_total_reward += calculate_round_reward(2000, 3000, 900, 3200);
		let mut bob_total_reward = calculate_round_reward(1000, 3000, 1600, 3200);

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			5,
		));
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			bob(),
			0,
			5,
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: calculate_round_reward(2000, 3000, 900, 3200),
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 5,
			}
		);
		assert_eq!(
			ScoreStaking::scores(bob()).unwrap(),
			ScorePayment {
				score: 1000,
				total_reward: bob_total_reward,
				last_round_reward: bob_total_reward,
				unpaid_reward: bob_total_reward,
				last_token_distributed_round: 5,
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

		run_to_block(23);

		assert_ok!(ParachainStaking::schedule_revoke_delegation(
			RuntimeOrigin::signed(bob()),
			alice()
		));

		run_to_block(27);

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 6 },
		));

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			6,
		));
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			bob(),
			0,
			6,
		));

		// total rewards fifth distribution
		alice_total_reward += calculate_round_reward(2000, 3000, 900, 3200);
		bob_total_reward += calculate_round_reward(1000, 3000, 1600, 3200);

		run_to_block(30);
		assert_ok!(ParachainStaking::execute_delegation_request(
			RuntimeOrigin::signed(bob()),
			bob(),
			alice()
		));

		run_to_block(33);

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 7 },
		));
		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			7,
		));
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			bob(),
			0,
			7,
		));

		// total reward sixth distribution
		alice_total_reward += calculate_round_reward(2000, 2000, 900, 1600);

		// here
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: calculate_round_reward(2000, 2000, 900, 1600),
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 7,
			}
		);

		// remove increased stake (keep only alice's stake)
		pallet_parachain_staking::Total::<Test>::put(900);

		run_to_block(37);

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 8 },
		));

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			8,
		));
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			bob(),
			0,
			8,
		));

		alice_total_reward += calculate_round_reward(2000, 2000, 900, 900);

		// alice should get all rewards
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: alice_total_reward,
				last_round_reward: round_reward(),
				unpaid_reward: alice_total_reward,
				last_token_distributed_round: 8,
			}
		);

		// bob should not participate in the reward calculation
		assert_eq!(
			ScoreStaking::scores(bob()).unwrap(),
			ScorePayment {
				score: 0, // bob's score should be cleared
				total_reward: bob_total_reward,
				last_round_reward: 0,
				unpaid_reward: bob_total_reward,
				last_token_distributed_round: 8,
			}
		);
		assert_eq!(ScoreStaking::total_score(), 2000);
		assert_eq!(ScoreStaking::score_user_count(), 2); // entry is not yet removed

		// remove_score works
		assert_ok!(ScoreStaking::remove_score(RuntimeOrigin::signed(alice()), bob().into()));
		assert_eq!(ScoreStaking::total_score(), 2000);
		assert_eq!(ScoreStaking::score_user_count(), 1);
	});
}

#[test]
fn claim_works() {
	new_test_ext(true).execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

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
			Event::<Test>::RewardDistributionStarted { round_index: 2 },
		));
		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 2000,
				total_reward: 0,
				last_round_reward: round_reward(),
				unpaid_reward: 0,
				last_token_distributed_round: 0,
			}
		);

		// calculates the rewards
		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			0,
			2,
		));

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
				last_token_distributed_round: 2,
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
				last_token_distributed_round: 2,
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
	new_test_ext_with_parachain_staking().execute_with(|| {
		let enclave = Enclave::new(WorkerType::Identity);
		pallet_teebag::EnclaveRegistry::<Test>::insert(alice(), enclave);

		run_to_block(2);
		assert_ok!(ScoreStaking::start_pool(RuntimeOrigin::root()));

		run_to_block(3);
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			alice(),
			Delegator::new(bob(), bob(), 900),
		);
		pallet_parachain_staking::Total::<Test>::put(900);
		assert_ok!(ScoreStaking::update_score(RuntimeOrigin::signed(alice()), alice().into(), 500));

		// run to next reward distribution round, alice should win all rewards
		run_to_block(7);
		let mut total_reward = round_reward();

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionStarted { round_index: 2 },
		));

		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			1000,
			2,
		));

		total_reward += 1000;

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 500,
				total_reward,
				last_round_reward: total_reward,
				unpaid_reward: total_reward,
				last_token_distributed_round: 2,
			}
		);

		run_to_block(12);

		total_reward += round_reward();

		assert_noop!(
			ScoreStaking::update_token_staking_amount(
				RuntimeOrigin::signed(alice()),
				alice(),
				1000,
				2,
			),
			Error::<Test>::TokenStakingAmountAlreadyUpdated
		);

		assert_ok!(ScoreStaking::update_token_staking_amount(
			RuntimeOrigin::signed(alice()),
			alice(),
			1200,
			3,
		));

		total_reward += 1200;

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::TokenStakingAmountUpdated {
				account: alice(),
				amount_distributed: round_reward() + 1200,
			},
		));

		assert_eq!(
			ScoreStaking::scores(alice()).unwrap(),
			ScorePayment {
				score: 500,
				total_reward,
				last_round_reward: round_reward() + 1200,
				unpaid_reward: total_reward,
				last_token_distributed_round: 3,
			}
		);
	})
}

#[test]
fn update_token_staking_amount_origin_check_works() {
	new_test_ext(false).execute_with(|| {
		assert_noop!(
			ScoreStaking::update_token_staking_amount(
				RuntimeOrigin::signed(alice()),
				alice(),
				1000,
				1
			),
			sp_runtime::DispatchError::BadOrigin
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
				alice(),
				1000,
				1
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

		assert_ok!(ScoreStaking::complete_reward_distribution(RuntimeOrigin::signed(alice())));

		System::assert_last_event(RuntimeEvent::ScoreStaking(
			Event::<Test>::RewardDistributionCompleted {},
		));
	});
}

#[test]
fn complete_reward_distribution_origin_check_works() {
	new_test_ext(false).execute_with(|| {
		assert_noop!(
			ScoreStaking::complete_reward_distribution(RuntimeOrigin::signed(alice())),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn on_all_delegation_removed_works() {
	new_test_ext(true).execute_with(|| {
		let bob = bob();
		pallet_parachain_staking::DelegatorState::<Test>::insert(
			bob.clone(),
			Delegator::new(alice(), alice(), 1600),
		);
		assert_ok!(ScoreStaking::update_score(
			RuntimeOrigin::signed(alice()),
			Identity::from(bob.clone()),
			1000
		));

		assert_eq!(ScoreStaking::total_score(), 1000);
		assert_eq!(Scores::<Test>::get(&bob).unwrap().score, 1000);

		assert_ok!(ScoreStaking::on_all_delegation_removed(&bob));
		assert_eq!(ScoreStaking::total_score(), 0);
		assert_eq!(Scores::<Test>::get(&bob).unwrap().score, 0);
	});
}
