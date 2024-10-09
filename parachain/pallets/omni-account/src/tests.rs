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
use sp_core_hashing::blake2_256;
use sp_runtime::{traits::BadOrigin, ModuleError};
use sp_std::vec;

fn remove_accounts_call(hashes: Vec<H256>) -> Box<RuntimeCall> {
	let call =
		RuntimeCall::OmniAccount(crate::Call::remove_accounts { member_account_hashes: hashes });
	Box::new(call)
}

fn publicize_account_call(hash: H256, id: MemberIdentity) -> Box<RuntimeCall> {
	let call = RuntimeCall::OmniAccount(crate::Call::publicize_account {
		member_account_hash: hash,
		public_identity: id,
	});
	Box::new(call)
}

fn make_balance_transfer_call(dest: AccountId, value: Balance) -> Box<RuntimeCall> {
	let call = RuntimeCall::Balances(pallet_balances::Call::transfer { dest, value });
	Box::new(call)
}

#[test]
fn add_account_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let bob_member_account = MemberAccount {
			id: MemberIdentity::Private(bob().encode()),
			hash: Identity::from(bob()).hash(),
		};
		let charlie_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(charlie())),
			hash: Identity::from(charlie()).hash(),
		};

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::from(who_identity.clone()),
					hash: who_identity_hash,
				},
				bob_member_account.clone(),
			]);
		let expected_account_store_hash = H256::from(blake2_256(
			&expected_member_accounts
				.iter()
				.map(|member| member.hash)
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			bob_member_account.clone(),
			None
		));
		System::assert_last_event(
			Event::AccountAdded { who: who.clone(), member_account_hash: bob_member_account.hash }
				.into(),
		);

		assert!(AccountStore::<TestRuntime>::contains_key(&who));
		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), expected_member_accounts);
		assert_eq!(
			AccountStoreHash::<TestRuntime>::get(&who).unwrap(),
			expected_account_store_hash
		);

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer),
			who_identity.clone(),
			charlie_member_account.clone(),
			Some(expected_account_store_hash),
		));
		System::assert_last_event(
			Event::AccountAdded {
				who: who.clone(),
				member_account_hash: charlie_member_account.hash,
			}
			.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::from(who_identity.clone()),
					hash: who_identity_hash,
				},
				bob_member_account.clone(),
				charlie_member_account.clone(),
			]);
		let expected_account_store_hash = H256::from(blake2_256(
			&expected_member_accounts
				.iter()
				.map(|member| member.hash)
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), expected_member_accounts);
		assert_eq!(
			AccountStoreHash::<TestRuntime>::get(&who).unwrap(),
			expected_account_store_hash
		);

		assert!(MemberAccountHash::<TestRuntime>::contains_key(bob_member_account.hash));
		assert!(MemberAccountHash::<TestRuntime>::contains_key(charlie_member_account.hash));
	});
}

#[test]
fn add_account_hash_checking_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who_identity = Identity::from(alice());

		let bob_member_account = MemberAccount {
			id: MemberIdentity::Private(bob().encode()),
			hash: Identity::from(bob()).hash(),
		};
		let charlie_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(charlie())),
			hash: Identity::from(charlie()).hash(),
		};

		// AccountStore gets created with the first account
		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			bob_member_account,
			None
		));

		// to mutate AccountStore with a new account, the current account_store_hash must be provided
		assert_noop!(
			OmniAccount::add_account(
				RuntimeOrigin::signed(tee_signer),
				who_identity,
				charlie_member_account,
				None
			),
			Error::<TestRuntime>::AccountStoreHashMissing
		);
	});
}

#[test]
fn add_account_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let member_account = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};

		assert_noop!(
			OmniAccount::add_account(RuntimeOrigin::signed(bob()), who, member_account, None),
			BadOrigin
		);
	});
}

