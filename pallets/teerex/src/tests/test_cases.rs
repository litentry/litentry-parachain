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

use crate::{
	mock::*, Enclave, EnclaveRegistry, Error, Event as TeerexEvent, ExecutedCalls, Request,
	ShardIdentifier, DATA_LENGTH_LIMIT,
};
use frame_support::{assert_err, assert_ok};
use hex_literal::hex;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use teerex_primitives::SgxBuildMode;
use test_utils::ias::consts::*;

fn list_enclaves() -> Vec<(u64, Enclave<AccountId, Vec<u8>>)> {
	<EnclaveRegistry<Test>>::iter().collect::<Vec<(u64, Enclave<AccountId, Vec<u8>>)>>()
}

// give get_signer a concrete type
fn get_signer(pubkey: &[u8; 32]) -> AccountId {
	test_utils::get_signer(pubkey)
}

/// Timestamp for which the collateral data must be valid. Represents 2022-12-21 08:12:27
const VALID_TIMESTAMP: Moment = 1671606747000;

#[test]
fn add_and_remove_dcap_enclave_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(VALID_TIMESTAMP);
		register_quoting_enclave();
		register_tcb_info();

		let pubkey: [u8; 32] = [
			65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20,
			55, 147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
		];
		let signer = get_signer(&pubkey);
		assert_ok!(Teerex::register_dcap_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST1_DCAP_QUOTE.to_vec(),
			URL.to_vec()
		));
		assert_eq!(Teerex::enclave_count(), 1);
		assert_eq!(Teerex::enclave(1).unwrap().timestamp, VALID_TIMESTAMP);
		assert_ok!(Teerex::unregister_enclave(RuntimeOrigin::signed(signer)));
		assert_eq!(Teerex::enclave_count(), 0);
		assert_eq!(list_enclaves(), vec![])
	})
}

fn register_quoting_enclave() {
	let quoting_enclave = br#"{"id":"QE","version":2,"issueDate":"2022-12-04T22:45:33Z","nextUpdate":"2023-01-03T22:45:33Z","tcbEvaluationDataNumber":13,"miscselect":"00000000","miscselectMask":"FFFFFFFF","attributes":"11000000000000000000000000000000","attributesMask":"FBFFFFFFFFFFFFFF0000000000000000","mrsigner":"8C4F5775D796503E96137F77C68A829A0056AC8DED70140B081B094490C57BFF","isvprodid":1,"tcbLevels":[{"tcb":{"isvsvn":6},"tcbDate":"2022-11-09T00:00:00Z","tcbStatus":"UpToDate"},{"tcb":{"isvsvn":5},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00477"]},{"tcb":{"isvsvn":4},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00334","INTEL-SA-00477"]},{"tcb":{"isvsvn":2},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00219","INTEL-SA-00293","INTEL-SA-00334","INTEL-SA-00477"]},{"tcb":{"isvsvn":1},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00202","INTEL-SA-00219","INTEL-SA-00293","INTEL-SA-00334","INTEL-SA-00477"]}]}"#;
	let signature = hex!("47accba321e57c20722a0d3d1db11c9b52661239857dc578ca1bde13976ee288cf39f72111ffe445c7389ef56447c79e30e6b83a8863ed9880de5bde4a8d5c91");
	let certificate_chain =
		include_bytes!("../../sgx-verify/test/dcap/qe_identity_issuer_chain.pem");

	let pubkey: [u8; 32] = [
		65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20, 55,
		147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
	];
	let signer = get_signer(&pubkey);
	assert_ok!(Teerex::register_quoting_enclave(
		RuntimeOrigin::signed(signer.clone()),
		quoting_enclave.to_vec(),
		signature.to_vec(),
		certificate_chain.to_vec(),
	));
}

#[test]
fn register_quoting_enclave_works() {
	new_test_ext().execute_with(|| {
		let qe = Teerex::quoting_enclave();
		assert_eq!(qe.mrsigner, [0u8; 32]);
		assert_eq!(qe.isvprodid, 0);
		Timestamp::set_timestamp(VALID_TIMESTAMP);
		register_quoting_enclave();
		let qe = Teerex::quoting_enclave();
		assert_eq!(qe.isvprodid, 1);
	})
}

