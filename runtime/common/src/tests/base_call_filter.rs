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

use codec::{Decode, Encode};
use frame_support::{
	assert_noop, assert_ok,
	traits::{VestingSchedule, WrapperKeepOpaque},
};
use frame_system::RawOrigin;
use sp_runtime::traits::Dispatchable;

use primitives::AccountId;

use crate::{currency::UNIT, xcm_impl::RuntimeConfig};

use crate::tests::setup::{alice, bob, charlie, ExtBuilder};

type OpaqueCall<R> = WrapperKeepOpaque<<R as pallet_multisig::Config>::Call>;
type ExtrinsicFilter<R> = pallet_extrinsic_filter::Pallet<R>;
type System<R> = frame_system::Pallet<R>;
type Balances<R> = pallet_balances::Pallet<R>;
type Vesting<R> = pallet_vesting::Pallet<R>;
type Multisig<R> = pallet_multisig::Pallet<R>;

pub fn default_mode<R: RuntimeConfig>() {
	ExtBuilder::<R>::default().build().execute_with(|| {
		assert_eq!(ExtrinsicFilter::<R>::mode(), pallet_extrinsic_filter::OperationalMode::Normal);
	});
}

pub fn multisig_enabled<
	R: RuntimeConfig + pallet_multisig::Config,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone
		+ Dispatchable<Origin = Origin>
		+ From<pallet_multisig::Call<R>>
		+ From<frame_system::Call<R>>
		+ Encode,
>()
where
	<R as frame_system::Config>::Call: Decode,
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<Call as Dispatchable>::PostInfo: sp_std::fmt::Debug + Default,
{
	ExtBuilder::<R>::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let _ = Multisig::<R>::multi_account_id(&[alice(), bob(), charlie()][..], 2);
			let remark_call: Call = frame_system::Call::remark { remark: vec![] }.into();
			let data = remark_call.encode();
			let multisig_call: Call = pallet_multisig::Call::as_multi {
				threshold: 2,
				other_signatories: vec![bob(), charlie()],
				maybe_timepoint: None,
				call: OpaqueCall::<R>::from_encoded(data),
				store_call: false,
				max_weight: 0,
			}
			.into();
			assert_ok!(multisig_call.dispatch(Origin::signed(alice())));
		})
}

pub fn balance_transfer_disabled<
	R: RuntimeConfig,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_balances::Call<R>> + Encode,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<Call as Dispatchable>::PostInfo: sp_std::fmt::Debug + Default,
{
	ExtBuilder::<R>::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let call: Call =
				pallet_balances::Call::transfer { dest: bob().into(), value: 1 * UNIT }.into();
			assert_noop!(
				call.dispatch(Origin::signed(alice())),
				frame_system::Error::<R>::CallFiltered
			);
		})
}

pub fn balance_transfer_with_sudo_works<
	R: RuntimeConfig,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_balances::Call<R>> + Encode,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<Call as Dispatchable>::PostInfo: sp_std::fmt::Debug + Default,
{
	ExtBuilder::<R>::default()
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
			assert_eq!(Balances::<R>::free_balance(&alice()), 9 * UNIT);
			assert_eq!(Balances::<R>::free_balance(&bob()), 1 * UNIT);
		})
}

pub fn block_core_call_has_no_effect<
	R: RuntimeConfig + frame_system::Config<Origin = Origin>,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<Origin = Origin> + From<frame_system::Call<R>> + Encode,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<Call as Dispatchable>::PostInfo: sp_std::fmt::Debug + Default,
{
	ExtBuilder::<R>::default()
		.balances(vec![(alice(), 10 * UNIT)])
		.build()
		.execute_with(|| {
			let call: Call = frame_system::Call::remark { remark: vec![] }.into();
			assert_ok!(call.clone().dispatch(Origin::signed(alice())));

			// try to block System call, which is a core call
			assert_ok!(ExtrinsicFilter::<R>::block_extrinsics(
				Origin::root(),
				b"System".to_vec(),
				None
			)); // it's stored in the storage
			assert_eq!(
				ExtrinsicFilter::<R>::blocked_extrinsics((
					b"System".to_vec(),
					Vec::<u8>::default()
				)),
				Some(())
			);
			// ...however, no effect in the actual call dispatching
			assert_ok!(call.dispatch(Origin::signed(alice())));
		})
}

pub fn block_non_core_call_works<
	R: RuntimeConfig
		+ frame_system::Config<Origin = Origin>
		+ pallet_vesting::Config<Currency = Balances<R>>,
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_vesting::Call<R>>,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,
	<Call as Dispatchable>::PostInfo: sp_std::fmt::Debug + Default,
{
	ExtBuilder::<R>::default()
		.balances(vec![(alice(), 100 * UNIT)])
		.build()
		.execute_with(|| {
			assert_ok!(Vesting::<R>::vested_transfer(
				Origin::signed(alice()),
				bob().into(),
				pallet_vesting::VestingInfo::new(10 * UNIT, 1 * UNIT, 0,),
			));
			let call: Call = pallet_vesting::Call::vest {}.into();
			assert_ok!(call.clone().dispatch(Origin::signed(bob())));
			assert_eq!(Balances::<R>::free_balance(&bob()), 10 * UNIT);
			assert_eq!(Balances::<R>::usable_balance(&bob()), 1 * UNIT);

			System::<R>::set_block_number(2);
			assert_eq!(Vesting::<R>::vesting_balance(&bob()), Some(8 * UNIT));

			// try to block Vesting call, which is a non-core call
			assert_ok!(ExtrinsicFilter::<R>::block_extrinsics(
				Origin::root(),
				b"Vesting".to_vec(),
				None
			));
			// it's stored in the storage
			assert_eq!(
				ExtrinsicFilter::<R>::blocked_extrinsics((
					b"Vesting".to_vec(),
					Vec::<u8>::default()
				)),
				Some(())
			);
			// ...and it will take effect
			assert_noop!(
				call.dispatch(Origin::signed(bob())),
				frame_system::Error::<R>::CallFiltered
			);
			// usable balance is unchanged
			assert_eq!(Balances::<R>::usable_balance(&bob()), 1 * UNIT);
		})
}
