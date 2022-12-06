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

use crate::{mock::*, Error};

use codec::Encode;
use frame_support::assert_noop;
use sp_core::{blake2_256, Pair, H256};

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

// The following tests are based on:
// - twitter for web2
// - polkadot for web3-substrate
// - ethereum for web3-evm
// TODO: maybe add more types

#[test]
fn create_twitter_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(5);
		setup_create_identity(2, create_mock_twitter_identity(b"alice"), 5);
	});
}

#[test]
fn create_polkadot_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let p = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		setup_create_identity(2, create_mock_polkadot_identity(p.public().0), 3);
	});
}

#[test]
fn create_eth_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let p = Random.generate();
		setup_create_identity(2, create_mock_eth_identity(p.address().0), 3);
	});
}

// actually it should always be successful, as we don't have on-chain web2 verification
// for the mock pallet
#[test]
fn verify_twitter_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		setup_verify_twitter_identity(2, create_mock_twitter_identity(b"alice"), 3);
	});
}

#[test]
fn verify_polkadot_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let p = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		setup_verify_polkadot_identity(2, p, 3);
	});
}

#[test]
fn verify_eth_identity_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(4);
		let p = Random.generate();
		setup_verify_eth_identity(2, p, 4);
	});
}

#[test]
fn double_create_twitter_identity_works() {
	new_test_ext().execute_with(|| {
		// create and verify the first twitter handle
		System::set_block_number(3);
		setup_verify_twitter_identity(2, create_mock_twitter_identity(b"alice"), 3);
		// create second twitter handle works
		System::set_block_number(4);
		setup_create_identity(2, create_mock_twitter_identity(b"bob"), 4);
	});
}

#[test]
fn wrong_polkadot_verification_message_fails() {
	new_test_ext().execute_with(|| {
		System::set_block_number(3);
		let p = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		let identity = create_mock_polkadot_identity(p.public().0);
		let who = 2;
		setup_create_identity(who, identity.clone(), 3);

		System::set_block_number(4);
		let encrypted_identity = tee_encrypt(identity.encode().as_slice());

		// intentionally construct a wrong verification message
		let wrong_msg = blake2_256(&[0u8; 16]).to_vec();
		let sig = p.sign(&wrong_msg);
		let common_validation_data = Web3CommonValidationData {
			message: wrong_msg.try_into().unwrap(),
			signature: IdentityMultiSignature::Sr25519(sig),
		};

		let validation_data = match &identity.web_type {
			IdentityWebType::Web3(Web3Network::Substrate(SubstrateNetwork::Polkadot)) =>
				ValidationData::Web3(Web3ValidationData::Substrate(common_validation_data)),
			_ => panic!("unxpected web_type"),
		};

		assert_noop!(
			IdentityManagementMock::verify_identity(
				Origin::signed(who),
				H256::random(),
				encrypted_identity,
				tee_encrypt(validation_data.encode().as_slice()),
			),
			Error::<Test>::UnexpectedMessage
		);
	});
}
