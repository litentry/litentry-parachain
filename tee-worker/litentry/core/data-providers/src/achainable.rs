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

use crate::{build_client, Error, HttpError, GLOBAL_DATA_PROVIDER};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath, RestPost,
};
use litentry_primitives::Web3Network;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

pub trait AchainableQuery<Query: ToAchainable> {
	fn verified_credentials_is_hodler(&mut self, params: Query) -> Result<IsHodlerOut, Error>;
	fn verified_credentials_total_transactions(
		&mut self,
		params: Query,
	) -> Result<Vec<TotalTxsStruct>, Error>;
}

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
			GLOBAL_DATA_PROVIDER.read().unwrap().achainable_auth_key.clone().as_str(),
		);
		let client = build_client(
			GLOBAL_DATA_PROVIDER.read().unwrap().achainable_url.clone().as_str(),
			headers,
		);

		AchainableClient { client }
	}
}

impl<Query: ToAchainable> AchainableQuery<Query> for AchainableClient {
	fn verified_credentials_is_hodler(&mut self, query: Query) -> Result<IsHodlerOut, Error> {
		let path = query.path();
		let query_value = query.to_achainable();
		debug!("verified_credentials_is_hodler query: {}", query_value);

		let query = vec![("query", query_value.as_str())];
		match self.client.get_with::<String, AchainableResponse>(path, query.as_slice()) {
			Ok(response) =>
				if let Some(value) = response.data.get("data") {
					debug!("	[Achainable] value: {:?}", value);

					serde_json::from_value(value.clone()).map_err(|e| {
						let error_msg = format!("Deserialize Achainable response error: {:?}", e);
						Error::AchainableError(error_msg)
					})
				} else {
					Err(Error::AchainableError("Invalid Achainable response".to_string()))
				},
			Err(e) => Err(Error::RequestError(format!("{:?}", e))),
		}
	}

	fn verified_credentials_total_transactions(
		&mut self,
		query: Query,
	) -> Result<Vec<TotalTxsStruct>, Error> {
		let path = query.path();
		let query_value = query.to_achainable();
		debug!("verified_credentials_total_transactions query: {}", query_value);

		let query = vec![("query", query_value.as_str())];
		let response = self
			.client
			.get_with::<String, AchainableResponse>(path, query.as_slice())
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
			Err(Error::AchainableError("Invalid Achainable response".to_string()))
		}
	}
}

pub trait ToAchainable {
	fn path(&self) -> String {
		"latest/achainable".to_string()
	}

	fn to_achainable(&self) -> String;
}

#[derive(Debug)]
pub struct VerifiedCredentialsIsHodlerIn {
	pub addresses: Vec<String>,
	pub from_date: String,
	pub network: Web3Network,
	pub token_address: String,
	pub min_balance: String,
}

impl VerifiedCredentialsIsHodlerIn {
	pub fn new(
		addresses: Vec<String>,
		from_date: String,
		network: Web3Network,
		token_address: String,
		min_balance: String,
	) -> Self {
		VerifiedCredentialsIsHodlerIn { addresses, from_date, network, token_address, min_balance }
	}
}

impl ToAchainable for VerifiedCredentialsIsHodlerIn {
	fn to_achainable(&self) -> String {
		let addresses_str = format!("{:?}", self.addresses);
		let network = format!("{:?}", self.network).to_lowercase();
		if self.token_address.is_empty() {
			format!("{{VerifiedCredentialsIsHodler(addresses:{}, fromDate:\"{}\", network:{}, minimumBalance:{}){{isHodler,address}}}}", addresses_str, self.from_date, network, self.min_balance)
		} else {
			format!("{{VerifiedCredentialsIsHodler(addresses:{}, fromDate:\"{}\", network:{}, tokenAddress:\"{}\",minimumBalance:{}){{isHodler,address}}}}", addresses_str, self.from_date, network, self.token_address, self.min_balance)
		}
	}
}

#[derive(Debug)]
pub struct VerifiedCredentialsTotalTxs {
	addresses: Vec<String>,
	networks: Vec<Web3Network>,
}

impl VerifiedCredentialsTotalTxs {
	pub fn new(addresses: Vec<String>, networks: Vec<Web3Network>) -> Self {
		VerifiedCredentialsTotalTxs { addresses, networks }
	}
}

