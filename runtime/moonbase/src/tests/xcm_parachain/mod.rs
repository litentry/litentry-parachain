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
use crate::{
    xcm_config::{LocationToAccountId, UnitWeightCost, XcmFeesAccount},
    Origin,
};
use codec::Encode;
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::ParachainInherentData;
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency, PalletInfoAccess},
};
use frame_system::RawOrigin;
use orml_traits::MultiCurrency;
use polkadot_parachain::primitives::RelayChainBlockNumber;
use runtime_common::{
    currency::*,
    xcm_impl::{CurrencyId as CommonCurrencyId, CurrencyIdMultiLocationConvert},
};
use sp_runtime::{traits::Convert, AccountId32};
use std::marker::PhantomData;
use xcm::prelude::*;
use xcm_executor::traits::Convert as xcmConvert;
use xcm_simulator::TestExt;

pub mod relay_sproof_builder;

pub const RELAY_UNIT: u128 = 1;

type CurrencyId = CommonCurrencyId<Runtime>;

fn para_account(x: u32) -> AccountId32 {
	<relay::SovereignAccountOf as xcmConvert<MultiLocation, AccountId32>>::convert(
		Parachain(x).into(),
	)
	.unwrap()
}

fn sibling_account(x: u32) -> AccountId32 {
	<LocationToAccountId as xcmConvert<MultiLocation, AccountId32>>::convert(
		(Parent, Parachain(x)).into(),
	)
	.unwrap()
}

fn relay_account() -> AccountId32 {
	<LocationToAccountId as xcmConvert<MultiLocation, AccountId32>>::convert(Parent.into()).unwrap()
}

fn para_native_token_multilocation(x: u32) -> MultiLocation {
	(Parent, Parachain(x), PalletInstance(<Balances as PalletInfoAccess>::index() as u8)).into()
}

#[test]
fn test_xtokens_recognize_multilocation() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		// Wrong Multilocation does not pass
		assert_noop!(
			XTokens::transfer(
				Origin::signed(alice()),
				CurrencyId::SelfReserve(PhantomData::default()),
				1 * UNIT,
				Box::new((Parent, Parachain(2)).into()),
				UnitWeightCost::get() * 4
			),
			orml_xtokens::Error::<Runtime>::NotSupportedMultiLocation
		);

		assert_ok!(XTokens::transfer(
			Origin::signed(alice()),
			CurrencyId::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			1 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::free_balance(1, &XcmFeesAccount::get()),
			u128::from(UnitWeightCost::get() * 4)
		);

		// Send ParaA token back to ParachainA's BOB
		assert_ok!(XTokens::transfer(
			Origin::signed(bob()),
			CurrencyId::ParachainReserve(Box::new(para_native_token_multilocation(1))),
			40 * CENTS,
			Box::new(
				(Parent, Parachain(1), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
	});

	ParaA::execute_with(|| {
		assert_eq!(
			Balances::free_balance(&bob()),
			40 * CENTS - u128::from(UnitWeightCost::get() * 4)
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			60 * CENTS /* When non-native assets transferred, the xcm fee is moved to
			            * XcmFeesAccount, which is Treasury, but native token just burned */
		);
		assert_eq!(Balances::free_balance(&XcmFeesAccount::get()), 0);
	});
}

// If this test fail, at least some part of XCM fee rule changes
#[test]
fn test_xtokens_weight_parameter() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		// Insufficient weight still pass, but has no effect on remote chain
		assert_ok!(XTokens::transfer(
			Origin::signed(alice()),
			CurrencyId::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 1
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});
	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			0
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(Tokens::free_balance(1, &XcmFeesAccount::get()), 0);
	});

	ParaA::execute_with(|| {
		// Redundant weight pass but remote the chain charges its own rule and returns the surplus
		assert_ok!(XTokens::transfer(
			Origin::signed(alice()),
			CurrencyId::SelfReserve(PhantomData::default()),
			1 * UNIT,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 5
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 2 * UNIT);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			2 * UNIT /* Notice this is interesting, as it suggest local preserve XCM
			          * fee belongs to remote chain, not local chain */
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			1 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
		// Check the treasury of remote chain's asset XCM
		assert_eq!(
			Tokens::free_balance(1, &XcmFeesAccount::get()),
			u128::from(UnitWeightCost::get() * 4)
		);
	});
}

