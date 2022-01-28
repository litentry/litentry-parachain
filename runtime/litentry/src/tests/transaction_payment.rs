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

use super::setup::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_runtime::{traits::SignedExtension, MultiAddress};

#[test]
fn test_set_balance() {
	ExtBuilder::default()
		.balances(vec![
			// fund Alice and BOB
			(AccountId::from(ALICE), 123456789123456789),
			(AccountId::from(BOB), 123456789123456789),
		])
		.build()
		.execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				MultiAddress::Id(AccountId::from(BOB)),
				100,
				0
			));
		})
}

#[test]
fn test_payment() {
	ExtBuilder::default()
		.balances(vec![
			// fund Alice and BOB
			(AccountId::from(ALICE), 123456789123456789),
			(AccountId::from(BOB), 123456789123456789),
			(Treasury::account_id(), 123456789123456789),
		])
		.build()
		.execute_with(|| {
			assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 123456789123456789);
			assert_eq!(Balances::free_balance(Treasury::account_id()), 123456789123456789);

			let byte_fee: u128 = 1_000_000;
			let base_fee: u128 = 25_000_000;
			let info: u64 = 85;

			let tranfer_call: &<Runtime as frame_system::Config>::Call =
				&Call::Balances(BalancesCall::transfer {
					dest: MultiAddress::Id(AccountId::from(BOB)),
					value: 69,
				});

			let mut sender_balance = Balances::free_balance(&AccountId::from(ALICE));
			let mut treasury_balance = Balances::free_balance(Treasury::account_id());
			let len: u128 = 1000;
			let pre = <pallet_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0)
				.pre_dispatch(
					&ALICE.into(),
					tranfer_call,
					&info_from_weight(info as u64),
					len as usize,
				)
				.unwrap();
			// 1: initial 1000 balance, withdraw 5 base fee, 85 weight fee, 10 len fee
			// Treasury unchanged
			let total = 5 * base_fee + info as u128 + len * byte_fee;
			assert_eq!(sender_balance - Balances::free_balance(&AccountId::from(ALICE)), total,);
			assert_eq!(Balances::free_balance(Treasury::account_id()) - treasury_balance, 0);
			sender_balance = Balances::free_balance(&AccountId::from(ALICE));
			treasury_balance = Balances::free_balance(Treasury::account_id());
			assert_ok!(
				<pallet_transaction_payment::ChargeTransactionPayment::<Runtime>>::post_dispatch(
					pre,
					&info_from_weight(info),
					// so acutal weight is 35 + 5 + 10 = 50
					&post_info_from_weight(35),
					len as usize,
					&Ok(())
				)
			);
			// 1: balance refund 50
			assert_eq!(Balances::free_balance(&AccountId::from(ALICE)) - sender_balance, 50);
			// treasury pallet account get distribution 40 out of (40+0+60) proprtion of 50 actual
			// weight
			assert_eq!(
				Balances::free_balance(Treasury::account_id()) - treasury_balance,
				(total - 50) * 40 / (40 + 60)
			);
			assert_eq!(true, true);
		})
}