fn register_tcb_info() {
	let tcb_info = br#"{"id":"SGX","version":3,"issueDate":"2022-11-17T12:45:32Z","nextUpdate":"2023-04-16T12:45:32Z","fmspc":"00906EA10000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":12,"tcbLevels":[{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":11},"tcbDate":"2021-11-10T00:00:00Z","tcbStatus":"SWHardeningNeeded","advisoryIDs":["INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":11},"tcbDate":"2021-11-10T00:00:00Z","tcbStatus":"ConfigurationAndSWHardeningNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":17},{"svn":17},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-11-11T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00477","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":15},{"svn":15},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-06-10T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":15},{"svn":15},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2020-06-10T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":14},{"svn":14},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":7},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2019-12-11T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":14},{"svn":14},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":10},"tcbDate":"2019-12-11T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":13},{"svn":13},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":3},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":9},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":13},{"svn":13},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":9},"tcbDate":"2019-11-13T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":6},{"svn":6},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00161","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":6},{"svn":6},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-05-15T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-01-09T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00233","INTEL-SA-00161","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":6},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":7},"tcbDate":"2019-01-09T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":5},{"svn":5},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":6},"tcbDate":"2018-08-15T00:00:00Z","tcbStatus":"OutOfDateConfigurationNeeded","advisoryIDs":["INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":4},{"svn":4},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":5},"tcbDate":"2018-01-04T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00106","INTEL-SA-00115","INTEL-SA-00135","INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]},{"tcb":{"sgxtcbcomponents":[{"svn":2},{"svn":2},{"svn":2},{"svn":4},{"svn":1},{"svn":128},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":4},"tcbDate":"2017-07-26T00:00:00Z","tcbStatus":"OutOfDate","advisoryIDs":["INTEL-SA-00088","INTEL-SA-00106","INTEL-SA-00115","INTEL-SA-00135","INTEL-SA-00203","INTEL-SA-00161","INTEL-SA-00233","INTEL-SA-00220","INTEL-SA-00270","INTEL-SA-00293","INTEL-SA-00219","INTEL-SA-00289","INTEL-SA-00320","INTEL-SA-00329","INTEL-SA-00381","INTEL-SA-00389","INTEL-SA-00477","INTEL-SA-00334"]}]}"#;
	let signature = hex!("71746f2148ecba04e35cf1ac77a7e6267ce99f6781c1031f724bb5bd94b8c1b6e4c07c01dc151692aa75be80dfba7350bb80c58314a6975189597e28e9bbc75c");
	let certificate_chain = include_bytes!("../../sgx-verify/test/dcap/tcb_info_issuer_chain.pem");

	let pubkey: [u8; 32] = [
		65, 89, 193, 118, 86, 172, 17, 149, 206, 160, 174, 75, 219, 151, 51, 235, 110, 135, 20, 55,
		147, 162, 106, 110, 143, 207, 57, 64, 67, 63, 203, 95,
	];
	let signer = get_signer(&pubkey);
	assert_ok!(Teerex::register_tcb_info(
		RuntimeOrigin::signed(signer.clone()),
		tcb_info.to_vec(),
		signature.to_vec(),
		certificate_chain.to_vec(),
	));
}

#[test]
fn register_tcb_info_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(VALID_TIMESTAMP);

		register_tcb_info();
		let fmspc = hex!("00906EA10000");
		let tcb_info = Teerex::tcb_info(fmspc);
		// This is the date that the is registered in register_tcb_info and represents the date
		// 2023-04-16T12:45:32Z
		assert_eq!(tcb_info.next_update, 1681649132000);
	})
}

#[test]
fn add_enclave_works() {
	new_test_ext().execute_with(|| {
		// set the now in the runtime such that the remote attestation reports are within accepted
		// range (24h)
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
	})
}

#[test]
fn add_and_remove_enclave_works() {
	new_test_ext().execute_with(|| {
		env_logger::init();
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		assert_ok!(Teerex::unregister_enclave(RuntimeOrigin::signed(signer)));
		assert_eq!(Teerex::enclave_count(), 0);
		assert_eq!(list_enclaves(), vec![])
	})
}

#[test]
fn add_enclave_without_timestamp_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(0);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		)
		.is_err());
		assert_eq!(Teerex::enclave_count(), 0);
	})
}

