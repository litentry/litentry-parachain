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
	mock::{
		assert_events, new_test_ext, new_test_ext_initialized, Balances, Bridge, ProposalLifetime,
		RuntimeCall, RuntimeEvent, RuntimeOrigin, System, Test, TestChainId, TreasuryAccount,
		ENDOWED_BALANCE, RELAYER_A, RELAYER_B, RELAYER_C, TEST_THRESHOLD,
	},
	pallet::Event as PalletEvent,
	*,
};
use frame_support::{assert_noop, assert_ok};
use frame_system as system;

#[test]
fn derive_ids() {
	let chain = 1;
	let id = [
		0x21, 0x60, 0x5f, 0x71, 0x84, 0x5f, 0x37, 0x2a, 0x9e, 0xd8, 0x42, 0x53, 0xd2, 0xd0, 0x24,
		0xb7, 0xb1, 0x09, 0x99, 0xf4,
	];
	let r_id = derive_resource_id(chain, &id);
	let expected = [
		0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x21, 0x60, 0x5f, 0x71, 0x84, 0x5f,
		0x37, 0x2a, 0x9e, 0xd8, 0x42, 0x53, 0xd2, 0xd0, 0x24, 0xb7, 0xb1, 0x09, 0x99, 0xf4, chain,
	];
	assert_eq!(r_id, expected);
}

#[test]
fn complete_proposal_approved() {
	let mut prop = ProposalVotes {
		votes_for: vec![1, 2],
		votes_against: vec![3],
		status: ProposalStatus::Initiated,
		expiry: ProposalLifetime::get(),
	};

	prop.try_to_complete(2, 3);
	assert_eq!(prop.status, ProposalStatus::Approved);
}

#[test]
fn complete_proposal_rejected() {
	let mut prop = ProposalVotes {
		votes_for: vec![1],
		votes_against: vec![2, 3],
		status: ProposalStatus::Initiated,
		expiry: ProposalLifetime::get(),
	};

	prop.try_to_complete(2, 3);
	assert_eq!(prop.status, ProposalStatus::Rejected);
}

#[test]
fn complete_proposal_bad_threshold() {
	let mut prop = ProposalVotes {
		votes_for: vec![1, 2],
		votes_against: vec![],
		status: ProposalStatus::Initiated,
		expiry: ProposalLifetime::get(),
	};

	prop.try_to_complete(3, 2);
	assert_eq!(prop.status, ProposalStatus::Initiated);

	let mut prop = ProposalVotes {
		votes_for: vec![],
		votes_against: vec![1, 2],
		status: ProposalStatus::Initiated,
		expiry: ProposalLifetime::get(),
	};

	prop.try_to_complete(3, 2);
	assert_eq!(prop.status, ProposalStatus::Initiated);
}

#[test]
fn setup_resources() {
	new_test_ext().execute_with(|| {
		let id: ResourceId = [1; 32];
		let method = "Pallet.do_something".as_bytes().to_vec();
		let method2 = "Pallet.do_somethingElse".as_bytes().to_vec();

		assert_ok!(Bridge::set_resource(RuntimeOrigin::root(), id, method.clone()));
		assert_eq!(Bridge::resources(id), Some(method));

		assert_ok!(Bridge::set_resource(RuntimeOrigin::root(), id, method2.clone()));
		assert_eq!(Bridge::resources(id), Some(method2));

		assert_ok!(Bridge::remove_resource(RuntimeOrigin::root(), id));
		assert_eq!(Bridge::resources(id), None);
	})
}

#[test]
fn whitelist_chain() {
	new_test_ext().execute_with(|| {
		assert!(!Bridge::chain_whitelisted(0));

		assert_ok!(Bridge::whitelist_chain(RuntimeOrigin::root(), 0));
		assert_noop!(
			Bridge::whitelist_chain(RuntimeOrigin::root(), TestChainId::get()),
			Error::<Test>::InvalidChainId
		);

		assert_events(vec![RuntimeEvent::Bridge(PalletEvent::ChainWhitelisted(0))]);
	})
}

