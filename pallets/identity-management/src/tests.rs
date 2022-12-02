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

use crate::{mock::*, ShardIdentifier};
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
			Origin::signed(1),
			shard,
			vec![1u8; 2048]
		));
		System::assert_last_event(Event::IdentityManagement(
			crate::Event::SetUserShieldingKeyRequested { shard },
		));
	});
}

#[test]
fn create_identity_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::create_identity(
			Origin::signed(1),
			shard,
			vec![1u8; 2048],
			Some(vec![1u8; 2048])
		));
		System::assert_last_event(Event::IdentityManagement(
			crate::Event::CreateIdentityRequested { shard },
		));
	});
}

#[test]
fn remove_identity_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::remove_identity(Origin::signed(1), shard, vec![1u8; 2048]));
		System::assert_last_event(Event::IdentityManagement(
			crate::Event::RemoveIdentityRequested { shard },
		));
	});
}

#[test]
fn verify_identity_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::verify_identity(
			Origin::signed(1),
			shard,
			vec![1u8; 2048],
			vec![1u8; 2048]
		));
		System::assert_last_event(Event::IdentityManagement(
			crate::Event::VerifyIdentityRequested { shard },
		));
	});
}

#[test]
fn tee_callback_with_registered_enclave_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teerex::register_enclave(
			Origin::signed(1),
			TEST_MRENCLAVE.to_vec(),
			URL.to_vec()
		));

		assert_ok!(IdentityManagement::some_error(Origin::signed(1), vec![1u8; 16], vec![2u8; 16]));
		System::assert_last_event(Event::IdentityManagement(crate::Event::SomeError {
			func: vec![1u8; 16],
			error: vec![2u8; 16],
		}));
	});
}

#[test]
fn tee_callback_with_unregistered_enclave_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			IdentityManagement::some_error(Origin::signed(1), vec![1u8; 16], vec![2u8; 16]),
			sp_runtime::DispatchError::BadOrigin,
		);
	});
}
