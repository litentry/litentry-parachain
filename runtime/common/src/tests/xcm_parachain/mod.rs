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

use std::marker::PhantomData;

use codec::Encode;
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::ParachainInherentData;
use frame_support::{
	assert_noop, assert_ok,
	pallet_prelude::Weight,
	traits::{Currency, Get, OriginTrait, PalletInfoAccess},
};
use frame_system::RawOrigin;
use orml_traits::MultiCurrency;
use polkadot_parachain::primitives::RelayChainBlockNumber;
use sp_runtime::{
	traits::{Convert, Dispatchable},
	AccountId32,
};
use xcm::prelude::{
	All, Any, AssetId as XCMAssetId, Fungibility, Here, Instruction, Junction, MultiAsset,
	MultiLocation, OriginKind, Outcome, PalletInstance, Parachain, Parent, WeightLimit, Xcm,
	XcmError,
};
use xcm_executor::traits::Convert as xcmConvert;
use xcm_simulator::TestExt;

use primitives::{AccountId, AssetId, Balance, XcmV2Weight};

use crate::{
	currency::{CENTS, MILLICENTS, UNIT},
	tests::setup::{alice, bob, relay::SovereignAccountOf, BOB, PARA_A_USER_INITIAL_BALANCE},
	xcm_impl::{CurrencyId, CurrencyIdMultiLocationConvert},
	ParaRuntimeRequirements,
};

pub mod relay_sproof_builder;

pub const RELAY_UNIT: u128 = 1;

type XTokens<R> = orml_xtokens::Pallet<R>;
type ExtrinsicFilter<R> = pallet_extrinsic_filter::Pallet<R>;
type Balances<R> = pallet_balances::Pallet<R>;
type Tokens<R> = orml_tokens::Pallet<R>;
type AssetManager<R> = pallet_asset_manager::Pallet<R>;
type ParachainSystem<R> = cumulus_pallet_parachain_system::Pallet<R>;
type PolkadotXcm<R> = pallet_xcm::Pallet<R>;
type System<R> = frame_system::Pallet<R>;
// type XcmFeesAccount<R> = pallet_treasury::Pallet<R>::account_id();

fn para_account(x: u32) -> AccountId32 {
	<SovereignAccountOf as xcmConvert<MultiLocation, AccountId32>>::convert(Parachain(x).into())
		.unwrap()
}

fn sibling_account<LocationToAccountId: xcmConvert<MultiLocation, AccountId32>>(
	x: u32,
) -> AccountId32 {
	<LocationToAccountId as xcmConvert<MultiLocation, AccountId32>>::convert(
		(Parent, Parachain(x)).into(),
	)
	.unwrap()
}

fn relay_account<LocationToAccountId: xcmConvert<MultiLocation, AccountId32>>() -> AccountId32 {
	<LocationToAccountId as xcmConvert<MultiLocation, AccountId32>>::convert(Parent.into()).unwrap()
}

fn para_native_token_multilocation<R: ParaRuntimeRequirements>(x: u32) -> MultiLocation {
	(Parent, Parachain(x), PalletInstance(<Balances<R> as PalletInfoAccess>::index() as u8)).into()
}

pub trait TestXCMRequirements {
	type ParaOrigin: frame_support::traits::OriginTrait<AccountId = AccountId>
		+ From<RawOrigin<AccountId>>;
	type ParaCall: Clone
		+ Dispatchable<RuntimeOrigin = Self::ParaOrigin>
		+ From<pallet_balances::Call<Self::ParaRuntime>>
		+ Encode;
	type ParaRuntime: ParaRuntimeRequirements
		+ frame_system::Config<AccountId = AccountId, RuntimeOrigin = Self::ParaOrigin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<Self::ParaRuntime>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<Self::ParaRuntime>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config;
	type ParaA: xcm_simulator::TestExt;
	type ParaB: xcm_simulator::TestExt;
	type Relay: xcm_simulator::TestExt;
	type RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId>
		+ From<RawOrigin<AccountId>>;
	type RelayCall: Clone
		+ Dispatchable<RuntimeOrigin = Self::RelayOrigin>
		+ From<pallet_balances::Call<Self::RelayRuntime>>
		+ Encode;
	type RelayRuntime: frame_system::Config<AccountId = AccountId, RuntimeOrigin = Self::RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>;
	type UnitWeightCost: frame_support::traits::Get<XcmV2Weight>;
	type LocationToAccountId: xcmConvert<MultiLocation, AccountId32>;