#[test]
fn add_account_already_linked_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());

		let member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(bob())),
			hash: Identity::from(bob()).hash(),
		};

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			member_account.clone(),
			None
		));
		assert_noop!(
			OmniAccount::add_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				who_identity.clone(),
				member_account,
				None
			),
			Error::<TestRuntime>::AccountAlreadyAdded
		);

		// intent to create a new AccountStore with an account that is already added
		let who = Identity::from(bob());
		let alice_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(alice())),
			hash: Identity::from(alice()).hash(),
		};
		assert_noop!(
			OmniAccount::add_account(
				RuntimeOrigin::signed(tee_signer),
				who.clone(),
				alice_member_account,
				None
			),
			Error::<TestRuntime>::AccountAlreadyAdded
		);
	});
}

#[test]
fn add_account_store_len_limit_reached_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let member_account_2 = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};
		let member_account_3 = MemberAccount {
			id: MemberIdentity::Private(vec![4, 5, 6]),
			hash: H256::from(blake2_256(&[4, 5, 6])),
		};

		let member_accounts: MemberAccounts<TestRuntime> = BoundedVec::truncate_from(vec![
			MemberAccount {
				id: MemberIdentity::from(who_identity.clone()),
				hash: who_identity_hash,
			},
			member_account_2.clone(),
			member_account_3.clone(),
		]);
		let account_store_hash = H256::from(blake2_256(&member_accounts.encode()));

		AccountStore::<TestRuntime>::insert(who.clone(), member_accounts.clone());
		AccountStoreHash::<TestRuntime>::insert(who.clone(), account_store_hash);

		assert_noop!(
			OmniAccount::add_account(
				RuntimeOrigin::signed(tee_signer),
				who_identity,
				MemberAccount {
					id: MemberIdentity::Private(vec![7, 8, 9]),
					hash: H256::from(blake2_256(&[7, 8, 9])),
				},
				Some(account_store_hash),
			),
			Error::<TestRuntime>::AccountStoreLenLimitReached
		);
	});
}

#[test]
fn add_account_store_hash_mismatch_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let member_account = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};

		let member_accounts: MemberAccounts<TestRuntime> = BoundedVec::truncate_from(vec![
			MemberAccount {
				id: MemberIdentity::from(who_identity.clone()),
				hash: who_identity_hash,
			},
			member_account.clone(),
		]);
		let account_store_hash = H256::from(blake2_256(
			&member_accounts.iter().map(|member| member.hash).collect::<Vec<H256>>().encode(),
		));

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			member_account.clone(),
			None
		));

		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), member_accounts);
		assert_eq!(AccountStoreHash::<TestRuntime>::get(&who).unwrap(), account_store_hash);

		// add another account to the store with the correct AccountStoreHash
		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			MemberAccount {
				id: MemberIdentity::Private(vec![4, 5, 6]),
				hash: H256::from(blake2_256(&[4, 5, 6])),
			},
			Some(account_store_hash),
		));

		let member_accounts: MemberAccounts<TestRuntime> = BoundedVec::truncate_from(vec![
			MemberAccount {
				id: MemberIdentity::from(who_identity.clone()),
				hash: who_identity_hash,
			},
			member_account.clone(),
			MemberAccount {
				id: MemberIdentity::Private(vec![4, 5, 6]),
				hash: H256::from(blake2_256(&[4, 5, 6])),
			},
		]);
		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), member_accounts);

		// attempt to add an account with an old AccountStoreHash
		assert_noop!(
			OmniAccount::add_account(
				RuntimeOrigin::signed(tee_signer),
				who_identity,
				MemberAccount {
					id: MemberIdentity::Private(vec![7, 8, 9]),
					hash: H256::from(blake2_256(&[7, 8, 9])),
				},
				Some(account_store_hash),
			),
			Error::<TestRuntime>::AccountStoreHashMismatch
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

		let member_account = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};
		let identity_hash = member_account.hash;
		let hashes = vec![identity_hash];

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			member_account.clone(),
			None
		));

		// normal signed origin should give `BadOrigin`, no matter
		// it's from TEE-worker, or omni-account itself
		assert_noop!(
			OmniAccount::remove_accounts(RuntimeOrigin::signed(tee_signer.clone()), hashes.clone()),
			sp_runtime::DispatchError::BadOrigin
		);

		assert_noop!(
			OmniAccount::remove_accounts(RuntimeOrigin::signed(who.clone()), hashes.clone()),
			sp_runtime::DispatchError::BadOrigin
		);

		let call = remove_accounts_call(hashes.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount { who: who.clone(), result: DispatchResult::Ok(()) }
				.into(),
		);

		System::assert_has_event(
			Event::AccountRemoved { who: who.clone(), member_account_hashes: hashes }.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![MemberAccount {
				id: MemberIdentity::Public(who_identity.clone()),
				hash: who_identity_hash,
			}]);

		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), expected_member_accounts);
		assert!(!MemberAccountHash::<TestRuntime>::contains_key(identity_hash));

		let call = remove_accounts_call(vec![who_identity_hash]);
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		assert!(!AccountStore::<TestRuntime>::contains_key(&who));
	});
}