impl ToAchainable for VerifiedCredentialsTotalTxs {
	fn to_achainable(&self) -> String {
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
pub struct AchainableResponse {
	#[serde(flatten)]
	data: serde_json::Value,
}
impl RestPath<String> for AchainableResponse {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsHodlerOut {
	#[serde(rename = "VerifiedCredentialsIsHodler")]
	pub hodlers: Vec<IsHodlerOutStruct>,
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

pub trait AchainableTagAccount {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error>;
	fn og_account(&mut self, address: &str) -> Result<bool, Error>;
	fn class_of_2020(&mut self, address: &str) -> Result<bool, Error>;
	fn class_of_2021(&mut self, address: &str) -> Result<bool, Error>;
	fn class_of_2022(&mut self, address: &str) -> Result<bool, Error>;
	fn found_on_bsc(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_validator(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_validator(&mut self, address: &str) -> Result<bool, Error>;
}

pub trait AchainableTagBalance {
	fn polkadot_dolphin(&mut self, address: &str) -> Result<bool, Error>;
	fn kusama_dolphin(&mut self, address: &str) -> Result<bool, Error>;
	fn polkadot_whale(&mut self, address: &str) -> Result<bool, Error>;
	fn kusama_whale(&mut self, address: &str) -> Result<bool, Error>;
	fn less_than_10_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn less_than_10_lit_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn not_less_than_100_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn between_10_to_100_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn eth_millionaire(&mut self, address: &str) -> Result<bool, Error>;
	fn eth2_validator_eligible(&mut self, address: &str) -> Result<bool, Error>;
	fn not_less_than_100_weth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn not_less_than_100_lit_bep20_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn native_lit_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn erc20_lit_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn bep20_lit_holder(&mut self, address: &str) -> Result<bool, Error>;
}

pub trait AchainableTagDotsama {
	fn is_polkadot_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_tip_finder(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_tip_finder(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_opengov_proposer(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_opengov_proposer(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_fellowship_member(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_fellowship_member(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_ex_councilor(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_ex_councilor(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_councilor(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_councilor(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_bounty_curator(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_bounty_curator(&mut self, address: &str) -> Result<bool, Error>;
}

pub trait AchainableTagDeFi {
	fn uniswap_v2_user(&mut self, address: &str) -> Result<bool, Error>;
	fn uniswap_v3_user(&mut self, address: &str) -> Result<bool, Error>;
	fn uniswap_v2_lp_in_2022(&mut self, address: &str) -> Result<bool, Error>;
	fn uniswap_v3_lp_in_2022(&mut self, address: &str) -> Result<bool, Error>;
	fn usdc_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error>;
	fn usdc_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error>;
	fn usdt_uniswap_lp(&mut self, address: &str) -> Result<bool, Error>;
	fn usdt_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error>;
	fn usdt_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error>;
	fn aave_v2_lender(&mut self, address: &str) -> Result<bool, Error>;
	fn aave_v2_borrower(&mut self, address: &str) -> Result<bool, Error>;
	fn aave_v3_lender(&mut self, address: &str) -> Result<bool, Error>;
	fn aave_v3_borrower(&mut self, address: &str) -> Result<bool, Error>;
	fn curve_trader(&mut self, address: &str) -> Result<bool, Error>;
	fn curve_trader_in_2022(&mut self, address: &str) -> Result<bool, Error>;
	fn curve_liquidity_provider(&mut self, address: &str) -> Result<bool, Error>;
	fn curve_liquidity_provider_in_2022(&mut self, address: &str) -> Result<bool, Error>;
	fn swapped_with_metamask_in_2022(&mut self, address: &str) -> Result<bool, Error>;
}

pub trait AchainablePost {
	fn post(&mut self, params: ReqParams, body: &ReqBody) -> Result<serde_json::Value, Error>;
}

impl AchainablePost for AchainableClient {
	fn post(&mut self, params: ReqParams, body: &ReqBody) -> Result<serde_json::Value, Error> {
		let response =
			self.client.post_capture::<ReqParams, ReqBody, serde_json::Value>(params, body);
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

fn check_achainable_label(
	client: &mut AchainableClient,
	address: &str,
	label_path: &str,
) -> Result<bool, Error> {
	let params = ReqParams::new(label_path);
	let body = ParamsAccount::new(address).into();
	let resp = client.post(params, &body)?;
	AchainableClient::parse(resp)
}

impl AchainableTagAccount for AchainableClient {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/1de85e1d215868788dfc91a9f04d7afd")
	}

	fn og_account(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/8a6e26b90dee869634215683ea2dad0d")
	}

	fn class_of_2020(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/9343efca78222a4fad82c635ab697ca0")
	}

	fn class_of_2021(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/6808c28c26908eb695f63b089cfdae80")
	}

	fn class_of_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/a4ee0c9e44cbc7b8a4b2074b3b8fb912")
	}

	fn found_on_bsc(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/3ace29836b372ae66a218dec16e37b62")
	}

	fn is_polkadot_validator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/eb66927e8f56fd7f9a8917d380e6100d")
	}

	fn is_kusama_validator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/a0d213ff009e43b4ecd0cae67bbabae9")
	}
}

impl AchainableTagBalance for AchainableClient {
	fn polkadot_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/6a0424e7544696a3e774dfc7e260dd6e")
	}

	fn kusama_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/3e226ee1bfb0d33564efe7f28f5015bd")
	}

	fn polkadot_whale(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/68390df24e8ac5d0984a8e9c0725a964")
	}

	fn kusama_whale(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/2bf33f5b3ae60293bf93784b80251129")
	}

	fn less_than_10_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/fee8171e2001d1605e018c74f64352da")
	}

	fn less_than_10_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/4a35e107005f1ea4077f119c10d18503")
	}

	fn not_less_than_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/8657c801983aed40012e387900d75726")
	}

	fn between_10_to_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/e4724ad5b7354ef85332887ee7852800")
	}

	fn eth_millionaire(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/83d16c4c31c55ae535472643e63f49ce")
	}

	fn eth2_validator_eligible(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/53b54e51090a3663173c2a97039ebf69")
	}

	fn not_less_than_100_weth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/f55a4c5a19b6817ad4faf90385f4df6e")
	}

	fn not_less_than_100_lit_bep20_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/0f26a13d7ff182641f9bb9168a3f1d84")
	}

	fn native_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/3f0469170cd271ebaac4ed2c92754479")
	}

	fn erc20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/7bf72e9190098776817afa763044ac1b")
	}

	fn bep20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/0dc166e3b588fb45a9cca36c60c61f79")
	}
}

impl AchainableTagDotsama for AchainableClient {
	fn is_polkadot_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/d4748f8b162a78a195cbbc6669333545")
	}

	fn is_kusama_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/d7cf879652ea3bcab1c043828f4d4478")
	}

	fn is_polkadot_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/fbf7f158c78d7eb95cb872b1a8d5fe07")
	}

	fn is_kusama_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/1181944a66c746042c2914080eb7155b")
	}

