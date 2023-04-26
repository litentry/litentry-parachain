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

use crate::{
	identity_context::IdentityContext, mock::*, Error, MetadataOf, ParentchainBlockNumber,
	UserShieldingKeyType,
};
use frame_support::{assert_noop, assert_ok};
use litentry_primitives::{Identity, IdentityString, Web2Network, USER_SHIELDING_KEY_LEN};
use sp_runtime::AccountId32;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);

#[test]
fn set_user_shielding_key_works() {
	new_test_ext(false).execute_with(|| {
		let shielding_key: UserShieldingKeyType = [0u8; USER_SHIELDING_KEY_LEN];
		assert_eq!(IMT::user_shielding_keys(BOB), None);

		let ss58_prefix = 131_u16;
		assert_ok!(IMT::set_user_shielding_key(
			RuntimeOrigin::signed(ALICE),
			BOB,
			shielding_key.clone(),
			ss58_prefix
		));
		assert_eq!(IMT::user_shielding_keys(BOB), Some(shielding_key.clone()));
		System::assert_last_event(RuntimeEvent::IMT(crate::Event::UserShieldingKeySet {
			who: BOB,
			key: shielding_key,
		}));
	});
}

#[test]
fn create_identity_works() {
	new_test_ext(true).execute_with(|| {
		let ss58_prefix = 131_u16;
		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata.clone()),
			1,
			ss58_prefix,
		));
		assert_eq!(
			IMT::id_graphs(BOB, alice_web3_identity()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(1),
				verification_request_block: None,
				is_verified: false,
			}
		);
	});
}

#[test]
fn remove_identity_works() {
	new_test_ext(false).execute_with(|| {
		let ss58_prefix = 31_u16;
		let shielding_key: UserShieldingKeyType = [0u8; USER_SHIELDING_KEY_LEN];
		assert_ok!(IMT::set_user_shielding_key(
			RuntimeOrigin::signed(ALICE),
			BOB,
			shielding_key.clone(),
			ss58_prefix.clone()
		));

		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_noop!(
			IMT::remove_identity(RuntimeOrigin::signed(ALICE), BOB, alice_web3_identity()),
			Error::<Test>::IdentityNotExist
		);
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata.clone()),
			1,
			ss58_prefix,
		));
		assert_eq!(
			IMT::id_graphs(BOB, alice_web3_identity()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(1),
				verification_request_block: None,
				is_verified: false,
			}
		);

		let id_graph = IMT::get_id_graph(&BOB);
		assert_eq!(id_graph.len(), 2);

		assert_ok!(IMT::remove_identity(RuntimeOrigin::signed(ALICE), BOB, alice_web3_identity()));
		assert_eq!(IMT::id_graphs(BOB, alice_web3_identity()), None);

		let id_graph = IMT::get_id_graph(&BOB);
		// "1": because of the main id is added by default when first calling set_user_shielding_key.
		assert_eq!(id_graph.len(), 1);

		assert_noop!(
			IMT::remove_identity(RuntimeOrigin::signed(ALICE), BOB, bob_web3_identity()),
			Error::<Test>::RemovePrimeIdentityDisallowed
		);
	});
}

#[test]
fn verify_identity_works() {
	new_test_ext(true).execute_with(|| {
		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		let ss58_prefix = 131_u16;
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata.clone()),
			1,
			ss58_prefix,
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			1
		));
		assert_eq!(
			IMT::id_graphs(BOB, alice_web3_identity()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(1),
				verification_request_block: Some(1),
				is_verified: true,
			}
		);
	});
}

