// Copyright 2020-2023 Trust Computing GmbH.
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
		.with_balances(vec![(u8_into_account_id(1u8), 130), (u8_into_account_id(2u8), 125)])
		.with_candidates(vec![(u8_into_account_id(1u8), 30)])
		.build()
		.execute_with(|| {
			precompiles()
				.prepare_test(
					u8_into_account_id(2u8),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegateWithAutoCompound)
						.write(u8_into_account_id(1u8))
						.write(10)
						.write(Percent::from_percent(50))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(true).build());

			assert_last_event!(MetaEvent::ParachainStaking(
				pallet_parachain_staking::Event::Delegation {
					delegator: u8_into_account_id(2u8),
					locked_amount: 10,
					candidate: u8_into_account_id(1u8),
					delegator_position: pallet_parachain_staking::DelegatorAdded::AddedToTop {
						new_total: 40
					},
					auto_compound: Percent::from_percent(50),
				}
			));
			assert_eq!(
				vec![pallet_parachain_staking::AutoCompoundConfig {
					delegator: u8_into_account_id(2u8),
					value: Percent::from_percent(50)
				}],
				ParachainStaking::auto_compounding_delegations(&u8_into_account_id(1u8)),
			);
		});
}

#[test]
fn delegation_request_is_pending_works() {
	ExtBuilder::default()
		.with_balances(vec![
			(u8_into_account_id(1), 10_000),
			(u8_into_account_id(2), 500),
			(u8_into_account_id(3), 500),
		])
		.with_candidates(vec![(u8_into_account_id(1), 1_000)])
		.with_delegations(vec![(u8_into_account_id(2), u8_into_account_id(1), 50)])
		.build()
		.execute_with(|| {
			// Assert that we dont have pending requests
			precompiles()
				.prepare_test(
					u8_into_account_id(1u8),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegationRequestIsPending)
						.write(u8_into_account_id(2u8))
						.write(u8_into_account_id(1u8))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(false).build());

			// Schedule Revoke request
			precompiles()
				.prepare_test(
					u8_into_account_id(2u8),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::ScheduleRevokeDelegation)
						.write(u8_into_account_id(1u8))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(true).build());

			// Assert that we have pending requests
			precompiles()
				.prepare_test(
					u8_into_account_id(1u8),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegationRequestIsPending)
						.write(u8_into_account_id(2u8))
						.write(u8_into_account_id(1u8))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(false).build());
		})
}
