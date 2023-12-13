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

use crate::*;
use codec::Decode;
use frame_support::{StorageHasher, Twox64Concat};
use hex_literal::hex;
use http::header::CONNECTION;
use itc_rest_client::{
	error::Error as RestClientError,
	http_client::{DefaultSend, HttpClient},
	rest_client::{Headers, RestClient},
	RestPath, RestPost,
};
use itp_stf_primitives::types::AccountId;
use itp_types::Balance;
use itp_utils::hex_display::AsBytesRef;
use lc_credentials::litentry_profile::lit_staking::UpdateLITStakingAmountCredential;
use lc_data_providers::build_client;
use litentry_primitives::types;
use serde::{Deserialize, Serialize};
use sp_core::twox_128;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestStakingData {
	params: Vec<String>,
	method: String,
	jsonrpc: String,
	id: usize,
}

impl RequestStakingData {
	pub fn new(storage_key: String) -> Self {
		Self {
			params: vec![storage_key],
			method: "state_getStorage".to_string(),
			jsonrpc: "2.0".to_string(),
			id: 1,
		}
	}
}

impl RestPath<String> for RequestStakingData {
	fn get_path(path: String) -> core::result::Result<String, RestClientError> {
		Ok(path)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LitentryStakingResponse {
	pub id: usize,
	pub jsonrpc: String,
	pub result: String,
}

pub struct LitentryStakingClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for LitentryStakingClient {
	fn default() -> Self {
		Self::new()
	}
}

impl LitentryStakingClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client(" https://litentry-rpc.dwellir.com:443", headers);
		LitentryStakingClient { client }
	}

	pub fn send_request(&mut self, data: &RequestStakingData) -> Result<LitentryStakingResponse> {
		self.client
			.post_capture::<String, RequestStakingData, LitentryStakingResponse>(
				String::default(),
				data,
			)
			.map_err(|e| {
				Error::RequestVCFailed(
					Assertion::LITStaking,
					ErrorDetail::DataProviderError(ErrorString::truncate_from(
						format!("{e:?}").as_bytes().to_vec(),
					)),
				)
			})
	}
}

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	debug!("Building LIT staking");

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let staking_amount = query_lit_staking(&addresses)?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_lit_staking_amount(staking_amount);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::LITStaking, e.into_error_detail()))
		},
	}
}

fn query_lit_staking(addresses: &[String]) -> Result<u128> {
	let total_staking_amount: u128 = 0_u128;

	let mut client = LitentryStakingClient::new();
	for address in addresses {
		let storage_key = delegator_state_storage_key(address);
		let data = RequestStakingData::new(storage_key);
		println!(">>> request data: {:#?}", serde_json::to_string(&data));

		let response = client.send_request(&data)?;
		println!(">>> response: {:?}", response);
		let result = response.result;
		let input = hex::decode(&result[2..]).unwrap();
		let delegator =
			types::Delegator::<AccountId, Balance>::decode(&mut input.as_bytes_ref()).unwrap();
		println!(">>> delegator: {:?}", delegator);
	}

	Ok(total_staking_amount)
}

fn twox_128_hex(name: &str) -> String {
	let pallet_name = twox_128(name.as_bytes());
	hex::encode(pallet_name)
}

fn delegator_state_storage_key(_address: &str) -> String {
	let pallet_name = "ParachainStaking";
	let pallet_name = twox_128_hex(pallet_name);
	println!("pallet_name: {}", pallet_name);

	let state = "DelegatorState";
	let state = twox_128_hex(state);
	println!("state: {}", state);

	let acc =
		AccountId::new(hex!["facd1c88fb4bc1448c4bc66af9e3ba040d0c236e9f85d8ffd313aed04278a61e"]);
	let cocat = Twox64Concat::hash(acc.as_ref());
	let acc = hex::encode(&cocat);
	println!("cocat: {:?}", acc);

	"0x".to_string() + &pallet_name + &state + &acc
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn query_lit_staking_works() {
		let addresses = vec!["".to_string()];
		let v = query_lit_staking(&addresses).is_ok();
		assert!(v);
	}
}
