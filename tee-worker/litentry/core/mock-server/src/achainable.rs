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
