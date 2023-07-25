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
	RestPath, RestPost,
};
use litentry_primitives::{Assertion, Web3Network};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
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
			GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_auth_key.clone().as_str(),
		);
		let client = build_client(
			GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_url.clone().as_str(),
			headers,
		);

		AchainableClient { client }
	}
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
			Ok(res) => Ok(res),
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
	fn parse(response: serde_json::Value) -> Result<Self::Item, Error> {
		if let Some(value) = response.get("result") {
			if let Some(b) = value.as_bool() {
				Ok(b)
			} else {
				Err(Error::AchainableError("Invalid boolean".to_string()))
			}
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

pub trait AchainableTotalTransactions {
	// Currently, supported networks: ["Litentry", "Litmus", "Polkadot", "Kusama", "Ethereum", "Khala"]
	fn total_transactions(
		&mut self,
		network: &Web3Network,
		addresses: &[String],
	) -> Result<u64, Error>;
}

const PATH_LENS: usize = 7;
const A4_ERC20_LIT_ETHEREUM_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/b65b955c-63eb-4cdf-acd9-46863f9362f2",
	"/v1/run/labels/02b46446-e0ce-43ac-83f1-7e55a2f590dd",
	"/v1/run/labels/3075da9e-a426-4fe6-bd49-2e0624374326",
	"/v1/run/labels/00b52b3f-da38-4ef1-aeb3-3b5fdb508cfb",
	"/v1/run/labels/61105e67-a432-454b-b7e0-1b67d4a37ac9",
	"/v1/run/labels/79237456-bc9a-4a70-a235-09c0e1d138d2",
	"/v1/run/labels/c9149a7e-ef69-4ae2-83e6-dbf6ebe0f796",
];
const A4_LIT_LITENTRY_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/2e7e4efb-f64f-4c05-8535-efa14915a566",
	"/v1/run/labels/3268a2f3-b6a5-4055-a7ba-0de414e47b73",
	"/v1/run/labels/7c7dcc3b-7cea-4180-841f-fb4c920afb69",
	"/v1/run/labels/1e40a32d-5da7-4969-9648-b391eab33da7",
	"/v1/run/labels/65796b73-92fd-456e-aa28-75862c1c0cb0",
	"/v1/run/labels/7365f0a2-b69f-465d-b48f-5fe4495bfcaf",
	"/v1/run/labels/dd1bddeb-723a-48e6-b9f0-174b67bd0ff5",
];
const A4_LIT_LITMUS_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/a3f4d87f-d10e-4e0c-9d1a-e05f7e89ea6b",
	"/v1/run/labels/45c636e1-c34f-4d91-aa84-186ca0ebb3aa",
	"/v1/run/labels/ad95aceb-603d-41c4-997d-df196d9b1f94",
	"/v1/run/labels/10b1725a-eafb-4ee1-bace-ed754e98d309",
	"/v1/run/labels/33ccc2bd-ae38-4e41-82d3-fe522880443b",
	"/v1/run/labels/b8bccd1a-ab90-48c3-bc8b-aca3c0d011a3",
	"/v1/run/labels/38ad2b09-4851-44c7-add5-619499788db0",
];
pub trait AchainableA4Holder {
	// Currently, supported networks: ["Litentry", "Litmus", "Ethereum"]
	fn lit_holder_on_network(
		&mut self,
		network: &Web3Network,
		address: &str,
		index: usize,
	) -> Result<bool, Error>;
}
impl AchainableA4Holder for AchainableClient {
	// consistently holding at least 10 LIT tokens
	fn lit_holder_on_network(
		&mut self,
		network: &Web3Network,
		address: &str,
		index: usize,
	) -> Result<bool, Error> {
		if index >= PATH_LENS {
			return Err(Error::AchainableError("Wrong index".to_string()))
		}

		let path = if *network == Web3Network::Ethereum {
			A4_ERC20_LIT_ETHEREUM_PATHS[index]
		} else if *network == Web3Network::Litentry {
			A4_LIT_LITENTRY_PATHS[index]
		} else {
			A4_LIT_LITMUS_PATHS[index]
		};

		let params = ReqParams::new(path);
		let body = ParamsAccount::new(address).into();
		let resp = self.post(params, &body)?;

		Self::parse(resp)
	}
}

