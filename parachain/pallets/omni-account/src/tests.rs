use crate::{identity_context::*, mock::*, IDGraphs, LinkedIdentities, *};
use core_primitives::{assertion::network::all_substrate_web3networks, Identity};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;
use sp_std::vec;

#[test]
fn link_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());

		let private_identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let private_identity_hash = H256::from(blake2_256(&private_identity.encode()));

		let public_identity = OmniAccountIdentity::Public(Identity::from(bob()));
		let public_identity_hash = H256::from(blake2_256(&public_identity.encode()));

		let expected_id_graph: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(private_identity.clone(), IdentityContext::new(1, all_substrate_web3networks())),
			(public_identity.clone(), IdentityContext::new(1, all_substrate_web3networks())),
		]);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			private_identity,
			all_substrate_web3networks()
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: private_identity_hash }.into(),
		);
		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer),
			who.clone(),
			public_identity,
			all_substrate_web3networks()
		));
		System::assert_last_event(
			Event::IdentityLinked { who: who.clone(), identity: public_identity_hash }.into(),
		);
		assert!(IDGraphs::<TestRuntime>::contains_key(&who));
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph);
		assert!(LinkedIdentities::<TestRuntime>::contains_key(private_identity_hash));
		assert!(LinkedIdentities::<TestRuntime>::contains_key(public_identity_hash));
	});
}

#[test]
fn link_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(bob()),
				who,
				identity,
				all_substrate_web3networks()
			),
			BadOrigin
		);
	});
}

#[test]
fn link_identity_already_linked_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Public(Identity::from(bob()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity,
				all_substrate_web3networks()
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);

		// intent to create a new id_graph with an identity that is already linked
		let who = Identity::from(bob());
		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				OmniAccountIdentity::Private(vec![1, 2, 3, 4]),
				all_substrate_web3networks()
			),
			Error::<TestRuntime>::IdentityAlreadyLinked
		);
	});
}

#[test]
fn link_identity_ig_graph_len_limit_reached_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		let id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(identity.clone(), IdentityContext::new(1, all_substrate_web3networks())),
			(
				OmniAccountIdentity::Public(Identity::from(bob())),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
		]);

		IDGraphs::<TestRuntime>::insert(who.clone(), id_graph_links.clone());

		assert_noop!(
			OmniAccount::link_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				OmniAccountIdentity::Private(vec![4, 5, 6]),
				all_substrate_web3networks()
			),
			Error::<TestRuntime>::IDGraphLenLimitReached
		);
	});
}

#[test]
fn deactivate_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));
		assert_ok!(OmniAccount::deactivate_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone()
		));
		System::assert_last_event(
			Event::IdentityDeactivated { who: who.clone(), identity: identity_hash }.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(
				identity.clone(),
				IdentityContext {
					link_block: 1,
					web3networks: all_substrate_web3networks(),
					status: IdentityStatus::Inactive,
				},
			),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
	});
}

#[test]
fn deactivate_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		assert_noop!(
			OmniAccount::deactivate_identity(RuntimeOrigin::signed(bob()), who, identity),
			BadOrigin
		);
	});
}

#[test]
fn deactivate_identity_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		assert_noop!(
			OmniAccount::deactivate_identity(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity.clone()
			),
			Error::<TestRuntime>::IdentityNotFound
		);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));

		let other_identity = OmniAccountIdentity::Private(vec![4, 5, 6]);

		assert_noop!(
			OmniAccount::deactivate_identity(
				RuntimeOrigin::signed(tee_signer),
				who,
				other_identity
			),
			Error::<TestRuntime>::IdentityNotFound
		);
	});
}

#[test]
fn activate_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));
		assert_ok!(OmniAccount::deactivate_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone()
		));
		assert_ok!(OmniAccount::activate_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone()
		));
		System::assert_last_event(
			Event::IdentityActivated { who: who.clone(), identity: identity_hash }.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(
				identity.clone(),
				IdentityContext {
					link_block: 1,
					web3networks: all_substrate_web3networks(),
					status: IdentityStatus::Active,
				},
			),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
	});
}

#[test]
fn activate_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		assert_noop!(
			OmniAccount::activate_identity(RuntimeOrigin::signed(bob()), who, identity),
			BadOrigin
		);
	});
}

#[test]
fn activate_identity_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);

		assert_noop!(
			OmniAccount::activate_identity(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity.clone()
			),
			Error::<TestRuntime>::IdentityNotFound
		);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));

		let other_identity = OmniAccountIdentity::Private(vec![4, 5, 6]);

		assert_noop!(
			OmniAccount::activate_identity(RuntimeOrigin::signed(tee_signer), who, other_identity),
			Error::<TestRuntime>::IdentityNotFound
		);
	});
}