#[test]
fn set_get_threshold() {
	new_test_ext().execute_with(|| {
		assert_eq!(<RelayerThreshold<Test>>::get(), 1);

		assert_ok!(Bridge::set_threshold(RuntimeOrigin::root(), TEST_THRESHOLD));
		assert_eq!(<RelayerThreshold<Test>>::get(), TEST_THRESHOLD);

		assert_ok!(Bridge::set_threshold(RuntimeOrigin::root(), 5));
		assert_eq!(<RelayerThreshold<Test>>::get(), 5);

		assert_events(vec![
			RuntimeEvent::Bridge(PalletEvent::RelayerThresholdChanged(TEST_THRESHOLD)),
			RuntimeEvent::Bridge(PalletEvent::RelayerThresholdChanged(5)),
		]);
	})
}

#[test]
fn add_remove_relayer() {
	new_test_ext().execute_with(|| {
		assert_ok!(Bridge::set_threshold(RuntimeOrigin::root(), TEST_THRESHOLD,));
		assert_eq!(Bridge::relayer_count(), 0);

		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_A));
		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_B));
		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_C));
		assert_eq!(Bridge::relayer_count(), 3);

		// Already exists
		assert_noop!(
			Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_A),
			Error::<Test>::RelayerAlreadyExists
		);

		// Confirm removal
		assert_ok!(Bridge::remove_relayer(RuntimeOrigin::root(), RELAYER_B));
		assert_eq!(Bridge::relayer_count(), 2);
		assert_noop!(
			Bridge::remove_relayer(RuntimeOrigin::root(), RELAYER_B),
			Error::<Test>::RelayerInvalid
		);
		assert_eq!(Bridge::relayer_count(), 2);

		assert_events(vec![
			RuntimeEvent::Bridge(PalletEvent::RelayerAdded(RELAYER_A)),
			RuntimeEvent::Bridge(PalletEvent::RelayerAdded(RELAYER_B)),
			RuntimeEvent::Bridge(PalletEvent::RelayerAdded(RELAYER_C)),
			RuntimeEvent::Bridge(PalletEvent::RelayerRemoved(RELAYER_B)),
		]);
	})
}

fn make_proposal(remark: Vec<u8>) -> RuntimeCall {
	RuntimeCall::System(system::Call::remark { remark })
}

#[test]
fn create_sucessful_proposal() {
	let src_id = 1;
	let r_id = derive_resource_id(src_id, b"remark");

	new_test_ext_initialized(src_id, r_id, b"System.remark".to_vec()).execute_with(|| {
		let prop_id = 1;
		let proposal = make_proposal(vec![10]);

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
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
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![RELAYER_B],
			status: ProposalStatus::Initiated,
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
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A, RELAYER_C],
			votes_against: vec![RELAYER_B],
			status: ProposalStatus::Approved,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_events(vec![
			RuntimeEvent::Bridge(PalletEvent::VoteFor(src_id, prop_id, RELAYER_A)),
			RuntimeEvent::Bridge(PalletEvent::VoteAgainst(src_id, prop_id, RELAYER_B)),
			RuntimeEvent::Bridge(PalletEvent::VoteFor(src_id, prop_id, RELAYER_C)),
			RuntimeEvent::Bridge(PalletEvent::ProposalApproved(src_id, prop_id)),
			RuntimeEvent::Bridge(PalletEvent::ProposalSucceeded(src_id, prop_id)),
		]);
	})
}