// consistently holding at least 5 DOT tokens
const A7_DOT_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/7bb7b42e-088d-46d5-9deb-b0e4f02a817d",
	"/v1/run/labels/b7426872-abdf-4680-9d74-0abe8904e410",
	"/v1/run/labels/23120e2b-1f2b-40c4-83b6-2836558506f0",
	"/v1/run/labels/a3df0656-af20-4d83-8ea6-fbbe9ccd0c1b",
	"/v1/run/labels/3674490b-e986-4cf2-a7ce-6358a6df238b",
	"/v1/run/labels/0d60bb2c-de34-4575-9997-69f7dec7daf7",
	"/v1/run/labels/05b87f44-bfab-46ca-a917-062a4064d9f0",
];

// consistently holding at least 0.001 WBTC tokens
const A10_WBTC_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/5a936ecc-abfd-4bbd-8e62-55a8fc7c4a6a",
	"/v1/run/labels/50e9f706-c610-4a21-b611-65052381061d",
	"/v1/run/labels/32184172-5316-4a95-b0d2-6d5a50b0eba3",
	"/v1/run/labels/4b4e0d0a-812e-4861-8361-b76cd357d20c",
	"/v1/run/labels/dbdbef34-35e3-4542-a50c-b40356747588",
	"/v1/run/labels/4a75aaaa-a4f0-4512-8200-3d259d7dac27",
	"/v1/run/labels/bd84b478-baea-4e2c-8e4d-0cf2eaeadb63",
];

// consistently holding at least 0.01 ETH tokens
const A11_ETH_PATHS: [&str; PATH_LENS] = [
	"/v1/run/labels/1e6053c6-1d09-42ee-9074-a4664957f9a7",
	"/v1/run/labels/060acc81-a9b0-4997-8f4b-b8d7953fe44b",
	"/v1/run/labels/892d4ddc-f70c-4fc2-acfc-1891099db41e",
	"/v1/run/labels/eb2f0c07-c3a4-48dc-a194-c254b26ff581",
	"/v1/run/labels/7f28c5cb-64c4-4880-9242-3cde638a57d4",
	"/v1/run/labels/0afc7c00-a1be-47aa-9903-2d99d2970091",
	"/v1/run/labels/078b2f54-4515-4513-9c67-33c30081b758",
];

pub trait AchainableHoldingAssertion {
	fn is_holder(
		&mut self,
		assertion: &Assertion,
		address: &str,
		index: usize,
	) -> Result<bool, Error>;
}
impl AchainableHoldingAssertion for AchainableClient {
	fn is_holder(
		&mut self,
		assertion: &Assertion,
		address: &str,
		index: usize,
	) -> Result<bool, Error> {
		if index >= PATH_LENS {
			return Err(Error::AchainableError("Wrong index".to_string()))
		}

		let path = match assertion {
			Assertion::A7(..) => A7_DOT_PATHS[index],
			Assertion::A10(..) => A10_WBTC_PATHS[index],
			Assertion::A11(..) => A11_ETH_PATHS[index],
			_ =>
				return Err(Error::AchainableError(
					"Unsupported holding Assertion type.".to_string(),
				)),
		};

		let params = ReqParams::new(path);
		let body = ParamsAccount::new(address).into();
		let resp = self.post(params, &body)?;

		Self::parse(resp)
	}
}

