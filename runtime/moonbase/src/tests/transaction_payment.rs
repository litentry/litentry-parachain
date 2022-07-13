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
use frame_support::{
	assert_ok,
	weights::{constants::ExtrinsicBaseWeight, DispatchClass, Weight},
};
use pallet_transaction_payment::Multiplier;
use runtime_common::{
    currency::*, MinimumMultiplier, RuntimeBlockWeights, SlowAdjustingFeeUpdate,
    TargetBlockFullness,
};
use sp_runtime::traits::{Convert, SignedExtension};

fn max_normal() -> Weight {
	RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_total
		.unwrap_or_else(|| RuntimeBlockWeights::get().max_block)
}

fn min_multiplier() -> Multiplier {
	MinimumMultiplier::get()
}

fn target() -> Weight {
	TargetBlockFullness::get() * max_normal()
}

#[test]
fn multiplier_can_grow_from_zero() {
	// if the min is too small, then this will not change, and we are doomed forever.
	// the weight is 1/100th bigger than target.
	run_with_system_weight(target() * 101 / 100, || {
		let next = SlowAdjustingFeeUpdate::<Runtime>::convert(min_multiplier());
		assert!(next > min_multiplier(), "{:?} !>= {:?}", next, min_multiplier());
	})
}

#[test]
fn transaction_payment_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 100 * UNIT), (Treasury::account_id(), 100 * UNIT)])
		.build()
		.execute_with(|| {
			assert_eq!(Balances::free_balance(&alice()), 100 * UNIT);
			assert_eq!(Balances::free_balance(Treasury::account_id()), 100 * UNIT);
			let initial_total_issuance = Balances::total_issuance();
			assert_eq!(initial_total_issuance, 200 * UNIT);

			let dispatch_info: u128 = 50;
			let post_dispatch_info: u128 = 35;
			let len = 10;

			let tranfer_call: Call =
				Call::Balances(BalancesCall::transfer { dest: bob().into(), value: 69 });

			let mut old_sender_balance = Balances::free_balance(&alice());
			let mut old_treasury_balance = Balances::free_balance(Treasury::account_id());

			let pre = pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0)
				.pre_dispatch(
					&alice(),
					&tranfer_call,
					&info_from_weight(dispatch_info as u64),
					len as usize,
				)
				.unwrap();

			let total_payment: Balance = ExtrinsicBaseWeight::get() as u128 +
				dispatch_info + len * TransactionByteFee::get();
			assert_eq!(old_sender_balance - Balances::free_balance(&alice()), total_payment);
			assert_eq!(Balances::free_balance(Treasury::account_id()), old_treasury_balance);

			old_sender_balance = Balances::free_balance(&alice());
			old_treasury_balance = Balances::free_balance(Treasury::account_id());
			assert_ok!(
				<pallet_transaction_payment::ChargeTransactionPayment::<Runtime>>::post_dispatch(
					Some(pre),
					&info_from_weight(dispatch_info as u64),
					&post_info_from_weight(post_dispatch_info as u64),
					len as usize,
					&Ok(())
				)
			);
			// (dispatch_info - post_dispatch_info) weights (toFee) are refunded
			let refunded = dispatch_info - post_dispatch_info;
			assert_eq!(Balances::free_balance(&alice()) - old_sender_balance, refunded);

			// treasury gets 40% of actual payment
			let actual_payment = total_payment - refunded;
			assert_eq!(
				Balances::free_balance(Treasury::account_id()) - old_treasury_balance,
				actual_payment * 40 / (40 + 60)
			);

			// ... and the rest (= 60% of actual payment) is burnt
			assert_eq!(
				initial_total_issuance - Balances::total_issuance(),
				actual_payment * 60 / (40 + 60)
			);
		})
}
