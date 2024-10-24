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

use crate::{mock::*, AccountStore, MemberAccountHash, *};
use core_primitives::{CallEthereum, Identity};
use frame_support::{assert_noop, assert_ok};
use sp_core::hashing::blake2_256;
use sp_core::H160;
use sp_runtime::{traits::BadOrigin, ModuleError};
use sp_std::vec;

fn add_account_call(account: MemberAccount) -> Box<RuntimeCall> {
	let call = RuntimeCall::OmniAccount(crate::Call::add_account { member_account: account });
	Box::new(call)
}

fn remove_accounts_call(hashes: Vec<H256>) -> Box<RuntimeCall> {
	let call =
		RuntimeCall::OmniAccount(crate::Call::remove_accounts { member_account_hashes: hashes });
	Box::new(call)
}

fn publicize_account_call(id: Identity) -> Box<RuntimeCall> {
	let call = RuntimeCall::OmniAccount(crate::Call::publicize_account { member_account: id });
	Box::new(call)
}

fn request_intent_call(intent: Intent) -> Box<RuntimeCall> {
	RuntimeCall::OmniAccount(crate::Call::request_intent { intent }).into()
}

fn make_balance_transfer_call(dest: AccountId, value: Balance) -> Box<RuntimeCall> {
	let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest, value });
	Box::new(call)
}

#[test]
fn create_account_store_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		System::assert_last_event(Event::AccountStoreCreated { who: alice().omni_account }.into());

		// create it the second time will fail
		assert_noop!(
			OmniAccount::create_account_store(RuntimeOrigin::signed(tee_signer), alice().identity),
			Error::<TestRuntime>::AccountAlreadyAdded
		);
	});
}

#[test]
fn add_account_without_creating_store_fails() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let call = add_account_call(private_member_account(bob()));
		let nonce = 0;

		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				alice().identity.hash(),
				call,
				nonce,
			),
			Error::<TestRuntime>::AccountNotFound
		);
	});
}

#[test]
fn add_account_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let bob = private_member_account(bob());
		let charlie = public_member_account(charlie());

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			vec![public_member_account(alice()), bob.clone()].try_into().unwrap();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let call = add_account_call(bob.clone());
		let mut nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		System::assert_has_event(
			Event::AccountAdded {
				who: alice().omni_account.clone(),
				member_account_hash: bob.hash(),
			}
			.into(),
		);

		assert_eq!(
			AccountStore::<TestRuntime>::get(alice().omni_account).unwrap(),
			expected_member_accounts
		);

		let call = add_account_call(charlie.clone());
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce,
		));

		System::assert_has_event(
			Event::AccountAdded { who: alice().omni_account, member_account_hash: charlie.hash() }
				.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			vec![public_member_account(alice()), bob.clone(), charlie.clone()]
				.try_into()
				.unwrap();

		assert_eq!(
			AccountStore::<TestRuntime>::get(alice().omni_account).unwrap(),
			expected_member_accounts
		);

		assert!(MemberAccountHash::<TestRuntime>::contains_key(bob.hash()));
		assert!(MemberAccountHash::<TestRuntime>::contains_key(charlie.hash()));
	});
}

#[test]
fn add_account_origin_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let bob = private_member_account(bob());

		assert_noop!(
			OmniAccount::add_account(RuntimeOrigin::signed(tee_signer), bob.clone()),
			BadOrigin
		);

		assert_noop!(
			OmniAccount::add_account(RuntimeOrigin::signed(alice().omni_account), bob),
			BadOrigin
		);
	});
}

#[test]
fn add_account_with_already_linked_account_fails() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let bob = public_member_account(bob());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.clone(),
		));

		let call = add_account_call(bob.clone());
		let mut alice_nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call.clone(),
			alice_nonce,
		));

		alice_nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			alice_nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [0, 0, 0, 0],
					message: Some("AccountAlreadyAdded"),
				})),
			}
			.into(),
		);

		// intent to create a new AccountStore with an account that is already added
		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			charlie().identity,
		));

		let call = add_account_call(public_member_account(alice()));
		let charlie_nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			charlie().identity.hash(),
			call,
			charlie_nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [0, 0, 0, 0],
					message: Some("AccountAlreadyAdded"),
				})),
			}
			.into(),
		);
	});
}

#[test]
fn add_account_store_len_limit_reached_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let member_accounts: MemberAccounts<TestRuntime> = vec![
			public_member_account(alice()),
			private_member_account(bob()),
			private_member_account(charlie()),
		]
		.try_into()
		.unwrap();

		AccountStore::<TestRuntime>::insert(alice().omni_account, member_accounts);

		let call = add_account_call(MemberAccount::Private(
			vec![7, 8, 9],
			H256::from(blake2_256(&[7, 8, 9])),
		));
		let nonce = 0;

		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [1, 0, 0, 0],
					message: Some("AccountStoreLenLimitReached"),
				})),
			}
			.into(),
		);
	});
}

