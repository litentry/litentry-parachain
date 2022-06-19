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
use codec::Encode;
use frame_support::{
	assert_noop, assert_ok,
	traits::{VestingSchedule, WrapperKeepOpaque},
};
use sp_runtime::traits::Dispatchable;
type OpaqueCall = WrapperKeepOpaque<<Runtime as frame_system::Config>::Call>;

#[test]
fn default_mode() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(ExtrinsicFilter::mode(), pallet_extrinsic_filter::OperationalMode::Normal);
	})
}

#[test]
fn multisig_enabled() {
	ExtBuilder::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let _ = Multisig::multi_account_id(&[alice(), bob(), charlie()][..], 2);
			let remark_call: Call = frame_system::Call::remark { remark: vec![] }.into();
			let data = remark_call.encode();
			let multisig_call: Call = pallet_multisig::Call::as_multi {
				threshold: 2,
				other_signatories: vec![bob(), charlie()],
				maybe_timepoint: None,
				call: OpaqueCall::from_encoded(data),
				store_call: false,
				max_weight: 0,
			}
			.into();
			assert_ok!(multisig_call.dispatch(Origin::signed(alice())));
		})
}

#[test]
fn balance_transfer_disabled() {
	ExtBuilder::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let call: Call =
				pallet_balances::Call::transfer { dest: bob().into(), value: 1 * UNIT }.into();
			assert_noop!(
				call.dispatch(Origin::signed(alice())),
				frame_system::Error::<Runtime>::CallFiltered
			);
		})
}

#[test]
fn balance_transfer_with_sudo_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let call: Call = pallet_balances::Call::force_transfer {
				source: alice().into(),
				dest: bob().into(),
				value: 1 * UNIT,
			}
			.into();
			assert_ok!(call.dispatch(Origin::root()),);
			assert_eq!(Balances::free_balance(&alice()), 9 * UNIT);
			assert_eq!(Balances::free_balance(&bob()), 1 * UNIT);
		})
}

#[test]
fn block_core_call_has_no_effect() {
	ExtBuilder::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let call: Call = frame_system::Call::remark { remark: vec![] }.into();
			assert_ok!(call.clone().dispatch(Origin::signed(alice())));

			// try to block System call, which is a core call
			assert_ok!(ExtrinsicFilter::block_extrinsics(Origin::root(), b"System".to_vec(), None));
			// it's stored in the storage
			assert_eq!(
				ExtrinsicFilter::blocked_extrinsics((b"System".to_vec(), Vec::<u8>::default())),
				Some(())
			);
			// ...however, no effect in the actual call dispatching
			assert_ok!(call.dispatch(Origin::signed(alice())));
		})
}

#[test]
fn block_non_core_call_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 100 * UNIT)])
		.build()
		.execute_with(|| {
			assert_ok!(Vesting::vested_transfer(
				Origin::signed(alice()),
				bob().into(),
				pallet_vesting::VestingInfo::new(10 * UNIT, 1 * UNIT, 0,),
			));
			let call: Call = pallet_vesting::Call::vest {}.into();
			assert_ok!(call.clone().dispatch(Origin::signed(bob())));
			assert_eq!(Balances::free_balance(&bob()), 10 * UNIT);
			assert_eq!(Balances::usable_balance(&bob()), 1 * UNIT);

			System::set_block_number(2);
			assert_eq!(Vesting::vesting_balance(&bob()), Some(8 * UNIT));

			// try to block Vesting call, which is a non-core call
			assert_ok!(ExtrinsicFilter::block_extrinsics(
				Origin::root(),
				b"Vesting".to_vec(),
				None
			));
			// it's stored in the storage
			assert_eq!(
				ExtrinsicFilter::blocked_extrinsics((b"Vesting".to_vec(), Vec::<u8>::default())),
				Some(())
			);
			// ...and it will take effect
			assert_noop!(
				call.dispatch(Origin::signed(bob())),
				frame_system::Error::<Runtime>::CallFiltered
			);
			// usable balance is unchanged
			assert_eq!(Balances::usable_balance(&bob()), 1 * UNIT);
		})
}
