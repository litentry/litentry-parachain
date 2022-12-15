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
	identity_context::IdentityContext, mock::*, Error, MetadataOf, ParentchainBlockNumber,
	UserShieldingKeyType,
};
use frame_support::{assert_noop, assert_ok};
use litentry_primitives::{
	Identity, IdentityHandle, IdentityString, IdentityWebType, Web2Network, USER_SHIELDING_KEY_LEN,
};
#[test]
fn set_user_shielding_key_works() {
	new_test_ext().execute_with(|| {
		let shielding_key: UserShieldingKeyType = [0u8; USER_SHIELDING_KEY_LEN];
		assert_eq!(IMT::user_shielding_keys(2), None);
		assert_ok!(IMT::set_user_shielding_key(RuntimeOrigin::signed(1), 2, shielding_key.clone()));
		assert_eq!(IMT::user_shielding_keys(2), Some(shielding_key.clone()));
		System::assert_last_event(RuntimeEvent::IMT(crate::Event::UserShieldingKeySet {
			who: 2,
			key: shielding_key,
		}));
	});
}

#[test]
fn create_identity_works() {
	new_test_ext().execute_with(|| {
		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata.clone()),
			1
		));
		assert_eq!(
			IMT::id_graphs(2, ALICE_WEB3_IDENTITY).unwrap(),
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
	new_test_ext().execute_with(|| {
		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_noop!(
			IMT::remove_identity(RuntimeOrigin::signed(1), 2, ALICE_WEB3_IDENTITY.clone()),
			Error::<Test>::IdentityNotExist
		);
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata.clone()),
			1
		));
		assert_eq!(
			IMT::id_graphs(2, ALICE_WEB3_IDENTITY.clone()).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(1),
				verification_request_block: None,
				is_verified: false,
			}
		);
		assert_ok!(IMT::remove_identity(RuntimeOrigin::signed(1), 2, ALICE_WEB3_IDENTITY.clone()));
		assert_eq!(IMT::id_graphs(2, ALICE_WEB3_IDENTITY), None);
	});
}

#[test]
fn verify_identity_works() {
	new_test_ext().execute_with(|| {
		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata.clone()),
			1
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			1
		));
		assert_eq!(
			IMT::id_graphs(2, ALICE_WEB3_IDENTITY).unwrap(),
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
	new_test_ext().execute_with(|| {
		let metadata3: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata3.clone()),
			3
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			3
		));

		let alice_web2_identity: Identity = Identity {
			web_type: IdentityWebType::Web2(Web2Network::Twitter),
			handle: IdentityHandle::String(
				IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
			),
		};
		let metadata2: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			alice_web2_identity.clone(),
			Some(metadata2.clone()),
			2
		));
		assert_ok!(IMT::verify_identity(
			RuntimeOrigin::signed(1),
			2,
			alice_web2_identity.clone(),
			2
		));

		let id_graph = IMT::get_id_graph(&2);
		assert_eq!(id_graph.len(), 2);
	});
}

#[test]
fn verify_identity_fails_when_too_early() {
	new_test_ext().execute_with(|| {
		const CREATION_REQUEST_BLOCK: ParentchainBlockNumber = 2;
		const VERIFICATION_REQUEST_BLOCK: ParentchainBlockNumber = 1;

		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata.clone()),
			CREATION_REQUEST_BLOCK
		));
		assert_noop!(
			IMT::verify_identity(
				RuntimeOrigin::signed(1),
				2,
				ALICE_WEB3_IDENTITY.clone(),
				VERIFICATION_REQUEST_BLOCK
			),
			Error::<Test>::VerificationRequestTooEarly
		);
		assert_eq!(
			IMT::id_graphs(2, ALICE_WEB3_IDENTITY).unwrap(),
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
	new_test_ext().execute_with(|| {
		const CREATION_REQUEST_BLOCK: ParentchainBlockNumber = 1;
		const VERIFICATION_REQUEST_BLOCK: ParentchainBlockNumber = 5;

		let metadata: MetadataOf<Test> = vec![0u8; 16].try_into().unwrap();
		assert_ok!(IMT::create_identity(
			RuntimeOrigin::signed(1),
			2,
			ALICE_WEB3_IDENTITY.clone(),
			Some(metadata.clone()),
			CREATION_REQUEST_BLOCK
		));
		assert_noop!(
			IMT::verify_identity(
				RuntimeOrigin::signed(1),
				2,
				ALICE_WEB3_IDENTITY.clone(),
				VERIFICATION_REQUEST_BLOCK
			),
			Error::<Test>::VerificationRequestTooLate
		);
		assert_eq!(
			IMT::id_graphs(2, ALICE_WEB3_IDENTITY).unwrap(),
			IdentityContext {
				metadata: Some(metadata),
				creation_request_block: Some(CREATION_REQUEST_BLOCK),
				verification_request_block: None,
				is_verified: false,
			}
		);
	});
}
