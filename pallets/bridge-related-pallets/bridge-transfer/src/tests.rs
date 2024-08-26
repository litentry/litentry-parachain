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

use super::{
	bridge,
	mock::{
		assert_events, balances, new_test_ext, Balances, Bridge, BridgeTransfer,
		NativeTokenResourceId, ProposalLifetime, RuntimeCall, RuntimeEvent, RuntimeOrigin, Test,
		TreasuryAccount, ENDOWED_BALANCE, RELAYER_A, RELAYER_B, RELAYER_C,
	},
	*,
};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use sp_runtime::ArithmeticError;

fn make_transfer_proposal(to: u64, amount: u64) -> RuntimeCall {
	let rid = NativeTokenResourceId::get();
	// let amount
	RuntimeCall::BridgeTransfer(crate::Call::transfer { to, amount, rid })
}

#[test]
fn constant_equality() {
	let r_id = bridge::derive_resource_id(1, &bridge::hashing::blake2_128(b"LIT"));
	let encoded: [u8; 32] =
		hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
	assert_eq!(r_id, encoded);
}

#[test]
fn transfer() {
	let resource_id = NativeTokenResourceId::get();

	new_test_ext().execute_with(|| {
		// Transfer and check result
		assert_ok!(BridgeTransfer::transfer(
			RuntimeOrigin::signed(Bridge::account_id()),
			RELAYER_A,
			10,
			resource_id,
		));
		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		assert_events(vec![RuntimeEvent::Balances(balances::Event::Minted {
			who: RELAYER_A,
			amount: 10,
		})]);
	})
}

#[test]
fn transfer_assets() {
	let dest_bridge_id: bridge::BridgeChainId = 0;
	let resource_id = NativeTokenResourceId::get();

	new_test_ext().execute_with(|| {
		let dest_account: Vec<u8> = vec![1];
		assert_ok!(Pallet::<Test>::transfer_assets(
			RuntimeOrigin::signed(RELAYER_A),
			100,
			dest_account.clone(),
			dest_bridge_id,
			resource_id
		));
		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(TreasuryAccount::get()),
			ENDOWED_BALANCE
		);
		assert_eq!(pallet_balances::Pallet::<Test>::free_balance(RELAYER_A), ENDOWED_BALANCE - 100);
		assert_events(vec![
			RuntimeEvent::Balances(balances::Event::Burned { who: RELAYER_A, amount: 100 }),
			RuntimeEvent::Balances(pallet_balances::Event::Minted {
				who: TreasuryAccount::get(),
				amount: 0,
			}),
			RuntimeEvent::AssetsHandler(pallet_assets_handler::Event::TokenBridgeOut {
				asset_id: None,
				from: RELAYER_A,
				amount: 100,
				fee: 0,
			}),
			RuntimeEvent::Bridge(bridge::Event::FungibleTransfer(
				dest_bridge_id,
				1,
				resource_id,
				100,
				dest_account,
			)),
		]);
	})
}

#[test]
fn mint_overflow() {
	let resource_id = NativeTokenResourceId::get();

	new_test_ext().execute_with(|| {
		let bridge_id: u64 = Bridge::account_id();
		assert_eq!(Balances::free_balance(bridge_id), ENDOWED_BALANCE);

		assert_noop!(
			BridgeTransfer::transfer(
				RuntimeOrigin::signed(Bridge::account_id()),
				RELAYER_A,
				u64::MAX,
				resource_id,
			),
			pallet_assets_handler::Error::<Test>::Overflow
		);
	})
}
