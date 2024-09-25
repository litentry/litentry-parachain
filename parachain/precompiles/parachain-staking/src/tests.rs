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

extern crate alloc;
use crate::{
	mock::{RuntimeEvent as MetaEvent, *},
	*,
};
use precompile_utils::testing::*;

fn precompiles() -> ParachainStakingMockPrecompile<Test> {
	PrecompilesValue::get()
}

#[test]
fn test_delegate_with_auto_compound_is_ok() {
	ExtBuilder::default()
		.with_balances(vec![(U8Wrapper(1u8).into(), 130), (U8Wrapper(2u8).into(), 125)])
		.with_candidates(vec![(U8Wrapper(1u8).into(), 30)])
		.build()
		.execute_with(|| {
			precompiles()
				.prepare_test(
					U8Wrapper(2u8),
					precompile_address(),
					PCall::<Test>::delegate_with_auto_compound {
						candidate: H256::from(U8Wrapper(1u8)),
						amount: 10u128.into(),
						auto_compound: 50u8,
					},
				)
				.expect_no_logs()
				.execute_returns(());

			assert_last_event!(MetaEvent::ParachainStaking(
				pallet_parachain_staking::Event::Delegation {
					delegator: U8Wrapper(2u8).into(),
					locked_amount: 10,
					candidate: U8Wrapper(1u8).into(),
					delegator_position: pallet_parachain_staking::DelegatorAdded::AddedToTop {
						new_total: 40
					},
					auto_compound: Percent::from_percent(50),
				}
			));
			assert_eq!(
				vec![pallet_parachain_staking::AutoCompoundConfig {
					delegator: U8Wrapper(2u8).into(),
					value: Percent::from_percent(50)
				}],
				ParachainStaking::auto_compounding_delegations(Into::<AccountId>::into(U8Wrapper(
					1u8
				))),
			);
		});
}

#[test]
fn delegation_request_is_pending_works() {
	ExtBuilder::default()
		.with_balances(vec![
			(U8Wrapper(1).into(), 10_000),
			(U8Wrapper(2).into(), 500),
			(U8Wrapper(3).into(), 500),
		])
		.with_candidates(vec![(U8Wrapper(1).into(), 1_000)])
		.with_delegations(vec![(U8Wrapper(2).into(), U8Wrapper(1).into(), 50)])
		.build()
		.execute_with(|| {
			// Assert that we dont have pending requests
			precompiles()
				.prepare_test(
					U8Wrapper(1u8),
					precompile_address(),
					PCall::<Test>::delegation_request_is_pending {
						delegator: H256::from(U8Wrapper(2u8)),
						candidate: H256::from(U8Wrapper(1u8)),
					},
				)
				.expect_no_logs()
				.execute_returns(false);

			// Schedule Revoke request
			precompiles()
				.prepare_test(
					U8Wrapper(2u8),
					precompile_address(),
					PCall::<Test>::schedule_revoke_delegation {
						candidate: H256::from(U8Wrapper(1u8)),
					},
				)
				.expect_no_logs()
				.execute_returns(());

			// Assert that we have pending requests
			precompiles()
				.prepare_test(
					U8Wrapper(1u8),
					precompile_address(),
					PCall::<Test>::delegation_request_is_pending {
						delegator: H256::from(U8Wrapper(2u8)),
						candidate: H256::from(U8Wrapper(1u8)),
					},
				)
				.expect_no_logs()
				.execute_returns(true);
		})
}
