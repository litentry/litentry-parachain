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
		assert_ok!(VCManagement::request_vc(RuntimeOrigin::signed(1), shard, Assertion::A1));
		System::assert_last_event(RuntimeEvent::VCManagement(crate::Event::VCRequested {
			shard,
			assertion: Assertion::A1,
		}));
	});
}

#[test]
fn vc_issued_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
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
			VCManagement::vc_issued(
				RuntimeOrigin::signed(2),
				2,
				0,
				H256::default(),
				AesOutput::default()
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn vc_issued_with_duplicated_id_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::vc_issued(
				RuntimeOrigin::signed(1),
				2,
				0,
				H256::default(),
				AesOutput::default()
			),
			Error::<Test>::VCAlreadyExists
		);
	});
}

#[test]
fn disable_vc_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert!(VCManagement::vc_registry(0).is_some());
		assert_ok!(VCManagement::disable_vc(RuntimeOrigin::signed(2), 0));
		// vc is not deleted
		assert!(VCManagement::vc_registry(0).is_some());
		let context = VCManagement::vc_registry(0).unwrap();
		assert_eq!(context.status, Status::Disabled);
	});
}

#[test]
fn disable_vc_with_non_existent_vc_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCManagement::disable_vc(RuntimeOrigin::signed(1), 0),
			Error::<Test>::VCNotExist
		);
	});
}

#[test]
fn disable_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::disable_vc(RuntimeOrigin::signed(1), 0),
			Error::<Test>::VCSubjectMismatch
		);
		assert_eq!(VCManagement::vc_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn revoke_vc_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert!(VCManagement::vc_registry(0).is_some());
		assert_ok!(VCManagement::revoke_vc(RuntimeOrigin::signed(2), 0));
		// vc is deleted
		assert!(VCManagement::vc_registry(0).is_none());
	});
}

#[test]
fn revokevc_with_non_existent_vc_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			VCManagement::revoke_vc(RuntimeOrigin::signed(1), 0),
			Error::<Test>::VCNotExist
		);
	});
}

#[test]
fn revoke_vc_with_other_subject_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(VCManagement::vc_issued(
			RuntimeOrigin::signed(1),
			2,
			0,
			H256::default(),
			AesOutput::default()
		));
		assert_noop!(
			VCManagement::revoke_vc(RuntimeOrigin::signed(1), 0),
			Error::<Test>::VCSubjectMismatch
		);
		assert_eq!(VCManagement::vc_registry(0).unwrap().status, Status::Active);
	});
}
