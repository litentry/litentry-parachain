/*
Copyright 2021 Integritee AG and Supercomputing Systems AG

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

use crate::{mock::*, Error, Event as SidechainEvent, Teerex};
use frame_support::{assert_err, assert_ok, dispatch::DispatchResultWithPostInfo};
use sp_core::H256;
use teerex_primitives::MrSigner;
use test_utils::ias::consts::*;

// give get_signer a concrete type
fn get_signer(pubkey: &[u8; 32]) -> AccountId {
	test_utils::get_signer(pubkey)
}

#[test]
fn confirm_imported_sidechain_block_invalid_next_finalization_candidate() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let hash = H256::default();
		let signer7 = get_signer(TEST7_SIGNER_PUB);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		let block_number = 1;

		register_enclave7();

		assert_err!(
			Sidechain::confirm_imported_sidechain_block(
				RuntimeOrigin::signed(signer7.clone()),
				shard7,
				block_number,
				block_number,
				hash
			),
			Error::<Test>::InvalidNextFinalizationCandidateBlockNumber,
		);
	})
}

#[test]
fn confirm_imported_sidechain_block_works_for_correct_shard() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let hash = H256::default();
		let signer7 = get_signer(TEST7_SIGNER_PUB);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		let block_number = 1;
		let next_finalization_block_candidate = 20;

		register_enclave7();

		assert_ok!(Sidechain::confirm_imported_sidechain_block(
			RuntimeOrigin::signed(signer7.clone()),
			shard7,
			block_number,
			next_finalization_block_candidate,
			hash
		));

		let expected_event =
			RuntimeEvent::Sidechain(SidechainEvent::FinalizedSidechainBlock(signer7, hash));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn confirm_imported_sidechain_block_from_shard_neq_mrenclave_errs() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let hash = H256::default();
		let signer7 = get_signer(TEST7_SIGNER_PUB);
		let shard4 = H256::from_slice(&TEST4_MRENCLAVE);

		register_enclave7();

		let block_number = 1;

		assert_err!(
			Sidechain::confirm_imported_sidechain_block(
				RuntimeOrigin::signed(signer7),
				shard4,
				block_number,
				block_number,
				hash
			),
			pallet_teerex::Error::<Test>::WrongMrenclaveForShard
		);
	})
}

#[test]
fn confirm_imported_sidechain_block_correct_order() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		register_enclave7();

		assert_ok!(confirm_block7(1, 2, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 1);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 2);
		assert_ok!(confirm_block7(2, 3, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 2);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 3);
		assert_ok!(confirm_block7(3, 4, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 3);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 4);
		assert_ok!(confirm_block7(4, 5, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 4);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 5);
		assert_ok!(confirm_block7(5, 6, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 5);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 6);
	})
}

#[test]
fn confirm_imported_sidechain_block_wrong_next() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		register_enclave7();

		assert_ok!(confirm_block7(1, 2, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 1);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 2);
		assert_ok!(confirm_block7(2, 4, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 2);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 4);
		assert_err!(
			confirm_block7(3, 4, H256::random(), true),
			Error::<Test>::ReceivedUnexpectedSidechainBlock
		);
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 2);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 4);
		assert_ok!(confirm_block7(4, 5, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 4);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 5);
	})
}

#[test]
fn confirm_imported_sidechain_block_outdated() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		register_enclave7();

		assert_ok!(confirm_block7(1, 2, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 1);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 2);
		assert_ok!(confirm_block7(2, 4, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 2);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 4);
		assert_err!(
			confirm_block7(2, 4, H256::random(), true),
			Error::<Test>::ReceivedUnexpectedSidechainBlock
		);
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 2);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 4);
		assert_ok!(confirm_block7(4, 5, H256::random(), true));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 4);
		assert_eq!(Sidechain::sidechain_block_finalization_candidate(shard7), 5);
	})
}

#[test]
fn dont_process_confirmation_of_second_registered_enclave() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let shard7 = H256::from_slice(&TEST7_MRENCLAVE);

		register_enclave(TEST7_SIGNER_PUB, TEST7_CERT, 1);
		register_enclave(TEST6_SIGNER_PUB, TEST6_CERT, 2);

		assert_ok!(confirm_block(shard7, TEST6_SIGNER_PUB, 1, 2, H256::default(), false));
		assert_eq!(Sidechain::latest_sidechain_block_confirmation(shard7).block_number, 0);
	})
}

fn register_enclave7() {
	register_enclave(TEST7_SIGNER_PUB, TEST7_CERT, 1);
}

fn register_enclave(signer_pub_key: &MrSigner, cert: &[u8], expected_enclave_count: u64) {
	let signer7 = get_signer(signer_pub_key);

	//Ensure that enclave is registered
	assert_ok!(Teerex::<Test>::register_enclave(
		RuntimeOrigin::signed(signer7),
		cert.to_vec(),
		URL.to_vec(),
		None,
	));
	assert_eq!(Teerex::<Test>::enclave_count(), expected_enclave_count);
}

fn confirm_block7(
	block_number: u64,
	next_finalized_block_number: u64,
	block_header_hash: H256,
	check_for_event: bool,
) -> DispatchResultWithPostInfo {
	let shard7 = H256::from_slice(&TEST7_MRENCLAVE);
	confirm_block(
		shard7,
		TEST7_SIGNER_PUB,
		block_number,
		next_finalized_block_number,
		block_header_hash,
		check_for_event,
	)
}

fn confirm_block(
	shard7: H256,
	signer_pub_key: &[u8; 32],
	block_number: u64,
	next_finalized_block_number: u64,
	block_header_hash: H256,
	check_for_event: bool,
) -> DispatchResultWithPostInfo {
	let signer7 = get_signer(signer_pub_key);

	Sidechain::confirm_imported_sidechain_block(
		RuntimeOrigin::signed(signer7.clone()),
		shard7,
		block_number,
		next_finalized_block_number,
		block_header_hash,
	)?;

	if check_for_event {
		let expected_event = RuntimeEvent::Sidechain(SidechainEvent::FinalizedSidechainBlock(
			signer7,
			block_header_hash,
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	}
	Ok(().into())
}
