// Copyright 2020-2024 Trust Computing GmbH.
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

use self::summary::{
	CRYPTO_SUMMARY_NFT_ADDRESSES_ETH, CRYPTO_SUMMARY_TOKEN_ADDRESSES_BSC,
	CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH,
};
use crate::*;
use lc_common::abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls};
use lc_data_providers::{
	achainable::web3_network_to_chain,
	nodereal_jsonrpc::{
		FungibleApiList, GetNFTHoldingsParam, NftApiList, NoderealChain, NoderealJsonrpcClient,
		TransactionCount,
	},
	DataProviderConfig,
};
use litentry_primitives::{ErrorDetail, IntoErrorDetail};
use serde::{Deserialize, Serialize};
use std::{string::String, vec, vec::Vec};

pub mod summary;

const ETH_TOKEN_DECIMALS: f64 = 1_000_000_000_000_000_000.0;

/*
SUMMARY: {
	TOKEN: [
		{
			network: BSC,
			list: [
				{
					name: PEPE,
					address: 0x123,
				},
				{
					name: SHIB,
					address: 0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE ,
				},
				{
					name: BNB,
					address: "", // No smart contract address
				},

				//...
			]
		},
		{
			network: Ethereum,
			list: [
				{
					name: PEPE,
					address: 0x123,
				},
				{
					name: SHIB,
					address: 0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE ,
				},
				{
					name: ETH,
					address: "", // No smart contract address
				},
				//...
			]
		},
	],
	NFT: [
		{
			network: Ethereum,
			list: [
				{
					name: Moonbirds,
					address: 0x23581767a106ae21c074b2276D25e5C3e136a68b
				}
			]
		}
	]
}

*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseToken {
	token_address: String,
	token_balance: String,
	token_name: String,
	token_symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTokenResult {
	pub total_count: String,
	pub native_token_balance: String,
	pub details: Option<Vec<ResponseToken>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NameAndAddress {
	pub name: String,
	pub address: String,
}

impl NameAndAddress {
	pub fn new(name: &str, address: &str) -> Self {
		Self { name: name.to_string(), address: address.to_string() }
	}
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
	pub network: String,
	pub list: Vec<NameAndAddress>,
}

impl Item {
	pub fn new(network: String, list: Vec<NameAndAddress>) -> Self {
		let mut item = Item::default();

		if !list.is_empty() {
			item.network = network;
			item.list = list;
		}

		item
	}

	pub fn is_empty(&self) -> bool {
		self.list.is_empty()
	}
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenAndNFT {
	#[serde(rename = "TOKEN")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub token: Vec<Item>,

	#[serde(rename = "NFT")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub nft: Vec<Item>,
}

impl TokenAndNFT {
	pub fn add_token(&mut self, item: Item) {
		if !item.is_empty() {
			self.token.push(item);
		}
	}

	pub fn add_nft(&mut self, item: Item) {
		if !item.is_empty() {
			self.nft.push(item);
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SummaryHoldings {
	#[serde(rename = "SUMMARRY")]
	pub summary: TokenAndNFT,
}

impl SummaryHoldings {
	pub fn is_empty(&self) -> bool {
		self.summary.token.is_empty() && self.summary.nft.is_empty()
	}

	pub fn construct(bsc_token: &[bool], eth_token: &[bool], eth_nft: &[bool]) -> Self {
		let bsc_token_list = Self::collect_list(&CRYPTO_SUMMARY_TOKEN_ADDRESSES_BSC, bsc_token);
		let eth_token_list = Self::collect_list(&CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH, eth_token);
		let eth_nft_list = Self::collect_list(&CRYPTO_SUMMARY_NFT_ADDRESSES_ETH, eth_nft);

		let mut token_and_nft = TokenAndNFT::default();
		token_and_nft
			.add_token(Item::new(web3_network_to_chain(&Web3Network::Bsc), bsc_token_list));
		token_and_nft
			.add_token(Item::new(web3_network_to_chain(&Web3Network::Ethereum), eth_token_list));
		token_and_nft
			.add_nft(Item::new(web3_network_to_chain(&Web3Network::Ethereum), eth_nft_list));

		SummaryHoldings { summary: token_and_nft }
	}

	fn collect_list(source: &[(&str, &str)], flags: &[bool]) -> Vec<NameAndAddress> {
		let mut list = vec![];
		for (index, is_holder) in flags.iter().enumerate() {
			if *is_holder {
				let (address, name) = source[index];
				let name_and_address = NameAndAddress::new(name, address);
				list.push(name_and_address)
			}
		}

		list
	}
}

pub struct CryptoSummaryClient {
	pub eth_client: NoderealJsonrpcClient,
	pub bsc_client: NoderealJsonrpcClient,
}

impl CryptoSummaryClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let eth_client = NoderealJsonrpcClient::new(NoderealChain::Eth, data_provider_config);
		let bsc_client = NoderealJsonrpcClient::new(NoderealChain::Bsc, data_provider_config);

		Self { eth_client, bsc_client }
	}

	pub fn logic(
		&mut self,
		identities: Vec<(Web3Network, Vec<String>)>,
	) -> core::result::Result<(u64, SummaryHoldings), ErrorDetail> {
		// Corresponds one-to-one with CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH
		let mut flag_bsc_token: [bool; 12] = [false; 12];
		let mut flag_eth_token: [bool; 15] = [false; 15];
		let mut flag_eth_nft: [bool; 15] = [false; 15];

		let mut total_txs = 0_u64;

		loop_with_abort_strategy::<fn(&_) -> bool, (Web3Network, Vec<String>), ErrorDetail>(
			identities,
			|(network, addresses)| match network {
				Web3Network::Bsc => {
					let result = self.bsc_logic(addresses.to_vec())?;

					total_txs += result.0;
					flag_bsc_token = result.1;

					Ok(LoopControls::Continue)
				},
				Web3Network::Ethereum => {
					let result = self.ethereum_logic(addresses.to_vec())?;

					total_txs += result.0;
					flag_eth_token = result.1;
					flag_eth_nft = result.2;

					Ok(LoopControls::Continue)
				},
				_ => Ok(LoopControls::Continue),
			},
			AbortStrategy::FailFast::<fn(&_) -> bool>,
		)
		.map_err(|errors| errors[0].clone())?;

		Ok((total_txs, SummaryHoldings::construct(&flag_bsc_token, &flag_eth_token, &flag_eth_nft)))
	}

	fn bsc_logic(
		&mut self,
		addresses: Vec<String>,
	) -> core::result::Result<(u64, [bool; 12]), ErrorDetail> {
		// Corresponds one-to-one with CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH
		let mut flag_bsc_token: [bool; 12] = [false; 12];

		// BNB holder use different APIs than other tokens, so they need to be handled separately
		let mut is_bnb_holder = false;

		let mut total_txs = 0_u64;

		loop_with_abort_strategy::<fn(&_) -> bool, String, ErrorDetail>(
			addresses,
			|address| {
				let res = self
					.bsc_client
					.get_token_holdings(address, false)
					.map_err(|e| e.into_error_detail())?;

				let result: ResponseTokenResult =
					serde_json::from_value(res.result).map_err(|_| ErrorDetail::ParseError)?;
				let mut token_addresses = vec![];
				if let Some(details) = result.details {
					details.iter().for_each(|detail: &ResponseToken| {
						token_addresses.push(detail.token_address.clone());
					});
					Self::update_holding_flag(
						&mut flag_bsc_token,
						&CRYPTO_SUMMARY_TOKEN_ADDRESSES_BSC,
						&token_addresses,
					);
				}

				// Query BNB balance on BSC
				if !is_bnb_holder {
					let native_balance = result.native_token_balance;
					let balance = get_native_token_balance(&native_balance);
					if balance > 0.0 {
						is_bnb_holder = true;
					}
				}

				// Total txs on BSC
				let tx = self
					.bsc_client
					.get_transaction_count(address, false)
					.map_err(|e| e.into_error_detail())?;
				total_txs += tx;
				Ok(LoopControls::Continue)
			},
			AbortStrategy::FailFast::<fn(&_) -> bool>,
		)
		.map_err(|errors| errors[0].clone())?;

		// Update BNB
		flag_bsc_token[11] = is_bnb_holder;

		Ok((total_txs, flag_bsc_token))
	}

	fn ethereum_logic(
		&mut self,
		addresses: Vec<String>,
	) -> core::result::Result<(u64, [bool; 15], [bool; 15]), ErrorDetail> {
		// Corresponds one-to-one with CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH
		let mut flag_eth_token: [bool; 15] = [false; 15];
		let mut flag_eth_nft: [bool; 15] = [false; 15];

		// ETH holder use different APIs than other tokens, so they need to be handled separately
		let mut is_eth_holder = false;

		let mut total_txs = 0_u64;

		loop_with_abort_strategy::<fn(&_) -> bool, String, ErrorDetail>(
			addresses,
			|address| {
				// Query Token
				let res_token = self
					.eth_client
					.get_token_holdings(address, false)
					.map_err(|e| e.into_error_detail())?;
				let result: ResponseTokenResult = serde_json::from_value(res_token.result)
					.map_err(|_| ErrorDetail::ParseError)?;

				let mut token_addresses = vec![];
				if let Some(details) = result.details {
					details.iter().for_each(|detail| {
						token_addresses.push(detail.token_address.clone());
					});
					Self::update_holding_flag(
						&mut flag_eth_token,
						&CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH,
						&token_addresses,
					);
				}

				// Query ETH balance on Ethereum
				if !is_eth_holder {
					let native_balance = result.native_token_balance;
					let balance = get_native_token_balance(&native_balance);
					if balance > 0.0 {
						is_eth_holder = true;
					}
				}

				// Query NFT
				let param = GetNFTHoldingsParam {
					account_address: address.to_string(),
					token_type: "ERC721".to_string(),
					page: 1,
					page_size: 100,
				};

				let res_nft = self
					.eth_client
					.get_nft_holdings(&param, false)
					.map_err(|e| e.into_error_detail())?;

				let details = res_nft.details;

				let mut nft_addresses = vec![];
				details.iter().for_each(|detail| {
					nft_addresses.push(detail.token_address.clone());
				});

				Self::update_holding_flag(
					&mut flag_eth_nft,
					&CRYPTO_SUMMARY_NFT_ADDRESSES_ETH,
					&nft_addresses,
				);

				// Total txs on Ethereum
				let tx = self
					.eth_client
					.get_transaction_count(address, false)
					.map_err(|e| e.into_error_detail())?;
				total_txs += tx;
				Ok(LoopControls::Continue)
			},
			AbortStrategy::FailFast::<fn(&_) -> bool>,
		)
		.map_err(|errors| errors[0].clone())?;

		// Update ETH
		flag_eth_token[14] = is_eth_holder;

		Ok((total_txs, flag_eth_token, flag_eth_nft))
	}

	fn update_holding_flag(
		flag_array: &mut [bool],
		source: &[(&'static str, &'static str)],
		token_addresses: &[String],
	) {
		for (index, is_holder) in flag_array.iter_mut().enumerate() {
			if !*is_holder {
				let (token_address, _token_name) = source[index];
				if token_addresses.contains(&token_address.to_lowercase()) {
					*is_holder = true;
				}
			}
		}
	}
}

fn get_native_token_balance(native_balance: &str) -> f64 {
	let native_balance = u64::from_str_radix(&native_balance[2..], 16).unwrap_or_default() as f64;
	native_balance / ETH_TOKEN_DECIMALS
}

#[cfg(test)]
mod tests {
	use super::{get_native_token_balance, CryptoSummaryClient, SummaryHoldings};
	use crate::nodereal::crypto_summary::summary::CRYPTO_SUMMARY_NFT_ADDRESSES_ETH;

	#[test]
	fn summary_construct_works() {
		let flag_bsc_token =
			[false, true, true, true, true, true, true, true, true, true, true, true];
		let flag_eth_token = [
			false, false, false, false, false, false, false, false, false, false, false, false,
			false, false, true,
		];
		let flag_eth_nft = [
			true, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false,
		];

		let summary = SummaryHoldings::construct(&flag_bsc_token, &flag_eth_token, &flag_eth_nft);
		assert!(!summary.is_empty());
	}

	#[test]
	fn update_nft_holding_flag_works() {
		let nft_addresses = vec![
			"0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197".to_lowercase(),
			"0x6339e5E072086621540D0362C4e3Cea0d643E114".to_lowercase(),
		];
		let mut flag_eth_nft = [
			false, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false,
		];
		CryptoSummaryClient::update_holding_flag(
			&mut flag_eth_nft,
			&CRYPTO_SUMMARY_NFT_ADDRESSES_ETH,
			&nft_addresses,
		);
		assert!(flag_eth_nft.contains(&true));
	}

	#[test]
	fn update_bsc_holding_flag_works() {
		let bsc_addresses = vec![
			"0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197".to_lowercase(),
			"0x6339e5E072086621540D0362C4e3Cea0d643E114".to_lowercase(),
		];
		let mut flag_bsc =
			[false, false, false, false, false, false, false, false, false, false, false, false];
		CryptoSummaryClient::update_holding_flag(
			&mut flag_bsc,
			&CRYPTO_SUMMARY_NFT_ADDRESSES_ETH,
			&bsc_addresses,
		);
		assert!(flag_bsc.contains(&true));
	}

	#[test]
	fn get_native_token_balance_works() {
		let native_token_balance =
			"0x00000000000000000000000000000000000000000000000000c92180664030d4";
		let balance = get_native_token_balance(native_token_balance);
		assert_eq!(balance, 0.05661330567385518);
	}
}
