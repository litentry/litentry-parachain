// Copyright 2020-2023 Trust Computing GmbH.
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

use itp_rpc::Id;
use lc_data_providers::nodereal_jsonrpc::{
	GetNFTHoldingsResult, GetNFTHoldingsResultDetail, RpcResponse,
};
use warp::{http::Response, hyper::body::Bytes, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("nodereal_jsonrpc" / "v1" / String))
		.and(warp::body::bytes())
		.map(|_, body: Bytes| {
			let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
			let method = json.get("method").unwrap().as_str().unwrap();
			let params: Vec<String> = json
				.get("params")
				.unwrap()
				.as_array()
				.unwrap()
				.iter()
				.filter_map(|v| v.as_str().map(String::from))
				.collect();
			match method {
				"nr_getNFTHoldings" => {
					let result = GetNFTHoldingsResult {
						total_count: "0x1".into(),
						details: vec![GetNFTHoldingsResultDetail {
							token_address: "0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197".into(),
							token_id_num: "0x12".into(),
							token_name: "Pancake Lottery Ticket".into(),
							token_symbol: "PLT".into(),
						}],
					};
					let body = RpcResponse {
						jsonrpc: "2.0".into(),
						id: Id::Number(1),
						result: serde_json::to_value(result).unwrap(),
					};
					Response::builder().body(serde_json::to_string(&body).unwrap())
				},
				"nr_getTokenBalance721" => {
					let body = RpcResponse {
						jsonrpc: "2.0".into(),
						id: Id::Number(1),
						result: serde_json::to_value("0x1").unwrap(),
					};
					Response::builder().body(serde_json::to_string(&body).unwrap())
				},
				"nr_getTokenBalance20" => {
					let value = match params[1].as_str() {
						// 100_000_000_000
						"0x85be4e2ccc9c85be8783798b6e8a101bdac6467f" => "0x174876E800",
						// 3000
						"0x90d53026a47ac20609accc3f2ddc9fb9b29bb310" => "0xBB8",
						// 800.1
						_ => "0x320.1",
					};
					let body = RpcResponse {
						jsonrpc: "2.0".into(),
						id: Id::Number(1),
						result: serde_json::to_value(value).unwrap(),
					};
					Response::builder().body(serde_json::to_string(&body).unwrap())
				},
				_ => Response::builder().status(404).body(String::from("Error query")),
			}
		})
}