#[test]
fn get_id_graph_works() {
	new_test_ext(true).execute_with(|| {
		let metadata3: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		let ss58_prefix = 131_u16;
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata3.clone()),
			3,
			ss58_prefix,
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			3
		));

		let alice_web2_identity = Identity::Web2 {
			network: Web2Network::Twitter,
			address: IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
		};
		let metadata2: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web2_identity.clone(),
			Some(metadata2.clone()),
			2,
			ss58_prefix,
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web2_identity.clone(),
			2
		));

		let id_graph = IMT::get_id_graph(&BOB);
		// "+1": because of the main id is added by default when first calling creat_identity.
		assert_eq!(id_graph.len(), 2 + 1);
	});
}

#[test]
fn verify_identity_fails_when_too_early() {
	new_test_ext(true).execute_with(|| {
		const CREATION_REQUEST_BLOCK: ParentchainBlockNumber = 2;
		const VERIFICATION_REQUEST_BLOCK: ParentchainBlockNumber = 1;

		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		let ss58_prefix = 131_u16;
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata.clone()),
			CREATION_REQUEST_BLOCK,
			ss58_prefix,
		));
		assert_noop!(
			IMT::verify_identity(
				RuntimeOrigin::signed(ALICE),
				BOB,
				alice_web3_identity(),
				VERIFICATION_REQUEST_BLOCK
			),
			Error::<Test>::VerificationRequestTooEarly
		);
		assert_eq!(
			IMT::id_graphs(BOB, alice_web3_identity()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(CREATION_REQUEST_BLOCK),
				verification_request_block: None,
				is_verified: false,
			}
		);
	});
}

#[test]
fn verify_identity_fails_when_too_late() {
	new_test_ext(true).execute_with(|| {
		const CREATION_REQUEST_BLOCK: ParentchainBlockNumber = 1;
		const VERIFICATION_REQUEST_BLOCK: ParentchainBlockNumber = 5;

		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		let ss58_prefix = 131_u16;
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(ALICE),
			BOB,
			alice_web3_identity(),
			Some(metadata.clone()),
			CREATION_REQUEST_BLOCK,
			ss58_prefix,
		));
		assert_noop!(
			IMT::verify_identity(
				RuntimeOrigin::signed(ALICE),
				BOB,
				alice_web3_identity(),
				VERIFICATION_REQUEST_BLOCK
			),
			Error::<Test>::VerificationRequestTooLate
		);
		assert_eq!(
			IMT::id_graphs(BOB, alice_web3_identity()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(CREATION_REQUEST_BLOCK),
				verification_request_block: None,
				is_verified: false,
			}
		);
	});
}

#[test]
fn get_id_graph_with_max_len_works() {
	new_test_ext(true).execute_with(|| {
		// fill in 21 identities, starting from 1 to reserve place for prime_id
		for i in 1..22 {
			assert_ok!(IMT::create_identity(
				RuntimeOrigin::signed(ALICE),
				BOB,
				alice_twitter_identity(i),
				None,
				i,
				131_u16,
			));
		}
		// the full id_graph should have 22 elements, including the prime_id
		assert_eq!(IMT::get_id_graph(&BOB).len(), 22);

		// only get the recent 15 identities
		let id_graph = IMT::get_id_graph_with_max_len(&BOB, 15);
		assert_eq!(id_graph.len(), 15);
		// index 0 has the most recent identity
		assert_eq!(String::from_utf8(id_graph.get(0).unwrap().0.flat()).unwrap(), "did:twitter:web2:_:alice21");
		// index 14 has the least recent identity
		assert_eq!(String::from_utf8(id_graph.get(14).unwrap().0.flat()).unwrap(), "did:twitter:web2:_:alice7");

		// try to get more than id_graph length
		let id_graph = IMT::get_id_graph_with_max_len(&BOB, 30);
		assert_eq!(id_graph.len(), 22);
		assert_eq!(String::from_utf8(id_graph.get(0).unwrap().0.flat()).unwrap(), "did:twitter:web2:_:alice21");
		assert_eq!(String::from_utf8(id_graph.get(21).unwrap().0.flat()).unwrap(), "did:litmus:web3:substrate:0x0202020202020202020202020202020202020202020202020202020202020202");
	});
}
