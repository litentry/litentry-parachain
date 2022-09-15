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
use frame_support::assert_ok;
use sp_core::H256;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];

#[test]
fn test_link_identity() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::link_identity(
			Origin::signed(1),
			shard,
			vec![1u8; 2048],
			vec![1u8; 2048]
		));
		System::assert_last_event(Event::IdentityManagement(crate::Event::LinkIdentityRequested {
			shard,
		}));
	});
}

#[test]
fn test_unlink_identity() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(IdentityManagement::unlink_identity(Origin::signed(1), shard, vec![1u8; 2048]));
		System::assert_last_event(Event::IdentityManagement(
			crate::Event::UnlinkIdentityRequested { shard },
		));
	});
}

#[test]
fn test_verify_identity() {
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
fn test_set_user_shielding_key() {
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
