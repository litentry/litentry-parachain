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

use codec::Encode;
use std::marker::PhantomData;

use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::ParachainInherentData;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, PalletInfoAccess},
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

use primitives::{AccountId, AssetId, Balance};

use crate::{
	currency::{CENTS, MILLICENTS, UNIT},
	xcm_impl::CurrencyIdMultiLocationConvert,
};

use super::{
	super::xcm_impl::{CurrencyId, RuntimeConfig},
	setup::{alice, bob, relay::SovereignAccountOf, BOB, PARA_A_USER_INITIAL_BALANCE},
};

pub mod relay_sproof_builder;

pub const RELAY_UNIT: u128 = 1;

type XTokens<R> = orml_xtokens::Pallet<R>;
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

fn para_native_token_multilocation<R: RuntimeConfig>(x: u32) -> MultiLocation {
	(Parent, Parachain(x), PalletInstance(<Balances<R> as PalletInfoAccess>::index() as u8)).into()
}

pub fn test_xtokens_recognize_multilocation<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	let xcm_fees_account = pallet_treasury::Pallet::<R>::account_id();
	ParaA::execute_with(|| {
		// Wrong Multilocation does not pass
		assert_noop!(
			XTokens::<R>::transfer(
				Origin::signed(alice()),
				CurrencyId::<R>::SelfReserve(PhantomData::default()),
				1 * UNIT,
				Box::new((Parent, Parachain(2)).into()),
				UnitWeightCost::get() * 4
			),
			orml_xtokens::Error::<R>::NotSupportedMultiLocation
		);

		assert_ok!(XTokens::<R>::transfer(
			Origin::signed(alice()),
			CurrencyId::<R>::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			1 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::<R>::free_balance(1, &xcm_fees_account),
			u128::from(UnitWeightCost::get() * 4)
		);

		// Send ParaA token back to ParachainA's BOB
		assert_ok!(XTokens::<R>::transfer(
			Origin::signed(bob()),
			CurrencyId::ParachainReserve(Box::new(para_native_token_multilocation::<R>(1))),
			40 * CENTS,
			Box::new(
				(Parent, Parachain(1), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
	});

	ParaA::execute_with(|| {
		assert_eq!(
			Balances::<R>::free_balance(&bob()),
			40 * CENTS - u128::from(UnitWeightCost::get() * 4)
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			60 * CENTS /* When non-native assets transferred, the xcm fee is moved to
			            * XcmFeesAccount, which is Treasury, but native token just burned */
		);
		assert_eq!(Balances::<R>::free_balance(&xcm_fees_account), 0);
	});
}

// If this test fail, at least some part of XCM fee rule changes
pub fn test_xtokens_weight_parameter<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	let xcm_fees_account = pallet_treasury::Pallet::<R>::account_id();
	ParaA::execute_with(|| {
		// Insufficient weight still pass, but has no effect on remote chain
		assert_ok!(XTokens::<R>::transfer(
			Origin::signed(alice()),
			CurrencyId::<R>::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 1
		));
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});
	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			0
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(Tokens::<R>::free_balance(1, &xcm_fees_account), 0);
	});

	ParaA::execute_with(|| {
		// Redundant weight pass but remote the chain charges its own rule and returns the surplus
		assert_ok!(XTokens::<R>::transfer(
			Origin::signed(alice()),
			CurrencyId::<R>::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 5
		));
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 2 * UNIT);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			2 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			1 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::<R>::free_balance(1, &xcm_fees_account),
			u128::from(UnitWeightCost::get() * 4)
		);
	});
}

