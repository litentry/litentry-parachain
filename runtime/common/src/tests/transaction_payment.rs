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

use frame_support::{
	assert_ok,
	dispatch::{DispatchClass, DispatchInfo, PostDispatchInfo, RawOrigin},
	weights::{constants::ExtrinsicBaseWeight, IdentityFee, Weight, WeightToFee},
};
use pallet_balances::Call as BalancesCall;
use pallet_transaction_payment::{Multiplier, OnChargeTransaction};
use primitives::{AccountId, Balance};
use sp_runtime::traits::{Convert, Dispatchable, SignedExtension};

use crate::{
	currency::UNIT,
	tests::setup::{
		alice, bob, info_from_weight, post_info_from_weight, run_with_system_weight, ExtBuilder,
	},
	BaseRuntimeRequirements, MinimumMultiplier, NegativeImbalance, RuntimeBlockWeights,
	SlowAdjustingFeeUpdate, TargetBlockFullness,
};

type Balances<R> = pallet_balances::Pallet<R>;
type Treasury<R> = pallet_treasury::Pallet<R>;

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

pub fn multiplier_can_grow_from_zero<R: BaseRuntimeRequirements>() {
	// if the min is too small, then this will not change, and we are doomed forever.
	// the weight is 1/100th bigger than target.
	run_with_system_weight::<_, R>(target() * 101 / 100, || {
		let next = SlowAdjustingFeeUpdate::<R>::convert(min_multiplier());
		assert!(next > min_multiplier(), "{:?} !>= {:?}", next, min_multiplier());
	})
}

pub fn transaction_payment_works<
	R: BaseRuntimeRequirements + frame_system::Config<RuntimeCall = Call>,
	TransactionByteFee: frame_support::traits::Get<Balance>,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<RuntimeOrigin = Origin> + From<pallet_balances::Call<R>>,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<R as frame_system::Config>::RuntimeCall:
		Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
	<R as pallet_transaction_payment::Config>::OnChargeTransaction:
		OnChargeTransaction<R, Balance = Balance, LiquidityInfo = Option<NegativeImbalance<R>>>,
{
	ExtBuilder::<R>::default()
		.balances(vec![(alice(), 100 * UNIT), (Treasury::<R>::account_id(), 100 * UNIT)])
		.build()
		.execute_with(|| {
			assert_eq!(Balances::<R>::free_balance(&alice()), 100 * UNIT);
			assert_eq!(Balances::<R>::free_balance(Treasury::<R>::account_id()), 100 * UNIT);
			let initial_total_issuance = Balances::<R>::total_issuance();
			assert_eq!(initial_total_issuance, 200 * UNIT);

			let dispatch_info: u128 = 50;
			let post_dispatch_info: u128 = 35;
			let len = 10;

			// let tranfer_call: Call =
			// 	Call::Balances(BalancesCall::transfer { dest: bob().into(), value: 69 });
			let tranfer_call: Call =
				BalancesCall::transfer { dest: bob().into(), value: 69 }.into();
			let mut old_sender_balance = Balances::<R>::free_balance(&alice());
			let mut old_treasury_balance = Balances::<R>::free_balance(Treasury::<R>::account_id());
			let fee: Balance = 0;
			let pre = pallet_transaction_payment::ChargeTransactionPayment::<R>::from(fee)
				.pre_dispatch(
					&alice(),
					&tranfer_call,
					&info_from_weight(Weight::from_ref_time(dispatch_info as u64)),
					len as usize,
				)
				.unwrap();

			// This test here already assume that we use IdentityFee
			let total_payment: Balance =
				IdentityFee::<Balance>::weight_to_fee(&ExtrinsicBaseWeight::get()) +
					IdentityFee::<Balance>::weight_to_fee(&Weight::from_ref_time(
						dispatch_info as u64,
					)) + (len as Balance) * TransactionByteFee::get();
			assert_eq!(old_sender_balance - Balances::<R>::free_balance(&alice()), total_payment);
			assert_eq!(
				Balances::<R>::free_balance(Treasury::<R>::account_id()),
				old_treasury_balance
			);

			old_sender_balance = Balances::<R>::free_balance(&alice());
			old_treasury_balance = Balances::<R>::free_balance(Treasury::<R>::account_id());
			assert_ok!(<pallet_transaction_payment::ChargeTransactionPayment::<R>>::post_dispatch(
				Some(pre),
				&info_from_weight(Weight::from_ref_time(dispatch_info as u64)),
				&post_info_from_weight(Weight::from_ref_time(post_dispatch_info as u64)),
				len as usize,
				&Ok(())
			));
			// (dispatch_info - post_dispatch_info) weights (toFee) are refunded
			let refunded = dispatch_info - post_dispatch_info;
			assert_eq!(Balances::<R>::free_balance(&alice()) - old_sender_balance, refunded);

			// treasury gets 40% of actual payment
			let actual_payment = total_payment - refunded;
			assert_eq!(
				Balances::<R>::free_balance(Treasury::<R>::account_id()) - old_treasury_balance,
				actual_payment * 40 / (40 + 60)
			);

			// ... and the rest (= 60% of actual payment) is burnt
			assert_eq!(
				initial_total_issuance - Balances::<R>::total_issuance(),
				actual_payment * 60 / (40 + 60)
			);
		})
}

#[macro_export]
macro_rules! run_transaction_payment_tests {
	() => {
		use runtime_common::tests::transaction_payment;

		#[test]
		fn multiplier_can_grow_from_zero() {
			transaction_payment::multiplier_can_grow_from_zero::<Runtime>();
		}

		#[test]
		fn transaction_payment_works() {
			transaction_payment::transaction_payment_works::<
				Runtime,
				TransactionByteFee,
				RuntimeOrigin,
				RuntimeCall,
			>();
		}
	};
}