#[test]
fn list_enclaves_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		let e_1: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer.clone(),
			mr_enclave: TEST4_MRENCLAVE,
			timestamp: TEST4_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		let enclaves = list_enclaves();
		assert_eq!(enclaves[0].1.pubkey, signer);
		assert!(enclaves.contains(&(1, e_1)));
	})
}

#[test]
fn remove_middle_enclave_works() {
	new_test_ext().execute_with(|| {
		// use the newest timestamp, is as now such that all reports are valid
		Timestamp::set_timestamp(TEST7_TIMESTAMP);

		let signer5 = get_signer(TEST5_SIGNER_PUB);
		let signer6 = get_signer(TEST6_SIGNER_PUB);
		let signer7 = get_signer(TEST7_SIGNER_PUB);

		// add enclave 1
		let e_1: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer5.clone(),
			mr_enclave: TEST5_MRENCLAVE,
			timestamp: TEST5_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		let e_2: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer6.clone(),
			mr_enclave: TEST6_MRENCLAVE,
			timestamp: TEST6_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		let e_3: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer7.clone(),
			mr_enclave: TEST7_MRENCLAVE,
			timestamp: TEST7_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer5),
			TEST5_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		assert_eq!(list_enclaves(), vec![(1, e_1.clone())]);

		// add enclave 2
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer6.clone()),
			TEST6_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 2);
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_1.clone())));
		assert!(enclaves.contains(&(2, e_2.clone())));

		// add enclave 3
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer7),
			TEST7_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 3);
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_1.clone())));
		assert!(enclaves.contains(&(2, e_2)));
		assert!(enclaves.contains(&(3, e_3.clone())));

		// remove enclave 2
		assert_ok!(Teerex::unregister_enclave(RuntimeOrigin::signed(signer6)));
		assert_eq!(Teerex::enclave_count(), 2);
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_1)));
		assert!(enclaves.contains(&(2, e_3)));
	})
}

#[test]
fn register_enclave_with_different_signer_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST7_SIGNER_PUB);
		assert_err!(
			Teerex::register_enclave(
				RuntimeOrigin::signed(signer),
				TEST5_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
			),
			Error::<Test>::SenderIsNotAttestedEnclave
		);
	})
}

#[test]
fn register_enclave_with_to_old_attestation_report_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP + TWENTY_FOUR_HOURS + 1);
		let signer = get_signer(TEST7_SIGNER_PUB);
		assert_err!(
			Teerex::register_enclave(
				RuntimeOrigin::signed(signer),
				TEST7_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
			),
			Error::<Test>::RemoteAttestationTooOld
		);
	})
}

#[test]
fn register_enclave_with_almost_too_old_report_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP + TWENTY_FOUR_HOURS - 1);
		let signer = get_signer(TEST7_SIGNER_PUB);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer),
			TEST7_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
	})
}

#[test]
fn update_enclave_url_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);

		let signer = get_signer(TEST4_SIGNER_PUB);
		let url2 = "my fancy url".as_bytes();
		let _e_1: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer.clone(),
			mr_enclave: TEST4_MRENCLAVE,
			timestamp: TEST4_TIMESTAMP,
			url: url2.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave(1).unwrap().url, URL.to_vec());

		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			url2.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave(1).unwrap().url, url2.to_vec());
		let enclaves = list_enclaves();
		assert_eq!(enclaves[0].1.pubkey, signer)
	})
}

