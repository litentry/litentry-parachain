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

use crate::{build_client, Error, HttpError, G_DATA_PROVIDERS};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use litentry_primitives::{EvmNetwork, IndexingNetwork, SubstrateNetwork};
use log::*;
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

/// TODO: https://github.com/litentry/litentry-parachain/pull/1534
/// There are two issues here that need to be refactored later
/// 1. The name of VerifiedCredentialsNetwork is a bit unclear
/// 2. VerifiedCredentialsNetwork and IndexingNetwork are repeated, they should be one-to-one relationship, just keep one.
/// What's even better is that we have a trait for each data provider, something like get_supported_network.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum VerifiedCredentialsNetwork {
	Litentry,
	Litmus,
	LitentryRococo,
	Polkadot,
	Kusama,
	Khala,
	Ethereum,
	TestNet,
}

impl From<SubstrateNetwork> for VerifiedCredentialsNetwork {
	fn from(network: SubstrateNetwork) -> Self {
		match network {
			SubstrateNetwork::Litmus => Self::Litmus,
			SubstrateNetwork::Litentry => Self::Litentry,
			SubstrateNetwork::LitentryRococo => Self::LitentryRococo,
			SubstrateNetwork::Polkadot => Self::Polkadot,
			SubstrateNetwork::Kusama => Self::Kusama,
			SubstrateNetwork::Khala => Self::Khala,
			SubstrateNetwork::TestNet => Self::TestNet,
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

impl From<IndexingNetwork> for VerifiedCredentialsNetwork {
	fn from(network: IndexingNetwork) -> Self {
		match network {
			IndexingNetwork::Litmus => Self::Litmus,
			IndexingNetwork::Litentry => Self::Litentry,
			IndexingNetwork::Polkadot => Self::Polkadot,
			IndexingNetwork::Kusama => Self::Kusama,
			IndexingNetwork::Khala => Self::Khala,
			IndexingNetwork::Ethereum => Self::Ethereum,
		}
	}
}

impl std::fmt::Display for VerifiedCredentialsNetwork {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			VerifiedCredentialsNetwork::Litentry => write!(f, "Litentry"),
			VerifiedCredentialsNetwork::Litmus => write!(f, "Litmus"),
			VerifiedCredentialsNetwork::LitentryRococo => write!(f, "LitentryRococo"),
			VerifiedCredentialsNetwork::Polkadot => write!(f, "Polkadot"),
			VerifiedCredentialsNetwork::Kusama => write!(f, "Kusama"),
			VerifiedCredentialsNetwork::Khala => write!(f, "Khala"),
			VerifiedCredentialsNetwork::Ethereum => write!(f, "Ethereum"),
			VerifiedCredentialsNetwork::TestNet => write!(f, "TestNet"),
		}
	}
}

pub struct VerifiedCredentialsIsHodlerIn {
	pub addresses: Vec<String>,
	pub from_date: String,
	pub network: VerifiedCredentialsNetwork,
	pub token_address: String,
	pub min_balance: String,
}

impl VerifiedCredentialsIsHodlerIn {
	pub fn new(
		addresses: Vec<String>,
		from_date: String,
		network: VerifiedCredentialsNetwork,
		token_address: String,
		min_balance: String,
	) -> Self {
		VerifiedCredentialsIsHodlerIn { addresses, from_date, network, token_address, min_balance }
	}

	pub fn to_graphql(&self) -> String {
		let addresses_str = format!("{:?}", self.addresses);
		let network = format!("{:?}", self.network).to_lowercase();
		if self.token_address.is_empty() {
			format!("{{VerifiedCredentialsIsHodler(addresses:{}, fromDate:\"{}\", network:{}, minimumBalance:{}){{isHodler,address}}}}", addresses_str, self.from_date, network, self.min_balance)
		} else {
			format!("{{VerifiedCredentialsIsHodler(addresses:{}, fromDate:\"{}\", network:{}, tokenAddress:\"{}\",minimumBalance:{}){{isHodler,address}}}}", addresses_str, self.from_date, network, self.token_address, self.min_balance)
		}
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
		debug!("check is_holder, credentials: {}", credentials.to_graphql());

		// FIXME: for the moment, the `path` is partially hard-code here.
		let path = "latest/graphql".to_string();
		let query_value = credentials.to_graphql();
		let query = vec![("query", query_value.as_str())];

		match self.client.get_with::<String, QLResponse>(path, query.as_slice()) {
			Ok(response) =>
				if let Some(value) = response.data.get("data") {
					debug!("	[Graphql] value: {:?}", value);

					serde_json::from_value(value.clone()).map_err(|e| {
						let error_msg = format!("Deserialize GraphQL response error: {:?}", e);
						Error::GraphQLError(error_msg)
					})
				} else {
					Err(Error::GraphQLError("Invalid GraphQL response".to_string()))
				},
			Err(e) => Err(Error::RequestError(format!("{:?}", e))),
		}
	}

	pub fn query_total_transactions(
		&mut self,
		credentials: VerifiedCredentialsTotalTxs,
	) -> Result<Vec<TotalTxsStruct>, Error> {
		debug!("check total_trx, credentials: {}", credentials.to_graphql());

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
		VerifiedCredentialsTotalTxs, G_DATA_PROVIDERS,
	};
	use itp_stf_primitives::types::AccountId;
	use lc_mock_server::run;
	use litentry_primitives::{ChallengeCode, Identity};

	use std::sync::Arc;

	const ACCOUNT_ADDRESS1: &str = "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad";
	const ACCOUNT_ADDRESS2: &str = "0x3394caf8e5ccaffb936e6407599543af46525e0b";
	const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(|_: &AccountId, _: &Identity| ChallengeCode::default()), 0).unwrap();
		G_DATA_PROVIDERS.write().unwrap().set_graphql_url(url.clone());
	}

	#[test]
	fn verified_credentials_is_hodler_work() {
		init();

		let mut client = GraphQLClient::new();
		let credentials = VerifiedCredentialsIsHodlerIn {
			addresses: vec![ACCOUNT_ADDRESS1.to_string(), ACCOUNT_ADDRESS2.to_string()],
			from_date: "2022-10-16T00:00:00Z".to_string(),
			network: VerifiedCredentialsNetwork::Ethereum,
			token_address: LIT_TOKEN_ADDRESS.to_string(),
			min_balance: "0.00000056".into(),
		};
		let response = client.check_verified_credentials_is_hodler(credentials);
		assert!(response.is_ok(), "due to error:{:?}", response.unwrap_err());
		let is_hodler_out = response.unwrap();
		assert_eq!(is_hodler_out.verified_credentials_is_hodler[0].is_hodler, false);
		assert_eq!(is_hodler_out.verified_credentials_is_hodler[1].is_hodler, false);
	}

	#[test]
	fn verified_credentials_total_transactions_work() {
		init();

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
