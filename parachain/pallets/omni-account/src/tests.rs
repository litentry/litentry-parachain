use crate::{mock::*, IDGraphs, LinkedIdentityHashes, *};
use core_primitives::Identity;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;
use sp_std::vec;

#[test]
fn link_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();

		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = H256::from(blake2_256(&who_identity.to_did().unwrap().encode()));

		let bob_private_identity = MemberIdentity::Private(bob().encode());
		let bob_did = Identity::from(bob()).to_did().unwrap();
		let bob_private_identity_hash = H256::from(blake2_256(&bob_did.encode()));

		let charlie_public_identity = MemberIdentity::Public(Identity::from(charlie()));
		let charlie_did = Identity::from(charlie()).to_did().unwrap();
		let charlie_public_identity_hash = H256::from(blake2_256(&charlie_did.encode()));

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(who_identity_hash, MemberIdentity::Public(who_identity.clone())),
			(bob_private_identity_hash, bob_private_identity.clone()),
		]);
		let expected_id_graph_hash = H256::from(blake2_256(&expected_id_graph_links.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			bob_private_identity_hash,
			bob_private_identity.clone(),
			None
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: bob_private_identity_hash }.into(),
		);

		assert!(IDGraphs::<TestRuntime>::contains_key(&who));
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
		assert_eq!(IDGraphHashes::<TestRuntime>::get(&who).unwrap(), expected_id_graph_hash);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer),
			who.clone(),
			charlie_public_identity_hash,
			charlie_public_identity.clone(),
			Some(expected_id_graph_hash),
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: charlie_public_identity_hash }
				.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(who_identity_hash, MemberIdentity::Public(who_identity.clone())),
			(bob_private_identity_hash, bob_private_identity.clone()),
			(charlie_public_identity_hash, charlie_public_identity.clone()),
		]);
		let expectec_id_graph_hash = H256::from(blake2_256(&expected_id_graph_links.encode()));

		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
		assert_eq!(IDGraphHashes::<TestRuntime>::get(&who).unwrap(), expectec_id_graph_hash);

		assert!(LinkedIdentityHashes::<TestRuntime>::contains_key(bob_private_identity_hash));
		assert!(LinkedIdentityHashes::<TestRuntime>::contains_key(charlie_public_identity_hash));
	});
}

#[test]
fn link_identity_exising_id_graph_id_graph_hash_missing_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();

		let bob_private_identity = MemberIdentity::Private(bob().encode());
		let bob_did = Identity::from(bob()).to_did().unwrap();
		let bob_private_identity_hash = H256::from(blake2_256(&bob_did.encode()));

		let charlie_public_identity = MemberIdentity::Public(Identity::from(charlie()));
		let charlie_did = Identity::from(charlie()).to_did().unwrap();
		let charlie_public_identity_hash = H256::from(blake2_256(&charlie_did.encode()));

		// IDGraph gets created with the first identity
		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			bob_private_identity_hash,
			bob_private_identity.clone(),
			None
		));

		// to mutate IDGraph with a new identity, the current id_graph_hash must be provided
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				charlie_public_identity_hash,
				charlie_public_identity.clone(),
				None
			),
			Error::<TestRuntime>::IDGraphHashMissing
		);
	});
}

#[test]
fn link_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = alice();
		let identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));

		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(bob()),
				who,
				identity_hash,
				identity,
				None
			),
			BadOrigin
		);
	});
}

#[test]
fn link_identity_already_linked_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let identity = Identity::from(bob());
		let member_identity = MemberIdentity::Public(identity.clone());
		let identity_hash = H256::from(blake2_256(&identity.to_did().unwrap().encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			member_identity.clone(),
			None
		));
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity_hash,
				member_identity,
				None
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);

		// intent to create a new id_graph with an identity that is already linked
		let who = bob();
		let identity = Identity::from(alice());
		let member_identity = MemberIdentity::Public(identity.clone());
		let identity_hash = H256::from(blake2_256(&identity.to_did().unwrap().encode()));
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who.clone(),
				identity_hash,
				member_identity,
				None
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);
	});
}

#[test]
fn link_identity_ig_graph_len_limit_reached_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity = Identity::from(who.clone());
		let who_identity_hash = H256::from(blake2_256(&who_identity.to_did().unwrap().encode()));

		let member_identity_2 = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash_2 = H256::from(blake2_256(&member_identity_2.encode()));

		let member_identity_3 = MemberIdentity::Private(vec![4, 5, 6]);
		let identity_hash_3 = H256::from(blake2_256(&member_identity_3.encode()));

		let id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(who_identity_hash, MemberIdentity::Public(who_identity.clone())),
			(identity_hash_2, member_identity_2.clone()),
			(identity_hash_3, member_identity_3.clone()),
		]);
		let id_graph_hash = H256::from(blake2_256(&id_graph_links.encode()));

		IDGraphs::<TestRuntime>::insert(who.clone(), id_graph_links.clone());
		IDGraphHashes::<TestRuntime>::insert(who.clone(), id_graph_hash);

		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				H256::from(blake2_256(&[7, 8, 9])),
				MemberIdentity::Private(vec![7, 8, 9]),
				Some(id_graph_hash),
			),
			Error::<TestRuntime>::IDGraphLenLimitReached
		);
	});
}

#[test]
fn remove_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = alice();
		let who_identity_hash =
			H256::from(blake2_256(&Identity::from(who.clone()).to_did().unwrap().encode()));

		let identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));
		let identities_to_remove = vec![identity_hash];

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			identity.clone(),
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

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![(
			H256::from(blake2_256(&Identity::from(who.clone()).to_did().unwrap().encode())),
			MemberIdentity::Public(Identity::from(who.clone())),
		)]);

		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
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
		let who = alice();

		let identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			identity.clone(),
			None
		));
		assert_noop!(
			OmniAccount::remove_identities(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				vec![],
			),
			Error::<TestRuntime>::IdentitiesEmpty
		);
	});
}

#[test]
fn remove_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = alice();
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

		let who = alice();
		let who_identity = Identity::from(who.clone());

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let public_identity = MemberIdentity::Public(Identity::from(bob()));
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			private_identity.clone(),
			None
		));

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				H256::from(blake2_256(&who_identity.to_did().unwrap().encode())),
				MemberIdentity::Public(who_identity.clone()),
			),
			(identity_hash, private_identity.clone()),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);

		assert_ok!(OmniAccount::make_identity_public(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			public_identity.clone()
		));
		System::assert_last_event(
			Event::IdentityMadePublic { who: who.clone(), identity_hash }.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				H256::from(blake2_256(&who_identity.to_did().unwrap().encode())),
				MemberIdentity::Public(who_identity.clone()),
			),
			(identity_hash, public_identity),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
	});
}

#[test]
fn make_identity_public_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = alice();
		let public_identity = MemberIdentity::Public(Identity::from(bob()));
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

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
		let who = alice();
		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let public_identity = MemberIdentity::Public(Identity::from(bob()));
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
			identity_hash,
			private_identity.clone(),
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
		let who = alice();

		let private_identity = MemberIdentity::Private(vec![1, 2, 3]);
		let identity_hash =
			H256::from(blake2_256(&Identity::from(bob()).to_did().unwrap().encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity_hash,
			private_identity.clone(),
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