#[test]
fn test_pallet_xcm_recognize_multilocation() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// It is sent but with XCM execution failed as Parachain is not exist.
		// Unregistereed Parachain Multilocation does not pass
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(4)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: AssetId::Concrete(
						CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve(PhantomData::default())).unwrap(),
					),
					fun: Fungibility::Fungible(1 * UNIT),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 1 * UNIT);
		assert_eq!(Balances::free_balance(&sibling_account(2)), 0);
		assert_eq!(
			last_event(),
			// Not XCMP_QUEUE in production environment
			// This is the error of mimic XcmRouter: decl_test_network
			Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Incomplete(
				UnitWeightCost::get(),
				XcmError::Unroutable
			)))
		);
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: AssetId::Concrete(
						CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve(PhantomData::default())).unwrap(),
					),
					fun: Fungibility::Fungible(2 * UNIT),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE - 3 * UNIT);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			2 * UNIT // Only non trpped asset is in sovereign account
		);
		assert_eq!(
			last_event(),
			Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(
				UnitWeightCost::get()
			)))
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			2 * UNIT - u128::from(UnitWeightCost::get() * 4)
		);
	});
	// Notice so far pallet_xcm does not handle the asset transfer back - 0.9.23
}

#[test]
fn test_methods_xtokens_expected_succeed() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		// Solve the DustLost first
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&sibling_account(2), 1 * UNIT);

		// Sending 10 ParaA token after xcm fee to BOB by XTokens::transfer_multiasset
		assert_ok!(XTokens::transfer_multiasset(
			Origin::signed(alice()),
			Box::new(
				MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 1 * CENTS)
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 4) - 1 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 4) + 1 * CENTS
		);

		// Sending 100 ParaA token after xcm fee to BOB by XTokens::transfer_with_fee
		assert_ok!(XTokens::transfer_with_fee(
			Origin::signed(alice()),
			CurrencyId::SelfReserve(PhantomData::default()),
			10 * CENTS,
			(UnitWeightCost::get() * 4).into(),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 8) + 11 * CENTS
		);

		// Sending 1 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiasset_with_fee
		assert_ok!(XTokens::transfer_multiasset_with_fee(
			Origin::signed(alice()),
			Box::new(
				MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(1 * UNIT)
				}
				.into()
			),
			Box::new(
				MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into())
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 12) - 111 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 12) + 111 * CENTS
		);

		// Sending 10 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multicurrencies
		assert_ok!(XTokens::transfer_multicurrencies(
			Origin::signed(alice()),
			vec![(CurrencyId::SelfReserve(PhantomData::default()), u128::from(UnitWeightCost::get() * 4) + 10 * UNIT)],
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 16) - 1111 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 16) + 1111 * CENTS
		);

		// Sending 100 UNIT ParaA token after xcm fee to BOB by XTokens::transfer_multiassets
		assert_ok!(XTokens::transfer_multiassets(
			Origin::signed(alice()),
			Box::new(
				vec![MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 100 * UNIT)
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
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 20) - 11111 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 20) + 11111 * CENTS
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11111 * CENTS
		);
	});
}

#[test]
fn test_methods_xtokens_expected_fail() {
	relaychain_parachains_set_up();
	// Sending 1 ParaA token after xcm fee to BOB by XTokens::transfer
	ParaA::execute_with(|| {
		// Dust Lost make transaction failed
		assert_noop!(
			XTokens::transfer(
				Origin::signed(alice()),
				CurrencyId::SelfReserve(PhantomData::default()),
				u128::from(UnitWeightCost::get() * 4) + 100 * MILLICENTS,
				Box::new(
					(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
				),
				UnitWeightCost::get() * 4
			),
			orml_xtokens::Error::<Runtime>::XcmExecutionFailed
		);
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			// This is caused by DustLost of pallet_balances
			// We keep this single weird test implementation to see if
			// omrl_xtoken changes way of handling such.
			// The issue is minor: We should fund/test real token
			// transfer with amount more than DustLost
			0
		);
	});
}

