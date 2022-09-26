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

use crate::{
	key::aes_encrypt_default, mock::*, Error, Identity, IdentityHandle, IdentityMultiSignature,
	IdentityWebType, ShardIdentifier, UserShieldingKeyType, ValidationData,
	Web3CommonValidationData, Web3Network, Web3ValidationData,
};
use codec::Encode;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, ReservableCurrency},
};
use sp_core::H256;

#[test]
fn unpriveledged_origin_call_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			IdentityManagementMock::set_user_shielding_key(
				Origin::signed(2),
				H256::random(),
				vec![]
			),
			Error::<Test>::CallerNotWhitelisted
		);
	});
}

#[test]
fn set_user_shielding_key_works() {
	new_test_ext().execute_with(|| {
		let _ = setup_user_shieding_key(2);
	});
}

#[test]
fn link_web2_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(5);
		let _ = setup_link_identity(2, create_alice_twitter_identity(), 5);
	});
}

#[test]
fn link_web3_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let _ = setup_link_identity(2, create_alice_polkadot_identity(), 3);
	});
}

// actually it should always be successful, as we don't have on-chain web2 verification
// for the mock pallet
#[test]
fn link_verify_twitter_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let _ = setup_verify_twitter_identity(2, create_alice_twitter_identity(), 3);
	});
}

#[test]
fn link_verify_polkadot_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let _ = setup_verify_polkadot_identity(2, create_alice_polkadot_identity(), 3);
	});
}