// TODO:
// This is a compromise. We need to judge the range of the sum of transactions of all linked accounts, but the achanable api
// currently only judges the range of a single account, so the current approach is to parse the returned data through
// an assertion such as under 1 to calculate the sum, and then perform interval judgment.
pub trait AchainableTotalTransactionsParser {
	fn parse_total_transactions(response: serde_json::Value) -> Result<u64, Error> {
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
impl AchainableTotalTransactionsParser for AchainableClient {}

impl AchainableTotalTransactions for AchainableClient {
	fn total_transactions(
		&mut self,
		network: &Web3Network,
		addresses: &[String],
	) -> Result<u64, Error> {
		let mut path = "";

		match network {
			Web3Network::Litentry => path = "/v1/run/labels/74655d14-3abd-4a25-b3a4-cd592ae26f4c",
			Web3Network::Litmus => path = "/v1/run/labels/b94fedfc-cb7b-4e59-a7a9-121550acd1c4",
			Web3Network::Polkadot => path = "/v1/run/labels/046e8968-d585-4421-8064-d48b58c75b9a",
			Web3Network::Kusama => path = "/v1/run/labels/060e12c8-b84e-4284-bab3-9a014d41266b",
			Web3Network::Ethereum => path = "/v1/run/labels/8e19fb04-57fc-4537-ac93-d6fa7cff5bbe",
			Web3Network::Khala => path = "/v1/run/labels/f6a5cbe7-605a-4f9f-8763-67f90f943fb4",
			_ => {
				error!("Unsupported network: {:?}", network);
			},
		}

		let mut txs = 0_u64;
		addresses.iter().for_each(|address| {
			let params = ReqParams::new(path);
			let body = ParamsAccount::new(address).into();
			match self.post(params, &body) {
				Ok(resp) => {
					let total = Self::parse_total_transactions(resp).unwrap_or_default();
					txs += total;
				},
				Err(e) => error!("Request total txs error: {:?}", e),
			}
		});

		Ok(txs)
	}
}

impl AchainableTagAccount for AchainableClient {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/1de85e1d215868788dfc91a9f04d7afd")
	}

	fn og_account(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/8a6e26b90dee869634215683ea2dad0d")
	}

	fn class_of_2020(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/9343efca78222a4fad82c635ab697ca0")
	}

	fn class_of_2021(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/6808c28c26908eb695f63b089cfdae80")
	}

	fn class_of_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/a4ee0c9e44cbc7b8a4b2074b3b8fb912")
	}

	fn found_on_bsc(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/3ace29836b372ae66a218dec16e37b62")
	}

	fn is_polkadot_validator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/eb66927e8f56fd7f9a8917d380e6100d")
	}

	fn is_kusama_validator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/a0d213ff009e43b4ecd0cae67bbabae9")
	}
}

impl AchainableTagBalance for AchainableClient {
	fn polkadot_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/6a0424e7544696a3e774dfc7e260dd6e")
	}

	fn kusama_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/3e226ee1bfb0d33564efe7f28f5015bd")
	}

	fn polkadot_whale(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/68390df24e8ac5d0984a8e9c0725a964")
	}

	fn kusama_whale(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/2bf33f5b3ae60293bf93784b80251129")
	}

	fn less_than_10_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/fee8171e2001d1605e018c74f64352da")
	}

	fn less_than_10_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/4a35e107005f1ea4077f119c10d18503")
	}

	fn not_less_than_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/8657c801983aed40012e387900d75726")
	}

	fn between_10_to_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/e4724ad5b7354ef85332887ee7852800")
	}

	fn eth_millionaire(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/83d16c4c31c55ae535472643e63f49ce")
	}

	fn eth2_validator_eligible(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/53b54e51090a3663173c2a97039ebf69")
	}

	fn not_less_than_100_weth_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/f55a4c5a19b6817ad4faf90385f4df6e")
	}

	fn not_less_than_100_lit_bep20_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/0f26a13d7ff182641f9bb9168a3f1d84")
	}

	fn native_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/3f0469170cd271ebaac4ed2c92754479")
	}

	fn erc20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/7bf72e9190098776817afa763044ac1b")
	}

	fn bep20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/0dc166e3b588fb45a9cca36c60c61f79")
	}
}