	fn is_polkadot_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/1829e887a62fa97113dd0cee977aa8d5")
	}

	fn is_kusama_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/3564145e6ca3f13185b2cd1490db65fc")
	}

	fn is_polkadot_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/ce5e845483b2fcbe42021ff91198b92b")
	}

	fn is_kusama_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/ee1a4e4a1e3e63e3e9d1c5af1674e15b")
	}

	fn is_polkadot_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/5c1a272ce054e729f1eca5c5a47bcbdd")
	}

	fn is_kusama_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/4aa1a72b5d1fae6dd0417671193fffe1")
	}

	fn is_polkadot_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/9b03668237a0a4a7bbdd45c839dbb0fd")
	}

	fn is_kusama_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/91b0529b323d6c1207dc601d0f677414")
	}

	fn is_polkadot_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/8cb42563adaacf8fd4609d6641ce7670")
	}

	fn is_kusama_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/50e6094ebf3df2e8bf2d2b41b2737ba0")
	}

	fn is_polkadot_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/e5bcdbdb20c07ffd9ff68ce206fb64d5")
	}

	fn is_kusama_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/a27e414ae882a5e5b291b437376e266a")
	}

	fn is_polkadot_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/1f39ff71595b1f0ff9f196b8f64f04e3")
	}

	fn is_kusama_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/6ecc10647157f1c34fe7d3734ba3d89f")
	}
}

