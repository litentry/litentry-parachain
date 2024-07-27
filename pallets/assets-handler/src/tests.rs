// Copyright 2020-2024 Trust Computing GmbH.
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

use super::{
	mock::{
		assert_events, new_test_ext, new_test_ext_initialized, Balances, Bridge, BridgeTransfer,
		NativeTokenResourceId, ProposalLifetime, RuntimeCall, RuntimeEvent, RuntimeOrigin, Test,
		TreasuryAccount, ENDOWED_BALANCE, MAXIMUM_ISSURANCE, RELAYER_A, RELAYER_B, RELAYER_C,
	},
	ExternalBalances, MaximumIssuance as MaximumIssuanceStorage, *,
};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use sp_runtime::ArithmeticError;

fn make_transfer_proposal(to: u64, amount: u64) -> RuntimeCall {
	let rid = NativeTokenResourceId::get();
	// let amount
	RuntimeCall::BridgeTransfer(pallet_bridge_transfer::Call::transfer { to, amount, rid })
}

#[test]
fn constant_equality() {
	let r_id = pallet_bridge::derive_resource_id(1, &pallet_bridge::hashing::blake2_128(b"LIT"));
	let encoded: [u8; 32] =
		hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
	assert_eq!(r_id, encoded);
}

#[test]
fn transfer() {
	let dest_bridge_id: pallet_bridge::BridgeChainId = 0;
	let resource_id = NativeTokenResourceId::get();
	let native_token_asset_info: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	> = AssetInfo { fee: 0u64, asset: None };

	new_test_ext_initialized(dest_bridge_id, resource_id, native_token_asset_info).execute_with(
		|| {
			// Transfer and check result
			assert_ok!(BridgeTransfer::transfer(
				RuntimeOrigin::signed(Bridge::account_id()),
				RELAYER_A,
				10,
				resource_id,
			));
			assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

			assert_events(vec![
				RuntimeEvent::AssetsHandler(Event::TokenBridgeIn {
					asset_id: None,
					to: RELAYER_A,
					amount: 10,
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Minted {
					who: RELAYER_A,
					amount: 10,
				}),
			]);
		},
	)
}

#[test]
fn transfer_assets() {
	let dest_bridge_id: pallet_bridge::BridgeChainId = 0;
	let resource_id = NativeTokenResourceId::get();
	let native_token_asset_info: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	> = AssetInfo { fee: 10u64, asset: None };

	new_test_ext_initialized(dest_bridge_id, resource_id, native_token_asset_info).execute_with(
		|| {
			let dest_account: Vec<u8> = vec![1];
			assert_ok!(pallet_bridge_transfer::Pallet::<Test>::transfer_assets(
				RuntimeOrigin::signed(RELAYER_A),
				100,
				dest_account.clone(),
				dest_bridge_id,
				resource_id
			));
			assert_eq!(
				pallet_balances::Pallet::<Test>::free_balance(TreasuryAccount::get()),
				ENDOWED_BALANCE + 10
			);
			assert_eq!(
				pallet_balances::Pallet::<Test>::free_balance(RELAYER_A),
				ENDOWED_BALANCE - 100
			);
			assert_events(vec![
				RuntimeEvent::AssetsHandler(Event::TokenBridgeOut {
					asset_id: None,
					to: RELAYER_A,
					amount: 100,
					fee: 10,
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Burned {
					who: RELAYER_A,
					amount: 100,
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Minted {
					who: TreasuryAccount::get(),
					amount: 10,
				}),
				RuntimeEvent::Bridge(pallet_bridge::Event::FungibleTransfer(
					dest_bridge_id,
					1,
					resource_id,
					100 - 10,
					dest_account,
				)),
			]);
		},
	)
}

#[test]
fn mint_overflow() {
	let dest_bridge_id: pallet_bridge::BridgeChainId = 0;
	let resource_id = NativeTokenResourceId::get();
	let native_token_asset_info: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	> = AssetInfo { fee: 0u64, asset: None };

	new_test_ext_initialized(dest_bridge_id, resource_id, native_token_asset_info).execute_with(
		|| {
			let bridge_id: u64 = Bridge::account_id();
			assert_eq!(Balances::free_balance(bridge_id), ENDOWED_BALANCE);

			assert_noop!(
				BridgeTransfer::transfer(
					RuntimeOrigin::signed(Bridge::account_id()),
					RELAYER_A,
					u64::MAX,
					resource_id,
				),
				ArithmeticError::Overflow
			);
		},
	)
}

#[test]
fn transfer_to_regular_account() {
	new_test_ext().execute_with(|| {
		let dest_chain = 0;
		let asset = pallet_bridge::derive_resource_id(
			dest_chain,
			&pallet_bridge::hashing::blake2_128(b"an asset"),
		);
		let amount: u64 = 100;

		assert_noop!(
			BridgeTransfer::transfer(
				RuntimeOrigin::signed(Bridge::account_id()),
				RELAYER_A,
				amount,
				asset,
			),
			Error::<Test>::InvalidResourceId
		);
	})
}

#[test]
fn create_successful_transfer_proposal() {
	let src_id: pallet_bridge::BridgeChainId = 0;
	let r_id = NativeTokenResourceId::get();
	let native_token_asset_info: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	> = AssetInfo { fee: 0u64, asset: None };

	new_test_ext_initialized(src_id, r_id, native_token_asset_info).execute_with(|| {
		let prop_id = 1;
		let proposal = make_transfer_proposal(RELAYER_A, 10);

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = pallet_bridge::ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: pallet_bridge::ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Second relayer votes against
		assert_ok!(Bridge::reject_proposal(
			RuntimeOrigin::signed(RELAYER_B),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = pallet_bridge::ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![RELAYER_B],
			status: pallet_bridge::ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Third relayer votes in favour
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_C),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal)).unwrap();
		let expected = pallet_bridge::ProposalVotes {
			votes_for: vec![RELAYER_A, RELAYER_C],
			votes_against: vec![RELAYER_B],
			status: pallet_bridge::ProposalStatus::Approved,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		assert_events(vec![
			RuntimeEvent::Bridge(pallet_bridge::Event::VoteFor(src_id, prop_id, RELAYER_A)),
			RuntimeEvent::Bridge(pallet_bridge::Event::VoteAgainst(src_id, prop_id, RELAYER_B)),
			RuntimeEvent::Bridge(pallet_bridge::Event::VoteFor(src_id, prop_id, RELAYER_C)),
			RuntimeEvent::Bridge(pallet_bridge::Event::ProposalApproved(src_id, prop_id)),
			RuntimeEvent::AssetsHandler(Event::TokenBridgeIn {
				asset_id: None,
				to: RELAYER_A,
				amount: 10,
			}),
			RuntimeEvent::Balances(pallet_balances::Event::Minted { who: RELAYER_A, amount: 10 }),
			RuntimeEvent::Bridge(pallet_bridge::Event::ProposalSucceeded(src_id, prop_id)),
		]);
	})
}

#[test]
fn exceed_max_supply() {
	new_test_ext().execute_with(|| {
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(bridge_id), ENDOWED_BALANCE);

		assert_noop!(
			BridgeTransfer::transfer(
				RuntimeOrigin::signed(Bridge::account_id()),
				RELAYER_A,
				MAXIMUM_ISSURANCE + 1,
				resource_id,
			),
			Error::<Test>::ReachMaximumSupply
		);
	})
}

#[test]
fn exceed_max_supply_second() {
	new_test_ext().execute_with(|| {
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(bridge_id), ENDOWED_BALANCE);

		assert_ok!(BridgeTransfer::transfer(
			RuntimeOrigin::signed(Bridge::account_id()),
			RELAYER_A,
			MAXIMUM_ISSURANCE - Balances::total_issuance(),
			resource_id,
		));

		assert_noop!(
			BridgeTransfer::transfer(
				RuntimeOrigin::signed(Bridge::account_id()),
				RELAYER_A,
				10,
				resource_id,
			),
			Error::<Test>::ReachMaximumSupply
		);
	})
}

#[test]
fn test_external_balances_adjusted() {
	let native_token_asset_info: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	> = AssetInfo { fee: 0u64, asset: None };
	new_test_ext().execute_with(|| {
		// Set the new external_balances
		assert_noop!(
			AssetsHandler::set_external_balances(
				RuntimeOrigin::signed(Bridge::account_id()),
				MaximumIssuanceStorage::<Test>::get() / 2
			),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_ok!(AssetsHandler::set_external_balances(
			RuntimeOrigin::root(),
			MaximumIssuanceStorage::<Test>::get() / 2
		));

		// Check inital state
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(bridge_id), ENDOWED_BALANCE);
		// Transfer and check result
		// Check the external_balances
		assert_eq!(ExternalBalances::<Test>::get(), MaximumIssuanceStorage::<Test>::get() / 2);
		assert_ok!(BridgeTransfer::transfer(
			RuntimeOrigin::signed(Bridge::account_id()),
			RELAYER_A,
			10,
			resource_id,
		));
		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		// Check the external_balances
		assert_eq!(ExternalBalances::<Test>::get(), MaximumIssuanceStorage::<Test>::get() / 2 - 10);

		assert_events(vec![
			RuntimeEvent::AssetsHandler(Event::TokenBridgeIn {
				asset_id: None,
				to: RELAYER_A,
				amount: 10,
			}),
			RuntimeEvent::Balances(balances::Event::Minted { who: RELAYER_A, amount: 10 }),
		]);

		// Token cross out of parachain
		// Whitelist setup
		let dest_chain = 0;
		assert_ok!(AssetsHandler::set_resource(
			RuntimeOrigin::root(),
			resource_id,
			native_token_asset_info
		));
		assert_ok!(pallet_bridge::Pallet::<Test>::whitelist_chain(
			RuntimeOrigin::root(),
			dest_chain
		));
		assert_ok!(BridgeTransfer::transfer_assets(
			RuntimeOrigin::signed(RELAYER_A),
			5,
			vec![0u8, 0u8, 0u8, 0u8], // no meaning
			dest_chain,
			resource_id,
		));

		// Check the external_balances
		assert_eq!(ExternalBalances::<Test>::get(), MaximumIssuanceStorage::<Test>::get() / 2 - 5);
	});
}

#[test]
fn set_maximum_issuance() {
	new_test_ext().execute_with(|| {
		assert_eq!(MaximumIssuanceStorage::<Test>::get(), mock::MaximumIssuance::get());
		assert_ok!(pallet::Pallet::<Test>::set_maximum_issuance(
			RuntimeOrigin::signed(RELAYER_A),
			2
		));
		assert_eq!(MaximumIssuanceStorage::<Test>::get(), 2);
		frame_system::Pallet::<Test>::assert_last_event(
			crate::Event::<Test>::MaximumIssuanceChanged {
				old_value: mock::MaximumIssuance::get(),
			}
			.into(),
		);
	});
}

#[test]
fn set_maximum_issuance_fails_with_unprivileged_origin() {
	new_test_ext().execute_with(|| {
		assert_eq!(MaximumIssuanceStorage::<Test>::get(), mock::MaximumIssuance::get());
		assert_noop!(
			pallet::Pallet::<Test>::set_maximum_issuance(RuntimeOrigin::signed(RELAYER_B), 2),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_eq!(MaximumIssuanceStorage::<Test>::get(), mock::MaximumIssuance::get());
	});
}