#[test]
fn create_unsucessful_proposal() {
	let src_id = 1;
	let r_id = derive_resource_id(src_id, b"transfer");

	new_test_ext_initialized(src_id, r_id, b"System.remark".to_vec()).execute_with(|| {
		let prop_id = 1;
		let proposal = make_proposal(vec![11]);

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
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
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![RELAYER_B],
			status: ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Third relayer votes against
		assert_ok!(Bridge::reject_proposal(
			RuntimeOrigin::signed(RELAYER_C),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal)).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![RELAYER_B, RELAYER_C],
			status: ProposalStatus::Rejected,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_eq!(Balances::free_balance(RELAYER_B), 0);
		assert_eq!(Balances::free_balance(Bridge::account_id()), ENDOWED_BALANCE);

		assert_events(vec![
			RuntimeEvent::Bridge(PalletEvent::VoteFor(src_id, prop_id, RELAYER_A)),
			RuntimeEvent::Bridge(PalletEvent::VoteAgainst(src_id, prop_id, RELAYER_B)),
			RuntimeEvent::Bridge(PalletEvent::VoteAgainst(src_id, prop_id, RELAYER_C)),
			RuntimeEvent::Bridge(PalletEvent::ProposalRejected(src_id, prop_id)),
		]);
	})
}

#[test]
fn execute_after_threshold_change() {
	let src_id = 1;
	let r_id = derive_resource_id(src_id, b"transfer");

	new_test_ext_initialized(src_id, r_id, b"System.remark".to_vec()).execute_with(|| {
		let prop_id = 1;
		let proposal = make_proposal(vec![11]);

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Change threshold
		assert_ok!(Bridge::set_threshold(RuntimeOrigin::root(), 1));

		// Attempt to execute
		assert_ok!(Bridge::eval_vote_state(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			Box::new(proposal.clone())
		));

		let prop = Bridge::votes(src_id, (prop_id, proposal)).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Approved,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_eq!(Balances::free_balance(RELAYER_B), 0);
		assert_eq!(Balances::free_balance(Bridge::account_id()), ENDOWED_BALANCE);

		assert_events(vec![
			RuntimeEvent::Bridge(PalletEvent::VoteFor(src_id, prop_id, RELAYER_A)),
			RuntimeEvent::Bridge(PalletEvent::RelayerThresholdChanged(1)),
			RuntimeEvent::Bridge(PalletEvent::ProposalApproved(src_id, prop_id)),
			RuntimeEvent::Bridge(PalletEvent::ProposalSucceeded(src_id, prop_id)),
		]);
	})
}

#[test]
fn proposal_expires() {
	let src_id = 1;
	let r_id = derive_resource_id(src_id, b"remark");

	new_test_ext_initialized(src_id, r_id, b"System.remark".to_vec()).execute_with(|| {
		let prop_id = 1;
		let proposal = make_proposal(vec![10]);

		// Create proposal (& vote)
		assert_ok!(Bridge::acknowledge_proposal(
			RuntimeOrigin::signed(RELAYER_A),
			prop_id,
			src_id,
			r_id,
			Box::new(proposal.clone())
		));
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// Increment enough blocks such that now == expiry
		System::set_block_number(ProposalLifetime::get() + 1);

		// Attempt to submit a vote should fail
		assert_noop!(
			Bridge::reject_proposal(
				RuntimeOrigin::signed(RELAYER_B),
				prop_id,
				src_id,
				r_id,
				Box::new(proposal.clone())
			),
			Error::<Test>::ProposalExpired
		);

		// Proposal state should remain unchanged
		let prop = Bridge::votes(src_id, (prop_id, proposal.clone())).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		// eval_vote_state should have no effect
		assert_noop!(
			Bridge::eval_vote_state(
				RuntimeOrigin::signed(RELAYER_C),
				prop_id,
				src_id,
				Box::new(proposal.clone())
			),
			Error::<Test>::ProposalExpired
		);
		let prop = Bridge::votes(src_id, (prop_id, proposal)).unwrap();
		let expected = ProposalVotes {
			votes_for: vec![RELAYER_A],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
			expiry: ProposalLifetime::get() + 1,
		};
		assert_eq!(prop, expected);

		assert_events(vec![RuntimeEvent::Bridge(PalletEvent::VoteFor(src_id, prop_id, RELAYER_A))]);
	})
}

