// Copyright 2020-2024 Trust Computing GmbH.
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

// TODO: add `sidechain_block_imported` tests

#![allow(dead_code, unused_imports)]
use crate::{
	mock::*, test_util::*, AttestationType, AuthorizedEnclave, DcapProvider, Enclave,
	EnclaveRegistry, Error, Event as TeebagEvent, SgxBuildMode, WorkerType, H256,
};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use sp_keyring::AccountKeyring;
use sp_runtime::AccountId32;

const VALID_TIMESTAMP: Moment = 1671606747000;

fn alice() -> AccountId32 {
	AccountKeyring::Alice.to_account_id()
}

fn default_enclave() -> Enclave {
	Enclave::new(WorkerType::Identity)
		.with_attestation_type(AttestationType::Ignore)
		.with_url(URL.to_vec())
		.with_last_seen_timestamp(pallet_timestamp::Pallet::<Test>::now())
}

fn register_quoting_enclave() {
	let quoting_enclave = br#"{"id":"QE","version":2,"issueDate":"2022-12-04T22:45:33Z","nextUpdate":"2023-01-03T22:45:33Z","tcbEvaluationDataNumber":13,"miscselect":"00000000","miscselectMask":"FFFFFFFF","attributes":"11000000000000000000000000000000","attributesMask":"FBFFFFFFFFFFFFFF0000000000000000","mrsigner":"8C4F5775D796503E96137F77C68A829A0056AC8DED70140B081B094490C57BFF","isvprodid":1,"tcbLevels":[{"tcb":{"isvsvn":6},"tcbDate":"2022-11-09T00:00:00Z","tcbStatus":"UpToDate"},{"tcb":{"isvsvn":5},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00477"]},{"tcb":{"isvsvn":4},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00334","INTEL-SA-00477"]},{"tcb":{"isvsvn":2},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00219","INTEL-SA-00293","INTEL-SA-00334","INTEL-SA-00477"]},{"tcb":{"isvsvn":1},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00202","INTEL-SA-00219","INTEL-SA-00293","INTEL-SA-00334","INTEL-SA-00477"]}]}"#;
	let signature = hex!("47accba321e57c20722a0d3d1db11c9b52661239857dc578ca1bde13976ee288cf39f72111ffe445c7389ef56447c79e30e6b83a8863ed9880de5bde4a8d5c91");
	let certificate_chain = include_bytes!("./sgx_verify/test/dcap/qe_identity_issuer_chain.pem");

	let pubkey: [u8; 32] = [
		65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20, 55,
		147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
	];
	let signer: AccountId32 = get_signer(&pubkey);
	assert_ok!(Teebag::register_quoting_enclave(
		RuntimeOrigin::signed(signer),
		quoting_enclave.to_vec(),
		signature.to_vec(),
		certificate_chain.to_vec(),
	));
}

fn register_tcb_info() {
	let tcb_info = br#"{"id":"SGX","version":3,"issueDate":"2022-11-17T12:45:32Z","nextUpdate":"2023-04-16T12:45:32Z","fmspc":"00906EA10000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":12,"tcbLevels":[{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":11},"tcbDate":"2021-11-10T00:00:00Z","tcbStatus":"SWHardeningNeeded","advisoryIDs":["INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":11},"tcbDate":"2021-11-10T00:00:00Z","tcbStatus":"ConfigurationAndSWHardeningNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00477","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":15},{"svn":15},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-06-10T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":15},{"svn":15},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-06-10T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":14},{"svn":14},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2019-12-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":14},{"svn":14},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2019-12-11T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":13},{"svn":13},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":3},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":9},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":13},{"svn":13},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":9},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":6},{"svn":6},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":6},{"svn":6},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-01-09T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00233","INTEL-SA-00161","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":6},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-01-09T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":6},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":4},{"svn":4},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":5},"tcbDate":"2018-01-04T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00106","INTEL-SA-00115","INTEL-SA-00135","INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":2},{"svn":2},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":4},"tcbDate":"2017-07-26T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00088","INTEL-SA-00106","INTEL-SA-00115","INTEL-SA-00135","INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]}]}"#;
	let signature = hex!("71746f2148ecba04e35cf1ac77a7e6267ce99f6781c1031f724bb5bd94b8c1b6e4c07c01dc151692aa75be80dfba7350bb80c58314a6975189597e28e9bbc75c");
	let certificate_chain = include_bytes!("./sgx_verify/test/dcap/tcb_info_issuer_chain.pem");

	let pubkey: [u8; 32] = [
		65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20, 55,
		147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
	];
	let signer: AccountId32 = get_signer(&pubkey);
	assert_ok!(Teebag::register_tcb_info(
		RuntimeOrigin::signed(signer),
		tcb_info.to_vec(),
		signature.to_vec(),
		certificate_chain.to_vec(),
	));
}

