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
#![allow(dead_code, unused_imports, const_item_mutation)]

use crate::{mock::*, ExchangeRates};
use frame_support::{assert_err, assert_noop, assert_ok};
use hex_literal::hex;
use pallet_teerex::Error;
use sp_runtime::DispatchError::BadOrigin;
use substrate_fixed::types::U32F32;
use teeracle_primitives::*;
use test_utils::ias::consts::{
	TEST4_CERT, TEST4_MRENCLAVE, TEST4_SIGNER_PUB, TEST4_TIMESTAMP, TEST5_MRENCLAVE,
	TEST5_SIGNER_PUB, TEST8_MRENCLAVE, URL,
};

const COINGECKO_SRC: &str = "https://api.coingecko.com";
const COINMARKETCAP_SRC: &str = "https://coinmarketcap.com/";

const DOT_USD_TRADING_PAIR: &str = "DOT/USD";

// give get_signer a concrete type
fn get_signer(pubkey: &[u8; 32]) -> AccountId {
	test_utils::get_signer(pubkey)
}

fn register_enclave_and_add_oracle_to_whitelist_ok(src: &str) {
	Timestamp::set_timestamp(TEST4_TIMESTAMP);
	let signer = get_signer(TEST4_SIGNER_PUB);
	assert_ok!(Teerex::register_enclave(
		RuntimeOrigin::signed(signer),
		TEST4_CERT.to_vec(),
		URL.to_vec(),
		None,
		None,
	));
	let mrenclave = Teerex::enclave(1).unwrap().mr_enclave;
	assert_ok!(Teeracle::add_to_whitelist(RuntimeOrigin::root(), src.to_owned(), mrenclave));
}

fn update_exchange_rate_dot_dollars_ok(src: &str, rate: Option<U32F32>) {
	let signer = get_signer(TEST4_SIGNER_PUB);
	assert_ok!(Teeracle::update_exchange_rate(
		RuntimeOrigin::signed(signer),
		src.to_owned(),
		DOT_USD_TRADING_PAIR.to_owned(),
		rate
	));
}

#[test]
fn update_exchange_rate_works() {
	new_test_ext().execute_with(|| {
		register_enclave_and_add_oracle_to_whitelist_ok(COINGECKO_SRC);

		let rate = U32F32::from_num(43.65);
		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, Some(rate));
		let expected_event = RuntimeEvent::Teeracle(crate::Event::ExchangeRateUpdated(
			COINGECKO_SRC.to_owned(),
			DOT_USD_TRADING_PAIR.to_owned(),
			Some(rate),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(
			Teeracle::exchange_rate(DOT_USD_TRADING_PAIR.to_owned(), COINGECKO_SRC.to_owned()),
			rate
		);

		let rate2 = U32F32::from_num(4294967295.65);
		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, Some(rate2));
		assert_eq!(
			Teeracle::exchange_rate(DOT_USD_TRADING_PAIR.to_owned(), COINGECKO_SRC.to_owned()),
			rate2
		);
	})
}

#[test]
fn update_oracle_works() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST4_SIGNER_PUB);
		register_enclave_and_add_oracle_to_whitelist_ok(&DataSource::from("Test_Source_Name"));
		let oracle_blob: crate::OracleDataBlob<Test> =
			vec![1].try_into().expect("Can Convert to BoundedVec; QED");
		assert_ok!(Teeracle::update_oracle(
			RuntimeOrigin::signed(signer),
			OracleDataName::from("Test_Oracle_Name"),
			DataSource::from("Test_Source_Name"),
			oracle_blob.clone()
		),);
		let expected_event = RuntimeEvent::Teeracle(crate::Event::OracleUpdated(
			OracleDataName::from("Test_Oracle_Name"),
			DataSource::from("Test_Source_Name"),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));

		assert_eq!(
			Teeracle::oracle_data(
				OracleDataName::from("Test_Oracle_Name"),
				DataSource::from("Test_Source_Name")
			),
			oracle_blob
		);
	})
}

#[test]
fn get_existing_exchange_rate_works() {
	new_test_ext().execute_with(|| {
		let rate = U32F32::from_num(43.65);
		register_enclave_and_add_oracle_to_whitelist_ok(COINGECKO_SRC);
		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, Some(rate));
		assert_eq!(
			Teeracle::exchange_rate(DOT_USD_TRADING_PAIR.to_owned(), COINGECKO_SRC.to_owned()),
			rate
		);
	})
}