impl AchainableTagDeFi for AchainableClient {
	fn uniswap_v2_user(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/6650ee41cda375e6d2a4d27746ce4805")
	}

	fn uniswap_v3_user(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/9e394bae4a87c67d1073d930e0dee58c")
	}

	fn uniswap_v2_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/17769b1442bb26a1604c85ad49045f1b")
	}

	fn uniswap_v3_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/e3da6466ef2e710b39f1139872a69eed")
	}

	fn usdc_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/aa7d5d57430bfb68708417aca6fa2e16")
	}

	fn usdc_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/3a0a5230a42c5dd2b3581218766cc7fb")
	}

	fn usdt_uniswap_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/05265d4009703337e7a57764b09d39d2")
	}

	fn usdt_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/1dcd359e078fb8fac92b76d2e9d720c8")
	}

	fn usdt_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/5a2a14a93b7352e93a6cf84a460c2c50")
	}

	fn aave_v2_lender(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/e79d42db5a0e1571262e5d7c056111ed")
	}

	fn aave_v2_borrower(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/b9256d66b76ad62b9ec25f27775e6d83")
	}

	fn aave_v3_lender(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/2f0470c59799e58f91929678d2a62c2b")
	}

	fn aave_v3_borrower(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/c090d9694c902141673c85a8f64d7f78")
	}

	fn curve_trader(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/054625c2a49a73876831b797c5c41cd3")
	}

	fn curve_trader_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/7d7c271af78ebf94d7f3b1ff6df30142")
	}

	fn curve_liquidity_provider(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/2c3d7189e1783880916cc56a1277cb13")
	}

	fn curve_liquidity_provider_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/112860d373ee427d80b2d687ca54dc8e")
	}

	fn swapped_with_metamask_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/label/5061d6de2687378f303b2f38538b350d")
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReqParams {
	path: String,
}