#[test]
fn transfer_fungible() {
	new_test_ext().execute_with(|| {
		let dest_id: BridgeChainId = 0;
		let resource_id = derive_resource_id(dest_id, b"remark");
		let dest_account: Vec<u8> = vec![1];
		assert_ok!(Pallet::<Test>::update_fee(RuntimeOrigin::root(), dest_id, 10));
		assert_ok!(Pallet::<Test>::whitelist_chain(RuntimeOrigin::root(), dest_id));
		assert_ok!(Pallet::<Test>::transfer_fungible(
			RELAYER_A,
			dest_id,
			resource_id,
			dest_account.clone(),
			100,
		));
		assert_eq!(ChainNonces::<Test>::get(dest_id), Some(1u64));
		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(&TreasuryAccount::get()),
			ENDOWED_BALANCE + 10
		);
		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(&RELAYER_A),
			ENDOWED_BALANCE - 100
		);
		assert_events(vec![
			mock::RuntimeEvent::Balances(pallet_balances::Event::Deposit {
				who: TreasuryAccount::get(),
				amount: 10,
			}),
			RuntimeEvent::Bridge(PalletEvent::FungibleTransfer(
				dest_id,
				1,
				resource_id,
				100 - 10,
				dest_account,
			)),
		]);
	})
}

#[test]
fn transfer_fungible_no_fee() {
	new_test_ext().execute_with(|| {
		let dest_id: BridgeChainId = 0;
		let resource_id = derive_resource_id(dest_id, b"remark");
		let dest_account: Vec<u8> = vec![1];
		assert_ok!(Pallet::<Test>::whitelist_chain(RuntimeOrigin::root(), dest_id));
		assert_noop!(
			Pallet::<Test>::transfer_fungible(RELAYER_A, dest_id, resource_id, dest_account, 100,),
			Error::<Test>::CannotPayAsFee
		);
	})
}

#[test]
fn transfer_fungible_no_whitelist() {
	new_test_ext().execute_with(|| {
		let dest_id: BridgeChainId = 0;
		let resource_id = derive_resource_id(dest_id, b"remark");
		let dest_account: Vec<u8> = vec![1];
		assert_noop!(
			Pallet::<Test>::transfer_fungible(RELAYER_A, dest_id, resource_id, dest_account, 100,),
			Error::<Test>::ChainNotWhitelisted
		);
	})
}

#[test]
fn transfer_fungible_insufficient_funds_fee() {
	new_test_ext().execute_with(|| {
		let dest_id: BridgeChainId = 0;
		let resource_id = derive_resource_id(dest_id, b"remark");
		let dest_account: Vec<u8> = vec![1];
		let fee: BalanceOf<Test> = 10;
		let transfer_amount = fee;
		assert_ok!(Pallet::<Test>::update_fee(RuntimeOrigin::root(), dest_id, fee));
		assert_ok!(Pallet::<Test>::whitelist_chain(RuntimeOrigin::root(), dest_id));
		assert_noop!(
			Pallet::<Test>::transfer_fungible(
				RELAYER_A,
				dest_id,
				resource_id,
				dest_account,
				transfer_amount
			),
			Error::<Test>::FeeTooExpensive
		);
	})
}

#[test]
fn transfer_fungible_insufficient_free_balance() {
	new_test_ext().execute_with(|| {
		let dest_id: BridgeChainId = 0;
		let resource_id = derive_resource_id(dest_id, b"remark");
		let dest_account: Vec<u8> = vec![1];
		let fee: BalanceOf<Test> = 10;
		let transfer_amount = 100;
		assert_ok!(Pallet::<Test>::update_fee(RuntimeOrigin::root(), dest_id, fee));
		assert_ok!(Pallet::<Test>::whitelist_chain(RuntimeOrigin::root(), dest_id));
		assert_noop!(
			Pallet::<Test>::transfer_fungible(
				0x7,
				dest_id,
				resource_id,
				dest_account,
				transfer_amount
			),
			Error::<Test>::InsufficientBalance
		);
	})
}
