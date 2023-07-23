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
use litentry_primitives::Web3Network;
use log::debug;
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
			// GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_auth_key.clone().as_str(),
			"26353d4c-b01c-4466-98a5-80d3fc53a9d8",
		);
		let client = build_client(
			GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_url.clone().as_str(),
			// "https://label-production.graph.tdf-labs.io",
			headers,
		);

		AchainableClient { client }
	}
}

pub trait AchainablePost {
	fn post(
		&mut self,
		params: SystemLabelReqPath,
		body: &ReqBody,
	) -> Result<serde_json::Value, Error>;
}

impl AchainablePost for AchainableClient {
	fn post(
		&mut self,
		params: SystemLabelReqPath,
		body: &ReqBody,
	) -> Result<serde_json::Value, Error> {
		let response = self
			.client
			.post_capture::<SystemLabelReqPath, ReqBody, serde_json::Value>(params, body);
		debug!("ReqBody response: {:?}", response);
		response.map_err(|e| Error::AchainableError(format!("Achainable response error: {}", e)))
	}
}

pub trait AchainableResultParser {
	type Item;
	fn parse(value: serde_json::Value) -> Result<Self::Item, Error>;
}

impl AchainableResultParser for AchainableClient {
	type Item = bool;
	fn parse(value: serde_json::Value) -> Result<Self::Item, Error> {
		value
			.get("result")
			.and_then(|res| res.as_bool())
			.ok_or_else(|| Error::AchainableError("Achainable Parse result error".to_string()))
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemLabelReqPath {
	path: String,
}

impl Default for SystemLabelReqPath {
	fn default() -> Self {
		Self { path: "/v1/run/system-labels".into() }
	}
}

impl SystemLabelReqPath {
	pub fn new(path: &str) -> Self {
		Self { path: path.into() }
	}
}

#[derive(Serialize, Deserialize, Debug)]
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
		ReqBody { name: params.name(), address, params }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Params {
	AmountHoding(AmountHoding), //A4-A7-A10-A11

	// Tag - Account
	ClassOfYear(ClassOfYear),
	EthDrainedInLastFortnight(EthDrainedInLastFortnight),

	// Tag - Dotsama
	ParamsBasicType(ParamsBasicType),
	ParamsBasicTypeWithAmount(ParamsBasicTypeWithAmount),
	ParamsBasicTypeWithAmounts(ParamsBasicTypeWithAmounts),
	ParamsBasicTypeWithDate(ParamsBasicTypeWithDate),
	ParamsBasicTypeWithAmountToken(ParamsBasicTypeWithAmountToken),
	ParamsBasicTypeWithBetweenPercents(ParamsBasicTypeWithBetweenPercents),
	ParamsBasicTypeWithDateInterval(ParamsBasicTypeWithDateInterval),
	ParamsBasicTypeWithToken(ParamsBasicTypeWithToken),
}

impl AchainableSystemLabelName for Params {
	fn name(&self) -> String {
		match self {
			Params::AmountHoding(a) => a.name(),
			Params::ClassOfYear(c) => c.name(),
			Params::EthDrainedInLastFortnight(e) => e.name.clone(),

			Params::ParamsBasicType(a) => a.name.clone(),
			Params::ParamsBasicTypeWithAmount(a) => a.name.clone(),
			Params::ParamsBasicTypeWithAmounts(a) => a.name.clone(),
			Params::ParamsBasicTypeWithDate(a) => a.name.clone(),
			Params::ParamsBasicTypeWithAmountToken(a) => a.name.clone(),
			Params::ParamsBasicTypeWithBetweenPercents(a) => a.name.clone(),
			Params::ParamsBasicTypeWithDateInterval(a) => a.name.clone(),
			Params::ParamsBasicTypeWithToken(a) => a.name.clone(),
		}
	}
}

/// The parameter types of the method are defined here
pub trait AchainableSystemLabelName {
	fn name(&self) -> String;
}

/// A4/A7/A10/A11 Holder params
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmountHoding {
	pub chain: String,
	pub amount: String,
	pub date: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
}

impl AmountHoding {
	pub fn new(chain: String, amount: String, date: String, token: Option<String>) -> Self {
		Self { chain, amount, date, token }
	}
}

impl AchainableSystemLabelName for AmountHoding {
	fn name(&self) -> String {
		if self.token.is_some() {
			"ERC20 hodling {amount} of {token} since {date}".into()
		} else {
			"Balance hodling {amount} since {date}".into()
		}
	}
}

// ClassOfYear
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassOfYear {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub date1: String,
	pub date2: String,
}

pub enum EClassOfYear {
	Year2020,
	Year2021,
	Year2022,
}

impl EClassOfYear {
	pub fn get(&self) -> ClassOfYear {
		match self {
			EClassOfYear::Year2020 => ClassOfYear::class_of_2020(),
			EClassOfYear::Year2021 => ClassOfYear::class_of_2021(),
			EClassOfYear::Year2022 => ClassOfYear::class_of_2022(),
		}
	}
}

impl ClassOfYear {
	fn class_of_2020() -> Self {
		Self {
			name: "Account created between {dates}".into(),
			chain: "ethereum".into(),
			date1: "2020-01-01T00:00:00.000Z".into(),
			date2: "2020-12-31T23:59:59.999Z".into(),
		}
	}

	fn class_of_2021() -> Self {
		Self {
			name: "Account created between {dates}".into(),
			chain: "ethereum".into(),
			date1: "2021-01-01T00:00:00.000Z".into(),
			date2: "2021-12-31T23:59:59.999Z".into(),
		}
	}

	fn class_of_2022() -> Self {
		Self {
			name: "Account created between {dates}".into(),
			chain: "ethereum".into(),
			date1: "2022-01-01T00:00:00.000Z".into(),
			date2: "2022-12-31T23:59:59.999Z".into(),
		}
	}
}

impl AchainableSystemLabelName for ClassOfYear {
	fn name(&self) -> String {
		"Account created between {dates}".into()
	}
}

// ETH Drained in Last Fortnight
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthDrainedInLastFortnight {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub token: String,
	pub date: String,
	pub percent: String,
}

impl Default for EthDrainedInLastFortnight {
	fn default() -> Self {
		Self {
			name: "Balance dropped {percent} since {date}".into(),
			chain: "ethereum".into(),
			token: "ETH".into(),
			date: "14D".into(),
			percent: "80".into(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicType {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
}

impl ParamsBasicType {
	pub fn new(name: String, chain: String) -> Self {
		Self { name, chain }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithAmount {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub amount: String,
}

impl ParamsBasicTypeWithAmount {
	pub fn new(name: String, chain: String, amount: String) -> Self {
		Self { name, chain, amount }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithDate {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub date: String,
}

impl ParamsBasicTypeWithDate {
	pub fn new(name: String, chain: String, date: String) -> Self {
		Self { name, chain, date }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithAmounts {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub amount1: String,
	pub amount2: String,
}

impl ParamsBasicTypeWithAmounts {
	pub fn new(name: String, chain: String, amount1: String, amount2: String) -> Self {
		Self { name, chain, amount1, amount2 }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithAmountToken {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub amount: String,
	pub token: Option<String>,
}

impl ParamsBasicTypeWithAmountToken {
	pub fn new(name: String, chain: String, amount: String, token: Option<String>) -> Self {
		Self { name, chain, amount, token }
	}
}

// Balance between percents
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithBetweenPercents {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub greater_than_or_equal_to: String,
	pub less_than_or_equal_to: String,
}

impl ParamsBasicTypeWithBetweenPercents {
	pub fn new(
		name: String,
		chain: String,
		greater_than_or_equal_to: String,
		less_than_or_equal_to: String,
	) -> Self {
		Self { name, chain, greater_than_or_equal_to, less_than_or_equal_to }
	}
}

// ParamsBasicTypeWithDateInterval
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithDateInterval {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub start_date: String,
	pub end_date: String,
}

impl ParamsBasicTypeWithDateInterval {
	pub fn new(name: String, chain: String, start_date: String, end_date: String) -> Self {
		Self { name, chain, start_date, end_date }
	}
}

// ParamsBasicTypeWithToken
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsBasicTypeWithToken {
	#[serde(skip_serializing)]
	#[serde(skip_deserializing)]
	pub name: String,

	pub chain: String,
	pub token: String,
}

impl ParamsBasicTypeWithToken {
	pub fn new(name: String, chain: String, token: String) -> Self {
		Self { name, chain, token }
	}
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn check_achainable_label(
	client: &mut AchainableClient,
	address: &str,
	params: Params,
) -> Result<bool, Error> {
	let body = ReqBody::new(address.into(), params);
	let resp = client.post(SystemLabelReqPath::default(), &body)?;
	AchainableClient::parse(resp)
}

/// A4/A7/A10/A11
pub trait AchainableHolder {
	fn is_holder(&mut self, address: &str, amount_holding: AmountHoding) -> Result<bool, Error>;
}

impl AchainableHolder for AchainableClient {
	fn is_holder(&mut self, address: &str, amount_holding: AmountHoding) -> Result<bool, Error> {
		check_achainable_label(self, address, Params::AmountHoding(amount_holding))
	}
}

/// A8 TODO:
/// TODO:
/// This is a compromise. We need to judge the range of the sum of transactions of all linked accounts, but the achanable api
/// currently only judges the range of a single account, so the current approach is to parse the returned data through
/// an assertion such as under 1 to calculate the sum, and then perform interval judgment.
pub trait AchainableTotalTransactionsParser {
	fn parse_txs(response: serde_json::Value) -> Result<u64, Error>;
}
impl AchainableTotalTransactionsParser for AchainableClient {
	fn parse_txs(response: serde_json::Value) -> Result<u64, Error> {
		let display_text = response
			.get("display")
			.and_then(|displays| {
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
	fn total_transactions(
		&mut self,
		network: &Web3Network,
		addresses: &[String],
	) -> Result<u64, Error>;
}

impl AchainableAccountTotalTransactions for AchainableClient {
	fn total_transactions(
		&mut self,
		network: &Web3Network,
		addresses: &[String],
	) -> Result<u64, Error> {
		let mut txs = 0_u64;
		addresses.iter().for_each(|address| {
			let name = "Account total transactions under {amount}".to_string();
			let chain = network.name().to_string();
			let amount = "1".to_string();

			let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
			let body = ReqBody::new(address.into(), Params::ParamsBasicTypeWithAmount(param));
			let tx = self.post(SystemLabelReqPath::default(), &body).and_then(Self::parse_txs);
			txs += tx.unwrap_or_default();
		});

		Ok(txs)
	}
}

pub trait AchainableTagAccount {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error>;
	fn og_account(&mut self, address: &str) -> Result<bool, Error>;
	fn class_of_year(&mut self, address: &str, year: EClassOfYear) -> Result<bool, Error>;
	fn address_found_on_bsc(&mut self, address: &str) -> Result<bool, Error>;
	fn eth_drained_in_last_fortnight(&mut self, address: &str) -> Result<bool, Error>;
	fn is_polkadot_validator(&mut self, address: &str) -> Result<bool, Error>;
	fn is_kusama_validator(&mut self, address: &str) -> Result<bool, Error>;
}

pub trait AchainableTagBalance {
	fn polkadot_dolphin(&mut self, address: &str) -> Result<bool, Error>;
	fn kusama_dolphin(&mut self, address: &str) -> Result<bool, Error>;
	fn polkadot_whale(&mut self, address: &str) -> Result<bool, Error>;
	fn kusama_whale(&mut self, address: &str) -> Result<bool, Error>;
	fn under_10_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn under_10_lit_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn over_100_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn between_10_to_100_eth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn eth_millionaire(&mut self, address: &str) -> Result<bool, Error>;
	fn eth2_validator_eligible(&mut self, address: &str) -> Result<bool, Error>;
	fn over_100_weth_holder(&mut self, address: &str) -> Result<bool, Error>;
	fn over_100_lit_bep20_amount(&mut self, address: &str) -> Result<bool, Error>;
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

impl AchainableTagAccount for AchainableClient {
	fn fresh_account(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Account created after {date}".to_string();
		let chain = "ethereum".to_string();
		let date = "30D".to_string();
		let param = ParamsBasicTypeWithDate::new(name, chain, date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDate(param))
	}

	fn og_account(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Account created before {date}".to_string();
		let chain = "ethereum".to_string();
		let date = "2020-01-01T00:00:00.000Z".to_string();
		let param = ParamsBasicTypeWithDate::new(name, chain, date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDate(param))
	}

	fn class_of_year(&mut self, address: &str, year: EClassOfYear) -> Result<bool, Error> {
		let param = year.get();
		check_achainable_label(self, address, Params::ClassOfYear(param))
	}

	fn address_found_on_bsc(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("Account found on {chain}".to_string(), "bsc".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn eth_drained_in_last_fortnight(&mut self, address: &str) -> Result<bool, Error> {
		let param = EthDrainedInLastFortnight::default();
		check_achainable_label(self, address, Params::EthDrainedInLastFortnight(param))
	}

	fn is_polkadot_validator(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("Validator".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_validator(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("Validator".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}
}

impl AchainableTagBalance for AchainableClient {
	fn polkadot_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance between percents".to_string();
		let chain = "polkadot".to_string();
		let a1 = "0.01".to_string();
		let a2 = "0.0999999999999999".to_string();
		let param = ParamsBasicTypeWithBetweenPercents::new(name, chain, a1, a2);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithBetweenPercents(param))
	}

	fn kusama_dolphin(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance between percents".to_string();
		let chain = "kusama".to_string();
		let a1 = "0.01".to_string();
		let a2 = "0.0999999999999999".to_string();
		let param = ParamsBasicTypeWithBetweenPercents::new(name, chain, a1, a2);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithBetweenPercents(param))
	}

	fn polkadot_whale(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance between percents".to_string();
		let chain = "polkadot".to_string();
		let a1 = "0.01".to_string();
		let a2 = "100".to_string();
		let param = ParamsBasicTypeWithBetweenPercents::new(name, chain, a1, a2);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithBetweenPercents(param))
	}

	fn kusama_whale(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance between percents".to_string();
		let chain = "kusama".to_string();
		let a1 = "0.01".to_string();
		let a2 = "100".to_string();
		let param = ParamsBasicTypeWithBetweenPercents::new(name, chain, a1, a2);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithBetweenPercents(param))
	}

	fn under_10_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance under {amount}".to_string();
		let chain = "ethereum".to_string();
		let amount = "10".to_string();
		let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmount(param))
	}

	fn under_10_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance under {amount}".to_string();
		let chain = "litentry".to_string();
		let amount = "10".to_string();
		let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmount(param))
	}

	fn over_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		let name = "Balance over {amount}".to_string();
		let chain = "ethereum".to_string();
		let amount = "100".to_string();
		let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmount(param))
	}

	fn between_10_to_100_eth_holder(&mut self, address: &str) -> Result<bool, Error> {
		// 10 - 100 ETH Holder
		let name = "Balance between {amounts}".to_string();
		let chain = "ethereum".to_string();
		let amount1 = "10".to_string();
		let amount2 = "100".to_string();
		let param = ParamsBasicTypeWithAmounts::new(name, chain, amount1, amount2);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmounts(param))
	}

	fn eth_millionaire(&mut self, address: &str) -> Result<bool, Error> {
		// ETH Millionaire
		let name = "Balance over {amount} dollars".to_string();
		let chain = "ethereum".to_string();
		let amount = "100".to_string();
		let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmount(param))
	}

	fn eth2_validator_eligible(&mut self, address: &str) -> Result<bool, Error> {
		// ETH2 Validator Eligible
		let name = "Balance over {amount}".to_string();
		let chain = "ethereum".to_string();
		let amount = "32".to_string();
		let param = ParamsBasicTypeWithAmount::new(name, chain, amount);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmount(param))
	}

	fn over_100_weth_holder(&mut self, address: &str) -> Result<bool, Error> {
		// 100+ WETH Holder
		let name = "ERC20 balance over {amount}".to_string();
		let chain = "ethereum".to_string();
		let amount = "100".to_string();
		let token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string();
		let param = ParamsBasicTypeWithAmountToken::new(name, chain, amount, Some(token));
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmountToken(param))
	}

	fn over_100_lit_bep20_amount(&mut self, address: &str) -> Result<bool, Error> {
		// 100+ LIT BEP20 Holder
		let name = "BEP20 balance over {amount}".to_string();
		let chain = "bsc".to_string();
		let amount = "100".to_string();
		let token = "0xb59490ab09a0f526cc7305822ac65f2ab12f9723".to_string();
		let param = ParamsBasicTypeWithAmountToken::new(name, chain, amount, Some(token));
		check_achainable_label(self, address, Params::ParamsBasicTypeWithAmountToken(param))
	}

	fn native_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		// Native LIT Hodler
		let param = AmountHoding::new(
			"litentry".to_string(),
			"10".to_string(),
			"2023-01-01T00:00:00.000Z".to_string(),
			None,
		);
		check_achainable_label(self, address, Params::AmountHoding(param))
	}

	fn erc20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		let param = AmountHoding::new(
			"ethereum".to_string(),
			"10".to_string(),
			"2022-01-01T00:00:00.000Z".to_string(),
			Some("0xb59490ab09a0f526cc7305822ac65f2ab12f9723".to_string()),
		);
		check_achainable_label(self, address, Params::AmountHoding(param))
	}

	fn bep20_lit_holder(&mut self, address: &str) -> Result<bool, Error> {
		let param = AmountHoding::new(
			"bsc".to_string(),
			"10".to_string(),
			"2022-01-01T00:00:00.000Z".to_string(),
			Some("0xb59490ab09a0f526cc7305822ac65f2ab12f9723".to_string()),
		);

		check_achainable_label(self, address, Params::AmountHoding(param))
	}
}

impl AchainableTagDotsama for AchainableClient {
	fn is_polkadot_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		let param =
			ParamsBasicType::new("TreasuryProposalBeneficiary".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_treasury_proposal_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		let param =
			ParamsBasicType::new("TreasuryProposalBeneficiary".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("TipFinder".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_tip_finder(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("TipFinder".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("TipBeneficiary".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_tip_beneficiary(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("TipBeneficiary".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("OpenGovProposer".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_opengov_proposer(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("OpenGovProposer".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("FellowshipProposer".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_fellowship_proposer(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("FellowshipProposer".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("FellowshipMember".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_fellowship_member(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("FellowshipMember".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("ExCouncilor".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_ex_councilor(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("ExCouncilor".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_councilor(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("Councilor".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_councilor(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("Councilor".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_polkadot_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("BountyCurator".to_string(), "polkadot".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}

	fn is_kusama_bounty_curator(&mut self, address: &str) -> Result<bool, Error> {
		let param = ParamsBasicType::new("BountyCurator".to_string(), "kusama".to_string());
		check_achainable_label(self, address, Params::ParamsBasicType(param))
	}
}

impl AchainableTagDeFi for AchainableClient {
	fn uniswap_v2_user(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V2 trader
		let name = "Uniswap V2 trader";
		let chain = "ethereum";
		let r1 = request_basic_type(self, address, name, chain).unwrap_or_default();

		// Uniswap V2 liquidity provider
		let name = "Uniswap V2 liquidity provider";
		let r2 = request_basic_type(self, address, name, chain)?;

		Ok(r1 || r2)
	}

	fn uniswap_v3_user(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V3 trader
		let name = "Uniswap V3 trader";
		let chain = "ethereum";
		let r1 = request_basic_type(self, address, name, chain).unwrap_or_default();

		// Uniswap V3 liquidity provider
		let name = "Uniswap V3 liquidity provider";
		let r2 = request_basic_type(self, address, name, chain)?;

		Ok(r1 || r2)
	}

	fn uniswap_v2_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V2 liquidity provider
		let name = "Uniswap V2 liquidity provider".to_string();
		let chain = "ethereum".to_string();
		let start_date = "2022-01-01T00:00:00.000Z".to_string();
		let end_date = "2022-12-31T23:59:59.999Z".to_string();

		let param = ParamsBasicTypeWithDateInterval::new(name, chain, start_date, end_date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDateInterval(param))
	}

	fn uniswap_v3_lp_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V3 liquidity provider
		let name = "Uniswap V3 liquidity provider".to_string();
		let chain = "ethereum".to_string();
		let start_date = "2022-01-01T00:00:00.000Z".to_string();
		let end_date = "2022-12-31T23:59:59.999Z".to_string();

		let param = ParamsBasicTypeWithDateInterval::new(name, chain, start_date, end_date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDateInterval(param))
	}

	fn usdc_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V2 {token} liquidity provider
		let name = "Uniswap V2 {token} liquidity provider";
		let chain = "ethereum";
		let token = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

		request_basic_type_with_token(self, address, name, chain, token)
	}

	fn usdc_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V3 {token} liquidity provider
		let name = "Uniswap V3 {token} liquidity provider";
		let chain = "ethereum";
		let token = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

		request_basic_type_with_token(self, address, name, chain, token)
	}

	fn usdt_uniswap_lp(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V2 {token} liquidity provider
		let r1 = self.usdt_uniswap_v2_lp(address).unwrap_or_default();

		// Uniswap V3 {token} liquidity provider
		let r2 = self.usdt_uniswap_v3_lp(address)?;

		Ok(r1 || r2)
	}

	fn usdt_uniswap_v2_lp(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V2 {token} liquidity provider
		let name = "Uniswap V2 {token} liquidity provider";
		let chain = "ethereum";
		let token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
		request_basic_type_with_token(self, address, name, chain, token)
	}

	fn usdt_uniswap_v3_lp(&mut self, address: &str) -> Result<bool, Error> {
		// Uniswap V3 {token} liquidity provider
		let name = "Uniswap V3 {token} liquidity provider";
		let chain = "ethereum";
		let token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
		request_basic_type_with_token(self, address, name, chain, token)
	}

	fn aave_v2_lender(&mut self, address: &str) -> Result<bool, Error> {
		// Aave V2 Lender
		let name = "Aave V2 Lender";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn aave_v2_borrower(&mut self, address: &str) -> Result<bool, Error> {
		// Aave V2 Borrower
		let name = "Aave V2 Borrower";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn aave_v3_lender(&mut self, address: &str) -> Result<bool, Error> {
		// Aave V3 Lender
		let name = "Aave V3 Lender";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn aave_v3_borrower(&mut self, address: &str) -> Result<bool, Error> {
		// Aave V3 Borrower
		let name = "Aave V3 Borrower";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn curve_trader(&mut self, address: &str) -> Result<bool, Error> {
		// Curve Trader
		let name = "Curve Trader";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn curve_trader_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		// Curve Trader
		let name = "Curve Trader".to_string();
		let chain = "ethereum".to_string();
		let start_date = "2022-01-01T00:00:00.000Z".to_string();
		let end_date = "2022-12-31T23:59:59.999Z".to_string();

		let param = ParamsBasicTypeWithDateInterval::new(name, chain, start_date, end_date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDateInterval(param))
	}

	fn curve_liquidity_provider(&mut self, address: &str) -> Result<bool, Error> {
		// Curve Liquidity Provider
		let name = "Curve Liquidity Provider";
		let chain = "ethereum";
		request_basic_type(self, address, name, chain)
	}

	fn curve_liquidity_provider_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		// Curve Liquidity Provider
		let name = "Curve Liquidity Provider".to_string();
		let chain = "ethereum".to_string();
		let start_date = "2022-01-01T00:00:00.000Z".to_string();
		let end_date = "2022-12-31T23:59:59.999Z".to_string();

		let param = ParamsBasicTypeWithDateInterval::new(name, chain, start_date, end_date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDateInterval(param))
	}

	fn swapped_with_metamask_in_2022(&mut self, address: &str) -> Result<bool, Error> {
		// MetaMask trader
		let name = "MetaMask trader".to_string();
		let chain = "ethereum".to_string();
		let start_date = "2022-01-01T00:00:00.000Z".to_string();
		let end_date = "2022-12-31T23:59:59.999Z".to_string();

		let param = ParamsBasicTypeWithDateInterval::new(name, chain, start_date, end_date);
		check_achainable_label(self, address, Params::ParamsBasicTypeWithDateInterval(param))
	}
}

/// Utils function
fn request_basic_type(
	client: &mut AchainableClient,
	address: &str,
	name: &str,
	chain: &str,
) -> Result<bool, Error> {
	let param = ParamsBasicType::new(name.to_string(), chain.to_string());
	check_achainable_label(client, address, Params::ParamsBasicType(param))
}

fn request_basic_type_with_token(
	client: &mut AchainableClient,
	address: &str,
	name: &str,
	chain: &str,
	token: &str,
) -> Result<bool, Error> {
	let param =
		ParamsBasicTypeWithToken::new(name.to_string(), chain.to_string(), token.to_string());
	check_achainable_label(client, address, Params::ParamsBasicTypeWithToken(param))
}

#[cfg(test)]
mod tests {
	use crate::achainable::{
		AchainableAccountTotalTransactions, AchainableClient, AchainableTagAccount,
		AchainableTagBalance, AchainableTagDeFi, AchainableTagDotsama, GLOBAL_DATA_PROVIDER_CONFIG,
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
		let res: Result<bool, crate::Error> =
			client.fresh_account("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn og_account_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.og_account("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn class_of_year_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.class_of_year(
			"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5",
			crate::achainable::EClassOfYear::Year2020,
		);
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn address_found_on_bsc_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.address_found_on_bsc("0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1");
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
		assert_eq!(res, true);
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
		assert_eq!(res, true);
	}

	#[test]
	fn kusama_whale_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.kusama_whale("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn under_10_eth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.under_10_eth_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn under_10_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.under_10_lit_holder("0x2A038e100F8B85DF21e4d44121bdBfE0c288A869");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn over_100_eth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.over_100_eth_holder("0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA");
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
	fn over_100_weth_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.over_100_weth_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn over_100_lit_bep20_amount_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.over_100_lit_bep20_amount("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn native_lit_holder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.native_lit_holder("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
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
		assert_eq!(res, true);
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
		assert_eq!(res, true);
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
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_tip_finder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_tip_finder("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_tip_finder_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_tip_finder("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_tip_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_tip_beneficiary("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_tip_beneficiary_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_tip_beneficiary("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_opengov_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_opengov_proposer("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_opengov_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_opengov_proposer("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_fellowship_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client
			.is_polkadot_fellowship_proposer("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_fellowship_proposer_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_fellowship_proposer("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_fellowship_member_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client
			.is_polkadot_fellowship_member("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_fellowship_member_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_fellowship_member("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_ex_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_ex_councilor("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_ex_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_ex_councilor("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_polkadot_councilor("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_councilor_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.is_kusama_councilor("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_polkadot_bounty_curator_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_polkadot_bounty_curator("5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn is_kusama_bounty_curator_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.is_kusama_bounty_curator("CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
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
		assert_eq!(res, true);
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
		assert_eq!(res, true);
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
		assert_eq!(res, true);
	}

	#[test]
	fn usdt_uniswap_v2_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdt_uniswap_v2_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn usdt_uniswap_v3_lp_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.usdt_uniswap_v3_lp("0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
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
		assert_eq!(res, true);
	}

	#[test]
	fn aave_v3_lender_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v3_lender("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn aave_v3_borrower_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.aave_v3_borrower("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn curve_trader_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_trader("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn curve_trader_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_trader_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn curve_liquidity_provider_works() {
		init();

		let mut client = AchainableClient::new();
		let res = client.curve_liquidity_provider("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
	}

	#[test]
	fn curve_liquidity_provider_in_2022_works() {
		init();

		let mut client = AchainableClient::new();
		let res =
			client.curve_liquidity_provider_in_2022("0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b");
		assert!(res.is_ok());
		let res = res.unwrap();
		assert_eq!(res, true);
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