impl ReqParams {
	pub fn new(path: &str) -> Self {
		Self { path: path.into() }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReqBody {
	pub params: ParamsAccount,
}

impl RestPath<ReqParams> for ReqBody {
	fn get_path(req_params: ReqParams) -> core::result::Result<String, HttpError> {
		Ok(req_params.path)
	}
}

impl From<ParamsAccount> for ReqBody {
	fn from(item: ParamsAccount) -> Self {
		ReqBody { params: item }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsAccount {
	pub address: String,
}

impl ParamsAccount {
	pub fn new(address: &str) -> Self {
		ParamsAccount { address: address.into() }
	}
}

impl RestPath<String> for ParamsAccount {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

#[cfg(test)]
mod tests {
	use crate::achainable::{
		AchainableClient, AchainableQuery, AchainableTagAccount, AchainableTagBalance,
		AchainableTagDeFi, AchainableTagDotsama, VerifiedCredentialsIsHodlerIn,
		VerifiedCredentialsTotalTxs, GLOBAL_DATA_PROVIDER,
	};
	use lc_mock_server::{default_getter, run};
	use litentry_primitives::Web3Network;
	use std::sync::Arc;

	const ACCOUNT_ADDRESS1: &str = "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad";
	const ACCOUNT_ADDRESS2: &str = "0x3394caf8e5ccaffb936e6407599543af46525e0b";
	const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER.write().unwrap().set_achainable_url(url);
	}

	#[test]
	fn verified_credentials_is_hodler_work() {
		init();

		let mut client = AchainableClient::new();
		let credentials = VerifiedCredentialsIsHodlerIn {
			addresses: vec![ACCOUNT_ADDRESS1.to_string(), ACCOUNT_ADDRESS2.to_string()],
			from_date: "2022-10-16T00:00:00Z".to_string(),
			network: Web3Network::Ethereum,
			token_address: LIT_TOKEN_ADDRESS.to_string(),
			min_balance: "0.00000056".into(),
		};
		let response = client.verified_credentials_is_hodler(credentials);
		assert!(response.is_ok(), "due to error:{:?}", response.unwrap_err());
		let is_hodler_out = response.unwrap();
		assert_eq!(is_hodler_out.hodlers[0].is_hodler, false);
		assert_eq!(is_hodler_out.hodlers[1].is_hodler, false);
	}

	#[test]
	fn verified_credentials_total_transactions_work() {
		init();

		let query = VerifiedCredentialsTotalTxs {
			addresses: vec!["EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg".to_string()],
			networks: vec![Web3Network::Kusama, Web3Network::Polkadot],
		};
		let mut client = AchainableClient::new();
		let r = client.verified_credentials_total_transactions(query);
		assert!(r.is_ok());
		let r = r.unwrap();
		assert!(!r.is_empty());
		assert!(r.get(0).unwrap().total_transactions >= 41)
	}

	#[test]
	fn fresh_account_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.fresh_account("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn og_account_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.og_account("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn class_of_2020_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.class_of_2020("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn class_of_2021_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.class_of_2021("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn class_of_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.class_of_2022("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn found_on_bsc_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.found_on_bsc("0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_validator_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_validator("17bR6rzVsVrzVJS1hM4dSJU43z2MUmz7ZDpPLh8y2fqVg7m");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_validator_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_validator("ESRBbWstgpPV1pVBsqjMo717rA8HLrtQvEUVwAGeFZyKcia");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn polkadot_dolphin_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.polkadot_dolphin("1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn kusama_dolphin_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.kusama_dolphin("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn polkadot_whale_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.polkadot_whale("1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn kusama_whale_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.kusama_whale("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn less_than_10_eth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.less_than_10_eth_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn less_than_10_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.less_than_10_lit_holder("0x2A038e100F8B85DF21e4d44121bdBfE0c288A869");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn not_less_than_100_eth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.not_less_than_100_eth_holder("0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn between_10_to_100_eth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.between_10_to_100_eth_holder("0x082aB5505CdeA46caeF670754E962830Aa49ED2C");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn eth_millionaire_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.eth_millionaire("0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn eth2_validator_eligible_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.eth2_validator_eligible("0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn not_less_than_100_weth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.not_less_than_100_weth_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn not_less_than_100_lit_bep20_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.not_less_than_100_lit_bep20_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn native_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.native_lit_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn erc20_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.erc20_lit_holder("0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn bep20_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.bep20_lit_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_treasury_proposal_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_treasury_proposal_beneficiary(
			"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW",
		);
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_treasury_proposal_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_treasury_proposal_beneficiary(
			"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq",
		);
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_tip_finder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_tip_finder("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_tip_finder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_tip_finder("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_tip_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_tip_beneficiary("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_tip_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_tip_beneficiary("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_opengov_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_opengov_proposer("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_opengov_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_opengov_proposer("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_fellowship_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client
			.is_polkadot_fellowship_proposer("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_fellowship_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_fellowship_proposer("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_fellowship_member_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client
			.is_polkadot_fellowship_member("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_fellowship_member_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_fellowship_member("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_ex_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_ex_councilor("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_ex_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_ex_councilor("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_councilor("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_councilor("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_polkadot_bounty_curator_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_bounty_curator("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn is_kusama_bounty_curator_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_bounty_curator("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn uniswap_v2_user_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.uniswap_v2_user("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn uniswap_v3_user_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.uniswap_v3_user("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn uniswap_v2_lp_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.uniswap_v2_lp_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn uniswap_v3_lp_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.uniswap_v3_lp_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn usdc_uniswap_v2_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdc_uniswap_v2_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn usdc_uniswap_v3_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdc_uniswap_v3_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn usdt_uniswap_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdt_uniswap_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn usdt_uniswap_v2_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdt_uniswap_v2_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn usdt_uniswap_v3_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdt_uniswap_v3_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn aave_v2_lender_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v2_lender("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn aave_v2_borrower_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v2_borrower("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn aave_v3_lender_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v3_lender("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn aave_v3_borrower_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v3_borrower("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn curve_trader_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_trader("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn curve_trader_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_trader_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn curve_liquidity_provider_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_liquidity_provider("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn curve_liquidity_provider_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.curve_liquidity_provider_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, false);
	}

	#[test]
	fn swapped_with_metamask_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.swapped_with_metamask_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}
}