#[test]
fn update_ipfs_hash_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let block_hash = H256::default();
		let merkle_root = H256::default();
		let block_number = 3;
		let signer = get_signer(TEST4_SIGNER_PUB);

		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		assert_ok!(Teerex::confirm_processed_parentchain_block(
			RuntimeOrigin::signed(signer.clone()),
			block_hash,
			block_number,
			merkle_root,
		));

		let expected_event = RuntimeEvent::Teerex(TeerexEvent::ProcessedParentchainBlock(
			signer,
			block_hash,
			merkle_root,
			block_number,
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn ipfs_update_from_unregistered_enclave_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_err!(
			Teerex::confirm_processed_parentchain_block(
				RuntimeOrigin::signed(signer),
				H256::default(),
				3,
				H256::default(),
			),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn call_worker_works() {
	new_test_ext().execute_with(|| {
		let req = Request { shard: ShardIdentifier::default(), cyphertext: vec![0u8, 1, 2, 3, 4] };
		// don't care who signs
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert!(Teerex::call_worker(RuntimeOrigin::signed(signer), req.clone()).is_ok());
		let expected_event = RuntimeEvent::Teerex(TeerexEvent::Forwarded(req.shard));
		println!("events:{:?}", System::events());
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn unshield_is_only_executed_once_for_the_same_call_hash() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		let call_hash: H256 = H256::from([1u8; 32]);
		let bonding_account = get_signer(&TEST4_MRENCLAVE);

		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		assert_ok!(Balances::transfer(
			RuntimeOrigin::signed(AccountKeyring::Alice.to_account_id()),
			bonding_account.clone(),
			1 << 50
		));

		assert!(Teerex::unshield_funds(
			RuntimeOrigin::signed(signer.clone()),
			AccountKeyring::Alice.to_account_id(),
			50,
			bonding_account.clone(),
			call_hash
		)
		.is_ok());

		assert!(Teerex::unshield_funds(
			RuntimeOrigin::signed(signer),
			AccountKeyring::Alice.to_account_id(),
			50,
			bonding_account,
			call_hash
		)
		.is_ok());

		assert_eq!(<ExecutedCalls<Test>>::get(call_hash), 2)
	})
}
#[test]
fn timestamp_callback_works() {
	new_test_ext().execute_with(|| {
		set_timestamp(TEST7_TIMESTAMP);

		let signer5 = get_signer(TEST5_SIGNER_PUB);
		let signer6 = get_signer(TEST6_SIGNER_PUB);
		let signer7 = get_signer(TEST7_SIGNER_PUB);

		// add enclave 1
		let e_2: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer6.clone(),
			mr_enclave: TEST6_MRENCLAVE,
			timestamp: TEST6_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		let e_3: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer7.clone(),
			mr_enclave: TEST7_MRENCLAVE,
			timestamp: TEST7_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		//Register 3 enclaves: 5, 6 ,7
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer5.clone()),
			TEST5_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer6.clone()),
			TEST6_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer7.clone()),
			TEST7_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 3);

		//enclave 5 silent since 49h -> unregistered
		run_to_block(2);
		set_timestamp(TEST5_TIMESTAMP + 2 * TWENTY_FOUR_HOURS + 1);

		let expected_event = RuntimeEvent::Teerex(TeerexEvent::RemovedEnclave(signer5));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teerex::enclave_count(), 2);
		//2 and 3 are still there. 3 and 1 were swapped -> 3 and 2
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_3)));
		assert!(enclaves.contains(&(2, e_2)));

		run_to_block(3);
		//enclave 6 and 7 still registered: not long enough silent
		set_timestamp(TEST6_TIMESTAMP + 2 * TWENTY_FOUR_HOURS);
		assert_eq!(Teerex::enclave_count(), 2);

		//unregister 6 to generate an error next call of callbakc
		assert_ok!(Teerex::unregister_enclave(RuntimeOrigin::signed(signer6.clone())));
		let expected_event = RuntimeEvent::Teerex(TeerexEvent::RemovedEnclave(signer6));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teerex::enclave_count(), 1);

		//enclave 6 and 7 silent since TWENTY_FOUR_HOURS + 1 -> unregistered
		run_to_block(4);
		set_timestamp(TEST7_TIMESTAMP + 2 * TWENTY_FOUR_HOURS + 1);
		let expected_event = RuntimeEvent::Teerex(TeerexEvent::RemovedEnclave(signer7));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teerex::enclave_count(), 0);
	})
}

#[test]
fn debug_mode_enclave_attest_works_when_sgx_debug_mode_is_allowed() {
	new_test_ext().execute_with(|| {
		set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		let e_0: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer4.clone(),
			mr_enclave: TEST4_MRENCLAVE,
			timestamp: TEST4_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Debug,
		};

		//Register an enclave compiled in debug mode
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_0)));
	})
}

#[test]
fn production_mode_enclave_attest_works_when_sgx_debug_mode_is_allowed() {
	new_test_ext().execute_with(|| {
		new_test_ext().execute_with(|| {
			set_timestamp(TEST8_TIMESTAMP);
			let signer8 = get_signer(TEST8_SIGNER_PUB);
			let e_0: Enclave<AccountId, Vec<u8>> = Enclave {
				pubkey: signer8.clone(),
				mr_enclave: TEST8_MRENCLAVE,
				timestamp: TEST8_TIMESTAMP,
				url: URL.to_vec(),
				shielding_key: None,
				vc_pubkey: None,
				sgx_mode: SgxBuildMode::Production,
			};

			//Register an enclave compiled in production mode
			assert_ok!(Teerex::register_enclave(
				RuntimeOrigin::signed(signer8),
				TEST8_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
			));
			assert_eq!(Teerex::enclave_count(), 1);
			let enclaves = list_enclaves();
			assert!(enclaves.contains(&(1, e_0)));
		})
	})
}

