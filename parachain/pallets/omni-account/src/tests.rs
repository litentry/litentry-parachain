use crate::{mock::*, IDGraphs, LinkedIdentityHashes, *};
use core_primitives::Identity;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;
use sp_std::vec;

#[test]
fn link_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = Identity::from(alice());
		let who_identity_hash = who.hash().unwrap();

		let bob_member_account = MemberAccount {
			id: MemberIdentity::Private(bob().encode()),
			hash: Identity::from(bob()).hash().unwrap(),
		};
		let charlie_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(charlie())),
			hash: Identity::from(charlie()).hash().unwrap(),
		};

		let expected_id_graph_members: IDGraphMembers<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount { id: MemberIdentity::from(who.clone()), hash: who_identity_hash },
				bob_member_account.clone(),
			]);
		let expected_id_graph_hash = H256::from(blake2_256(
			&expected_id_graph_members
				.iter()
				.map(|member| member.hash)
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			bob_member_account.clone(),
			None
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: bob_member_account.hash }.into(),
		);

		assert!(IDGraphs::<TestRuntime>::contains_key(&who));
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_members);
		assert_eq!(IDGraphHashes::<TestRuntime>::get(&who).unwrap(), expected_id_graph_hash);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer),
			who.clone(),
			charlie_member_account.clone(),
			Some(expected_id_graph_hash),
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: charlie_member_account.hash }
				.into(),
		);

		let expected_id_graph_members: IDGraphMembers<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount { id: MemberIdentity::from(who.clone()), hash: who_identity_hash },
				bob_member_account.clone(),
				charlie_member_account.clone(),
			]);
		let expecte_id_graph_hash = H256::from(blake2_256(
			&expected_id_graph_members
				.iter()
				.map(|member| member.hash)
				.collect::<Vec<H256>>()
				.encode(),
		));

		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_members);
		assert_eq!(IDGraphHashes::<TestRuntime>::get(&who).unwrap(), expecte_id_graph_hash);

		assert!(LinkedIdentityHashes::<TestRuntime>::contains_key(bob_member_account.hash));
		assert!(LinkedIdentityHashes::<TestRuntime>::contains_key(charlie_member_account.hash));
	});
}

#[test]
fn link_identity_exising_id_graph_id_graph_hash_missing_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let bob_member_account = MemberAccount {
			id: MemberIdentity::Private(bob().encode()),
			hash: Identity::from(bob()).hash().unwrap(),
		};
		let charlie_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(charlie())),
			hash: Identity::from(charlie()).hash().unwrap(),
		};

		// IDGraph gets created with the first identity
		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			bob_member_account,
			None
		));

		// to mutate IDGraph with a new identity, the current id_graph_hash must be provided
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				charlie_member_account,
				None
			),
			Error::<TestRuntime>::IDGraphHashMissing
		);
	});
}

#[test]
fn link_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let member_account = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};

		assert_noop!(
			OmniAccount::link_identity(RuntimeOrigin::signed(bob()), who, member_account, None),
			BadOrigin
		);
	});
}

#[test]
fn link_identity_already_linked_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(bob())),
			hash: Identity::from(bob()).hash().unwrap(),
		};

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			member_account.clone(),
			None
		));
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				member_account,
				None
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);

		// intent to create a new id_graph with an identity that is already linked
		let who = Identity::from(bob());
		let alice_member_account = MemberAccount {
			id: MemberIdentity::Public(Identity::from(alice())),
			hash: Identity::from(alice()).hash().unwrap(),
		};
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who.clone(),
				alice_member_account,
				None
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);
	});
}

#[test]
fn link_identity_id_graph_len_limit_reached_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = Identity::from(alice());
		let who_identity_hash = who.hash().unwrap();

		let member_account_2 = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};
		let member_account_3 = MemberAccount {
			id: MemberIdentity::Private(vec![4, 5, 6]),
			hash: H256::from(blake2_256(&[4, 5, 6])),
		};

		let id_graph_members: IDGraphMembers<TestRuntime> = BoundedVec::truncate_from(vec![
			MemberAccount { id: MemberIdentity::from(who.clone()), hash: who_identity_hash },
			member_account_2.clone(),
			member_account_3.clone(),
		]);
		let id_graph_hash = H256::from(blake2_256(&id_graph_members.encode()));

		IDGraphs::<TestRuntime>::insert(who.clone(), id_graph_members.clone());
		IDGraphHashes::<TestRuntime>::insert(who.clone(), id_graph_hash);

		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				MemberAccount {
					id: MemberIdentity::Private(vec![7, 8, 9]),
					hash: H256::from(blake2_256(&[7, 8, 9])),
				},
				Some(id_graph_hash),
			),
			Error::<TestRuntime>::IDGraphLenLimitReached
		);
	});
}

