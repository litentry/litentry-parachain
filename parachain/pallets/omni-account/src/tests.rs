use crate::{mock::*, OpaqueIDGraphs, OpaqueLinkedIdentities, *};
use core_primitives::Identity as PrimeIdentity;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;
use sp_std::vec;

#[test]
fn create_id_graph_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let prime_identity = PrimeIdentity::from(alice());
		let opaque_id_graph = vec![1, 2, 3];

		assert_ok!(OmniAccount::insert_id_graph(
			RuntimeOrigin::signed(tee_signer),
			prime_identity.clone(),
			opaque_id_graph.clone()
		));
		assert!(OpaqueIDGraphs::<TestRuntime>::contains_key(&prime_identity));
		assert_eq!(OpaqueIDGraphs::<TestRuntime>::get(&prime_identity).unwrap(), opaque_id_graph);
		System::assert_last_event(Event::IDGraphCreated(prime_identity).into());
	});
}

#[test]
fn create_id_graph_origin_check_works() {
	new_test_ext().execute_with(|| {
		let prime_identity = PrimeIdentity::from(alice());
		let opaque_id_graph = vec![1, 2, 3];

		assert_noop!(
			OmniAccount::insert_id_graph(
				RuntimeOrigin::signed(bob()),
				prime_identity,
				opaque_id_graph
			),
			BadOrigin
		);
	});
}

#[test]
fn update_id_graph_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let prime_identity = PrimeIdentity::from(alice());
		let opaque_id_graph = vec![1, 2, 3];
		let updated_id_graph = vec![4, 5, 6];

		assert_ok!(OmniAccount::insert_id_graph(
			RuntimeOrigin::signed(tee_signer.clone()),
			prime_identity.clone(),
			opaque_id_graph.clone()
		));
		assert_ok!(OmniAccount::insert_id_graph(
			RuntimeOrigin::signed(tee_signer.clone()),
			prime_identity.clone(),
			updated_id_graph.clone()
		));
		assert_eq!(OpaqueIDGraphs::<TestRuntime>::get(&prime_identity).unwrap(), updated_id_graph);
		System::assert_last_event(Event::IDGraphUpdated(prime_identity).into());
	});
}

#[test]
fn remove_id_graph_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let prime_identity = PrimeIdentity::from(alice());
		let opaque_id_graph = vec![1, 2, 3];

		assert_ok!(OmniAccount::insert_id_graph(
			RuntimeOrigin::signed(tee_signer.clone()),
			prime_identity.clone(),
			opaque_id_graph.clone()
		));
		assert_ok!(OmniAccount::remove_id_graph(
			RuntimeOrigin::signed(tee_signer),
			prime_identity.clone()
		));
		assert!(!OpaqueIDGraphs::<TestRuntime>::contains_key(&prime_identity));
		System::assert_last_event(Event::IDGraphRemoved(prime_identity).into());
	});
}

#[test]
fn remove_id_graph_origin_check_works() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			OmniAccount::remove_id_graph(
				RuntimeOrigin::signed(alice()),
				PrimeIdentity::from(bob())
			),
			BadOrigin
		);
	});
}

#[test]
fn remove_id_graph_not_found_works() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			OmniAccount::remove_id_graph(
				RuntimeOrigin::signed(get_tee_signer()),
				PrimeIdentity::from(bob())
			),
			Error::<TestRuntime>::IDGraphNotFound
		);
	});
}

#[test]
fn add_linked_identity_works() {
	new_test_ext().execute_with(|| {
		let opaque_linked_identity = vec![1, 2, 3];

		assert_ok!(OmniAccount::add_linked_identity(
			RuntimeOrigin::signed(get_tee_signer()),
			opaque_linked_identity.clone()
		));
		assert!(OpaqueLinkedIdentities::<TestRuntime>::contains_key(&opaque_linked_identity));
		System::assert_last_event(Event::LinkedIdentityAdded(opaque_linked_identity).into());
	});
}

#[test]
fn add_linked_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let opaque_linked_identity = vec![1, 2, 3];

		assert_noop!(
			OmniAccount::add_linked_identity(RuntimeOrigin::signed(bob()), opaque_linked_identity),
			BadOrigin
		);
	});
}

#[test]
fn add_linked_identity_already_added_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let opaque_linked_identity = vec![1, 2, 3];

		assert_ok!(OmniAccount::add_linked_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			opaque_linked_identity.clone()
		));
		assert_noop!(
			OmniAccount::add_linked_identity(
				RuntimeOrigin::signed(tee_signer),
				opaque_linked_identity
			),
			Error::<TestRuntime>::LinkedIdentityAlreadyAdded
		);
	});
}

#[test]
fn remove_linked_identity_works() {
	new_test_ext().execute_with(|| {
		let tee_signer = get_tee_signer();
		let opaque_linked_identity = vec![1, 2, 3];

		assert_ok!(OmniAccount::add_linked_identity(
			RuntimeOrigin::signed(tee_signer.clone()),
			opaque_linked_identity.clone()
		));
		assert_ok!(OmniAccount::remove_linked_identity(
			RuntimeOrigin::signed(tee_signer),
			opaque_linked_identity.clone()
		));
		assert!(!OpaqueLinkedIdentities::<TestRuntime>::contains_key(&opaque_linked_identity));
		System::assert_last_event(Event::LinkedIdentityRemoved(opaque_linked_identity).into());
	});
}

#[test]
fn remove_linked_identity_origin_check_works() {
	new_test_ext().execute_with(|| {
		let opaque_linked_identity = vec![1, 2, 3];

		assert_noop!(
			OmniAccount::remove_linked_identity(
				RuntimeOrigin::signed(bob()),
				opaque_linked_identity
			),
			BadOrigin
		);
	});
}

#[test]
fn remove_linked_identity_not_found_works() {
	new_test_ext().execute_with(|| {
		let opaque_linked_identity = vec![1, 2, 3];

		assert_noop!(
			OmniAccount::remove_linked_identity(
				RuntimeOrigin::signed(get_tee_signer()),
				opaque_linked_identity
			),
			Error::<TestRuntime>::LinkedIdentityNotFound
		);
	});
}