// =====================================================
// Unittest in `Development` mode, where:
// - AttestationType::Ignore is possible
// - No authorized enclave check
// - No sgx_build_mode check
// =====================================================

#[test]
fn register_enclave_dev_works_with_no_authorized_enclave() {
	new_test_ext(true).execute_with(|| {
		// it works with no entry in authorized_enclave
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(alice()),
			Default::default(),
			Default::default(),
			TEST4_MRENCLAVE.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ignore,
		));

		let enclave = default_enclave().with_mrenclave(TEST4_MRENCLAVE);

		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 0);
		assert_eq!(EnclaveRegistry::<Test>::get(alice()).unwrap(), enclave);
		let authorized_enclave = AuthorizedEnclave::<Test>::get(WorkerType::default());
		assert_eq!(authorized_enclave.len(), 1);
		assert_eq!(authorized_enclave.first().unwrap(), TEST4_MRENCLAVE.as_ref());
	})
}

#[test]
fn register_enclave_dev_works_with_sgx_build_mode_debug() {
	new_test_ext(true).execute_with(|| {
		// we'll need to use real attestation data
		set_timestamp(TEST4_TIMESTAMP);
		let signer4: AccountId32 = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			Default::default(),
			Default::default(),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));

		let enclave = default_enclave()
			.with_mrenclave(TEST4_MRENCLAVE)
			.with_last_seen_timestamp(TEST4_TIMESTAMP)
			.with_sgx_build_mode(SgxBuildMode::Debug)
			.with_attestation_type(AttestationType::Ias);

		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 0);
		assert_eq!(EnclaveRegistry::<Test>::get(signer4).unwrap(), enclave);
	})
}

#[test]
fn parentchain_block_processed_works() {
	new_test_ext(true).execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);

		// start from block 2, otherwise we get `TimeStamp not set` error,
		// because `run_to_block` calls `Timestamp::on_finalize`
		run_to_block(2);
		Timestamp::set_timestamp(TEST7_TIMESTAMP + 12 * 1000);

		let block_hash = H256::default();
		let merkle_root = H256::default();
		let block_number = 2;
		let signer7: AccountId32 = get_signer(TEST7_SIGNER_PUB);

		// Ensure that enclave is registered
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer7.clone()),
			WorkerType::BitAcross,
			Default::default(),
			TEST7_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 1);

		run_to_block(3);
		Timestamp::set_timestamp(TEST7_TIMESTAMP + 24 * 1000);

		assert_ok!(Teebag::parentchain_block_processed(
			RuntimeOrigin::signed(signer7.clone()),
			block_hash,
			block_number,
			merkle_root,
		));

		let expected_event = RuntimeEvent::Teebag(TeebagEvent::ParentchainBlockProcessed {
			who: signer7,
			block_number,
			block_hash,
			task_merkle_root: merkle_root,
		});
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn register_dcap_enclave_works() {
	new_test_ext(true).execute_with(|| {
		Timestamp::set_timestamp(VALID_TIMESTAMP);
		register_quoting_enclave();
		register_tcb_info();

		let pubkey: [u8; 32] = [
			65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20,
			55, 147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
		];
		let signer: AccountId = get_signer(&pubkey);
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			WorkerType::Identity,
			Default::default(),
			TEST1_DCAP_QUOTE.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Dcap(DcapProvider::Integritee)
		));
		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_registry(&signer).unwrap().last_seen_timestamp, VALID_TIMESTAMP);
		assert_ok!(Teebag::unregister_enclave(RuntimeOrigin::signed(signer)));
		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 0);
	})
}

// =====================================================
// Unittest in `Production` mode
// =====================================================

#[test]
fn register_enclave_prod_works_with_sgx_build_mode_debug() {
	new_test_ext(false).execute_with(|| {
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::Identity,
			TEST4_MRENCLAVE
		));

		set_timestamp(TEST4_TIMESTAMP);
		let signer4: AccountId32 = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			Default::default(),
			Default::default(),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));

		let enclave = default_enclave()
			.with_mrenclave(TEST4_MRENCLAVE)
			.with_last_seen_timestamp(TEST4_TIMESTAMP)
			.with_sgx_build_mode(SgxBuildMode::Debug)
			.with_attestation_type(AttestationType::Ias);

		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 0);
		assert_eq!(EnclaveRegistry::<Test>::get(signer4).unwrap(), enclave);
	})
}