#[test]
fn debug_mode_enclave_attest_fails_when_sgx_debug_mode_not_allowed() {
	new_test_production_ext().execute_with(|| {
		set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		//Try to register an enclave compiled in debug mode
		assert_err!(
			Teerex::register_enclave(
				RuntimeOrigin::signed(signer4),
				TEST4_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
			),
			Error::<Test>::SgxModeNotAllowed
		);
		assert_eq!(Teerex::enclave_count(), 0);
	})
}
#[test]
fn production_mode_enclave_attest_works_when_sgx_debug_mode_not_allowed() {
	new_test_production_ext().execute_with(|| {
		set_timestamp(TEST8_TIMESTAMP);
		let signer8 = get_signer(TEST8_SIGNER_PUB);
		let e_0: Enclave<AccountId, Vec<u8>> = Enclave {
			pubkey: signer8.clone(),
			mr_enclave: TEST8_MRENCLAVE,
			timestamp: TEST8_TIMESTAMP,
			url: URL.to_vec(),
			shielding_key: None,
			vc_pubkey: None,
			sgx_mode: SgxBuildMode::Production,
		};

		//Register an enclave compiled in production mode
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer8),
			TEST8_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);
		let enclaves = list_enclaves();
		assert!(enclaves.contains(&(1, e_0)));
	})
}

#[test]
fn verify_unshield_funds_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		let call_hash: H256 = H256::from([1u8; 32]);
		let bonding_account = get_signer(&TEST4_MRENCLAVE);
		let incognito_account = INCOGNITO_ACCOUNT.to_vec();

		//Register enclave
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);

		assert!(Teerex::shield_funds(
			RuntimeOrigin::signed(AccountKeyring::Alice.to_account_id()),
			incognito_account.clone(),
			100,
			bonding_account.clone(),
		)
		.is_ok());

		assert_eq!(Balances::free_balance(bonding_account.clone()), 100);

		let expected_event = RuntimeEvent::Teerex(TeerexEvent::ShieldFunds(incognito_account));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		assert!(Teerex::unshield_funds(
			RuntimeOrigin::signed(signer4),
			AccountKeyring::Alice.to_account_id(),
			50,
			bonding_account.clone(),
			call_hash
		)
		.is_ok());
		assert_eq!(Balances::free_balance(bonding_account), 50);

		let expected_event = RuntimeEvent::Teerex(TeerexEvent::UnshieldedFunds(
			AccountKeyring::Alice.to_account_id(),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn unshield_funds_from_not_registered_enclave_errs() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		let call_hash: H256 = H256::from([1u8; 32]);

		assert_eq!(Teerex::enclave_count(), 0);

		assert_err!(
			Teerex::unshield_funds(
				RuntimeOrigin::signed(signer4.clone()),
				AccountKeyring::Alice.to_account_id(),
				51,
				signer4,
				call_hash
			),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn unshield_funds_from_enclave_neq_bonding_account_errs() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		let call_hash: H256 = H256::from([1u8; 32]);
		let bonding_account = get_signer(&TEST4_MRENCLAVE);
		let incognito_account = INCOGNITO_ACCOUNT;
		let not_bonding_account = get_signer(&TEST7_MRENCLAVE);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		//Ensure that bonding account has funds
		assert!(Teerex::shield_funds(
			RuntimeOrigin::signed(AccountKeyring::Alice.to_account_id()),
			incognito_account.to_vec(),
			100,
			bonding_account.clone(),
		)
		.is_ok());

		assert!(Teerex::shield_funds(
			RuntimeOrigin::signed(AccountKeyring::Alice.to_account_id()),
			incognito_account.to_vec(),
			50,
			not_bonding_account.clone(),
		)
		.is_ok());

		assert_err!(
			Teerex::unshield_funds(
				RuntimeOrigin::signed(signer4),
				AccountKeyring::Alice.to_account_id(),
				50,
				not_bonding_account.clone(),
				call_hash
			),
			Error::<Test>::WrongMrenclaveForBondingAccount
		);

		assert_eq!(Balances::free_balance(bonding_account), 100);
		assert_eq!(Balances::free_balance(not_bonding_account), 50);
	})
}

