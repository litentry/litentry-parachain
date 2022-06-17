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
	xcm_config::{CurrencyId, CurrencyIdMultiLocationConvert, UnitWeightCost},
	Origin,
};
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::ParachainInherentData;

use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, PalletInfoAccess},
};
use frame_system::RawOrigin;
use orml_traits::MultiCurrency;
use polkadot_parachain::primitives::{AccountIdConversion, RelayChainBlockNumber, Sibling};
use sp_runtime::{traits::Convert, AccountId32};
use xcm::prelude::*;
// use xcm::latest::prelude::*;
use xcm_simulator::TestExt;

pub mod relay_sproof_builder;

fn _para_account(x: u32) -> AccountId32 {
	ParaId::from(x).into_account()
}

fn sibling_account(x: u32) -> AccountId32 {
	use sp_runtime::traits::AccountIdConversion;
	Sibling::from(x).into_account()
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
				Origin::signed(AccountId::from(ALICE)),
				CurrencyId::SelfReserve,
				1_000_000_000_000,
				Box::new((Parent, Parachain(2)).into()),
				UnitWeightCost::get() * 4
			),
			orml_xtokens::Error::<Runtime>::NotSupportedMultiLocation
		);

		assert_ok!(XTokens::transfer(
			Origin::signed(AccountId::from(ALICE)),
			CurrencyId::SelfReserve,
			1_000_000_000_000,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(
			Balances::free_balance(&AccountId::from(ALICE)),
			500_000_000_000_000_000 - 1_000_000_000_000
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000  
			// Notice this is interesting, as it suggest local preserve XCM fee belongs to remote chain, not local chain
		);
		
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token in Para B
				&AccountId::from(BOB)
			),
			u128::from(1_000_000_000_000 - UnitWeightCost::get() * 4)
		);

		// Send ParaA token back to ParachainA's BOB
		assert_ok!(XTokens::transfer(
			Origin::signed(AccountId::from(BOB)),
			CurrencyId::ParachainReserve(Box::new(para_native_token_multilocation(1))),
			500_000_000_000,
			Box::new(
				(Parent, Parachain(1), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
	});

	ParaA::execute_with(|| {
		assert_eq!(
			Balances::free_balance(&AccountId::from(BOB)),
			u128::from(500_000_000_000 - UnitWeightCost::get() * 4)
		);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			500_000_000_000 // When assets transfer back, the xcm fee is moved to XcmFeesAccount, which is Treasury
		);
	});
}

#[test]
fn test_pallet_xcm_recognize_multilocation() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 500_000_000_000_000_000);
		// It is sent but with XCM execution failed as Parachain is not exist.
		// Unregistereed Parachain Multilocation does not pass
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(AccountId::from(ALICE)),
			Box::new((Parent, Parachain(4)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: AssetId::Concrete(
						CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve).unwrap(),
					),
					fun: Fungibility::Fungible(1_000_000_000_000),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 499_999_000_000_000_000);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			0
		);
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
			Origin::signed(AccountId::from(ALICE)),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: AssetId::Concrete(
						CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve).unwrap(),
					),
					fun: Fungibility::Fungible(2_000_000_000_000),
				}]
				.into()
			),
			0
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 499_997_000_000_000_000);
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			2_000_000_000_000 // Only non trpped asset is in sovereign account
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
				&AccountId::from(BOB)
			),
			u128::from(2_000_000_000_000 - UnitWeightCost::get() * 4)
		);
	});
	// Notice so far pallet_xcm does not handle the asset transfer back - 0.9.23
}

#[test]
fn test_methods_xtokens_expected_succeed() {
	relaychain_parachains_set_up();
	ParaA::execute_with(|| {
		// Sending 1 ParaA token after xcm fee to BOB by XTokens::transfer
		assert_ok!(XTokens::transfer(
			Origin::signed(AccountId::from(ALICE)),
			CurrencyId::SelfReserve,
			(UnitWeightCost::get() * 4 + 1).into(),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 4 - 1));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			// u128::from(UnitWeightCost::get() * 4 + 1)
			// This is caused by DustLost of pallet_balances
			// We keep this single weird test implementation to see if there will be a fix
			// The issue is minor: We should fund/test real token transfer with amount more than DustLost
			0
		);

		// Solve the DustLost
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(
			&sibling_account(2),
			1_000_000_000_000,
		);

		// Sending 10 ParaA token after xcm fee to BOB by XTokens::transfer_multiasset
		assert_ok!(XTokens::transfer_multiasset(
			Origin::signed(AccountId::from(ALICE)),
			Box::new(
				MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible((UnitWeightCost::get() * 4 + 10).into())
				}
				.into()
			),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 8 - 11));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000 + u128::from(UnitWeightCost::get() * 4 + 10)
		);

		// Sending 100 ParaA token after xcm fee to BOB by XTokens::transfer_with_fee
		assert_ok!(XTokens::transfer_with_fee(
			Origin::signed(AccountId::from(ALICE)),
			CurrencyId::SelfReserve,
			100,
			(UnitWeightCost::get() * 4).into(),
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 12 - 111));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000 + u128::from(UnitWeightCost::get() * 8 + 110)
		);

		// Sending 1_000 ParaA token after xcm fee to BOB by XTokens::transfer_multiasset_with_fee
		assert_ok!(XTokens::transfer_multiasset_with_fee(
			Origin::signed(AccountId::from(ALICE)),
			Box::new(
				MultiAsset {
					id: Concrete(para_native_token_multilocation(1)),
					fun: Fungible(1000)
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
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 16 - 1_111));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000 + u128::from(UnitWeightCost::get() * 12 + 1_110)
		);

		// Sending 10_000 ParaA token after xcm fee to BOB by XTokens::transfer_multicurrencies
		assert_ok!(XTokens::transfer_multicurrencies(
			Origin::signed(AccountId::from(ALICE)),
			vec![(CurrencyId::SelfReserve, (UnitWeightCost::get() * 4 + 10_000).into())],
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 20 - 11_111));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000 + u128::from(UnitWeightCost::get() * 16 + 11_110)
		);

		// Sending 100_000 ParaA token after xcm fee to BOB by XTokens::transfer_multiassets
		assert_ok!(XTokens::transfer_multiassets(
			Origin::signed(AccountId::from(ALICE)),
			Box::new(
				vec![MultiAsset {
					id: Concrete(
						para_native_token_multilocation(1)
					),
					fun: Fungible((UnitWeightCost::get() * 4 + 100_000).into())
				}]
				.into()
			),
			0,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB }).into()
			),
			UnitWeightCost::get() * 4
		));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), u128::from(500_000_000_000_000_000 - UnitWeightCost::get() * 24 - 111_111));
		assert_eq!(
			Balances::free_balance(&sibling_account(2)),
			1_000_000_000_000 + u128::from(UnitWeightCost::get() * 20 + 111_110)
		);
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&AccountId::from(BOB)
			),
			111_111 // Xtoken: The DustLost does not effect the minting on remote chain
		);
	});
}