	fn reset();
}

pub fn test_xtokens_recognize_multilocation<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	let xcm_fees_account = pallet_treasury::Pallet::<R::ParaRuntime>::account_id();
	R::ParaA::execute_with(|| {
		// Wrong Multilocation does not pass
		assert_noop!(
			XTokens::<R::ParaRuntime>::transfer(
				R::ParaOrigin::signed(alice()),
				CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
				UNIT,
				Box::new((Parent, Parachain(2)).into()),
				xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
			),
			orml_xtokens::Error::<R::ParaRuntime>::NotSupportedMultiLocation
		);

		assert_ok!(XTokens::<R::ParaRuntime>::transfer(
			R::ParaOrigin::signed(alice()),
			CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
			UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - UNIT
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT /* Notice this is interesting, as it suggest local preserve XCM
			      * fee belongs to remote chain, not local chain */
		);
	});

	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			UNIT - u128::from(R::UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(0, &xcm_fees_account),
			u128::from(R::UnitWeightCost::get() * 4)
		);

		// Send ParaA token back to ParachainA's BOB
		assert_ok!(XTokens::<R::ParaRuntime>::transfer(
			R::ParaOrigin::signed(bob()),
			CurrencyId::ParachainReserve(Box::new(
				para_native_token_multilocation::<R::ParaRuntime>(1)
			)),
			40 * CENTS,
			Box::new(
				(Parent, Parachain(1), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
	});

	R::ParaA::execute_with(|| {
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&bob()),
			40 * CENTS - u128::from(R::UnitWeightCost::get() * 4)
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			60 * CENTS /* When non-native assets transferred, the xcm fee is moved to
			            * XcmFeesAccount, which is Treasury, but native token just burned */
		);
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&xcm_fees_account), 0);
	});
}

// If this test fail, at least some part of XCM fee rule changes
pub fn test_xtokens_weight_parameter<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	let xcm_fees_account = pallet_treasury::Pallet::<R::ParaRuntime>::account_id();
	R::ParaA::execute_with(|| {
		// Insufficient weight still pass, but has no effect on remote chain
		assert_ok!(XTokens::<R::ParaRuntime>::transfer(
			R::ParaOrigin::signed(alice()),
			CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
			UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get())
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - UNIT
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT /* Notice this is interesting, as it suggest local preserve XCM
			      * fee belongs to remote chain, not local chain */
		);
	});
	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			0
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(Tokens::<R::ParaRuntime>::free_balance(0, &xcm_fees_account), 0);
	});

	R::ParaA::execute_with(|| {
		// Redundant weight pass but remote the chain charges its own rule and returns the surplus
		assert_ok!(XTokens::<R::ParaRuntime>::transfer(
			R::ParaOrigin::signed(alice()),
			CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
			UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			// R::UnitWeightCost::get() * 5
			xcm_simulator::Limited(R::UnitWeightCost::get() * 5)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - 2 * UNIT
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			2 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});

	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			UNIT - u128::from(R::UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(0, &xcm_fees_account),
			u128::from(R::UnitWeightCost::get() * 4)
		);
	});
}