#[test]
fn remove_account_empty_account_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity,
			MemberAccount {
				id: MemberIdentity::Private(vec![1, 2, 3]),
				hash: H256::from(blake2_256(&[1, 2, 3])),
			},
			None
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
				who,
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
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let public_identity = MemberIdentity::Public(Identity::from(bob()));
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity.clone(),
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::Public(who_identity.clone()),
					hash: who_identity.hash(),
				},
				MemberAccount { id: private_identity.clone(), hash: identity_hash },
			]);
		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), expected_member_accounts);

		let call = publicize_account_call(identity_hash, public_identity.clone());
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));

		System::assert_has_event(
			Event::DispatchedAsOmniAccount { who: who.clone(), result: DispatchResult::Ok(()) }
				.into(),
		);

		System::assert_has_event(
			Event::AccountMadePublic { who: who.clone(), member_account_hash: identity_hash }
				.into(),
		);

		let expected_member_accounts: MemberAccounts<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::Public(who_identity.clone()),
					hash: who_identity.hash(),
				},
				MemberAccount { id: public_identity.clone(), hash: identity_hash },
			]);
		assert_eq!(AccountStore::<TestRuntime>::get(&who).unwrap(), expected_member_accounts);
	});
}

#[test]
fn publicize_account_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity = Identity::from(bob());
		let public_identity = MemberIdentity::Public(identity.clone());
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		let call = publicize_account_call(identity_hash, public_identity.clone());
		assert_noop!(
			OmniAccount::dispatch_as_omni_account(
				RuntimeOrigin::signed(tee_signer.clone()),
				who_identity_hash,
				call
			),
			Error::<TestRuntime>::AccountNotFound
		);

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity,
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let charlie_identity = Identity::from(charlie());
		let other_identity = MemberIdentity::Public(charlie_identity.clone());
		let other_identity_hash =
			H256::from(blake2_256(&charlie_identity.to_did().unwrap().encode()));

		let call = publicize_account_call(other_identity_hash, other_identity);
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who,
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
fn publicize_account_identity_is_private_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = who_identity.hash();

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = Identity::from(bob()).hash();

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity,
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let call = publicize_account_call(identity_hash, private_identity);
		assert_ok!(OmniAccount::dispatch_as_omni_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsOmniAccount {
				who,
				result: Err(DispatchError::Module(ModuleError {
					index: 5,
					error: [5, 0, 0, 0],
					message: Some("AccountIsPrivate"),
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

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = Identity::from(bob()).hash();

		assert_ok!(OmniAccount::add_account(
			RuntimeOrigin::signed(tee_signer.clone()),
			who_identity,
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let call = make_balance_transfer_call(bob(), 5);
		assert_ok!(OmniAccount::dispatch_as_signed(
			RuntimeOrigin::signed(tee_signer),
			who_identity_hash,
			call
		));
		System::assert_has_event(
			Event::DispatchedAsSigned { who, result: DispatchResult::Ok(()) }.into(),
		);

		assert_eq!(Balances::free_balance(bob()), 5);
	});
}