#[test]
fn register_enclave_prod_works_with_sgx_build_mode_production() {
	new_test_ext(false).execute_with(|| {
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::Identity,
			TEST8_MRENCLAVE
		));

		set_timestamp(TEST8_TIMESTAMP);
		let signer8: AccountId32 = get_signer(TEST8_SIGNER_PUB);
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer8.clone()),
			Default::default(),
			Default::default(),
			TEST8_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));

		let enclave = default_enclave()
			.with_mrenclave(TEST8_MRENCLAVE)
			.with_last_seen_timestamp(TEST8_TIMESTAMP)
			.with_sgx_build_mode(SgxBuildMode::Production)
			.with_attestation_type(AttestationType::Ias);

		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 0);
		assert_eq!(EnclaveRegistry::<Test>::get(signer8).unwrap(), enclave);

		// remove authorized enclave should remove enclave too
		assert_ok!(Teebag::force_remove_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::Identity,
			TEST8_MRENCLAVE
		));
		assert_eq!(Teebag::authorized_enclave(WorkerType::Identity).len(), 0);
		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 0);
	})
}

#[test]
fn register_enclave_prod_fails_with_wrong_attestation_type() {
	new_test_ext(false).execute_with(|| {
		assert_noop!(
			Teebag::register_enclave(
				RuntimeOrigin::signed(alice()),
				Default::default(),
				Default::default(),
				TEST4_MRENCLAVE.to_vec(),
				URL.to_vec(),
				None,
				None,
				AttestationType::Ignore, // only allowed in dev mode
			),
			Error::<Test>::InvalidAttestationType
		);
	})
}

#[test]
fn register_enclave_prod_fails_with_no_authorized_enclave() {
	new_test_ext(false).execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_noop!(
			Teebag::register_enclave(
				RuntimeOrigin::signed(signer),
				Default::default(),
				Default::default(),
				TEST4_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
				AttestationType::Ias,
			),
			Error::<Test>::EnclaveNotAuthorized
		);
	})
}

#[test]
fn register_enclave_prod_fails_with_max_limit_reached() {
	new_test_ext(false).execute_with(|| {
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::Identity,
			TEST4_MRENCLAVE
		));
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::Identity,
			TEST6_MRENCLAVE
		));
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::BitAcross,
			TEST4_MRENCLAVE
		));
		assert_ok!(Teebag::force_add_authorized_enclave(
			RuntimeOrigin::signed(alice()),
			WorkerType::BitAcross,
			TEST6_MRENCLAVE
		));

		let signer4: AccountId32 = get_signer(TEST4_SIGNER_PUB);
		let signer6: AccountId32 = get_signer(TEST6_SIGNER_PUB);

		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			WorkerType::BitAcross,
			Default::default(),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));

		Timestamp::set_timestamp(TEST6_TIMESTAMP);
		assert_noop!(
			Teebag::register_enclave(
				RuntimeOrigin::signed(signer6.clone()),
				WorkerType::BitAcross,
				Default::default(),
				TEST6_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
				AttestationType::Ias,
			),
			Error::<Test>::MaxEnclaveIdentifierOverflow
		);

		// re-register them as WorkerType::Identity is not allowed
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		assert_noop!(
			Teebag::register_enclave(
				RuntimeOrigin::signed(signer4.clone()),
				WorkerType::Identity,
				Default::default(),
				TEST4_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
				AttestationType::Ias,
			),
			Error::<Test>::UnexpectedWorkerType
		);

		// remove and re-register it should work
		assert_ok!(Teebag::force_remove_enclave(RuntimeOrigin::signed(alice()), signer4.clone(),));

		assert_ok!(Teebag::register_enclave(
			RuntimeOrigin::signed(signer4),
			WorkerType::Identity,
			Default::default(),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
			AttestationType::Ias,
		));

		Timestamp::set_timestamp(TEST6_TIMESTAMP);
		assert_noop!(
			Teebag::register_enclave(
				RuntimeOrigin::signed(signer6),
				WorkerType::Identity,
				Default::default(),
				TEST6_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
				AttestationType::Ias,
			),
			Error::<Test>::MaxEnclaveIdentifierOverflow
		);

		assert_eq!(Teebag::enclave_count(WorkerType::Identity), 1);
		assert_eq!(Teebag::enclave_count(WorkerType::BitAcross), 0);
	})
}

#[test]
fn register_quoting_enclave_works() {
	new_test_ext(false).execute_with(|| {
		let qe = Teebag::quoting_enclave_registry();
		assert_eq!(qe.mrsigner, [0u8; 32]);
		assert_eq!(qe.isvprodid, 0);
		Timestamp::set_timestamp(VALID_TIMESTAMP);
		register_quoting_enclave();
		let qe = Teebag::quoting_enclave_registry();
		assert_eq!(qe.isvprodid, 1);
	})
}

#[test]
fn register_tcb_info_works() {
	new_test_ext(false).execute_with(|| {
		Timestamp::set_timestamp(VALID_TIMESTAMP);

		register_tcb_info();
		let fmspc = hex!("00906EA10000");
		let tcb_info = Teebag::tcb_info(fmspc);
		// This is the date that the is registered in register_tcb_info and represents the date
		// 2023-04-16T12:45:32Z
		assert_eq!(tcb_info.next_update, 1681649132000);
	})
}
