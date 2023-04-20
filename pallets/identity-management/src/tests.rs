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
#[allow(unused)]
use crate::{mock::*, Error, ShardIdentifier};
use core_primitives::{ErrorDetail, IMPError};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[cfg(feature = "skip-ias-check")]
use test_utils::ias::consts::TEST8_CERT;

use test_utils::ias::consts::TEST8_MRENCLAVE;
type SystemAccountId = <Test as frame_system::Config>::AccountId;
const ALICE_PUBKEY: &[u8; 32] = &[1u8; 32];
const BOB_PUBKEY: &[u8; 32] = &[2u8; 32];
const EDDIE_PUBKEY: &[u8; 32] = &[5u8; 32];

#[test]
fn set_user_shielding_key_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(IdentityManagement::set_user_shielding_key(
			RuntimeOrigin::signed(alice),
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
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(IdentityManagement::create_identity(
			RuntimeOrigin::signed(alice.clone()),
			shard,
			alice,
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
		let eddie: SystemAccountId = test_utils::get_signer(EDDIE_PUBKEY);
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(IdentityManagement::create_identity(
			RuntimeOrigin::signed(eddie), // authorised delegatee set in initialisation
			shard,
			alice,
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
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let bob: SystemAccountId = test_utils::get_signer(BOB_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_noop!(
			IdentityManagement::create_identity(
				RuntimeOrigin::signed(bob),
				shard,
				alice,
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
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(IdentityManagement::remove_identity(
			RuntimeOrigin::signed(alice),
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
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_ok!(IdentityManagement::verify_identity(
			RuntimeOrigin::signed(alice),
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
#[cfg(feature = "skip-ias-check")]
fn tee_callback_with_registered_enclave_works() {
	// copied from https://github.com/integritee-network/pallets/blob/5b0706e8b9f726d81d8aff74efbae8e023e783b7/test-utils/src/ias.rs#L147
	const URL: &[u8] =
		&[119, 115, 58, 47, 47, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 57, 57, 57, 49];
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(alice.clone()),
			TEST8_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		assert_ok!(IdentityManagement::some_error(
			RuntimeOrigin::signed(alice),
			None,
			IMPError::VerifyIdentityFailed(ErrorDetail::WrongWeb2Handle),
			H256::default(),
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::VerifyIdentityFailed {
				account: None,
				detail: ErrorDetail::WrongWeb2Handle,
				req_ext_hash: H256::default(),
			},
		));
	});
}

#[test]
fn tee_callback_with_unregistered_enclave_fails() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		assert_noop!(
			IdentityManagement::some_error(
				RuntimeOrigin::signed(alice),
				None,
				IMPError::VerifyIdentityFailed(ErrorDetail::WrongWeb2Handle),
				H256::default(),
			),
			sp_runtime::DispatchError::BadOrigin,
		);
	});
}

#[test]
fn extrinsic_whitelist_origin_works() {
	new_test_ext().execute_with(|| {
		let alice: SystemAccountId = test_utils::get_signer(ALICE_PUBKEY);
		// activate the whitelist which is empty at the beginning
		assert_ok!(IMPExtrinsicWhitelist::switch_group_control_on(RuntimeOrigin::root()));
		let shard: ShardIdentifier = H256::from_slice(&TEST8_MRENCLAVE);
		assert_noop!(
			IdentityManagement::set_user_shielding_key(
				RuntimeOrigin::signed(alice.clone()),
				shard,
				vec![1u8; 2048]
			),
			sp_runtime::DispatchError::BadOrigin
		);

		// add `alice` to whitelist group
		assert_ok!(IMPExtrinsicWhitelist::add_group_member(RuntimeOrigin::root(), alice.clone()));
		assert_ok!(IdentityManagement::set_user_shielding_key(
			RuntimeOrigin::signed(alice),
			shard,
			vec![1u8; 2048]
		));
		System::assert_last_event(RuntimeEvent::IdentityManagement(
			crate::Event::SetUserShieldingKeyRequested { shard },
		));
	});
}