pub fn test_pallet_xcm_recognize_multilocation<R: TestXCMRequirements>()
where
	<R::ParaRuntime as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<R::ParaRuntime>>,
{
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// It is sent but with XCM execution failed as Parachain is not exist.
		// Unregistereed Parachain Multilocation does not pass
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::reserve_transfer_assets(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(4)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R::ParaRuntime>::convert(CurrencyId::<
							R::ParaRuntime,
						>::SelfReserve(
							PhantomData::default()
						))
						.unwrap(),
					),
					fun: Fungibility::Fungible(UNIT),
				}]
				.into()
			),
			0
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - UNIT
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			0
		);
		// Not XCMP_QUEUE in production environment
		// This is the error of mimic XcmRouter: decl_test_network
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::<R::ParaRuntime>::Attempted(Outcome::Incomplete(
				R::UnitWeightCost::get(),
				XcmError::Unroutable,
			))
			.into(),
		);

		assert_ok!(PolkadotXcm::<R::ParaRuntime>::reserve_transfer_assets(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R::ParaRuntime>::convert(CurrencyId::<
							R::ParaRuntime,
						>::SelfReserve(
							PhantomData::default()
						))
						.unwrap(),
					),
					fun: Fungibility::Fungible(2 * UNIT),
				}]
				.into()
			),
			0
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - 3 * UNIT
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			2 * UNIT // Only non trpped asset is in sovereign account
		);
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::<R::ParaRuntime>::Attempted(Outcome::Complete(
				R::UnitWeightCost::get(),
			))
			.into(),
		);
	});

	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			2 * UNIT - u128::from(R::UnitWeightCost::get() * 4)
		);
	});
	// Notice so far pallet_xcm does not handle the asset transfer back - 0.9.23
}

pub fn test_methods_xtokens_expected_succeed<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		// Solve the DustLost first
		let _ = pallet_balances::Pallet::<R::ParaRuntime>::deposit_creating(
			&sibling_account::<R::LocationToAccountId>(2),
			UNIT,
		);

		// Sending 10 ParaA token after xcm fee to BOB by XTokens::transfer_multiasset
		assert_ok!(XTokens::<R::ParaRuntime>::transfer_multiasset(
			R::ParaOrigin::signed(alice()),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + CENTS)
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 4) - CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 4) + CENTS
		);

		// Sending 100 ParaA token after xcm fee to BOB by XTokens::transfer_with_fee
		assert_ok!(XTokens::<R::ParaRuntime>::transfer_with_fee(
			R::ParaOrigin::signed(alice()),
			CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
			10 * CENTS,
			(R::UnitWeightCost::get() * 4).into(),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 8) + 11 * CENTS
		);

		// Sending 1 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiasset_with_fee
		assert_ok!(XTokens::<R::ParaRuntime>::transfer_multiasset_with_fee(
			R::ParaOrigin::signed(alice()),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(UNIT)
				}
				.into()
			),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into())
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 12) - 111 * CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 12) + 111 * CENTS
		);

		// Sending 10 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multicurrencies
		assert_ok!(XTokens::<R::ParaRuntime>::transfer_multicurrencies(
			R::ParaOrigin::signed(alice()),
			vec![(
				CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
				u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT
			)],
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 16) - 1111 * CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 16) + 1111 * CENTS
		);

		// Sending 100 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiassets
		assert_ok!(XTokens::<R::ParaRuntime>::transfer_multiassets(
			R::ParaOrigin::signed(alice()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(
						u128::from(R::UnitWeightCost::get() * 4) + 100 * UNIT
					)
				}]
				.into()
			),
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
		));
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 20) - 11111 * CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 20) + 11111 * CENTS
		);
	});

	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11111 * CENTS
		);
	});
}

