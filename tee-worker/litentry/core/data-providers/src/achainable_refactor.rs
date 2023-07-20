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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::{build_client, Error, HttpError, GLOBAL_DATA_PROVIDER_CONFIG};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath, RestPost,
};
use litentry_primitives::Web3Network;
use log::{debug, error};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::{
	collections::HashMap,
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};
pub struct AchainableClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for AchainableClient {
	fn default() -> Self {
		Self::new()
	}
}

impl AchainableClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			// GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_auth_key.clone().as_str(),
			"26353d4c-b01c-4466-98a5-80d3fc53a9d8",
		);
		let client = build_client(
			// GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_url.clone().as_str(),
			"https://label-production.graph.tdf-labs.io",
			headers,
		);

		AchainableClient { client }
	}
}


pub trait AchainablePost {
	fn post(&mut self, params: SystemLabelReqPath, body: &ReqBody) -> Result<serde_json::Value, Error>;
}

impl AchainablePost for AchainableClient {
	fn post(&mut self, params: SystemLabelReqPath, body: &ReqBody) -> Result<serde_json::Value, Error> {
		let response =
			self.client.post_capture::<SystemLabelReqPath, ReqBody, serde_json::Value>(params, body);
		debug!("ReqBody response: {:?}", response);
		match response {
			Ok(res) =>
				if let Some(value) = res.get("result") {
					Ok(value.clone())
				} else {
					Err(Error::AchainableError("Invalid response".to_string()))
				},
			Err(e) => Err(Error::RequestError(format!("{:?}", e))),
		}
	}
}

pub trait AchainableResultParser {
	type Item;
	fn parse(value: serde_json::Value) -> Result<Self::Item, Error>;
}