#[test]
fn test_methods_xtokens_expected_fail() {
	relaychain_parachains_set_up();
	//TODOTODOTODOTODOTODOTODO
}

#[test]
fn test_methods_pallet_xcm_expected_succeed() {
	relaychain_parachains_set_up();

	ParaA::execute_with(|| {
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 500_000_000_000_000_000);
		//-------------
		// assert_ok!(PolkadotXcm::reserve_transfer_assets(
		// 	Origin::signed(AccountId::from(ALICE)),
		// 	Box::new((Parent, Parachain(2)).into()),
		// 	Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
		// 	Box::new(
		// 		vec![MultiAsset {
		// 			id: AssetId::Concrete(
		// 				CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve).unwrap(),
		// 			),
		// 			fun: Fungibility::Fungible((UnitWeightCost::get() * 4 + 10_000_000_000).into()),
		// 		}]
		// 		.into()
		// 	),
		// 	0
		// ));
		// assert_eq!(
		// 	last_event(),
		// 	Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(
		// 		UnitWeightCost::get()
		// 	)))
		// );
		//-------------
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			Origin::signed(AccountId::from(ALICE)),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: Concrete(
						para_native_token_multilocation(1)
					),
					fun: Fungible(100_000_000_000)
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

		assert_ok!(PolkadotXcm::limited_reserve_transfer_assets(
			Origin::signed(AccountId::from(ALICE)),
			Box::new((Parent, Parachain(2)).into()),
			Box::new((Junction::AccountId32 { network: Any, id: BOB }).into().into()),
			Box::new(
				vec![MultiAsset {
					id: Concrete(
						para_native_token_multilocation(1)
					),
					fun: Fungible((UnitWeightCost::get() * 4 + 10).into())
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
	});

	ParaB::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(
				1, // Asset_id=1. The first registered Token: ParaA Token in Para B
				&AccountId::from(BOB)
			),
			11
		);
	});
}

#[test]
fn test_methods_pallet_xcm_expected_fail() {
	relaychain_parachains_set_up();
	//TODOTODOTODOTODOTODOTODO
	ParaA::execute_with(|| {
		// Mimic the Xcm message sending
		let assets = vec![MultiAsset {
			id: Concrete(
				para_native_token_multilocation(1)
			),
			fun: Fungible((UnitWeightCost::get() * 4 + 100).into()),
		}]
		.into();
		let dest = (Parent, Parachain(2)).into();
		let xcm = Xcm(vec![
			BuyExecution {
				fees: MultiAsset {
					id: Concrete(
						para_native_token_multilocation(1)
					),
					fun: Fungible((UnitWeightCost::get() * 4).into()),
				}
				.into(),
				weight_limit: Limited(UnitWeightCost::get() * 4),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Junction::AccountId32 { network: Any, id: BOB }.into(),
			},
		]);
		let message = Xcm(vec![TransferReserveAsset { assets, dest, xcm }]);
		// Stopped by filter
		assert_noop!(
			PolkadotXcm::execute(
				Origin::signed(AccountId::from(ALICE)),
				Box::new(xcm::VersionedXcm::V2(message)),
				UnitWeightCost::get() * 4
			),
			pallet_xcm::Error::<Runtime>::Filtered
		);


	})
}

#[test]
fn test_pallet_xcm_send_capacity() {
	relaychain_parachains_set_up();
	//TODOTODOTODOTODOTODOTODO
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

fn relaychain_parachains_set_up() {
	TestNet::reset();
	Relay::execute_with(|| {
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&sibling_account(1),
			1_000_000_000_000,
		);
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&sibling_account(2),
			1_000_000_000_000,
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
			CurrencyId::ParachainReserve(Box::new(
				para_native_token_multilocation(2)
			)),
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
			CurrencyId::ParachainReserve(Box::new(
				para_native_token_multilocation(1)
			)),
			Default::default()
		));
		assert_ok!(AssetManager::set_asset_units_per_second(
			RawOrigin::Root.into(),
			1,
			1_000_000_000_000
		));
	});
}