#[test]
fn get_inexisting_exchange_rate_is_zero() {
	new_test_ext().execute_with(|| {
		assert!(!ExchangeRates::<Test>::contains_key(
			DOT_USD_TRADING_PAIR.to_owned(),
			COINGECKO_SRC.to_owned()
		));
		assert_eq!(
			Teeracle::exchange_rate(DOT_USD_TRADING_PAIR.to_owned(), COINGECKO_SRC.to_owned()),
			U32F32::from_num(0)
		);
	})
}

#[test]
fn update_exchange_rate_to_none_delete_exchange_rate() {
	new_test_ext().execute_with(|| {
		register_enclave_and_add_oracle_to_whitelist_ok(COINGECKO_SRC);
		let rate = U32F32::from_num(43.65);
		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, Some(rate));

		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, None);

		let expected_event = RuntimeEvent::Teeracle(crate::Event::ExchangeRateDeleted(
			COINGECKO_SRC.to_owned(),
			DOT_USD_TRADING_PAIR.to_owned(),
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert!(!ExchangeRates::<Test>::contains_key(
			DOT_USD_TRADING_PAIR.to_owned(),
			COINGECKO_SRC.to_owned()
		));
	})
}

#[test]
fn update_exchange_rate_to_zero_delete_exchange_rate() {
	new_test_ext().execute_with(|| {
		register_enclave_and_add_oracle_to_whitelist_ok(COINGECKO_SRC);
		let rate = Some(U32F32::from_num(43.65));
		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, rate);

		update_exchange_rate_dot_dollars_ok(COINGECKO_SRC, Some(U32F32::from_num(0)));

		let expected_event = RuntimeEvent::Teeracle(crate::Event::ExchangeRateDeleted(
			COINGECKO_SRC.to_owned(),
			DOT_USD_TRADING_PAIR.to_owned(),
		));

		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert!(!ExchangeRates::<Test>::contains_key(
			DOT_USD_TRADING_PAIR.to_owned(),
			COINGECKO_SRC.to_owned()
		));
	})
}

#[test]
fn update_exchange_rate_from_not_registered_enclave_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST4_SIGNER_PUB);
		let rate = U32F32::from_num(43.65);
		assert_err!(
			Teeracle::update_exchange_rate(
				RuntimeOrigin::signed(signer),
				COINGECKO_SRC.to_owned(),
				DOT_USD_TRADING_PAIR.to_owned(),
				Some(rate)
			),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn update_oracle_from_not_registered_enclave_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_noop!(
			Teeracle::update_oracle(
				RuntimeOrigin::signed(signer),
				OracleDataName::from("Test_Oracle_Name"),
				DataSource::from("Test_Source_Name"),
				vec![0].try_into().expect("Can Convert to BoundedVec; QED")
			),
			Error::<Test>::EnclaveIsNotRegistered
		);
	})
}

#[test]
fn update_exchange_rate_from_not_whitelisted_oracle_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		let rate = U32F32::from_num(43.65);
		assert_err!(
			Teeracle::update_exchange_rate(
				RuntimeOrigin::signed(signer),
				COINGECKO_SRC.to_owned(),
				DOT_USD_TRADING_PAIR.to_owned(),
				Some(rate)
			),
			crate::Error::<Test>::ReleaseNotWhitelisted
		);
	})
}

#[test]
fn update_oracle_from_not_whitelisted_oracle_fails() {
	new_test_ext().execute_with(|| {
		Timestamp::set_timestamp(TEST4_TIMESTAMP);
		let signer = get_signer(TEST4_SIGNER_PUB);
		assert_ok!(Teerex::register_enclave(
			RuntimeOrigin::signed(signer.clone()),
			TEST4_CERT.to_vec(),
			URL.to_vec(),
			None,
			None,
		));

		assert_noop!(
			Teeracle::update_oracle(
				RuntimeOrigin::signed(signer),
				OracleDataName::from("Test_Oracle_Name"),
				DataSource::from("Test_Source_Name"),
				vec![0].try_into().expect("Can Convert to BoundedVec; QED")
			),
			crate::Error::<Test>::ReleaseNotWhitelisted
		);
	})
}

