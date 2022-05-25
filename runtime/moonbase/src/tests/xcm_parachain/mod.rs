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
	xcm_config::{CurrencyId, CurrencyIdMultiLocationConvert},
	Origin,
};
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use cumulus_primitives_parachain_inherent::ParachainInherentData;

use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, PalletInfoAccess},
};
use frame_system::RawOrigin;
use polkadot_parachain::primitives::{AccountIdConversion, RelayChainBlockNumber, Sibling};
use sp_runtime::{traits::Convert, AccountId32};
use xcm::prelude::*;
// use xcm::latest::prelude::*;
use xcm_simulator::TestExt;

pub mod relay_sproof_builder;

fn para_a_account() -> AccountId32 {
	ParaId::from(1).into_account()
}

fn para_b_account() -> AccountId32 {
	ParaId::from(2).into_account()
}

fn _sibling_a_account() -> AccountId32 {
	use sp_runtime::traits::AccountIdConversion;
	Sibling::from(1).into_account()
}

fn _sibling_b_account() -> AccountId32 {
	use sp_runtime::traits::AccountIdConversion;
	Sibling::from(2).into_account()
}

#[test]
fn test_xtokens_recognize_multilocation() {
	TestNet::reset();

	Relay::execute_with(|| {
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&para_a_account(),
			1_000_000_000_000,
		);
		let _ = pallet_balances::Pallet::<relay::Runtime>::deposit_creating(
			&para_b_account(),
			1_000_000_000_000,
		);
	});
	ParaA::execute_with(|| {
		register_channel_info(1, 2);
	});
	ParaB::execute_with(|| {
		register_channel_info(2, 1);
	});

	ParaB::execute_with(|| {
		assert_ok!(AssetManager::register_foreign_asset(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new(
				(
					Parent,
					Parachain(1),
					PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
				)
					.into()
			)),
			Default::default()
		));
		assert_ok!(AssetManager::set_asset_units_per_second(
			RawOrigin::Root.into(),
			CurrencyId::ParachainReserve(Box::new(
				(
					Parent,
					Parachain(1),
					PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
				)
					.into()
			)),
			1_000_000_000_000,
			1
		));
	});

	ParaA::execute_with(|| {
		// Wrong Multilocation does not pass
		assert_noop!(
			XTokens::transfer(
				Origin::signed(AccountId::from(ALICE).into()),
				CurrencyId::SelfReserve,
				1_000_000_000_000,
				Box::new((Parent, Parachain(2)).into()),
				800_000_000
			),
			orml_xtokens::Error::<Runtime>::NotSupportedMultiLocation
		);

		assert_ok!(XTokens::transfer(
			Origin::signed(AccountId::from(ALICE).into()),
			CurrencyId::SelfReserve,
			1_000_000_000_000,
			Box::new(
				(Parent, Parachain(2), Junction::AccountId32 { network: Any, id: BOB.into() })
					.into()
			),
			800_000_000
		));
	});

	// TODO::This mimic does not finialized yet. Still need mimic XcmRouter
	// ParaB::execute_with(|| {
	// 	XcmpQueue::on_idle(0, u64::MAX);
	// 	assert_eq!(Tokens::free_balance(0, &AccountId::from(BOB)), 999_200_000_000);
	// })
}

#[test]
fn test_pallet_xcm_recognize_multilocation() {
	ExtBuilder::default()
		.balances(vec![
			// fund Alice and BOB
			(AccountId::from(ALICE), 500_000_000_000_000_000),
			(AccountId::from(BOB), 500_000_000_000_000_000),
		])
		.build()
		.execute_with(|| {
			register_channel_info(1, 2);

			assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 500_000_000_000_000_000);
			// It is sent but with XCM execution failed as Parachain is not exist.
			assert_ok!(PolkadotXcm::reserve_transfer_assets(
				Origin::signed(AccountId::from(ALICE).into()),
				Box::new((Parent, Parachain(2)).into()),
				Box::new((Junction::AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new(
					vec![MultiAsset {
						id: AssetId::Concrete(
							CurrencyIdMultiLocationConvert::convert(CurrencyId::SelfReserve)
								.unwrap(),
						),
						fun: Fungibility::Fungible(1_000_000_000_000),
					}]
					.into()
				),
				0
			));
			assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 499_999_000_000_000_000);
			assert_eq!(
				last_event(),
				Event::PolkadotXcm(pallet_xcm::Event::Attempted(Outcome::Complete(200_000_000)))
			);
		})
}

#[test]
fn test_xcm_executor_recognize_multilocation() {
	ExtBuilder::default()
		.balances(vec![
			// fund Alice and BOB
			(AccountId::from(ALICE), 123456789123456789),
			(AccountId::from(BOB), 123456789123456789),
		])
		.build()
		.execute_with(|| {
			// CumulusXcm
		})
}

#[test]
fn test_xcm_executor_recognize_false_multilocation() {
	ExtBuilder::default()
		.balances(vec![
			// fund Alice and BOB
			(AccountId::from(ALICE), 123456789123456789),
			(AccountId::from(BOB), 123456789123456789),
		])
		.build()
		.execute_with(|| {})
}

fn register_channel_info(self_para_id: u32, remote_para_id: u32) {
	// TODO::More striaght forward method?
	// We mimic the consequence of HRMP Channel request for cumulus_pallet_parachain_system
	// set_validation_data inherent_extrinsics

	let mut sproof_builder = relay_sproof_builder::RelayStateSproofBuilder::default();
	sproof_builder.para_id = ParaId::from(self_para_id);
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
		validation_data: vfp.clone(),
		relay_chain_state,
		downward_messages: Default::default(),
		horizontal_messages: Default::default(),
	};
	// Add HrmpChannel Info manually

	assert_ok!(ParachainSystem::set_validation_data(RawOrigin::None.into(), system_inherent_data));
}
