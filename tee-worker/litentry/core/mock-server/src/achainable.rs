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
#![allow(opaque_hidden_inferred_bound)]

use lc_data_providers::achainable::{
	ReqBody, ToAchainable, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsTotalTxs,
};
use litentry_primitives::SupportedNetwork;
use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("latest" / "achainable"))
		.and(warp::query::<HashMap<String, String>>())
		.map(|p: HashMap<String, String>| {
			let default = String::default();
			let query = p.get("query").unwrap_or(&default);

			let expected_query_total_txs = VerifiedCredentialsTotalTxs::new(
				vec!["EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg".to_string()],
				vec![SupportedNetwork::Kusama, SupportedNetwork::Polkadot],
			)
			.to_achainable();

			let expected_query_is_hodler = VerifiedCredentialsIsHodlerIn::new(
				vec![
					"0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad".to_string(),
					"0x3394caf8e5ccaffb936e6407599543af46525e0b".to_string(),
				],
				"2022-10-16T00:00:00Z".to_string(),
				SupportedNetwork::Ethereum,
				"0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723".to_string(),
				"0.00000056".into(),
			)
			.to_achainable();

			if query == &expected_query_total_txs {
				let body = r#"
{
	"data": {
		"kusama": [
			{
				"address": "EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg",
				"totalTransactions": 42
			}
		],
		"polkadot": [
			{
				"address": "EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg",
				"totalTransactions": 0
			}
		]
	}
}"#;
				Response::builder().body(body.to_string())
			} else if query == &expected_query_is_hodler {
				let body = r#"
{
  "data": {
    "VerifiedCredentialsIsHodler": [
      {
        "isHodler": false,
        "address": "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad"
      },
      {
        "isHodler": false,
        "address": "0x2eD157cd084Cee5861BdCC773c89881DdA373693"
      }
    ]
  }
}"#;
				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn fresh_account(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "1de85e1d215868788dfc91a9f04d7afd"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn og_account(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "8a6e26b90dee869634215683ea2dad0d"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn class_of_2020(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "9343efca78222a4fad82c635ab697ca0"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn class_of_2021(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "6808c28c26908eb695f63b089cfdae80"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn class_of_2022(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "a4ee0c9e44cbc7b8a4b2074b3b8fb912"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn found_on_bsc(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "3ace29836b372ae66a218dec16e37b62"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn is_polkadot_validator(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "eb66927e8f56fd7f9a8917d380e6100d"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"17bR6rzVsVrzVJS1hM4dSJU43z2MUmz7ZDpPLh8y2fqVg7m".to_string()
			{
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn is_kusama_validator(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "a0d213ff009e43b4ecd0cae67bbabae9"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"ESRBbWstgpPV1pVBsqjMo717rA8HLrtQvEUVwAGeFZyKcia".to_string()
			{
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn polkadot_dolphin(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "6a0424e7544696a3e774dfc7e260dd6e"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP".to_string()
			{
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn kusama_dolphin(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "3e226ee1bfb0d33564efe7f28f5015bd"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string()
			{
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn polkadot_whale(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "68390df24e8ac5d0984a8e9c0725a964"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP".to_string()
			{
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn kusama_whale(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "2bf33f5b3ae60293bf93784b80251129"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string()
			{
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn less_than_10_eth_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "fee8171e2001d1605e018c74f64352da"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn less_than_10_lit_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "4a35e107005f1ea4077f119c10d18503"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x2A038e100F8B85DF21e4d44121bdBfE0c288A869".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn not_less_than_100_eth_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "8657c801983aed40012e387900d75726"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn between_10_to_100_eth_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "e4724ad5b7354ef85332887ee7852800"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x082aB5505CdeA46caeF670754E962830Aa49ED2C".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn eth_millionaire(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "83d16c4c31c55ae535472643e63f49ce"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn eth2_validator_eligible(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "53b54e51090a3663173c2a97039ebf69"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn not_less_than_100_weth_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "f55a4c5a19b6817ad4faf90385f4df6e"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn not_less_than_100_lit_bep20_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "0f26a13d7ff182641f9bb9168a3f1d84"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn native_lit_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "3f0469170cd271ebaac4ed2c92754479"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn erc20_lit_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "7bf72e9190098776817afa763044ac1b"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string() {
				let body = r#"
			{
				"result": true
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn bep20_lit_holder(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("v1" / "run" / "label" / "0dc166e3b588fb45a9cca36c60c61f79"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.map(|body: ReqBody| {
			if body.params.address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string() {
				let body = r#"
			{
				"result": false
			}"#;

				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}
