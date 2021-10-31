use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::traits::SignedExtension;

#[test]
fn signed_extension_transaction_payment_work() {
	ExtBuilder::default()
		.balance_factor(10)
		.base_weight(5)
		.build()
		.execute_with(|| {
			let mut sender_balance = Balances::free_balance(1);
			let mut treasury_balance = Balances::free_balance(Treasury::account_id());
			let len = 10;
			let pre = pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0)
				.pre_dispatch(&1, CALL, &info_from_weight(85), len)
				.unwrap();
			// 1: initial 1000 balance, withdraw 5 base fee, 85 weight fee, 10 len fee
			// Treasury unchanged
			assert_eq!(sender_balance - Balances::free_balance(1), 5 + 85 + 10);
			assert_eq!(Balances::free_balance(Treasury::account_id()) - treasury_balance, 0);
			sender_balance = Balances::free_balance(1);
			treasury_balance = Balances::free_balance(Treasury::account_id());
			assert_ok!(
				pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::post_dispatch(
					pre,
					&info_from_weight(85),
					// so acutal weight is 35 + 5 + 10 = 50
					&post_info_from_weight(35),
					len,
					&Ok(())
				)
			);
			// 1: balance refund 50
			assert_eq!(Balances::free_balance(1) - sender_balance, 50);
			// treasury pallet account get distribution 40 out of (40+0+60) proprtion of 50 actual
			// weight
			assert_eq!(
				Balances::free_balance(Treasury::account_id()) - treasury_balance,
				50 * 40 / (40 + 0 + 60)
			);
			// TODO: author account get distribution
		});
}