#[test]
fn update_exchange_rate_with_too_long_trading_pair_fails() {
	new_test_ext().execute_with(|| {
		register_enclave_and_add_oracle_to_whitelist_ok(COINGECKO_SRC);

		let rate = Some(U32F32::from_num(43.65));
		let signer = get_signer(TEST4_SIGNER_PUB);
		let too_long_trading_pair = "123456789_12".to_owned();
		assert_err!(
			Teeracle::update_exchange_rate(
				RuntimeOrigin::signed(signer),
				COINGECKO_SRC.to_owned(),
				too_long_trading_pair,
				rate
			),
			crate::Error::<Test>::TradingPairStringTooLong
		);
	})
}

#[test]
fn add_to_whitelist_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		let expected_event = RuntimeEvent::Teeracle(crate::Event::AddedToWhitelist(
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE,
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 1);
	})
}

#[test]
fn add_mulitple_src_to_whitelists_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINMARKETCAP_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		let expected_event = RuntimeEvent::Teeracle(crate::Event::AddedToWhitelist(
			COINMARKETCAP_SRC.to_owned(),
			TEST4_MRENCLAVE,
		));

		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 1);
		assert_eq!(Teeracle::whitelist(COINMARKETCAP_SRC.to_owned()).len(), 1);
	})
}

#[test]
fn add_two_times_to_whitelist_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_err!(
			Teeracle::add_to_whitelist(
				RuntimeOrigin::root(),
				COINGECKO_SRC.to_owned(),
				TEST4_MRENCLAVE
			),
			crate::Error::<Test>::ReleaseAlreadyWhitelisted
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 1);
	})
}

#[test]
fn add_too_many_oracles_to_whitelist_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST5_MRENCLAVE
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d2")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d3")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d4")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d5")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d6")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d7")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d8")
		));
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d9")
		));
		assert_err!(
			Teeracle::add_to_whitelist(
				RuntimeOrigin::root(),
				COINGECKO_SRC.to_owned(),
				TEST8_MRENCLAVE
			),
			crate::Error::<Test>::ReleaseWhitelistOverflow
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 10);
	})
}
#[test]
fn add_to_whitelist_too_long_source_fails() {
	new_test_ext().execute_with(|| {
		let too_long_source = "123456789_223456789_323456789_423456789_1".to_owned();
		assert_err!(
			Teeracle::add_to_whitelist(RuntimeOrigin::root(), too_long_source, TEST4_MRENCLAVE),
			crate::Error::<Test>::DataSourceStringTooLong
		);
	})
}

#[test]
fn non_root_add_to_whitelist_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST5_SIGNER_PUB);
		assert_err!(
			Teeracle::add_to_whitelist(
				RuntimeOrigin::signed(signer),
				COINGECKO_SRC.to_owned(),
				TEST4_MRENCLAVE
			),
			BadOrigin
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 0);
	})
}

#[test]
fn remove_from_whitelist_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_ok!(Teeracle::remove_from_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		let expected_event = RuntimeEvent::Teeracle(crate::Event::RemovedFromWhitelist(
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE,
		));
		assert!(System::events().iter().any(|a| a.event == expected_event));
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 0);
	})
}

#[test]
fn remove_from_whitelist_not_whitelisted_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_err!(
			Teeracle::remove_from_whitelist(
				RuntimeOrigin::root(),
				COINGECKO_SRC.to_owned(),
				TEST5_MRENCLAVE
			),
			crate::Error::<Test>::ReleaseNotWhitelisted
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 1);
	})
}

#[test]
fn remove_from_empty_whitelist_doesnt_crash() {
	new_test_ext().execute_with(|| {
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 0);
		assert_err!(
			Teeracle::remove_from_whitelist(
				RuntimeOrigin::root(),
				COINGECKO_SRC.to_owned(),
				TEST5_MRENCLAVE
			),
			crate::Error::<Test>::ReleaseNotWhitelisted
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 0);
	})
}

#[test]
fn non_root_remove_from_whitelist_fails() {
	new_test_ext().execute_with(|| {
		let signer = get_signer(TEST5_SIGNER_PUB);
		assert_ok!(Teeracle::add_to_whitelist(
			RuntimeOrigin::root(),
			COINGECKO_SRC.to_owned(),
			TEST4_MRENCLAVE
		));
		assert_err!(
			Teeracle::remove_from_whitelist(
				RuntimeOrigin::signed(signer),
				COINGECKO_SRC.to_owned(),
				TEST4_MRENCLAVE
			),
			BadOrigin
		);
		assert_eq!(Teeracle::whitelist(COINGECKO_SRC.to_owned()).len(), 1);
	})
}