pub fn test_pallet_xcm_recognize_multilocation<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) where
	<R as frame_system::Config>::Event: From<pallet_xcm::Event<R>>,
{
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// It is sent but with XCM execution failed as Parachain is not exist.
		// Unregistereed Parachain Multilocation does not pass
		assert_ok!(PolkadotXcm::<R>::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(4)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R>::convert(CurrencyId::<R>::SelfReserve(
							PhantomData::default()
						))
						.unwrap(),
					),
					fun: Fungibility::Fungible(1 * UNIT),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)), 0);
		// Not XCMP_QUEUE in production environment
		// This is the error of mimic XcmRouter: decl_test_network
		System::<R>::assert_last_event(
			pallet_xcm::Event::<R>::Attempted(Outcome::Incomplete(
				UnitWeightCost::get(),
				XcmError::Unroutable,
			))
			.into(),
		);

		assert_ok!(PolkadotXcm::<R>::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R>::convert(CurrencyId::<R>::SelfReserve(
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
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 3 * UNIT);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			2 * UNIT // Only non trpped asset is in sovereign account
		);
		System::<R>::assert_last_event(
			pallet_xcm::Event::<R>::Attempted(Outcome::Complete(UnitWeightCost::get())).into(),
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			2 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
	});
	// Notice so far pallet_xcm does not handle the asset transfer back - 0.9.23
}

pub fn test_methods_xtokens_expected_succeed<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		// Solve the DustLost first
		let _ = pallet_balances::Pallet::<R>::deposit_creating(
			&sibling_account::<LocationToAccountId>(2),
			1 * UNIT,
		);

		// Sending 10 ParaA token after xcm fee to BOB by XTokens::transfer_multiasset
		assert_ok!(XTokens::<R>::transfer_multiasset(
			Origin::signed(alice()),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 1 * CENTS)
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 4) - 1 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 4) + 1 * CENTS
		);

		// Sending 100 ParaA token after xcm fee to BOB by XTokens::transfer_with_fee
		assert_ok!(XTokens::<R>::transfer_with_fee(
			Origin::signed(alice()),
			CurrencyId::<R>::SelfReserve(PhantomData::default()),
			10 * CENTS,
			(UnitWeightCost::get() * 4).into(),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 8) + 11 * CENTS
		);

		// Sending 1 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiasset_with_fee
		assert_ok!(XTokens::<R>::transfer_multiasset_with_fee(
			Origin::signed(alice()),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(1 * UNIT)
				}
				.into()
			),
			Box::new(
				MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into())
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 12) - 111 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 12) + 111 * CENTS
		);

		// Sending 10 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multicurrencies
		assert_ok!(XTokens::<R>::transfer_multicurrencies(
			Origin::signed(alice()),
			vec![(
				CurrencyId::<R>::SelfReserve(PhantomData::default()),
				u128::from(UnitWeightCost::get() * 4) + 10 * UNIT
			)],
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 16) - 1111 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 16) + 1111 * CENTS
		);

		// Sending 100 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiassets
		assert_ok!(XTokens::<R>::transfer_multiassets(
			Origin::signed(alice()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 100 * UNIT)
				}]
				.into()
			),
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 20) - 11111 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 20) + 11111 * CENTS
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11111 * CENTS
		);
	});
}

pub fn test_methods_xtokens_expected_fail<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	// Sending 1 ParaA token after xcm fee to BOB by XTokens::transfer
	ParaA::execute_with(|| {
		// Dust Lost make transaction failed
		assert_noop!(
			XTokens::<R>::transfer(
				Origin::signed(alice()),
				CurrencyId::<R>::SelfReserve(PhantomData::default()),
				u128::from(UnitWeightCost::get() * 4) + 100 * MILLICENTS,
				Box::new(
					(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
				),
				UnitWeightCost::get() * 4
			),
			orml_xtokens::Error::<R>::XcmExecutionFailed
		);
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			// This is caused by DustLost of pallet_balances
			// We keep this single weird test implementation to see if
			// omrl_xtoken changes way of handling such.
			// The issue is minor: We should fund/test real token
			// transfer with amount more than DustLost
			0
		);
	});
}

