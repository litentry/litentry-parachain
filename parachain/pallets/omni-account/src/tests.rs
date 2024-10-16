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
use core_primitives::Identity;
use frame_support::{assert_noop, assert_ok};
use sp_core::hashing::blake2_256;
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

fn make_balance_transfer_call(dest: AccountId, value: Balance) -> Box<RuntimeCall> {
	let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest, value });
	Box::new(call)
}

#[test]
fn create_account_store_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_omni_account = who_identity.to_omni_account();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let member_accounts: MemberAccounts<TestRuntime> =
			vec![MemberAccount::Public(who_identity.clone())].try_into().unwrap();

		System::assert_last_event(
			Event::AccountStoreCreated { who: who_omni_account, account_store: member_accounts }
				.into(),
		);

		// create it the second time will fail
		assert_noop!(
			OmniAccount::create_account_store(RuntimeOrigin::signed(tee_signer), who_identity,),
			Error::<TestRuntime>::AccountAlreadyAdded
		);
	});
}

#[test]
fn add_account_without_creating_store_fails() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());

		let bob_member_account =
			MemberAccount::Private(bob().encode(), Identity::from(bob()).hash());

		let call = add_account_call(bob_member_account.clone());
		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				who_identity.hash(),
				call
			),
			Error::<TestRuntime>::AccountNotFound
		);
	});
}

#[test]
fn add_account_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_omni_account = who_identity.to_omni_account();

		let bob_member_account =
			MemberAccount::Private(bob().encode(), Identity::from(bob()).hash());
		let charlie_member_account = MemberAccount::Public(Identity::from(charlie()));

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount::Public(who_identity.clone()),
				bob_member_account.clone(),
			]);
		let expected_account_store_hash = H256::from(blake2_256(
			&expected_member_accounts
				.iter()
				.map(|member| member.hash())
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(bob_member_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		System::assert_has_event(
			Event::AccountAdded {
				who: who_omni_account.clone(),
				member_account_hash: bob_member_account.hash(),
				account_store: expected_member_accounts.clone(),
			}
			.into(),
		);

		assert_eq!(
			AccountStore::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_member_accounts
		);
		assert_eq!(
			AccountStoreHash::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_account_store_hash
		);

		let call = add_account_call(charlie_member_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));
		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount::Public(who_identity.clone()),
				bob_member_account.clone(),
				charlie_member_account.clone(),
			]);

		System::assert_has_event(
			Event::AccountAdded {
				who: who_identity.to_omni_account(),
				member_account_hash: charlie_member_account.hash(),
				account_store: expected_member_accounts.clone(),
			}
			.into(),
		);

		let expected_account_store_hash = H256::from(blake2_256(
			&expected_member_accounts
				.iter()
				.map(|member| member.hash())
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_eq!(
			AccountStore::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_member_accounts
		);
		assert_eq!(
			AccountStoreHash::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_account_store_hash
		);

		assert!(MemberAccountHash::<TestRuntime>::contains_key(bob_member_account.hash()));
		assert!(MemberAccountHash::<TestRuntime>::contains_key(charlie_member_account.hash()));
	});
}

#[test]
fn add_account_origin_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let member_account =
			MemberAccount::Private(vec![1, 2, 3], H256::from(blake2_256(&[1, 2, 3])));

		assert_noop!(
			OmniAccount::add_account(RuntimeOrigin::signed(tee_signer), member_account.clone()),
			BadOrigin
		);

		assert_noop!(
			OmniAccount::add_account(RuntimeOrigin::signed(who.to_omni_account()), member_account),
			BadOrigin
		);
	});
}

