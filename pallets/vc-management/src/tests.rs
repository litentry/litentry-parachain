// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::{mock::*, AesOutput, Assertion, Error, ShardIdentifier, Status};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

use test_utils::ias::consts::{TEST8_MRENCLAVE, TEST8_SIGNER_PUB};
const VC_HASH: H256 = H256::zero();
const VC_INDEX: H256 = H256::zero();

type SystemAccountId = <Test as frame_system::Config>::AccountId;
const ALICE_PUBKEY: &[u8; 32] = &[1u8; 32];
const BOB_PUBKEY: &[u8; 32] = &[2u8; 32];

#[test]
fn request_vc_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_ok!(VCManagement::request_vc(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			Assertion::A1
		));
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::VCRequested {
			account: alice,
			shard,
			assertion: Assertion::A1,
		}));
	});
}

#[test]
fn vc_issued_works() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer),
			alice.clone(),
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		let context = VCManagement::vc_registry(VC_INDEX).unwrap();
		assert_eq!(context.subject, alice);
		assert_eq!(context.assertion, Assertion::A1);
		assert_eq!(context.status, Status::Active);
	});
}

#[test]
fn vc_issued_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_noop!(
			VCManagement::vc_issued(
				RuntimeOrigin::signed(bob),
				alice,
				Assertion::A1,
				H256::default(),
				H256::default(),
				AesOutput::default(),
				H256::default(),
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn vc_issued_with_duplicated_index_fails() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer.clone()),
			alice.clone(),
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert_noop!(
			VCManagement::vc_issued(
				RuntimeOrigin::signed(teerex_signer),
				alice,
				Assertion::A1,
				VC_INDEX,
				VC_HASH,
				AesOutput::default(),
				H256::default(),
			),
			Error::<Test>::VCAlreadyExists
		);
	});
}

#[test]
fn disable_vc_works() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer),
			bob.clone(),
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		assert_ok!(VCManagement::disable_vc(RuntimeOrigin::signed(bob), VC_INDEX));
		// vc is not deleted
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		let context = VCManagement::vc_registry(VC_INDEX).unwrap();
		assert_eq!(context.status, Status::Disabled);
	});
}

#[test]
fn disable_vc_with_non_existent_vc_event() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_noop!(
			VCManagement::disable_vc(RuntimeOrigin::signed(alice), VC_INDEX),
			Error::<Test>::VCNotExist
		);
	});
}

#[test]
fn disable_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer),
			bob,
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert_noop!(
			VCManagement::disable_vc(RuntimeOrigin::signed(alice), VC_HASH),
			Error::<Test>::VCSubjectMismatch
		);

		assert_eq!(VCManagement::vc_registry(VC_INDEX).unwrap().status, Status::Active);
	});
}

#[test]
fn revoke_vc_works() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer),
			bob.clone(),
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		assert_ok!(VCManagement::revoke_vc(RuntimeOrigin::signed(bob), VC_INDEX));
		// vc is deleted
		assert!(VCManagement::vc_registry(VC_INDEX).is_none());
	});
}

#[test]
fn revokevc_with_non_existent_vc_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_noop!(
			VCManagement::revoke_vc(RuntimeOrigin::signed(alice), VC_INDEX),
			Error::<Test>::VCNotExist
		);
	});
}

#[test]
fn revoke_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(teerex_signer),
			bob,
			Assertion::A1,
			VC_INDEX,
			VC_HASH,
			AesOutput::default(),
			H256::default(),
		));
		assert_noop!(
			VCManagement::revoke_vc(RuntimeOrigin::signed(alice), VC_HASH),
			Error::<Test>::VCSubjectMismatch
		);
		assert_eq!(VCManagement::vc_registry(VC_INDEX).unwrap().status, Status::Active);
	});
}

#[test]
fn set_admin_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_eq!(VCManagement::admin().unwrap(), alice);
		assert_ok!(VCManagement::set_admin(RuntimeOrigin::root(), bob.clone()));
		assert_eq!(VCManagement::admin().unwrap(), bob);
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::AdminChanged {
			old_admin: Some(alice),
			new_admin: Some(bob),
		}));
	});
}

#[test]
fn set_admin_fails_with_unprivileged_origin() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		assert_eq!(VCManagement::admin().unwrap(), alice);
		assert_noop!(
			VCManagement::set_admin(RuntimeOrigin::signed(bob.clone()), bob),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_eq!(VCManagement::admin().unwrap(), alice);
	});
}

#[test]
fn add_schema_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_eq!(VCManagement::schema_index(), 0);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::SchemaIssued {
			account: alice,
			shard,
			index: 0,
		}));
		assert_eq!(VCManagement::schema_index(), 1);
	});
}

#[test]
fn add_schema_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_noop!(
			VCManagement::add_schema(RuntimeOrigin::signed(bob), shard, id, content),
			Error::<Test>::RequireAdmin
		);
	});
}

#[test]
fn add_two_schemas_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_eq!(VCManagement::schema_index(), 0);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id.clone(),
			content.clone()
		));
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::SchemaIssued {
			account: alice,
			shard,
			index: 1,
		}));
		assert_eq!(VCManagement::schema_index(), 2);
	});
}