#[test]
fn remove_account_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let bob = private_member_account(bob());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let call = add_account_call(bob.clone());
		let mut nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		// normal signed origin should give `BadOrigin`, no matter
		// it's from TEE-worker, or omni-account itself
		assert_noop!(
			OmniAccount::remove_accounts(
				RuntimeOrigin::signed(tee_signer.clone()),
				vec![bob.hash()]
			),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			OmniAccount::remove_accounts(
				RuntimeOrigin::signed(alice().omni_account),
				vec![bob.hash()]
			),
			sp_runtime::DispatchError::BadOrigin
		);

		let call = remove_accounts_call(vec![bob.hash()]);
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		System::assert_has_event(
			Event::AccountRemoved {
				who: alice().omni_account,
				member_account_hashes: vec![bob.hash()],
			}
			.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			vec![public_member_account(alice())].try_into().unwrap();

		assert_eq!(
			AccountStore::<TestRuntime>::get(alice().omni_account).unwrap(),
			expected_member_accounts
		);
		assert!(!MemberAccountHash::<TestRuntime>::contains_key(bob.hash()));

		let call = remove_accounts_call(vec![alice().identity.hash()]);
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		assert!(!AccountStore::<TestRuntime>::contains_key(alice().omni_account));
	});
}

#[test]
fn remove_account_empty_account_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let bob = private_member_account(bob());
		let call = add_account_call(bob);
		let mut nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let call = remove_accounts_call(vec![]);
		nonce += 1;
		// execution itself is ok, but error is shown in the dispatch result
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [6, 0, 0, 0],
					message: Some("EmptyAccount"),
				})),
			}
			.into(),
		);
	});
}

#[test]
fn publicize_account_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let private_bob = private_member_account(bob());
		let public_bob = public_member_account(bob());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.clone(),
		));

		let call = add_account_call(private_bob.clone());
		let mut nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			vec![public_member_account(alice()), private_bob.clone()].try_into().unwrap();
		assert_eq!(
			AccountStore::<TestRuntime>::get(alice().omni_account).unwrap(),
			expected_member_accounts
		);

		let call = publicize_account_call(bob().identity);
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		System::assert_has_event(
			Event::AccountMadePublic {
				who: alice().omni_account,
				member_account_hash: public_bob.hash(),
			}
			.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			vec![public_member_account(alice()), public_bob].try_into().unwrap();
		assert_eq!(
			AccountStore::<TestRuntime>::get(alice().omni_account).unwrap(),
			expected_member_accounts
		);
	});
}

#[test]
fn publicize_account_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let bob = private_member_account(bob());

		let call = publicize_account_call(charlie().identity);
		let mut nonce = 0;
		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				alice().identity.hash(),
				call,
				nonce
			),
			Error::<TestRuntime>::AccountNotFound
		);

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let call = add_account_call(bob.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let call = publicize_account_call(charlie().identity);
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [2, 0, 0, 0],
					message: Some("AccountNotFound"),
				})),
			}
			.into(),
		);
	});
}

#[test]
fn request_intent_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let bob = private_member_account(bob());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity
		));

		let call = add_account_call(bob);
		let mut nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let intent =
			Intent::CallEthereum(CallEthereum { address: H160::zero(), input: BoundedVec::new() });

		let call = request_intent_call(intent.clone());
		nonce += 1;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: alice().omni_account,
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		System::assert_has_event(
			Event::IntentRequested { who: alice().omni_account, intent }.into(),
		);
	});
}

#[test]
fn dispatch_as_signed_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		assert_ok!(Balances::transfer(
			RuntimeOrigin::signed(alice().native_account),
			alice().omni_account,
			6
		));

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let call = add_account_call(private_member_account(bob()));
		let nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let call = make_balance_transfer_call(bob().native_account, 5);
		assert_ok!(OmniAccount::dispatch_as_signed(
			RuntimeOrigin::signed(tee_signer),
			alice().identity.hash(),
			call
		));
		System::assert_has_event(
			Event::DispatchedAsSigned { who: alice().omni_account, result: DispatchResult::Ok(()) }
				.into(),
		);

		assert_eq!(Balances::free_balance(bob().native_account), 5);
	});
}

#[test]
fn dispatch_as_omni_account_nonce_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let bob = private_member_account(bob());
		let charlie = public_member_account(charlie());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity,
		));

		let call = add_account_call(bob.clone());
		let nonce = 0;
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			alice().identity.hash(),
			call,
			nonce
		));

		let call = add_account_call(charlie.clone());
		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				alice().identity.hash(),
				call,
				nonce,
			),
			Error::<TestRuntime>::InvalidNonce
		);
	});
}