#[test]
fn test_methods_pallet_xcm_expected_succeed() {
	relaychain_parachains_set_up();

	ParaA::execute_with(|| {
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: AssetId::Concrete(
						CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve(PhantomData::default())).unwrap(),
					),
					fun: Fungibility::Fungible(
						u128::from(UnitWeightCost::get() * 4) + 100 * MILLICENTS
					),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::free_balance(&alice()), PARA_A_USER_INITIAL_BALANCE);
		// Unlike orml_xtoken, pallet_xcm fails with event when DustLost issue happens
		assert_eq!(
			last_event(),
			Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Incomplete(
				UnitWeightCost::get(),
				XcmError::FailedToTransactAsset("")
			)))
		);
		// Solve the DustLost
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&sibling_account(2), 1 * UNIT);

		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 1 * CENTS)
				}]
				.into()
			),
			0
		));
		assert_eq!(
			last_event(),
			Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(
				UnitWeightCost::get()
			)))
		);
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 4) - 1 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 4) + 1 * CENTS
		);

		assert_ok!(PolkadotXcm::limited_reserve_transfer_assets(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * CENTS)
				}]
				.into()
			),
			0,
			WeightLimit::Limited(UnitWeightCost::get() * 4)
		));
		assert_eq!(
			last_event(),
			Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(
				UnitWeightCost::get()
			)))
		);
		assert_eq!(
			Balances::free_balance(&alice()),
			PARA_A_USER_INITIAL_BALANCE - u128::from(UnitWeightCost::get() * 8) - 11 * CENTS
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1 * UNIT + u128::from(UnitWeightCost::get() * 8) + 11 * CENTS
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&bob()
			),
			11 * CENTS
		);
	});
}

#[test]
fn test_methods_pallet_xcm_expected_fail() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * CENTS),
		}]
		.into();
		let dest = (Parent, Parachain(2)).into();
		let xcm = Xcm(vec![
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		let message = Xcm(vec![TransferReserveAsset { assets, dest, xcm }]);
		// Stopped by filter， nothing passed by execute, pallet_xcm::XcmExecuteFilter
		// If there is no pallet_xcm filter protection, then we should test XcmExexutor::Barrier
		// setting here in future
		assert_noop!(
			PolkadotXcm::execute(
				Origin::signed(alice()),
				Box::new(xcm::VersionedXcm::V2(message)),
				UnitWeightCost::get() * 4
			),
			pallet_xcm::Error::<Runtime>::Filtered
		);

		// Stopped by filter， nothing passed by execute, pallet_xcm::XcmTeleportFilter
		assert_noop!(
			PolkadotXcm::teleport_assets(
				Origin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: Concrete(para_native_token_multilocation(1)),
						fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 1 * CENTS)
					}]
					.into()
				),
				0
			),
			pallet_xcm::Error::<Runtime>::Filtered
		);

		assert_noop!(
			PolkadotXcm::limited_teleport_assets(
				Origin::signed(alice()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: Concrete(para_native_token_multilocation(1)),
						fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * CENTS)
					}]
					.into()
				),
				0,
				WeightLimit::Limited(UnitWeightCost::get() * 4)
			),
			pallet_xcm::Error::<Runtime>::Filtered
		);
	})
}

// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_between_sibling() {
	relaychain_parachains_set_up();
	// Send result Xcm of pallet_xcm::reserve_transfer_assets by user
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::send(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaB::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Tokens::free_balance(
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
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 10 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(2)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaB::execute_with(|| {
		// The remote received and handled exactly same result as normal transaction
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&bob()
			),
			10 * UNIT
		);
	});
	ParaA::execute_with(|| {
		// Fill up the missing assets
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(
			&sibling_account(2),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT,
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT
		);
	});

	// Users on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			WithdrawAsset(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::send(
			Origin::signed(alice()),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// The remote received and ignored
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			u128::from(UnitWeightCost::get() * 4) + 10 * UNIT
		);
		assert_eq!(Balances::free_balance(&bob()), 0);
	});

	// Root on Parachain B want to manipulate the soveregin account of Parachain B on Parachain A
	ParaB::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 4) + 7 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			WithdrawAsset(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				},
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::send(
			RawOrigin::Root.into(),
			Box::new((Parent, Parachain(1)).into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// The remote received and handled; So we trust root power no matter.
		assert_eq!(Balances::free_balance(&sibling_account(2)), 3 * UNIT);
		assert_eq!(Balances::free_balance(&bob()), 7 * UNIT);
	});
}

// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_without_transact() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		assert_ok!(AssetManager::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new((Parent, Here).into())),
			Default::default()
		));
		assert_ok!(AssetManager::set_asset_units_per_second(
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
			id: Concrete((Parent, Here).into()),
			fun: Fungible(10 * 4 * RELAY_UNIT + 10_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset { id: Concrete((Parent, Here).into()), fun: Fungible(10 * 4) },
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// User sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<relay::Runtime>::send(
			relay::Origin::signed(alice()),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// Message ignored
		assert_eq!(Tokens::free_balance(2, &bob()), 0);
	});

	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	Relay::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete((Parent, Here).into()),
			fun: Fungible(10 * 4 * RELAY_UNIT + 10_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			ClearOrigin,
			BuyExecution {
				fees: MultiAsset { id: Concrete((Parent, Here).into()), fun: Fungible(10 * 4) },
				weight_limit: Limited((200_000_000 * 4 * RELAY_UNIT).try_into().unwrap()),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<relay::Runtime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// Relay root is similar Sibling root
		assert_eq!(Tokens::free_balance(2, &bob()), 10_000 * RELAY_UNIT);
	});

	// But as relay, Xcm without Buy execution is also fine
	// Relay root manipulate the soveregin account of Relay on Parachain A succeed
	Relay::execute_with(|| {
		// Mimic the Xcm message sending
		// It should fail since XcmExecutor::IsReserve setting
		let assets = vec![MultiAsset {
			id: Concrete((Parent, Here).into()),
			fun: Fungible(20_000 * RELAY_UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<relay::Runtime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// their asset on our chain
		assert_eq!(Tokens::free_balance(2, &bob()), 30_000 * RELAY_UNIT);
	});

	// Relay root manipulate LIT on Parachain A failed
	Relay::execute_with(|| {
		// Mimic the Xcm message sending, Here we even try to manipulate local parachainA token LIT
		// It should fail since XcmExecutor::IsReserve setting
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(2 * UNIT),
		}]
		.into();
		let xcm = Xcm(vec![
			ReserveAssetDeposited(assets),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<relay::Runtime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm)),
		));
	});
	ParaA::execute_with(|| {
		// We trust Relay root with even more power than Sibling root. They can easily manipulate
		// our asset But extra XcmExecutor::IsReserve filter stop chain root handle non-"self
		// reserve" asset
		assert_eq!(Balances::free_balance(&bob()), 0);
	});
}

// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
#[test]
fn test_pallet_xcm_send_capacity_relay_manipulation() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&relay_account(), 10 * UNIT);
		assert_eq!(Balances::free_balance(&relay_account()), 10 * UNIT);
		assert_eq!(Balances::free_balance(&bob()), 0);
	});
	Relay::execute_with(|| {
		let call_message =
			Call::Balances(pallet_balances::Call::transfer { dest: bob().into(), value: 2 * UNIT })
				.encode()
				.into();
		let assets = vec![MultiAsset {
			id: Concrete(para_native_token_multilocation(1)),
			fun: Fungible(u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS), /* Assets used for
			                                                                          * fee */
		}]
		.into();
		let xcm = Xcm(vec![
			WithdrawAsset(assets),
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS),
				},
				weight_limit: Limited(UnitWeightCost::get() * 5 + 1_000_000_000),
			},
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call_message,
			},
			RefundSurplus,
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: relay_account().into() }
					.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(pallet_xcm::Pallet::<relay::Runtime>::send(
			RawOrigin::Root.into(),
			Box::new(Parachain(1).into().into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		assert_eq!(
			relay::System::events().pop().expect("Event expected").event,
			relay::Event::XcmPallet(
				pallet_xcm::Event::Sent(Here.into(), Parachain(1).into(), xcm,)
			)
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
		assert_eq!(Balances::free_balance(&bob()), 0);
		assert_eq!(pallet_balances::Pallet::<relay::Runtime>::free_balance(&bob()), 0);
		let xcm_fee = u128::from(UnitWeightCost::get() * 5) + 100 * MILLICENTS;
		assert_eq!(Balances::free_balance(&relay_account()), 10 * UNIT - xcm_fee);
	});
}

// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
#[test]
fn test_pallet_xcm_send_capacity_parachain_manipulation() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		let call_message =
			relay::Call::Balances(pallet_balances::Call::transfer { dest: bob(), value: 2 * UNIT })
				.encode()
				.into();
		let assets = vec![MultiAsset {
			id: Concrete(Here.into()),
			fun: Fungible(2_000_000_000 * RELAY_UNIT), // Assets used for fee
		}]
		.into();
		let xcm = Xcm(vec![
			WithdrawAsset(assets),
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(Here.into()),
					fun: Fungible(2_000_000_000 * RELAY_UNIT),
				},
				weight_limit: Limited(2_000_000_000),
			},
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 1_000_000_000,
				call: call_message,
			},
			RefundSurplus,
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: para_account(1).into() }
					.into(),
			},
		]);
		// Root sending the raw Xcm works successfully
		assert_ok!(PolkadotXcm::send(
			RawOrigin::Root.into(),
			Box::new(Parent.into()),
			Box::new(xcm::VersionedXcm::V2(xcm.clone())),
		));
		assert_eq!(
			System::events().pop().expect("Event expected").event,
			Event::PolkadotXcm(pallet_xcm::Event::Sent(Here.into(), Parent.into(), xcm,))
		);
	});
	Relay::execute_with(|| {
		// Manipulation successful
		assert_eq!(pallet_balances::Pallet::<relay::Runtime>::free_balance(&bob()), 2 * UNIT);
		let xcm_fee = 1_000_000_000 * RELAY_UNIT + 5 * 10 * RELAY_UNIT;
		// So Transact simply consume all "require_weight_at_most" as long as qualified for dispatch
		// weight.
		assert_eq!(
			pallet_balances::Pallet::<relay::Runtime>::free_balance(&para_account(1)),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE - 2 * UNIT - xcm_fee
		);
	});
}

fn register_channel_info(self_para_id: u32, remote_para_id: u32) {
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

	assert_ok!(ParachainSystem::set_validation_data(RawOrigin::None.into(), system_inherent_data));
}

pub const RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE: u128 = 100_000_000_000_000 * RELAY_UNIT;
fn relaychain_parachains_set_up() {
	TestNet::reset();
	Relay::execute_with(|| {
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&para_account(1),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&para_account(2),
			RELAY_SOVEREIGN_ACCOUNT_INITIAL_BALANCE,
		);
	});
	ParaA::execute_with(|| {
		register_channel_info(1, 2);
	});
	ParaB::execute_with(|| {
		register_channel_info(2, 1);
	});
	ParaA::execute_with(|| {
		assert_ok!(AssetManager::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new(para_native_token_multilocation(2))),
			Default::default()
		));
		assert_ok!(AssetManager::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			1_000_000_000_000
		));
	});
	ParaB::execute_with(|| {
		assert_ok!(AssetManager::register_foreign_asset_type(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new(para_native_token_multilocation(1))),
			Default::default()
		));
		assert_ok!(AssetManager::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			1_000_000_000_000
		));
	});
}

// TODO::figure out the other OriginKind scenario