pub fn test_methods_xtokens_expected_fail<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	// Sending 1 ParaA token after xcm fee to BOB by XTokens::transfer
	R::ParaA::execute_with(|| {
		// Dust Lost make transaction failed
		assert_noop!(
			XTokens::<R::ParaRuntime>::transfer(
				R::ParaOrigin::signed(alice()),
				CurrencyId::<R::ParaRuntime>::SelfReserve(PhantomData::default()),
				u128::from(R::UnitWeightCost::get() * 4) + 100 * MILLICENTS,
				Box::new(
					(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
				),
				// R::UnitWeightCost::get() * 4
				xcm_simulator::Limited(R::UnitWeightCost::get() * 4)
			),
			orml_xtokens::Error::<R::ParaRuntime>::XcmExecutionFailed
		);
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			// This is caused by DustLost of pallet_balances
			// We keep this single weird test implementation to see if
			// omrl_xtoken changes way of handling such.
			// The issue is minor: We should fund/test real token
			// transfer with amount more than DustLost
			0
		);
	});
}

pub fn test_methods_pallet_xcm_expected_succeed<R: TestXCMRequirements>()
where
	<R::ParaRuntime as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<R::ParaRuntime>>,
{
	relaychain_parachains_set_up::<R>();

	R::ParaA::execute_with(|| {
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::reserve_transfer_assets(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R::ParaRuntime>::convert(CurrencyId::<
							R::ParaRuntime,
						>::SelfReserve(
							PhantomData::default()
						))
						.unwrap(),
					),
					fun: Fungibility::Fungible(
						u128::from(R::UnitWeightCost::get() * 4) + 100 * MILLICENTS
					),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// Unlike orml_xtoken, pallet_xcm fails with event when DustLost issue happens
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Incomplete(
				R::UnitWeightCost::get(),
				XcmError::FailedToTransactAsset(""),
			))
			.into(),
		);
		// Solve the DustLost
		let _ = pallet_balances::Pallet::<R::ParaRuntime>::deposit_creating(
			&sibling_account::<R::LocationToAccountId>(2),
			UNIT,
		);

		assert_ok!(PolkadotXcm::<R::ParaRuntime>::reserve_transfer_assets(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + CENTS)
				}]
				.into()
			),
			0
		));
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Complete(R::UnitWeightCost::get())).into(),
		);

		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 4) - CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 4) + CENTS
		);

		assert_ok!(PolkadotXcm::<R::ParaRuntime>::limited_reserve_transfer_assets(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(
						u128::from(R::UnitWeightCost::get() * 4) + 10 * CENTS
					)
				}]
				.into()
			),
			0,
			WeightLimit::Limited(R::UnitWeightCost::get() * 4)
		));
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Complete(R::UnitWeightCost::get())).into(),
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(R::UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			UNIT + u128::from(R::UnitWeightCost::get() * 8) + 11 * CENTS
		);
	});

	R::ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11 * CENTS
		);
	});
}

pub fn test_methods_pallet_xcm_expected_fail<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + 10 * CENTS),
		}]
		.into();
		let dest = (Parent, Parachain(2)).into();
		let xcm = Xcm(vec![
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		let message = Xcm(vec![Instruction::TransferReserveAsset { assets, dest, xcm }]);
		// Stopped by filter， nothing passed by execute, pallet_xcm::XcmExecuteFilter
		// If there is no pallet_xcm filter protection, then we should test XcmExexutor::Barrier
		// setting here in future
		assert_noop!(
			PolkadotXcm::<R::ParaRuntime>::execute(
				R::ParaOrigin::signed(alice()),
				Box::new(xcm::VersionedXcm::V2(message)),
				Weight::from_ref_time(R::UnitWeightCost::get() * 4)
			),
			pallet_xcm::Error::<R::ParaRuntime>::Filtered
		);

		// Stopped by filter， nothing passed by execute, pallet_xcm::XcmTeleportFilter
		assert_noop!(
			PolkadotXcm::<R::ParaRuntime>::teleport_assets(
				R::ParaOrigin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: XCMAssetId::Concrete(
							para_native_token_multilocation::<R::ParaRuntime>(1)
						),
						fun: Fungibility::Fungible(
							u128::from(R::UnitWeightCost::get() * 4) + CENTS
						)
					}]
					.into()
				),
				0
			),
			pallet_xcm::Error::<R::ParaRuntime>::Filtered
		);

		assert_noop!(
			PolkadotXcm::<R::ParaRuntime>::limited_teleport_assets(
				R::ParaOrigin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: XCMAssetId::Concrete(
							para_native_token_multilocation::<R::ParaRuntime>(1)
						),
						fun: Fungibility::Fungible(
							u128::from(R::UnitWeightCost::get() * 4) + 10 * CENTS
						)
					}]
					.into()
				),
				0,
				WeightLimit::Limited(R::UnitWeightCost::get() * 4)
			),
			pallet_xcm::Error::<R::ParaRuntime>::Filtered
		);
	})
}

// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
pub fn test_pallet_xcm_send_capacity_between_sibling<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	// Send result Xcm of pallet_xcm::reserve_transfer_assets by user
	R::ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::send(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaB::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			0
		);
	});

	// Send result Xcm of pallet_xcm::reserve_transfer_assets by root
	R::ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaB::execute_with(|| {
		// The remote received and handled exactly same result as normal transaction
		assert_eq!(
			Tokens::<R::ParaRuntime>::free_balance(
				0, // Asset_id=0. The first registered Token in Para B
				&bob()
			),
			10 * UNIT
		);
	});
	R::ParaA::execute_with(|| {
		// Fill up the missing assets
		let _ = pallet_balances::Pallet::<R::ParaRuntime>::deposit_creating(
			&sibling_account::<R::LocationToAccountId>(2),
			u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT,
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT
		);
	});

	// Users on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	R::ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::send(
			R::ParaOrigin::signed(alice()),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			u128::from(R::UnitWeightCost::get() * 4) + 10 * UNIT
		);
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&bob()), 0);
	});

	// Root on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	R::ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible((R::UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// The remote received and handled; So we trust root power no matter.
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&sibling_account::<R::LocationToAccountId>(2)),
			3 * UNIT
		);
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&bob()), 7 * UNIT);
	});
}

// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
pub fn test_pallet_xcm_send_capacity_without_transact<R: TestXCMRequirements>() {
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		assert_ok!(AssetManager::<R::ParaRuntime>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new((Parent, Here).into())),
			Default::default()
		));
		assert_ok!(AssetManager::<R::ParaRuntime>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			50_000 * RELAY_UNIT /*  Although does not matter here
			                     *1_000_000_000_000 / 20_000_000; Since Para UnitWeightCost :
			                     * Relay UnitWeightCost = 200_000_000 : 10 */
		));
	});

	// Relay users manipulate the soveregin account of Relay on Parachain A fail
	R::Relay::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete((Parent, Here).into()),
			fun: Fungibility::Fungible(10 * 4 * RELAY_UNIT + 10_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete((Parent, Here).into()),
					fun: Fungibility::Fungible(10 * 4),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<R::RelayRuntime>::send(
			R::RelayOrigin::signed(alice()),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// Message ignored
		assert_eq!(Tokens::<R::ParaRuntime>::free_balance(1, &bob()), 0);
	});

	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	R::Relay::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete((Parent, Here).into()),
			fun: Fungibility::Fungible(10 * 4 * RELAY_UNIT + 10_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete((Parent, Here).into()),
					fun: Fungibility::Fungible(10 * 4),
				},
				weight_limit: WeightLimit::Limited(
					(200_000_000 * 4 * RELAY_UNIT).try_into().unwrap(),
				),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]); // Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<R::RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// Relay root is similar Sibling root
		assert_eq!(Tokens::<R::ParaRuntime>::free_balance(1, &bob()), 10_000 * RELAY_UNIT);
	});

	// But as relay, Xcm without Buy execution is also fine
	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	R::Relay::execute_with(|| {
		// Mimic the Xcm message sending
		// It should fail since XcmExecutor::IsReserve setting
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete((Parent, Here).into()),
			fun: Fungibility::Fungible(20_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<R::RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// thei R::ParaRuntime asset on our chain
		assert_eq!(Tokens::<R::ParaRuntime>::free_balance(1, &bob()), 30_000 * RELAY_UNIT);
	});

	// Relay root manipulate LIT on Parachain A failed
	R::Relay::execute_with(|| {
		// Mimic the Xcm message sending, Here we even try to manipulate local parachainA token LIT
		// It should fail since XcmExecutor::IsReserve setting
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			fun: Fungibility::Fungible(2 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<R::RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	R::ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// ou R::ParaRuntime asset But extra XcmExecutor::IsReserve filter stop chain root handle
		// non-"self reserve" asset
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&bob()), 0);
	});
}

// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
pub fn test_pallet_xcm_send_capacity_relay_manipulation<R: TestXCMRequirements>()
where
	<R::RelayRuntime as frame_system::Config>::RuntimeEvent:
		From<pallet_xcm::Event<R::RelayRuntime>>,
	<<R::ParaRuntime as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<AccountId32>,
{
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		let _ = pallet_balances::Pallet::<R::ParaRuntime>::deposit_creating(
			&relay_account::<R::LocationToAccountId>(),
			10 * UNIT,
		);
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&relay_account::<R::LocationToAccountId>()),
			10 * UNIT
		);
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&bob()), 0);
		assert_ok!(ExtrinsicFilter::<R::ParaRuntime>::set_mode(
			RawOrigin::Root.into(),
			pallet_extrinsic_filter::OperationalMode::Test
		));
	});
	R::Relay::execute_with(|| {
		let call_message: R::ParaCall =
			pallet_balances::Call::transfer { dest: bob().into(), value: 2 * UNIT }.into();

		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
			/* Assets used for fee */
			fun: Fungibility::Fungible(u128::from(R::UnitWeightCost::get() * 5) + 100 * MILLICENTS),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R::ParaRuntime>(1)),
					fun: Fungibility::Fungible(
						u128::from(R::UnitWeightCost::get() * 5) + 100 * MILLICENTS,
					),
				},
				weight_limit: WeightLimit::Limited(R::UnitWeightCost::get() * 5 + 1_000_000_000),
			},
			Instruction::Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call_message.encode().into(),
			},
			Instruction::RefundSurplus,
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 {
					network: Any,
					id: relay_account::<R::LocationToAccountId>().into(),
				}
				.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<R::RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		System::<R::RelayRuntime>::assert_last_event(
			pallet_xcm::Event::Sent(Here.into(), Parachain(1).into(), xcm).into(),
		);
	});
	R::ParaA::execute_with(|| {
		// assert_eq!(
		// 	System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>(),
		// 	vec![]
		// );
		// The whole Xcm get Executed but fee paid without Transact executed ??????????
		// TODO:: Some very detials need to be checked
		// We leave it here for now. As neither do we have to consider Relay root attack Parachain
		assert_eq!(Balances::<R::ParaRuntime>::free_balance(&bob()), 2 * UNIT);
		assert_eq!(pallet_balances::Pallet::<R::RelayRuntime>::free_balance(&bob()), 0);
		let xcm_fee = u128::from(R::UnitWeightCost::get() * 5) + 100 * MILLICENTS;
		assert_eq!(
			Balances::<R::ParaRuntime>::free_balance(&relay_account::<R::LocationToAccountId>()),
			10 * UNIT - xcm_fee - 2 * UNIT
		);
		// restore normal mode?
		assert_ok!(ExtrinsicFilter::<R::ParaRuntime>::set_mode(
			RawOrigin::Root.into(),
			pallet_extrinsic_filter::OperationalMode::Normal
		));
	});
}

// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
pub fn test_pallet_xcm_send_capacity_parachain_manipulation<R: TestXCMRequirements>()
where
	<R::ParaRuntime as frame_system::Config>::RuntimeEvent: From<pallet_xcm::Event<R::ParaRuntime>>,
	<<R::RelayRuntime as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<AccountId32>,
{
	relaychain_parachains_set_up::<R>();
	R::ParaA::execute_with(|| {
		let call_message: R::RelayCall =
			pallet_balances::Call::transfer { dest: bob().into(), value: 2 * UNIT }.into();
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(Here.into()),
			fun: Fungibility::Fungible(2_000_000_000 * RELAY_UNIT), // Assets used for fee
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(Here.into()),
					fun: Fungibility::Fungible(2_000_000_000 * RELAY_UNIT),
				},
				weight_limit: WeightLimit::Limited(2_000_000_000),
			},
			Instruction::Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call_message.encode().into(),
			},
			Instruction::RefundSurplus,
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: para_account(1).into() }
					.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R::ParaRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parent.into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		System::<R::ParaRuntime>::assert_last_event(
			pallet_xcm::Event::Sent(Here.into(), Parent.into(), xcm).into(),
		);
		// assert_eq!(
		// 	System::events().pop().expect("Event expected").event,
		// 	Event::PolkadotXcm(pallet_xcm::Event::Sent(Here.into(), Parent.into(), xcm,))
		// );
	});
	R::Relay::execute_with(|| {
		// Manipulation successful
		assert_eq!(pallet_balances::Pallet::<R::RelayRuntime>::free_balance(&bob()), 2 * UNIT);
		let xcm_fee = 1_000_000_000 * RELAY_UNIT + 5 * 10 * RELAY_UNIT;
		// So Transact simply consume all "require_weight_at_most" as long as qualified for dispatch
		// weight.
		assert_eq!(
			pallet_balances::Pallet::<R::RelayRuntime>::free_balance(&para_account(1)),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE - 2 * UNIT - xcm_fee
		);
	});
}

fn register_channel_info<R: ParaRuntimeRequirements + cumulus_pallet_parachain_system::Config>(
	self_para_id: u32,
	remote_para_id: u32,
) {
	// TODO::More striaght forward method?
	// We mimic the consequence of HRMP Channel request for cumulus_pallet_parachain_system
	// set_validation_data inherent_extrinsics

	let mut sproof_builder = relay_sproof_builder::RelayStateSproofBuilder {
		para_id: ParaId::from(self_para_id),
		..Default::default()
	};
	sproof_builder.upsert_ingress_channel(ParaId::from(remote_para_id));
	sproof_builder.upsert_egress_channel(ParaId::from(remote_para_id));

	let (relay_parent_storage_root, relay_chain_state) = sproof_builder.into_state_root_and_proof();
	let n = 1;
	let vfp = PersistedValidationData {
		relay_parent_number: n as RelayChainBlockNumber,
		relay_parent_storage_root,
		..Default::default()
	};
	let system_inherent_data = ParachainInherentData {
		validation_data: vfp,
		relay_chain_state,
		downward_messages: Default::default(),
		horizontal_messages: Default::default(),
	};
	// Add HrmpChannel Info manually

	assert_ok!(ParachainSystem::<R>::set_validation_data(
		RawOrigin::None.into(),
		system_inherent_data
	));
}

pub const RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE: u128 = 100_000_000_000_000 * RELAY_UNIT;

