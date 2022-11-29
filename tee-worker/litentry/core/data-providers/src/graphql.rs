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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::{
	base_url::{GRAPHQL_AUTH_KEY, GRAPHQL_URL},
	build_client, Error, HttpError,
};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use serde::{Deserialize, Serialize};
use std::{
	default::Default,
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

pub struct GraphQLClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for GraphQLClient {
	fn default() -> Self {
		Self::new()
	}
}

pub enum VerifiedCredentialsNetwork {
	Litentry,
	Litmus,
	Polkadot,
	Kusama,
	Khala,
	Ethereum,
}
pub struct VerifiedCredentialsIsHodlerIn {
	pub addresses: Vec<String>,
	pub from_date: String,
	pub network: VerifiedCredentialsNetwork,
	pub token_address: String,
	pub mini_balance: f64,
}

impl VerifiedCredentialsIsHodlerIn {
	pub fn new(
		addresses: Vec<String>,
		from_date: String,
		network: VerifiedCredentialsNetwork,
		token_address: String,
		mini_balance: f64,
	) -> Self {
		VerifiedCredentialsIsHodlerIn { addresses, from_date, network, token_address, mini_balance }
	}

	pub fn conv_to_string(&self) -> String {
		let mut flat = "addresses:[".to_string();
		for addr in self.addresses.iter() {
			flat += &format!("\"{}\",", addr);
		}
		flat += "],";
		flat += &format!("fromDate:\"{}\",", self.from_date.clone());
		match &self.network {
			VerifiedCredentialsNetwork::Litentry => flat += "network:litentry",
			VerifiedCredentialsNetwork::Litmus => flat += "network:litmus",
			VerifiedCredentialsNetwork::Polkadot => flat += "network:polkadot",
			VerifiedCredentialsNetwork::Kusama => flat += "network:kusama",
			VerifiedCredentialsNetwork::Khala => flat += "network:khala",
			VerifiedCredentialsNetwork::Ethereum => flat += "network:ethereum",
		}
		flat += &format!(",tokenAddress:\"{}\"", &self.token_address.clone());
		flat += &format!(",minimumBalance:{:?}", self.mini_balance.clone());
		flat
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QLResponse {
	#[serde(flatten)]
	// data: HashMap<String, serde_json::Value>,
	data: serde_json::Value,
}
impl RestPath<String> for QLResponse {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct IsHodlerOut {
	pub verified_credentials_is_hodler: Vec<IsHodlerOutStruct>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IsHodlerOutStruct {
	pub address: String,
	pub is_hodler: bool,
}

impl GraphQLClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(AUTHORIZATION.as_str(), GRAPHQL_AUTH_KEY);
		let client = build_client(GRAPHQL_URL, headers);
		GraphQLClient { client }
	}

	pub fn verified_credentials_is_hodler(
		&mut self,
		credentials: VerifiedCredentialsIsHodlerIn,
	) -> Result<IsHodlerOut, Error> {
		// FIXME: for the moment, the `path` is partially hard-code here.
		let path = "latest/graphql?query=query{VerifiedCredentialsIsHodler(".to_string()
			+ &credentials.conv_to_string()
			+ "){isHodler, address}}";

		let response = self
			.client
			.get_with::<String, QLResponse>(path, vec![].as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		if let Some(value) = response.data.get("data") {
			let is_hodler_out: IsHodlerOut = serde_json::from_value(value.clone()).unwrap();
			Ok(is_hodler_out)
		} else {
			Err(Error::GraphQLError("Invalid GraphQL response".to_string()))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::graphql::{
		GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
	};

	const ACCOUNT_ADDRESS1: &str = "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad";
	const ACCOUNT_ADDRESS2: &str = "0x3394caf8e5ccaffb936e6407599543af46525e0b";
	const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

	#[test]
	fn verified_credentials_is_hodler_work() {
		let mut client = GraphQLClient::new();

		let credentials = VerifiedCredentialsIsHodlerIn {
			addresses: vec![ACCOUNT_ADDRESS1.to_string(), ACCOUNT_ADDRESS2.to_string()],
			// from_date: format!("{:?}", Utc::now()),
			from_date: "2022-10-16T00:00:00Z".to_string(),
			network: VerifiedCredentialsNetwork::Ethereum,
			token_address: LIT_TOKEN_ADDRESS.to_string(),
			mini_balance: 0.00000056,
		};
		let response = client.verified_credentials_is_hodler(credentials);

		if let Ok(is_hodler_out) = response {
			assert_eq!(is_hodler_out.verified_credentials_is_hodler[0].is_hodler, true);
			assert_eq!(is_hodler_out.verified_credentials_is_hodler[1].is_hodler, false);
		} else {
			assert!(false);
		}
	}
}