#[test]
fn confirm_processed_parentchain_block_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST7_TIMESTAMP);
		let block_hash = H256::default();
		let merkle_root = H256::default();
		let block_number = 3;
		let signer7 = get_signer(TEST7_SIGNER_PUB);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer7.clone()),
			TEST7_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_eq!(Teerex::enclave_count(), 1);

		let enclaves = list_enclaves();
		let old_timestamp = enclaves[0].1.timestamp;

		assert_ok!(Teerex::confirm_processed_parentchain_block(
			RuntimeOrigin::signed(signer7.clone()),
			block_hash,
			block_number,
			merkle_root,
		));

		let enclaves = list_enclaves();
		let new_timestamp = enclaves[0].1.timestamp;
		assert_ne!(old_timestamp, new_timestamp);

		let expected_event = RuntimeEvent::Teerex(TeerexEvent::ProcessedParentchainBlock(
			signer7,
			block_hash,
			merkle_root,
			block_number,
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
	})
}

#[test]
fn ensure_registered_enclave_works() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);
		let signer6 = get_signer(TEST6_SIGNER_PUB);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));
		assert_ok!(Teerex::ensure_registered_enclave(&signer4));
		assert_err!(
			Teerex::ensure_registered_enclave(&signer6),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn publish_hash_works() {
	use frame_system::{EventRecord, Phase};

	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		// There are no events emitted at the genesis block.
		System::set_block_number(1);
		System::reset_events();

		let hash = H256::from([1u8; 32]);
		let extra_topics = vec![H256::from([2u8; 32]), H256::from([3u8; 32])];
		let data = b"hello world".to_vec();

		// publish with extra topics and data
		assert_ok!(Teerex::publish_hash(
			RuntimeOrigin::signed(signer4.clone()),
			hash,
			extra_topics.clone(),
			data.clone()
		));

		// publish without extra topics and data
		assert_ok!(Teerex::publish_hash(
			RuntimeOrigin::signed(signer4.clone()),
			hash,
			vec![],
			vec![]
		));

		let mr_enclave = Teerex::get_enclave(&signer4).unwrap().mr_enclave;
		let mut topics = extra_topics;
		topics.push(mr_enclave.into());

		// Check that topics are reflected in the event record.
		assert_eq!(
			System::events(),
			vec![
				EventRecord {
					phase: Phase::Initialization,
					event: TeerexEvent::PublishedHash { mr_enclave, hash, data }.into(),
					topics,
				},
				EventRecord {
					phase: Phase::Initialization,
					event: TeerexEvent::PublishedHash { mr_enclave, hash, data: vec![] }.into(),
					topics: vec![mr_enclave.into()],
				},
			]
		);
	})
}

#[test]
fn publish_hash_with_unregistered_enclave_fails() {
	new_test_ext().execute_with(|| {
		let signer4 = get_signer(TEST4_SIGNER_PUB);

		assert_err!(
			Teerex::publish_hash(RuntimeOrigin::signed(signer4), [1u8; 32].into(), vec![], vec![]),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn publish_hash_with_too_many_topics_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		let hash = H256::from([1u8; 32]);
		let extra_topics = vec![
			H256::from([0u8; 32]),
			H256::from([1u8; 32]),
			H256::from([2u8; 32]),
			H256::from([3u8; 32]),
			H256::from([4u8; 32]),
			H256::from([5u8; 32]),
		];

		assert_err!(
			Teerex::publish_hash(RuntimeOrigin::signed(signer4), hash, extra_topics, vec![]),
			Error::<Test>::TooManyTopics
		);
	})
}

#[test]
fn publish_hash_with_too_much_data_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer4 = get_signer(TEST4_SIGNER_PUB);

		//Ensure that enclave is registered
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer4.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		let hash = H256::from([1u8; 32]);
		let data = vec![0u8; DATA_LENGTH_LIMIT + 1];

		assert_err!(
			Teerex::publish_hash(RuntimeOrigin::signed(signer4), hash, vec![], data),
			Error::<Test>::DataTooLong
		);
	})
}