#[test]
fn disable_schema_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::disable_schema(RuntimeOrigin::signed(alice), shard, 0));
		assert!(VCManagement::schema_registry(0).is_some());
		let context = VCManagement::schema_registry(0).unwrap();
		assert_eq!(context.status, Status::Disabled);
	});
}

#[test]
fn disable_schema_with_non_existent_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_noop!(
			VCManagement::disable_schema(RuntimeOrigin::signed(alice), shard, 2),
			Error::<Test>::SchemaNotExists
		);
	});
}

#[test]
fn disable_schema_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(RuntimeOrigin::signed(alice), shard, id, content));
		assert_noop!(
			VCManagement::disable_schema(RuntimeOrigin::signed(bob), shard, 0),
			Error::<Test>::RequireAdmin
		);
	});
}

#[test]
fn activate_schema_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::disable_schema(RuntimeOrigin::signed(alice.clone()), shard, 0));
		// schema is disabled
		assert_eq!(VCManagement::schema_registry(0).unwrap().status, Status::Disabled);
		// schema is activated
		assert_ok!(VCManagement::activate_schema(RuntimeOrigin::signed(alice), shard, 0));
		assert_eq!(VCManagement::schema_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn activate_already_activated_schema_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_noop!(
			VCManagement::activate_schema(RuntimeOrigin::signed(alice), shard, 0),
			Error::<Test>::SchemaAlreadyActivated
		);
	});
}

#[test]
fn revoke_schema_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			id,
			content
		));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::revoke_schema(RuntimeOrigin::signed(alice), shard, 0));
		// schema is deleted
		assert!(VCManagement::schema_registry(0).is_none());
	});
}

#[test]
fn revoke_schema_with_non_existent_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_noop!(
			VCManagement::revoke_schema(RuntimeOrigin::signed(alice), shard, 2),
			Error::<Test>::SchemaNotExists
		);
	});
}

#[test]
fn revoke_schema_with_unprivileged_origin_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(RuntimeOrigin::signed(alice), shard, id, content));
		assert_noop!(
			VCManagement::revoke_schema(RuntimeOrigin::signed(bob), shard, 0),
			Error::<Test>::RequireAdmin
		);
	});
}

#[test]
fn manual_add_remove_vc_registry_item_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		// Can not remove non-existing vc
		assert_noop!(
			VCManagement::remove_vc_registry_item(RuntimeOrigin::signed(alice.clone()), VC_INDEX),
			Error::<Test>::VCNotExist
		);
		// Unauthorized party can not add vc
		assert_noop!(
			VCManagement::add_vc_registry_item(
				RuntimeOrigin::signed(bob.clone()),
				VC_INDEX,
				bob.clone(),
				Assertion::A1,
				VC_HASH
			),
			Error::<Test>::RequireAdmin
		);
		// Successfully add vc
		assert_ok!(VCManagement::add_vc_registry_item(
			RuntimeOrigin::signed(alice.clone()),
			VC_INDEX,
			alice.clone(),
			Assertion::A1,
			VC_HASH
		));
		// Check result
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::VCRegistryItemAdded {
			account: alice.clone(),
			assertion: Assertion::A1,
			index: VC_INDEX,
		}));
		// Unauthorized party can not remove vc
		assert_noop!(
			VCManagement::remove_vc_registry_item(RuntimeOrigin::signed(bob), VC_INDEX),
			Error::<Test>::RequireAdmin
		);
		// Successfully remove vc
		assert_ok!(VCManagement::remove_vc_registry_item(RuntimeOrigin::signed(alice), VC_INDEX));
		// Check result and events
		assert!(VCManagement::vc_registry(VC_INDEX).is_none());
		System::assert_last_event(RuntimeEvent::VCManagement(
			crate::Event::VCRegistryItemRemoved { index: VC_INDEX },
		));
	});
}

#[test]
fn manual_add_clear_vc_registry_item_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		// Unauthorized party can not add vc
		assert_noop!(
			VCManagement::add_vc_registry_item(
				RuntimeOrigin::signed(bob.clone()),
				VC_INDEX,
				bob.clone(),
				Assertion::A1,
				VC_HASH
			),
			Error::<Test>::RequireAdmin
		);
		// Successfully add vc
		assert_ok!(VCManagement::add_vc_registry_item(
			RuntimeOrigin::signed(alice.clone()),
			VC_INDEX,
			alice.clone(),
			Assertion::A1,
			VC_HASH
		));
		// Check result
		assert!(VCManagement::vc_registry(VC_INDEX).is_some());
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::VCRegistryItemAdded {
			account: alice.clone(),
			assertion: Assertion::A1,
			index: VC_INDEX,
		}));
		// Unauthorized party can not clear vc
		assert_noop!(
			VCManagement::clear_vc_registry(RuntimeOrigin::signed(bob)),
			Error::<Test>::RequireAdmin
		);
		// Successfully clear vc
		assert_ok!(VCManagement::clear_vc_registry(RuntimeOrigin::signed(alice)));
		// Check result and events
		assert!(VCManagement::vc_registry(VC_INDEX).is_none());
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::VCRegistryCleared));
	});
}