impl AchainableTagDotsama for AchainableClient {
	fn is_polkadot_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/d4748f8b162a78a195cbbc6669333545")
	}

	fn is_kusama_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/d7cf879652ea3bcab1c043828f4d4478")
	}

	fn is_polkadot_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/fbf7f158c78d7eb95cb872b1a8d5fe07")
	}

	fn is_kusama_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/1181944a66c746042c2914080eb7155b")
	}

	fn is_polkadot_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/1829e887a62fa97113dd0cee977aa8d5")
	}

	fn is_kusama_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/3564145e6ca3f13185b2cd1490db65fc")
	}

	fn is_polkadot_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/ce5e845483b2fcbe42021ff91198b92b")
	}

	fn is_kusama_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/ee1a4e4a1e3e63e3e9d1c5af1674e15b")
	}

	fn is_polkadot_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/5c1a272ce054e729f1eca5c5a47bcbdd")
	}

	fn is_kusama_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/4aa1a72b5d1fae6dd0417671193fffe1")
	}

	fn is_polkadot_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/9b03668237a0a4a7bbdd45c839dbb0fd")
	}

	fn is_kusama_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/91b0529b323d6c1207dc601d0f677414")
	}

	fn is_polkadot_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/8cb42563adaacf8fd4609d6641ce7670")
	}

	fn is_kusama_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/50e6094ebf3df2e8bf2d2b41b2737ba0")
	}

	fn is_polkadot_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/e5bcdbdb20c07ffd9ff68ce206fb64d5")
	}

	fn is_kusama_councilor(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/a27e414ae882a5e5b291b437376e266a")
	}

	fn is_polkadot_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/1f39ff71595b1f0ff9f196b8f64f04e3")
	}

	fn is_kusama_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/6ecc10647157f1c34fe7d3734ba3d89f")
	}
}

impl AchainableTagDeFi for AchainableClient {
	fn uniswap_v2_user(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/6650ee41cda375e6d2a4d27746ce4805")
	}

	fn uniswap_v3_user(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/9e394bae4a87c67d1073d930e0dee58c")
	}

	fn uniswap_v2_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/17769b1442bb26a1604c85ad49045f1b")
	}

	fn uniswap_v3_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/e3da6466ef2e710b39f1139872a69eed")
	}

	fn usdc_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/aa7d5d57430bfb68708417aca6fa2e16")
	}

	fn usdc_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/3a0a5230a42c5dd2b3581218766cc7fb")
	}

	fn usdt_uniswap_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/05265d4009703337e7a57764b09d39d2")
	}

	fn usdt_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/1dcd359e078fb8fac92b76d2e9d720c8")
	}

	fn usdt_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/5a2a14a93b7352e93a6cf84a460c2c50")
	}

	fn aave_v2_lender(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/e79d42db5a0e1571262e5d7c056111ed")
	}

	fn aave_v2_borrower(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/b9256d66b76ad62b9ec25f27775e6d83")
	}

	fn aave_v3_lender(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/2f0470c59799e58f91929678d2a62c2b")
	}

	fn aave_v3_borrower(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/c090d9694c902141673c85a8f64d7f78")
	}

	fn curve_trader(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/054625c2a49a73876831b797c5c41cd3")
	}

	fn curve_trader_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/7d7c271af78ebf94d7f3b1ff6df30142")
	}

	fn curve_liquidity_provider(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/2c3d7189e1783880916cc56a1277cb13")
	}

	fn curve_liquidity_provider_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/112860d373ee427d80b2d687ca54dc8e")
	}

	fn swapped_with_metamask_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		check_achainable_label(self, address, "/v1/run/labels/5061d6de2687378f303b2f38538b350d")
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
		AchainableClient, AchainableTagAccount, AchainableTagBalance, AchainableTagDeFi,
		AchainableTagDotsama, AchainableTotalTransactions, GLOBAL_DATA_PROVIDER_CONFIG,
	};
	use lc_mock_server::{default_getter, run};
	use litentry_primitives::Web3Network;
	use std::sync::Arc;

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER_CONFIG.write().unwrap().set_achainable_url(url);
	}

	#[test]
	fn total_transactions_work() {
		init();

		let addresses = vec!["0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string()];

		let mut client = AchainableClient::new();
		let r = client.total_transactions(&Web3Network::Litentry, &addresses);
		assert!(r.is_ok());
		let r = r.unwrap();
		assert!(r == 41)
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