// TOOD: add test for IDGraphHashMismatch

#[test]
fn remove_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let who_identity_hash = who.hash().unwrap();

		let member_account = MemberAccount {
			id: MemberIdentity::Private(vec![1, 2, 3]),
			hash: H256::from(blake2_256(&[1, 2, 3])),
		};
		let identity_hash = member_account.hash;
		let identities_to_remove = vec![identity_hash];

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			member_account.clone(),
			None
		));
		assert_ok!(OmniAccount::remove_identities(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identities_to_remove.clone()
		));
		System::assert_last_event(
			Event::IdentityRemoved { who: who.clone(), identity_hashes: identities_to_remove }
				.into(),
		);

		let expected_id_graph_members: IDGraphMembers<TestRuntime> =
			BoundedVec::truncate_from(vec![MemberAccount {
				id: MemberIdentity::Public(who.clone()),
				hash: who_identity_hash,
			}]);

		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_members);
		assert!(!LinkedIdentityHashes::<TestRuntime>::contains_key(identity_hash));

		assert_ok!(OmniAccount::remove_identities(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			vec![who_identity_hash],
		));
		System::assert_last_event(
			Event::IdentityRemoved { who: who.clone(), identity_hashes: vec![who_identity_hash] }
				.into(),
		);

		assert!(!IDGraphs::<TestRuntime>::contains_key(&who));
	});
}

#[test]
fn remove_identity_empty_identity_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			MemberAccount {
				id: MemberIdentity::Private(vec![1, 2, 3]),
				hash: H256::from(blake2_256(&[1, 2, 3])),
			},
			None
		));
		assert_noop!(
			OmniAccount::remove_identities(RuntimeOrigin::signed(tee_signer.clone()), who, vec![],),
			Error::<TestRuntime>::IdentitiesEmpty
		);
	});
}

#[test]
fn remove_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identities_to_remove = vec![H256::from(blake2_256(&[1, 2, 3]))];

		assert_noop!(
			OmniAccount::remove_identities(RuntimeOrigin::signed(bob()), who, identities_to_remove),
			BadOrigin
		);
	});
}

#[test]
fn make_identity_public_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let public_identity = MemberIdentity::Public(Identity::from(bob()));
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let expected_id_graph_members: IDGraphMembers<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::Public(who.clone()),
					hash: who.hash().unwrap(),
				},
				MemberAccount { id: private_identity.clone(), hash: identity_hash },
			]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_members);

		assert_ok!(OmniAccount::make_identity_public(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			public_identity.clone()
		));
		System::assert_last_event(
			Event::IdentityMadePublic { who: who.clone(), identity_hash }.into(),
		);

		let expected_id_graph_members: IDGraphMembers<TestRuntime> =
			BoundedVec::truncate_from(vec![
				MemberAccount {
					id: MemberIdentity::Public(who.clone()),
					hash: who.hash().unwrap(),
				},
				MemberAccount { id: public_identity.clone(), hash: identity_hash },
			]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_members);
	});
}

#[test]
fn make_identity_public_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identity = Identity::from(bob());
		let identity_hash = identity.hash().unwrap();
		let public_identity = MemberIdentity::Public(identity.clone());

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(bob()),
				who,
				identity_hash,
				public_identity
			),
			BadOrigin
		);
	});
}

#[test]
fn make_identity_public_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity = Identity::from(bob());
		let public_identity = MemberIdentity::Public(identity.clone());
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity_hash,
				public_identity.clone()
			),
			Error::<TestRuntime>::PrimeIdentityNotFound
		);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		let charlie_identity = Identity::from(charlie());
		let other_identity = MemberIdentity::Public(charlie_identity.clone());
		let other_identity_hash =
			H256::from(blake2_256(&charlie_identity.to_did().unwrap().encode()));

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(tee_signer),
				who,
				other_identity_hash,
				other_identity,
			),
			Error::<TestRuntime>::IdentityNotFound
		);
	});
}

#[test]
fn make_identity_public_identity_is_private_check_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = Identity::from(bob()).hash().unwrap();

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			MemberAccount { id: private_identity.clone(), hash: identity_hash },
			None
		));

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(tee_signer),
				who,
				identity_hash,
				private_identity,
			),
			Error::<TestRuntime>::IdentityIsPrivate
		);
	});
}