impl AchainableResultParser for AchainableClient {
	type Item = bool;
	fn parse(value: serde_json::Value) -> Result<Self::Item, Error> {
		if let Some(b) = value.as_bool() {
			Ok(b)
		} else {
			Err(Error::AchainableError("Invalid response".to_string()))
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemLabelReqPath {
	path: String,
}

impl Default for SystemLabelReqPath {
	fn default() -> Self {
		Self {
			path: "/v1/run/system-labels".into(),
		}
	}
}

impl SystemLabelReqPath {
	pub fn new(path: &str) -> Self {
		Self { path: path.into() }
	}
}

// #[derive(Debug)]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReqBody {
	pub name: String,
	pub address: String,
	pub params: Params,
}

impl RestPath<SystemLabelReqPath> for ReqBody {
	fn get_path(req_params: SystemLabelReqPath) -> core::result::Result<String, HttpError> {
		Ok(req_params.path)
	}
}

impl ReqBody {
	pub fn new(address: String, params: Params) -> Self {
		ReqBody {
			name: params.name(),
			address,
			params
		}
	}
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Params {
	TotalTransaction(TotalTransaction), //A8
	FreshAccount(FreshAccount),
	OgAccount(OgAccount),
}

impl AchainableSystemLabelName for Params {
	fn name(&self) -> String {
		match self {
			Params::TotalTransaction(t) => t.name(),
			Params::FreshAccount(i) => i.name(),
			Params::OgAccount(i) => i.name(),
		}
	}
}

/// The parameter types of the method are defined here

pub trait AchainableSystemLabelName {
	fn name(&self) -> String;
}

/// A8 Total transaction params
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TotalTransaction {
	pub chain: String,
	pub amount: String,
}

impl TotalTransaction {
	fn new(chain: String) -> Self {
		Self {
			chain,
			amount: "1".into(), //TODO: Be 1 always.
		}
	}
}

impl AchainableSystemLabelName for TotalTransaction {
	fn name(&self) -> String {
		"Account total transactions under {amount}".into()
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FreshAccount {
	pub chain: String,
	pub date: String,
}

impl Default for FreshAccount {
	fn default() -> Self {
		Self {
			chain: "ethereum".into(),
			date: "30D".into(),
		}
	}
}

impl AchainableSystemLabelName for FreshAccount {
	fn name(&self) -> String {
		"Account created after {date}".into()
	}
}

// OgAccount
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OgAccount {
	pub chain: String,
	pub date: String,
}

impl Default for OgAccount {
	fn default() -> Self {
		Self {
			chain: "ethereum".into(),
			date: "2020-01-01T00:00:00.000Z".into(),
		}
	}
}

impl AchainableSystemLabelName for OgAccount {
	fn name(&self) -> String {
		"Account created before {date}".into()
	}
}

fn check_achainable_label(
	client: &mut AchainableClient,
	address: String,
	params: Params,
) -> Result<bool, Error> {
	let req_path = SystemLabelReqPath::default();
	let body = ReqBody::new(address, params);
	debug!("x>>> body: {:?}", body);
	let resp = client.post(req_path, &body)?;
	AchainableClient::parse(resp)
}

/// A8 TODO:
// TODO:
// This is a compromise. We need to judge the range of the sum of transactions of all linked accounts, but the achanable api
// currently only judges the range of a single account, so the current approach is to parse the returned data through
// an assertion such as under 1 to calculate the sum, and then perform interval judgment.
pub trait AchainableTotalTransactionsParser {
	fn parse_txs(response: serde_json::Value) -> Result<u64, Error>;
}
impl AchainableTotalTransactionsParser for AchainableClient {
	fn parse_txs(response: serde_json::Value) -> Result<u64, Error> {
		let display_text = response
			.get("label")
			.and_then(|value| {
				value.get("display").and_then(|displays| {
					displays.as_array().map(|displays| {
						let mut text: std::option::Option<String> = None;
						for v in displays.iter() {
							text = v
								.get("text")
								.and_then(|text| {
									text.as_str().map(|display_text| Some(display_text.to_string()))
								})
								.flatten();
						}
						text
					})
				})
			})
			.flatten();

		debug!("Total txs, display text: {:?}", display_text);

		if let Some(display_text) = display_text {
			// TODO:
			// text field format: Total transactions under 1 (Transactions: 0)
			let split_text = display_text.split(": ").collect::<Vec<&str>>();
			if split_text.len() != 2 {
				return Err(Error::AchainableError("Invalid array".to_string()))
			}

			let mut value_text = split_text[1].to_string();

			// pop the last char: ")"
			value_text.pop();

			let value: u64 = value_text.parse::<u64>().unwrap_or_default();

			return Ok(value)
		}

		Err(Error::AchainableError("Invalid response".to_string()))
	}
}
pub trait AchainableAccountTotalTransactions {
	/// NOTE: Achinable "chain" fieild must be one of [ethereum, polkadot, kusama, litmus, litentry, khala]
	fn total_transactions(&mut self, network: &Web3Network, addresses: &[String]) -> Result<u64, Error>;
}

impl AchainableAccountTotalTransactions for AchainableClient {
	fn total_transactions(&mut self, network: &Web3Network, addresses: &[String]) -> Result<u64, Error> {
		let mut txs = 0_u64;
		addresses.iter().for_each(|address| {
			let chain = network.to_string();		
			let param = TotalTransaction::new(chain);
			let body = ReqBody::new(address.into(), Params::TotalTransaction(param));
			let req_path = SystemLabelReqPath::default();
	
			let tx = self.post(req_path, &body).and_then(|resp| {
				Self::parse_txs(resp)
			});
			txs += tx.unwrap_or_default();
		});

		Ok(txs)
	}
}

pub trait AchainableTagAccount {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error>;
	fn og_account(&mut self, address: &str) -> Result<bool, Error>;
}

impl AchainableTagAccount for AchainableClient {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error> {
		let param = FreshAccount::default();
		check_achainable_label(self, address.into(), Params::FreshAccount(param))
	}

	fn og_account(&mut self, address: &str) -> Result<bool, Error> {
		let param = OgAccount::default();
		check_achainable_label(self, address.into(), Params::OgAccount(param))
	}
}

///////////////////////////////////////////////
