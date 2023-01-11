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

use crate::{build_client, Error, HttpError, G_DATA_PROVIDERS};
use core::ops::Not;
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use litentry_primitives::{EvmNetwork, SubstrateNetwork};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
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

#[derive(PartialEq, Debug)]
pub enum VerifiedCredentialsNetwork {
	Litentry,
	Litmus,
	Polkadot,
	Kusama,
	Khala,
	Ethereum,
}

impl From<SubstrateNetwork> for VerifiedCredentialsNetwork {
	fn from(network: SubstrateNetwork) -> Self {
		match network {
			SubstrateNetwork::Litmus => Self::Litmus,
			SubstrateNetwork::Litentry => Self::Litentry,
			SubstrateNetwork::Polkadot => Self::Polkadot,
			SubstrateNetwork::Kusama => Self::Kusama,
		}
	}
}

impl From<EvmNetwork> for VerifiedCredentialsNetwork {
	fn from(network: EvmNetwork) -> Self {
		match network {
			EvmNetwork::Ethereum => Self::Ethereum,
			// TODO: how about BSC?
			EvmNetwork::BSC => unreachable!("support BSC?"),
		}
	}
}

pub struct VerifiedCredentialsIsHodlerIn {
	pub addresses: Vec<String>,
	pub from_date: String,
	pub network: VerifiedCredentialsNetwork,
	pub token_address: String,
	pub min_balance: f64,
}

impl VerifiedCredentialsIsHodlerIn {
	pub fn new(
		addresses: Vec<String>,
		from_date: String,
		network: VerifiedCredentialsNetwork,
		token_address: String,
		min_balance: f64,
	) -> Self {
		VerifiedCredentialsIsHodlerIn { addresses, from_date, network, token_address, min_balance }
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
		if self.token_address.is_empty().not() {
			flat += &format!(",tokenAddress:\"{}\"", &self.token_address.clone());
		}
		flat += &format!(",minimumBalance:{:?}", self.min_balance.clone());
		flat
	}
}

// TODO make the struct name more intuitive
pub struct VerifiedCredentialsTotalTxs {
	addresses: Vec<String>,
	networks: Vec<VerifiedCredentialsNetwork>,
}

impl VerifiedCredentialsTotalTxs {
	pub fn new(addresses: Vec<String>, networks: Vec<VerifiedCredentialsNetwork>) -> Self {
		VerifiedCredentialsTotalTxs { addresses, networks }
	}

	pub fn to_graphql(&self) -> String {
		let addresses_str = format!("{:?}", self.addresses);
		let q = self
			.networks
			.iter()
			.map(|n| {
				let network = format!("{:?}", n).to_lowercase();
				format!("{}: VerifiedCredentialsTotalTransactions(network: {} addresses: {}){{address,totalTransactions}}",
						network,
						network,
					addresses_str
				)
			})
			.collect::<Vec<String>>();
		format!("query {{{}}}", q.join(","))
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QLResponse {
	#[serde(flatten)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalTxsStruct {
	pub address: String,
	pub total_transactions: u64,
}

impl GraphQLClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().graphql_auth_key.clone().as_str(),
		);
		let client =
			build_client(G_DATA_PROVIDERS.read().unwrap().graphql_url.clone().as_str(), headers);
		GraphQLClient { client }
	}

	pub fn check_verified_credentials_is_hodler(
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
			// Valid response will always match the IsHodlerOut structure.
			let is_hodler_out: IsHodlerOut = serde_json::from_value(value.clone()).unwrap();
			Ok(is_hodler_out)
		} else {
			Err(Error::GraphQLError("Invalid GraphQL response".to_string()))
		}
	}

	pub fn query_total_transactions(
		&mut self,
		credentials: VerifiedCredentialsTotalTxs,
	) -> Result<Vec<TotalTxsStruct>, Error> {
		let path = "latest/graphql".to_string();
		let query_value = credentials.to_graphql();
		let query = vec![("query", query_value.as_str())];
		let response = self
			.client
			.get_with::<String, QLResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		let mut result: HashMap<String, TotalTxsStruct> = HashMap::new();

		response.data.get("data").and_then(|v| v.as_object()).and_then(|map| {
			for (_network, list) in map {
				list.as_array().and_then(|element| {
					for x in element {
						// aggregate total_transactions from different networks, like group_by.
						if let Ok(obj) = serde_json::from_value::<TotalTxsStruct>(x.clone()) {
							if let Some(origin) = result.get_mut(&obj.address) {
								origin.total_transactions += obj.total_transactions;
							} else {
								result.insert(obj.address.clone(), obj.clone());
							}
						}
					}
					None::<u8>
				});
			}
			None::<u8>
		});
		if !result.is_empty() {
			// Ok(result.iter().map(|(_, v)| v.clone()).collect::<Vec<TotalTxsStruct>>())
			Ok(result.values().cloned().collect::<Vec<TotalTxsStruct>>())
		} else {
			Err(Error::GraphQLError("Invalid GraphQL response".to_string()))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::graphql::{
		GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
		VerifiedCredentialsTotalTxs,
	};

	const ACCOUNT_ADDRESS1: &str = "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad";
	const ACCOUNT_ADDRESS2: &str = "0x3394caf8e5ccaffb936e6407599543af46525e0b";
	const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

	#[test]
	fn verified_credentials_is_hodler_work() {
		let mut client = GraphQLClient::new();

		let credentials = VerifiedCredentialsIsHodlerIn {
			addresses: vec![ACCOUNT_ADDRESS1.to_string(), ACCOUNT_ADDRESS2.to_string()],
			from_date: "2022-10-16T00:00:00Z".to_string(),
			network: VerifiedCredentialsNetwork::Ethereum,
			token_address: LIT_TOKEN_ADDRESS.to_string(),
			min_balance: 0.00000056,
		};
		let response = client.check_verified_credentials_is_hodler(credentials);
		assert!(response.is_ok());
		let is_hodler_out = response.unwrap();
		assert_eq!(is_hodler_out.verified_credentials_is_hodler[0].is_hodler, false);
		assert_eq!(is_hodler_out.verified_credentials_is_hodler[1].is_hodler, false);
	}

	#[test]
	fn verified_credentials_total_transactions_work() {
		let query = VerifiedCredentialsTotalTxs {
			addresses: vec!["EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg".to_string()],
			networks: vec![
				VerifiedCredentialsNetwork::Kusama,
				VerifiedCredentialsNetwork::Polkadot,
			],
		};
		let mut client = GraphQLClient::new();
		let r = client.query_total_transactions(query);
		assert!(r.is_ok());
		let r = r.unwrap();
		assert!(!r.is_empty());
		assert!(r.get(0).unwrap().total_transactions >= 41)
	}
}
