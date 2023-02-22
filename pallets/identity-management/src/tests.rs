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

use crate::{mock::*, Error, ShardIdentifier};
use core_primitives::IMPError;
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];
// copied from https://github.com/integritee-network/pallets/blob/5b0706e8b9f726d81d8aff74efbae8e023e783b7/test-utils/src/ias.rs#L147
const URL: &[u8] = &[119, 115, 58, 47, 47, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 57, 57, 57, 49];

#[test]
fn set_user_shielding_key_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::set_user_shielding_key(
			RuntimeOrigin::signed(1),
			shard,
			vec![1u8; 2048]
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::SetUserShieldingKeyRequested { shard },
		));
	});
}

#[test]
fn create_identity_without_delegatee_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::create_identity(
			RuntimeOrigin::signed(1),
			shard,
			1,
			vec![1u8; 2048],
			Some(vec![1u8; 2048])
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::CreateIdentityRequested { shard },
		));
	});
}

#[test]
fn create_identity_with_authorised_delegatee_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::create_identity(
			RuntimeOrigin::signed(5), // authorised delegatee set in initialisation
			shard,
			1,
			vec![1u8; 2048],
			Some(vec![1u8; 2048]),
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::CreateIdentityRequested { shard },
		));
	});
}

#[test]
fn create_identity_with_unauthorised_delegatee_fails() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_noop!(
			IdentityManagement::create_identity(
				RuntimeOrigin::signed(3),
				shard,
				1,
				vec![1u8; 2048],
				Some(vec![1u8; 2048]),
			),
			Error::<Test>::UnauthorisedUser
		);
	});
}

#[test]
fn remove_identity_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::remove_identity(
			RuntimeOrigin::signed(1),
			shard,
			vec![1u8; 2048]
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::RemoveIdentityRequested { shard },
		));
	});
}

#[test]
fn verify_identity_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::verify_identity(
			RuntimeOrigin::signed(1),
			shard,
			vec![1u8; 2048],
			vec![1u8; 2048]
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::VerifyIdentityRequested { shard },
		));
	});
}

#[test]
fn tee_callback_with_registered_enclave_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(1),
			TEST_MRENCLAVE.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		assert_ok!(IdentityManagement::some_error(
			RuntimeOrigin::signed(1),
			IMPError::WrongWeb2Handle
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(crate::Event::WrongWeb2Handle));
	});
}

#[test]
fn tee_callback_with_unregistered_enclave_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			IdentityManagement::some_error(RuntimeOrigin::signed(1), IMPError::WrongWeb2Handle),
			sp_runtime::DispatchError::BadOrigin,
		);
	});
}
