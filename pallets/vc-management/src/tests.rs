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

use crate::{mock::*, AesOutput, Assertion, Error, ShardIdentifier, Status};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];

#[test]
fn request_vc_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::request_vc(Origin::signed(1), shard, Assertion::A1));
		System::assert_last_event(Event::VCManagement(crate::Event::VCRequested {
			shard,
			assertion: Assertion::A1,
		}));
	});
}

#[test]
fn vc_issued_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			1,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert!(VCManagement::vc_registry(0).is_some());
		let context = VCManagement::vc_registry(0).unwrap();
		assert_eq!(context.subject, 1);
		assert_eq!(context.hash, H256::default());
		assert_eq!(context.status, Status::Active);
	});
}

#[test]
fn vc_issued_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCManagement::vc_issued(Origin::signed(2), 2, 0, H256::default(), AesOutput::default()),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn vc_issued_with_duplicated_id_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::vc_issued(Origin::signed(1), 2, 0, H256::default(), AesOutput::default()),
			Error::<Test>::VCAlreadyExists
		);
	});
}

#[test]
fn disable_vc_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert!(VCManagement::vc_registry(0).is_some());
		assert_ok!(VCManagement::disable_vc(Origin::signed(2), 0));
		// vc is not deleted
		assert!(VCManagement::vc_registry(0).is_some());
		let context = VCManagement::vc_registry(0).unwrap();
		assert_eq!(context.status, Status::Disabled);
	});
}

#[test]
fn disable_vc_with_non_existent_vc_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(VCManagement::disable_vc(Origin::signed(1), 0), Error::<Test>::VCNotExist);
	});
}

#[test]
fn disable_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::disable_vc(Origin::signed(1), 0),
			Error::<Test>::VCSubjectMismatch
		);
		assert_eq!(VCManagement::vc_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn revoke_vc_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert!(VCManagement::vc_registry(0).is_some());
		assert_ok!(VCManagement::revoke_vc(Origin::signed(2), 0));
		// vc is deleted
		assert!(VCManagement::vc_registry(0).is_none());
	});
}

#[test]
fn revokevc_with_non_existent_vc_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(VCManagement::revoke_vc(Origin::signed(1), 0), Error::<Test>::VCNotExist);
	});
}

#[test]
fn revoke_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			Origin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::revoke_vc(Origin::signed(1), 0),
			Error::<Test>::VCSubjectMismatch
		);
		assert_eq!(VCManagement::vc_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn set_schema_admin_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCManagement::schema_admin().unwrap(), 1);
		assert_ok!(VCManagement::set_schema_admin(Origin::signed(1), 2));
		assert_eq!(VCManagement::schema_admin().unwrap(), 2);
		System::assert_last_event(Event::VCManagement(crate::Event::SchemaAdminChanged {
			old_admin: Some(1),
			new_admin: Some(2),
		}));
	});
}

#[test]
fn set_schema_admin_fails_with_unprivileged_origin() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCManagement::schema_admin().unwrap(), 1);
		assert_noop!(
			VCManagement::set_schema_admin(Origin::signed(2), 2),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_eq!(VCManagement::schema_admin().unwrap(), 1);
	});
}

#[test]
fn add_schema_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCManagement::schema_index(), 0);
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id.clone(), content.clone()));
		System::assert_last_event(Event::VCManagement(crate::Event::SchemaIssued {
			account: 1,
			shard,
			index: 0,
		}));
		assert_eq!(VCManagement::schema_index(), 1);
	});
}

#[test]
fn add_schema_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_noop!(
			VCManagement::add_schema(Origin::signed(2), shard, id, content),
			Error::<Test>::RequireSchemaAdmin
		);
	});
}

#[test]
fn add_two_schemas_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCManagement::schema_index(), 0);
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id.clone(), content.clone()));
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id.clone(), content.clone()));
		System::assert_last_event(Event::VCManagement(crate::Event::SchemaIssued {
			account: 1,
			shard,
			index: 1,
		}));
		assert_eq!(VCManagement::schema_index(), 2);
	});
}

#[test]
fn disable_schema_works() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::disable_schema(Origin::signed(1), shard, 0));
		assert!(VCManagement::schema_registry(0).is_some());
		let context = VCManagement::schema_registry(0).unwrap();
		assert_eq!(context.status, Status::Disabled);
	});
}

#[test]
fn disable_schema_with_non_existent_fails() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_noop!(
			VCManagement::disable_schema(Origin::signed(1), shard, 2),
			Error::<Test>::SchemaNotExists
		);
	});
}

#[test]
fn disable_schema_with_unpriviledged_origin_fails() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert_noop!(
			VCManagement::disable_schema(Origin::signed(2), shard, 0),
			Error::<Test>::RequireSchemaAdmin
		);
	});
}

#[test]
fn activate_schema_works() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::disable_schema(Origin::signed(1), shard, 0));
		// schema is disabled
		assert_eq!(VCManagement::schema_registry(0).unwrap().status, Status::Disabled);
		// schema is activated
		assert_ok!(VCManagement::activate_schema(Origin::signed(1), shard, 0));
		assert_eq!(VCManagement::schema_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn activate_already_activated_schema_fails() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_noop!(
			VCManagement::activate_schema(Origin::signed(1), shard, 0),
			Error::<Test>::SchemaAlreadyActivated
		);
	});
}

#[test]
fn revoke_schema_works() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert!(VCManagement::schema_registry(0).is_some());
		assert_ok!(VCManagement::revoke_schema(Origin::signed(1), shard, 0));
		// schema is deleted
		assert!(VCManagement::schema_registry(0).is_none());
	});
}

#[test]
fn revoke_schema_with_non_existent_fails() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_noop!(
			VCManagement::revoke_schema(Origin::signed(1), shard, 2),
			Error::<Test>::SchemaNotExists
		);
	});
}

#[test]
fn revoke_schema_with_unprivileged_origin_fails() {
	new_test_ext().execute_with(|| {
		let id: Vec<u8> = vec![1, 2, 3, 4].try_into().unwrap();
		let content: Vec<u8> = vec![5, 6, 7, 8].try_into().unwrap();
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::add_schema(Origin::signed(1), shard, id, content));
		assert_noop!(
			VCManagement::revoke_schema(Origin::signed(2), shard, 0),
			Error::<Test>::RequireSchemaAdmin
		);
	});
}
