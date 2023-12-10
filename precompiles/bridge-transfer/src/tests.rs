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
use fp_evm::ExitError;
use frame_support::assert_ok;
use precompile_utils::testing::*;
use sp_core::H160;
use sp_runtime::{traits::Zero, AccountId32, Perbill};
use super::mock::*;

fn precompiles() -> BridgeTransferMockPrecompile<Test> {
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
					2.into(),
					precompile_address(),
					EvmDataWriter::new_with_selector(Action::TransferNative)
						.write(Address(1u8.into()))
						.write(10)
						.write(Percent::from_percent(50))
						.build(),
				)
				.expect_no_logs()
				.execute_returns(EvmDataWriter::new().write(true).build());

			assert_event_emitted!(Event::Delegation {
				delegator: 2u8.into(),
				locked_amount: 10,
				candidate: 1u8.into(),
				delegator_position: DelegatorAdded::AddedToTop { new_total: 40 },
				auto_compound: Percent::from_percent(50),
			});
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
fn transfer_native_is_ok() {
	new_test_ext().execute_with(|| {
		let dest_bridge_id: bridge::BridgeChainId = 0;
		let resource_id = NativeTokenResourceId::get();
		let dest_account: Vec<u8> = vec![1];
		assert_ok!(pallet_bridge::Pallet::<Test>::update_fee(
			RuntimeOrigin::root(),
			dest_bridge_id,
			10
		));
		assert_ok!(pallet_bridge::Pallet::<Test>::whitelist_chain(
			RuntimeOrigin::root(),
			dest_bridge_id
		));

		precompiles()
			.prepare_test(
				1u8.into(),
				precompile_address(),
				EvmDataWriter::new_with_selector(Action::TransferNative)
					.write(100)
					.write(dest_account.clone())
					.write(dest_bridge_id)
					.build(),
			)
			.expect_no_logs()
			.execute_returns(EvmDataWriter::new().write(true).build());

		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(TreasuryAccount::get()),
			ENDOWED_BALANCE + 10
		);
		assert_eq!(pallet_balances::Pallet::<Test>::free_balance(RELAYER_A), ENDOWED_BALANCE - 100);
		assert_events(vec![
			mock::RuntimeEvent::Balances(pallet_balances::Event::Deposit {
				who: TreasuryAccount::get(),
				amount: 10,
			}),
			RuntimeEvent::Bridge(bridge::Event::FungibleTransfer(
				dest_bridge_id,
				1,
				resource_id,
				100 - 10,
				dest_account,
			)),
		]);
	})
}