#[test]
fn add_account_with_already_linked_account_fails() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_omni_account = who_identity.to_omni_account();

		let member_account = MemberAccount::Public(Identity::from(bob()));

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(member_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call.clone()
		));

		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [0, 0, 0, 0],
					message: Some("AccountAlreadyAdded"),
				})),
			}
			.into(),
		);

		// intent to create a new AccountStore with an account that is already added
		let who = Identity::from(charlie());
		let alice_member_account = MemberAccount::Public(Identity::from(alice()));

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
		));

		let call = add_account_call(alice_member_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.hash(),
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who.to_omni_account(),
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

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_omni_account = who_identity.to_omni_account();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let member_account_2 =
			MemberAccount::Private(vec![1, 2, 3], H256::from(blake2_256(&[1, 2, 3])));
		let member_account_3 =
			MemberAccount::Private(vec![4, 5, 6], H256::from(blake2_256(&[4, 5, 6])));

		let member_accounts: MemberAccounts<TestRuntime> = BoundedVec::truncate_from(vec![
			MemberAccount::Public(who_identity.clone()),
			member_account_2.clone(),
			member_account_3.clone(),
		]);

		AccountStore::<TestRuntime>::insert(who_omni_account.clone(), member_accounts);

		let call = add_account_call(MemberAccount::Private(
			vec![7, 8, 9],
			H256::from(blake2_256(&[7, 8, 9])),
		));
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account,
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
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();
		let who_omni_account = who_identity.to_omni_account();

		let member_account =
			MemberAccount::Private(vec![1, 2, 3], H256::from(blake2_256(&[1, 2, 3])));
		let identity_hash = member_account.hash();
		let hashes = vec![identity_hash];

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(member_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		// normal signed origin should give `BadOrigin`, no matter
		// it's from TEE-worker, or omni-account itself
		assert_noop!(
			OmniAccount::remove_accounts(RuntimeOrigin::signed(tee_signer.clone()), hashes.clone()),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			OmniAccount::remove_accounts(
				RuntimeOrigin::signed(who_omni_account.clone()),
				hashes.clone()
			),
			sp_runtime::DispatchError::BadOrigin
		);

		let call = remove_accounts_call(hashes.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account.clone(),
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![MemberAccount::Public(who_identity.clone())]);

		System::assert_has_event(
			Event::AccountRemoved {
				who: who_omni_account.clone(),
				member_account_hashes: hashes,
				account_store: expected_member_accounts.clone(),
			}
			.into(),
		);

		assert_eq!(
			AccountStore::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_member_accounts
		);
		assert!(!MemberAccountHash::<TestRuntime>::contains_key(identity_hash));

		let call = remove_accounts_call(vec![who_identity_hash]);
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		assert!(!AccountStore::<TestRuntime>::contains_key(&who_omni_account));
	});
}

#[test]
fn remove_account_empty_account_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();
		let who_omni_account = who_identity.to_omni_account();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(MemberAccount::Private(
			vec![1, 2, 3],
			H256::from(blake2_256(&[1, 2, 3])),
		));
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		let call = remove_accounts_call(vec![]);
		// execution itself is ok, but error is shown in the dispatch result
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [5, 0, 0, 0],
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
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();
		let who_omni_account = who_identity.to_omni_account();

		let private_account = MemberAccount::Private(vec![1, 2, 3], Identity::from(bob()).hash());
		let public_account = MemberAccount::Public(Identity::from(bob()));
		let public_account_hash = public_account.hash();

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(private_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount::Public(who_identity.clone()),
				private_account.clone(),
			]);
		assert_eq!(
			AccountStore::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_member_accounts
		);

		let call = publicize_account_call(Identity::from(bob()));
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account.clone(),
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount::Public(who_identity.clone()),
				MemberAccount::Public(Identity::from(bob())),
			]);

		System::assert_has_event(
			Event::AccountMadePublic {
				who: who_omni_account.clone(),
				member_account_hash: public_account_hash,
				account_store: expected_member_accounts.clone(),
			}
			.into(),
		);

		assert_eq!(
			AccountStore::<TestRuntime>::get(&who_omni_account).unwrap(),
			expected_member_accounts
		);
	});
}

#[test]
fn publicize_account_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();
		let who_omni_account = who_identity.to_omni_account();

		let private_account =
			MemberAccount::Private(vec![1, 2, 3], H256::from(blake2_256(&[1, 2, 3])));

		let call = publicize_account_call(Identity::from(bob()));
		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				who_identity_hash,
				call
			),
			Error::<TestRuntime>::AccountNotFound
		);

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(private_account.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		let charlie_identity = Identity::from(charlie());

		let call = publicize_account_call(charlie_identity.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who: who_omni_account,
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
fn dispatch_as_signed_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();
		let who_omni_account = who_identity.to_omni_account();

		assert_ok!(Balances::transfer(RuntimeOrigin::signed(who.clone()), who_omni_account, 6));

		let private_account = MemberAccount::Private(vec![1, 2, 3], Identity::from(bob()).hash());

		assert_ok!(OmniAccount::create_account_store(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
		));

		let call = add_account_call(private_account);
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.hash(),
			call
		));

		let call = make_balance_transfer_call(bob(), 5);
		assert_ok!(OmniAccount::dispatch_as_signed(
			RuntimeOrigin::signed(tee_signer),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsSigned {
				who: who_identity.to_omni_account(),
				result: DispatchResult::Ok(()),
			}
			.into(),
		);

		assert_eq!(Balances::free_balance(bob()), 5);
	});
}
