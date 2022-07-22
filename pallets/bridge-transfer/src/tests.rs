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

#![cfg(test)]

use super::{
	bridge,
	mock::{
		assert_events, balances, new_test_ext, Balances, Bridge, BridgeTransfer, Call, Event,
		NativeTokenResourceId, Origin, ProposalLifetime, Test, ENDOWED_BALANCE, MAXIMUM_ISSURANCE,
		RELAYER_A, RELAYER_B, RELAYER_C,
	},
	*,
};
use frame_support::{assert_noop, assert_ok};

use hex_literal::hex;

const TEST_THRESHOLD: u32 = 2;

fn make_transfer_proposal(to: u64, amount: u64) -> Call {
	let rid = NativeTokenResourceId::get();
	// let amount
	Call::BridgeTransfer(crate::Call::transfer { to, amount, rid })
}

#[test]
fn constant_equality() {
	let r_id = bridge::derive_resource_id(1, &bridge::hashing::blake2_128(b"LIT"));
	let encoded: [u8; 32] =
		hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
	assert_eq!(r_id, encoded);
}

#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		// Check inital state
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);
		// Transfer and check result
		assert_ok!(BridgeTransfer::transfer(
			Origin::signed(Bridge::account_id()),
			RELAYER_A,
			10,
			resource_id,
		));
		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		assert_events(vec![Event::Balances(balances::Event::Deposit {
			who: RELAYER_A,
			amount: 10,
		})]);
	})
}

#[test]
fn mint_overflow() {
	new_test_ext().execute_with(|| {
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);

		assert_noop!(
			BridgeTransfer::transfer(
				Origin::signed(Bridge::account_id()),
				RELAYER_A,
				u64::MAX,
				resource_id,
			),
			Error::<Test>::OverFlow
		);
	})
}

#[test]
fn exceed_max_supply() {
	new_test_ext().execute_with(|| {
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);

		assert_noop!(
			BridgeTransfer::transfer(
				Origin::signed(Bridge::account_id()),
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
		assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);

		assert_ok!(BridgeTransfer::transfer(
			Origin::signed(Bridge::account_id()),
			RELAYER_A,
			MAXIMUM_ISSURANCE - Balances::total_issuance(),
			resource_id,
		));

		assert_noop!(
			BridgeTransfer::transfer(
				Origin::signed(Bridge::account_id()),
				RELAYER_A,
				10,
				resource_id,
			),
			Error::<Test>::ReachMaximumSupply
		);
	})
}

#[test]
fn transfer_to_regular_account() {
	new_test_ext().execute_with(|| {
		let dest_chain = 0;
		let asset =
			bridge::derive_resource_id(dest_chain, &bridge::hashing::blake2_128(b"an asset"));
		let amount: u64 = 100;

		assert_noop!(
			BridgeTransfer::transfer(
				Origin::signed(Bridge::account_id()),
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
	new_test_ext().execute_with(|| {
		let prop_id = 1;
		let src_id = 1;
		let r_id = bridge::derive_resource_id(src_id, b"transfer");
		let resource = b"BridgeTransfer.transfer".to_vec();
		let proposal = make_transfer_proposal(RELAYER_A, 10);

		assert_ok!(Bridge::set_threshold(Origin::root(), TEST_THRESHOLD,));
		assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_A));
		assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_B));
		assert_ok!(Bridge::add_relayer(Origin::root(), RELAYER_C));
		assert_ok!(Bridge::whitelist_chain(Origin::root(), src_id));
		assert_ok!(Bridge::set_resource(Origin::root(), r_id, resource));

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			Origin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = bridge::ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: bridge::ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Second relayer votes against
		assert_ok!(Bridge::reject_proposal(
			Origin::signed(RELAYER_B),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = bridge::ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![RELAYER_B],
			status: bridge::ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Third relayer votes in favour
		assert_ok!(Bridge::acknowledge_proposal(
			Origin::signed(RELAYER_C),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal)).unwrap();
		let expected = bridge::ProposalVotes {
			votes_for: vec![RELAYER_A, RELAYER_C],
			votes_against: vec![RELAYER_B],
			status: bridge::ProposalStatus::Approved,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		assert_events(vec![
			Event::Bridge(bridge::Event::VoteFor(src_id, prop_id, RELAYER_A)),
			Event::Bridge(bridge::Event::VoteAgainst(src_id, prop_id, RELAYER_B)),
			Event::Bridge(bridge::Event::VoteFor(src_id, prop_id, RELAYER_C)),
			Event::Bridge(bridge::Event::ProposalApproved(src_id, prop_id)),
			Event::Balances(balances::Event::Deposit { who: RELAYER_A, amount: 10 }),
			Event::Bridge(bridge::Event::ProposalSucceeded(src_id, prop_id)),
		]);
	})
}

#[test]
fn test_external_balances_adjusted() {
	new_test_ext().execute_with(|| {
		// Check the default external_balances
		assert_eq!(ExternalBalances::<Test>::get(), <Test as Config>::MaximumIssuance::get());
		// Set the new external_balances
		assert_noop!(
			BridgeTransfer::set_external_balances(
				Origin::signed(Bridge::account_id()),
				<Test as Config>::MaximumIssuance::get() / 2
			),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_ok!(BridgeTransfer::set_external_balances(
			Origin::root(),
			<Test as Config>::MaximumIssuance::get() / 2
		));

		// Check inital state
		let bridge_id: u64 = Bridge::account_id();
		let resource_id = NativeTokenResourceId::get();
		assert_eq!(Balances::free_balance(&bridge_id), ENDOWED_BALANCE);
		// Transfer and check result
		// Check the external_balances
		assert_eq!(ExternalBalances::<Test>::get(), <Test as Config>::MaximumIssuance::get() / 2);
		assert_ok!(BridgeTransfer::transfer(
			Origin::signed(Bridge::account_id()),
			RELAYER_A,
			10,
			resource_id,
		));
		assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);

		// Check the external_balances
		assert_eq!(
			ExternalBalances::<Test>::get(),
			<Test as Config>::MaximumIssuance::get() / 2 - 10
		);

		assert_events(vec![Event::Balances(balances::Event::Deposit {
			who: RELAYER_A,
			amount: 10,
		})]);
	})
}