#[test]
fn set_identity_networks_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));
		let mut new_web3networks = vec![Web3Network::Ethereum, Web3Network::Polkadot];
		assert_ok!(OmniAccount::set_identity_networks(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			new_web3networks.clone()
		));
		System::assert_last_event(
			Event::Web3NetworksUpdated {
				identity: identity_hash,
				web3networks: new_web3networks.clone(),
			}
			.into(),
		);

		new_web3networks.sort();
		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(
				identity.clone(),
				IdentityContext {
					link_block: 1,
					web3networks: new_web3networks,
					status: IdentityStatus::Active,
				},
			),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
	});
}

#[test]
fn set_identity_networks_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let new_web3networks = vec![Web3Network::Ethereum, Web3Network::Polkadot];

		assert_noop!(
			OmniAccount::set_identity_networks(
				RuntimeOrigin::signed(bob()),
				who,
				identity,
				new_web3networks
			),
			BadOrigin
		);
	});
}

#[test]
fn set_identity_networks_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let new_web3networks = vec![Web3Network::Ethereum, Web3Network::Polkadot];

		assert_noop!(
			OmniAccount::set_identity_networks(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				identity.clone(),
				new_web3networks.clone()
			),
			Error::<TestRuntime>::IdentityNotFound
		);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));

		let other_identity = OmniAccountIdentity::Private(vec![4, 5, 6]);

		assert_noop!(
			OmniAccount::set_identity_networks(
				RuntimeOrigin::signed(tee_signer),
				who,
				other_identity,
				new_web3networks
			),
			Error::<TestRuntime>::IdentityNotFound
		);
	});
}

#[test]
fn remove_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let identity_hash = H256::from(blake2_256(&identity.encode()));
		let identities_to_remove = vec![identity.clone()];

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identity.clone(),
			all_substrate_web3networks()
		));
		assert_ok!(OmniAccount::remove_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			identities_to_remove.clone()
		));
		System::assert_last_event(
			Event::IdentityRemoved { who: who.clone(), identities: identities_to_remove }.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![(
			OmniAccountIdentity::Public(who.clone()),
			IdentityContext::new(1, all_substrate_web3networks()),
		)]);

		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
		assert!(!LinkedIdentities::<TestRuntime>::contains_key(identity_hash));

		assert_ok!(OmniAccount::remove_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			vec![]
		));
		System::assert_last_event(
			Event::IdentityRemoved { who: who.clone(), identities: vec![] }.into(),
		);

		assert!(!IDGraphs::<TestRuntime>::contains_key(&who));
	});
}

#[test]
fn remove_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let identities = vec![OmniAccountIdentity::Private(vec![1, 2, 3])];

		assert_noop!(
			OmniAccount::remove_identity(RuntimeOrigin::signed(bob()), who, identities),
			BadOrigin
		);
	});
}

#[test]
fn make_identity_public_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let who = Identity::from(alice());
		let private_identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let public_identity = OmniAccountIdentity::Public(Identity::from(bob()));
		let public_identity_hash = H256::from(blake2_256(&public_identity.encode()));

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			private_identity.clone(),
			all_substrate_web3networks()
		));

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(
				private_identity.clone(),
				IdentityContext {
					link_block: 1,
					web3networks: all_substrate_web3networks(),
					status: IdentityStatus::Active,
				},
			),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);

		assert_ok!(OmniAccount::make_identity_public(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			private_identity,
			public_identity.clone()
		));
		System::assert_last_event(
			Event::IdentityMadePublic { who: who.clone(), identity: public_identity_hash }.into(),
		);

		let expected_id_graph_links: IDGraphLinks<TestRuntime> = BoundedVec::truncate_from(vec![
			(
				OmniAccountIdentity::Public(who.clone()),
				IdentityContext::new(1, all_substrate_web3networks()),
			),
			(
				public_identity,
				IdentityContext {
					link_block: 1,
					web3networks: all_substrate_web3networks(),
					status: IdentityStatus::Active,
				},
			),
		]);
		assert_eq!(IDGraphs::<TestRuntime>::get(&who).unwrap(), expected_id_graph_links);
	});
}

#[test]
fn make_identity_public_origin_check_works() {
	new_test_ext().execute_with(|| {
		let who = Identity::from(alice());
		let private_identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let public_identity = OmniAccountIdentity::Public(Identity::from(bob()));

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(bob()),
				who,
				private_identity,
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
		let private_identity = OmniAccountIdentity::Private(vec![1, 2, 3]);
		let public_identity = OmniAccountIdentity::Public(Identity::from(bob()));

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(tee_signer.clone()),
				who.clone(),
				private_identity.clone(),
				public_identity.clone()
			),
			Error::<TestRuntime>::IdentityNotFound
		);

		assert_ok!(OmniAccount::link_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			who.clone(),
			private_identity.clone(),
			all_substrate_web3networks()
		));

		let other_identity = OmniAccountIdentity::Private(vec![4, 5, 6]);

		assert_noop!(
			OmniAccount::make_identity_public(
				RuntimeOrigin::signed(tee_signer),
				who,
				other_identity,
				public_identity
			),
			Error::<TestRuntime>::IdentityNotFound
		);
	});
}
