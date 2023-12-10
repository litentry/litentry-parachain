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
		.with_balances(vec![(1u8.into(), 130), (2u8.into(), 125)])
		.with_candidates(vec![(1u8.into(), 30)])
		.build()
		.execute_with(|| {
			precompiles()
				.prepare_test(
					2u8.into(),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegateWithAutoCompound)
						.write(1u8.into())
						.write(10)
						.write(Percent::from_percent(50))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(true).build());

			assert_last_event!(MetaEvent::ParachainStaking(Event::Delegation {
				delegator: 2u8.into(),
				locked_amount: 10,
				candidate: 1u8.into(),
				delegator_position: DelegatorAdded::AddedToTop { new_total: 40 },
				auto_compound: Percent::from_percent(50),
			}));
			assert_eq!(
				vec![AutoCompoundConfig {
					delegator: 2u8.into(),
					value: Percent::from_percent(50)
				}],
				ParachainStaking::auto_compounding_delegations(&1u8.into()),
			);
		});
}

#[test]
fn delegation_request_is_pending_works() {
	ExtBuilder::default()
		.with_balances(vec![(1.into(), 10_000), (2.into(), 500), (3.into(), 500)])
		.with_candidates(vec![(1.into(), 1_000)])
		.with_delegations(vec![(2.into(), 1.into(), 50)])
		.build()
		.execute_with(|| {
			// Assert that we dont have pending requests
			precompiles()
				.prepare_test(
					1u8.into(),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegationRequestIsPending)
						.write(2u8.into())
						.write(1u8.into())
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(false).build());

			// Schedule Revoke request
			precompiles()
				.prepare_test(
					2u8.into(),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::ScheduleRevokeDelegation)
						.write(1u8.into())
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(true).build());

			// Assert that we have pending requests
			precompiles()
				.prepare_test(
					1u8.into(),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::DelegationRequestIsPending)
						.write(2u8.into())
						.write(1u8.into())
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(false).build());
		})
}