pub fn test_methods_pallet_xcm_expected_succeed<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) where
	<R as frame_system::Config>::Event: From<pallet_xcm::Event<R>>,
{
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);

	ParaA::execute_with(|| {
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_ok!(PolkadotXcm::<R>::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(
						CurrencyIdMultiLocationConvert::<R>::convert(CurrencyId::<R>::SelfReserve(
							PhantomData::default()
						))
						.unwrap(),
					),
					fun: Fungibility::Fungible(
						u128::from(UnitWeightCost::get() * 4) + 100 * MILLICENTS
					),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::<R>::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// Unlike orml_xtoken, pallet_xcm fails with event when DustLost issue happens
		System::<R>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Incomplete(
				UnitWeightCost::get(),
				XcmError::FailedToTransactAsset(""),
			))
			.into(),
		);
		// Solve the DustLost
		let _ = pallet_balances::Pallet::<R>::deposit_creating(
			&sibling_account::<LocationToAccountId>(2),
			1 * UNIT,
		);

		assert_ok!(PolkadotXcm::<R>::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 1 * CENTS)
				}]
				.into()
			),
			0
		));
		System::<R>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Complete(UnitWeightCost::get())).into(),
		);

		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 4) - 1 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 4) + 1 * CENTS
		);

		assert_ok!(PolkadotXcm::<R>::limited_reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * CENTS)
				}]
				.into()
			),
			0,
			WeightLimit::Limited(UnitWeightCost::get() * 4)
		));
		System::<R>::assert_last_event(
			pallet_xcm::Event::Attempted(Outcome::Complete(UnitWeightCost::get())).into(),
		);
		assert_eq!(
			Balances::<R>::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 8) + 11 * CENTS
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11 * CENTS
		);
	});
}

pub fn test_methods_pallet_xcm_expected_fail<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * CENTS),
		}]
		.into();
		let dest = (Parent, Parachain(2)).into();
		let xcm = Xcm(vec![
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
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
			PolkadotXcm::<R>::execute(
				Origin::signed(alice()),
				Box::new(xcm::VersionedXcm::V2(message)),
				UnitWeightCost::get() * 4
			),
			pallet_xcm::Error::<R>::Filtered
		);

		// Stopped by filter， nothing passed by execute, pallet_xcm::XcmTeleportFilter
		assert_noop!(
			PolkadotXcm::<R>::teleport_assets(
				Origin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
						fun: Fungibility::Fungible(
							u128::from(UnitWeightCost::get() * 4) + 1 * CENTS
						)
					}]
					.into()
				),
				0
			),
			pallet_xcm::Error::<R>::Filtered
		);

		assert_noop!(
			PolkadotXcm::<R>::limited_teleport_assets(
				Origin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
						fun: Fungibility::Fungible(
							u128::from(UnitWeightCost::get() * 4) + 10 * CENTS
						)
					}]
					.into()
				),
				0,
				WeightLimit::Limited(UnitWeightCost::get() * 4)
			),
			pallet_xcm::Error::<R>::Filtered
		);
	})
}

// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
pub fn test_pallet_xcm_send_capacity_between_sibling<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	// Send result Xcm of pallet_xcm::reserve_transfer_assets by user
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R>::send(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaB::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			0
		);
	});

	// Send result Xcm of pallet_xcm::reserve_transfer_assets by root
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::ReserveAssetDeposited(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R>::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaB::execute_with(|| {
		// The remote received and handled exactly same result as normal transaction
		assert_eq!(
			Tokens::<R>::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			10 * UNIT
		);
	});
	ParaA::execute_with(|| {
		// Fill up the missing assets
		let _ = pallet_balances::Pallet::<R>::deposit_creating(
			&sibling_account::<LocationToAccountId>(2),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT,
		);
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT
		);
	});

	// Users on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R>::send(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT
		);
		assert_eq!(Balances::<R>::free_balance(&bob()), 0);
	});

	// Root on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::ClearOrigin,
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::<R>::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// The remote received and handled; So we trust root power no matter.
		assert_eq!(
			Balances::<R>::free_balance(&sibling_account::<LocationToAccountId>(2)),
			3 * UNIT
		);
		assert_eq!(Balances::<R>::free_balance(&bob()), 7 * UNIT);
	});
}

// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
pub fn test_pallet_xcm_send_capacity_without_transact<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) {
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		assert_ok!(AssetManager::<R>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new((Parent, Here).into())),
			Default::default()
		));
		assert_ok!(AssetManager::<R>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			2,
			50_000 * RELAY_UNIT /*  Although does not matter here
			                     *1_000_000_000_000 / 20_000_000; Since Para UnitWeightCost :
			                     * Relay UnitWeightCost = 200_000_000 : 10 */
		));
	});

	// Relay users manipulate the soveregin account of Relay on Parachain A fail
	Relay::execute_with(|| {
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
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 4),
			},
			Instruction::DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<RelayRuntime>::send(
			RelayOrigin::signed(alice()),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// Message ignored
		assert_eq!(Tokens::<R>::free_balance(2, &bob()), 0);
	});

	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	Relay::execute_with(|| {
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
		assert_ok!(pallet_xcm::Pallet::<RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// Relay root is similar Sibling root
		assert_eq!(Tokens::<R>::free_balance(2, &bob()), 10_000 * RELAY_UNIT);
	});

	// But as relay, Xcm without Buy execution is also fine
	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	Relay::execute_with(|| {
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
		assert_ok!(pallet_xcm::Pallet::<RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// their asset on our chain
		assert_eq!(Tokens::<R>::free_balance(2, &bob()), 30_000 * RELAY_UNIT);
	});

	// Relay root manipulate LIT on Parachain A failed
	Relay::execute_with(|| {
		// Mimic the Xcm message sending, Here we even try to manipulate local parachainA token LIT
		// It should fail since XcmExecutor::IsReserve setting
		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
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
		assert_ok!(pallet_xcm::Pallet::<RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// our asset But extra XcmExecutor::IsReserve filter stop chain root handle non-"self
		// reserve" asset
		assert_eq!(Balances::<R>::free_balance(&bob()), 0);
	});
}

// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
pub fn test_pallet_xcm_send_capacity_relay_manipulation<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_balances::Call<R>> + Encode,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) where
	<RelayRuntime as frame_system::Config>::Event: From<pallet_xcm::Event<RelayRuntime>>,
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<AccountId32>,
{
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		let _ = pallet_balances::Pallet::<R>::deposit_creating(
			&relay_account::<LocationToAccountId>(),
			10 * UNIT,
		);
		assert_eq!(Balances::<R>::free_balance(&relay_account::<LocationToAccountId>()), 10 * UNIT);
		assert_eq!(Balances::<R>::free_balance(&bob()), 0);
	});
	Relay::execute_with(|| {
		let call_message: Call =
			pallet_balances::Call::transfer { dest: bob().into(), value: 2 * UNIT }.into();

		let assets = vec![MultiAsset {
			id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
			/* Assets used for fee */
			fun: Fungibility::Fungible(u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS),
		}]
		.into();
		let xcm = Xcm(vec![
			Instruction::WithdrawAsset(assets),
			Instruction::BuyExecution {
				fees: MultiAsset {
					id: XCMAssetId::Concrete(para_native_token_multilocation::<R>(1)),
					fun: Fungibility::Fungible(
						u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS,
					),
				},
				weight_limit: WeightLimit::Limited(UnitWeightCost::get() * 5 + 1_000_000_000),
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
					id: relay_account::<LocationToAccountId>().into(),
				}
				.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<RelayRuntime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		System::<RelayRuntime>::assert_last_event(
			pallet_xcm::Event::Sent(Here.into(), Parachain(1).into(), xcm).into(),
		);
	});
	ParaA::execute_with(|| {
		// assert_eq!(
		// 	System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>(),
		// 	vec![]
		// );
		// The whole Xcm get Executed but fee paid without Transact executed ??????????
		// TODO:: Some very detials need to be checked
		// We leave it here for now. As neither do we have to consider Relay root attack Parachain
		assert_eq!(Balances::<R>::free_balance(&bob()), 0);
		assert_eq!(pallet_balances::Pallet::<RelayRuntime>::free_balance(&bob()), 0);
		let xcm_fee = u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS;
		assert_eq!(
			Balances::<R>::free_balance(&relay_account::<LocationToAccountId>()),
			10 * UNIT - xcm_fee
		);
	});
}

// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
pub fn test_pallet_xcm_send_capacity_parachain_manipulation<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>
		+ pallet_treasury::Config
		+ cumulus_pallet_parachain_system::Config
		+ pallet_xcm::Config,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_xcm::Config
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	RelayCall: Clone + Dispatchable<Origin = RelayOrigin> + From<pallet_balances::Call<RelayRuntime>> + Encode,
	UnitWeightCost: frame_support::traits::Get<frame_support::weights::Weight>,
	LocationToAccountId: xcmConvert<MultiLocation, AccountId32>,
>(
	reset: Reset,
) where
	<R as frame_system::Config>::Event: From<pallet_xcm::Event<R>>,
	<<RelayRuntime as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<AccountId32>,
{
	relaychain_parachains_set_up::<Reset, Origin, R, ParaA, ParaB, Relay, RelayRuntime, RelayOrigin>(
		reset,
	);
	ParaA::execute_with(|| {
		let call_message: RelayCall =
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
		assert_ok!(PolkadotXcm::<R>::send(
			RawOrigin::Root.into(),
			Box::new(Parent.into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		System::<R>::assert_last_event(
			pallet_xcm::Event::Sent(Here.into(), Parent.into(), xcm).into(),
		);
		// assert_eq!(
		// 	System::events().pop().expect("Event expected").event,
		// 	Event::PolkadotXcm(pallet_xcm::Event::Sent(Here.into(), Parent.into(), xcm,))
		// );
	});
	Relay::execute_with(|| {
		// Manipulation successful
		assert_eq!(pallet_balances::Pallet::<RelayRuntime>::free_balance(&bob()), 2 * UNIT);
		let xcm_fee = 1_000_000_000 * RELAY_UNIT + 5 * 10 * RELAY_UNIT;
		// So Transact simply consume all "require_weight_at_most" as long as qualified for dispatch
		// weight.
		assert_eq!(
			pallet_balances::Pallet::<RelayRuntime>::free_balance(&para_account(1)),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE - 2 * UNIT - xcm_fee
		);
	});
}

fn register_channel_info<R: RuntimeConfig + cumulus_pallet_parachain_system::Config>(
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

fn relaychain_parachains_set_up<
	Reset: FnOnce(),
	Origin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
	R: RuntimeConfig
		+ cumulus_pallet_parachain_system::Config
		+ frame_system::Config<AccountId = AccountId, Origin = Origin>
		+ orml_xtokens::Config<Balance = Balance, CurrencyId = CurrencyId<R>>
		+ orml_tokens::Config<Balance = Balance, CurrencyId = AssetId>
		+ pallet_asset_manager::Config<ForeignAssetType = CurrencyId<R>>,
	ParaA: xcm_simulator::TestExt,
	ParaB: xcm_simulator::TestExt,
	Relay: xcm_simulator::TestExt,
	RelayRuntime: frame_system::Config<AccountId = AccountId, Origin = RelayOrigin>
		+ pallet_balances::Config<Balance = Balance>,
	RelayOrigin: frame_support::traits::OriginTrait<AccountId = AccountId> + From<RawOrigin<AccountId>>,
>(
	reset: Reset,
) {
	reset(); //TestNet::reset();
	Relay::execute_with(|| {
		let _ = Balances::<RelayRuntime>::deposit_creating(
			&para_account(1),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
		let _ = Balances::<RelayRuntime>::deposit_creating(
			&para_account(2),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
	});
	ParaA::execute_with(|| {
		register_channel_info::<R>(1, 2);
	});
	ParaB::execute_with(|| {
		register_channel_info::<R>(2, 1);
	});
	ParaA::execute_with(|| {
		assert_ok!(AssetManager::<R>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::<R>::ParachainReserve(Box::new(para_native_token_multilocation::<R>(2))),
			Default::default()
		));
		assert_ok!(AssetManager::<R>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			1_000_000_000_000
		));
	});
	ParaB::execute_with(|| {
		assert_ok!(AssetManager::<R>::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::<R>::ParachainReserve(Box::new(para_native_token_multilocation::<R>(1))),
			Default::default()
		));
		assert_ok!(AssetManager::<R>::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			1_000_000_000_000
		));
	});
}

// TODO::figure out the other OriginKind scenario