fn relaychain_parachains_set_up<R: TestXCMRequirements>() {
	R::reset();
	R::Relay::execute_with(|| {
		let _ = Balances::<R::RelayRuntime>::deposit_creating(
			&para_account(1),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
		let _ = Balances::<R::RelayRuntime>::deposit_creating(
			&para_account(2),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
	});
	R::ParaA::execute_with(|| {
		register_channel_info::<R::ParaRuntime>(1, 2);
	});
	R::ParaB::execute_with(|| {
		register_channel_info::<R::ParaRuntime>(2, 1);
	});
	R::ParaA::execute_with(|| {
		assert_ok!(AssetManager::<R::ParaRuntime>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::<R::ParaRuntime>::ParachainReserve(Box::new(
				para_native_token_multilocation::<R::ParaRuntime>(2)
			)),
			Default::default()
		));
		assert_ok!(AssetManager::<R::ParaRuntime>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			0,
			1_000_000_000_000
		));
	});
	R::ParaB::execute_with(|| {
		assert_ok!(AssetManager::<R::ParaRuntime>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::<R::ParaRuntime>::ParachainReserve(Box::new(
				para_native_token_multilocation::<R::ParaRuntime>(1)
			)),
			Default::default()
		));
		assert_ok!(AssetManager::<R::ParaRuntime>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			0,
			1_000_000_000_000
		));
	});
}
// TODO::figure out the other OriginKind scenario

#[macro_export]
macro_rules! run_xcm_tests {
	() => {
		use runtime_common::tests::{xcm_parachain, xcm_parachain::TestXCMRequirements};

		struct XCMRequirements;

		impl TestXCMRequirements for XCMRequirements {
			type ParaOrigin = RuntimeOrigin;
			type ParaCall = RuntimeCall;
			type ParaRuntime = Runtime;
			type ParaA = ParaA;
			type ParaB = ParaB;
			type Relay = Relay;
			type RelayOrigin = RelayOrigin;
			type RelayCall = RelayCall;
			type RelayRuntime = RelayChainRuntime;
			type UnitWeightCost = UnitWeightCost;
			type LocationToAccountId = LocationToAccountId;

			fn reset() {
				TestNet::reset()
			}
		}

		#[test]
		fn test_xtokens_recognize_multilocation() {
			xcm_parachain::test_xtokens_recognize_multilocation::<XCMRequirements>();
		}

		// If this test fail, at least some part of XCM fee rule changes
		#[test]
		fn test_xtokens_weight_parameter() {
			xcm_parachain::test_xtokens_weight_parameter::<XCMRequirements>();
		}

		#[test]
		fn test_pallet_xcm_recognize_multilocation() {
			xcm_parachain::test_pallet_xcm_recognize_multilocation::<XCMRequirements>();
		}

		#[test]
		fn test_methods_xtokens_expected_succeed() {
			xcm_parachain::test_methods_xtokens_expected_succeed::<XCMRequirements>();
		}

		#[test]
		fn test_methods_xtokens_expected_fail() {
			xcm_parachain::test_methods_xtokens_expected_fail::<XCMRequirements>();
		}

		#[test]
		fn test_methods_pallet_xcm_expected_succeed() {
			xcm_parachain::test_methods_pallet_xcm_expected_succeed::<XCMRequirements>();
		}

		#[test]
		fn test_methods_pallet_xcm_expected_fail() {
			xcm_parachain::test_methods_pallet_xcm_expected_fail::<XCMRequirements>();
		}

		// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
		#[test]
		fn test_pallet_xcm_send_capacity_between_sibling() {
			xcm_parachain::test_pallet_xcm_send_capacity_between_sibling::<XCMRequirements>();
		}

		// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
		#[test]
		fn test_pallet_xcm_send_capacity_without_transact() {
			xcm_parachain::test_pallet_xcm_send_capacity_without_transact::<XCMRequirements>();
		}

		// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
		#[test]
		fn test_pallet_xcm_send_capacity_relay_manipulation() {
			xcm_parachain::test_pallet_xcm_send_capacity_relay_manipulation::<XCMRequirements>();
		}

		// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
		#[test]
		fn test_pallet_xcm_send_capacity_parachain_manipulation() {
			xcm_parachain::test_pallet_xcm_send_capacity_parachain_manipulation::<XCMRequirements>(
			);
		}
	};
}
