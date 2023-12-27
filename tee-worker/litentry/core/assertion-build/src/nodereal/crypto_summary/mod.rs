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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

mod summary;

use crate::*;
use lc_credentials::nodereal::crypto_summary::{
	CRYPTO_SUMMARY_NFT_ADDRESSES_ETH, CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH,
};
use lc_data_providers::nodereal_jsonrpc::{
	FungibleApiList, GetNFTHoldingsParam, NftApiList, NoderealChain, NoderealJsonrpcClient,
};
use litentry_primitives::ErrorDetail;
use serde::{Deserialize, Serialize};
use std::{string::String, vec, vec::Vec};
pub use summary::*;

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
				//...
			]
		}
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
	pub details: Vec<ResponseToken>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NameAndAddress {
	pub name: String,
	pub address: String,
}

impl NameAndAddress {
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
        }
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
	pub token: Vec<Item>,

	#[serde(rename = "NFT")]
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
		let bsc_token_list = Self::collect_list(&CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH, &bsc_token);
		let eth_token_list = Self::collect_list(&CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH, &eth_token);
		let eth_nft_list = Self::collect_list(&CRYPTO_SUMMARY_NFT_ADDRESSES_ETH, &eth_nft);

		let mut token_and_nft = TokenAndNFT::default();
		token_and_nft.add_token(Item::new(web3_network_to_chain(&Web3Network::Bsc), bsc_token_list));
		token_and_nft.add_token(Item::new(web3_network_to_chain(&Web3Network::Ethereum), eth_token_list));
		token_and_nft.add_nft(Item::new(web3_network_to_chain(&Web3Network::Ethereum), eth_nft_list));

		SummaryHoldings { summary: token_and_nft }
	}

	fn collect_list(source: &[(&str, &str); 15], flags: &[bool]) -> Vec<NameAndAddress> {
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

pub struct CryptoSummary {
	pub eth_client: NoderealJsonrpcClient,
	pub bsc_client: NoderealJsonrpcClient,
}

impl CryptoSummary {
	pub fn new() -> Self {
		let eth_client = NoderealJsonrpcClient::new(NoderealChain::Eth);
		let bsc_client = NoderealJsonrpcClient::new(NoderealChain::Bsc);

		Self { eth_client, bsc_client }
	}

	pub fn logic(
		&mut self,
		identities: &Vec<(Web3Network, Vec<String>)>,
	) -> core::result::Result<SummaryHoldings, ErrorDetail> {
		let mut flag_bsc_token: [bool; 15] = [false; 15];
		let mut flag_eth_token: [bool; 15] = [false; 15];
		let mut flag_eth_nft: [bool; 15] = [false; 15];

		for (network, addresses) in identities {
			if *network == Web3Network::Bsc {
				// Token
				for address in addresses {
					let res = self
						.bsc_client
						.get_token_holdings(address)
						.map_err(|e| e.into_error_detail())?;

					let result: ResponseTokenResult =
						serde_json::from_value(res.result).map_err(|e| ErrorDetail::ParseError)?;

					Self::update_holding_flag(&mut flag_bsc_token, &result.details);
				}
			}

			if *network == Web3Network::Ethereum {
				for address in addresses {
					// Tokens
					let res_token = self
						.eth_client
						.get_token_holdings(address)
						.map_err(|e| e.into_error_detail())?;
					let result: ResponseTokenResult = serde_json::from_value(res_token.result)
						.map_err(|e| ErrorDetail::ParseError)?;

					Self::update_holding_flag(&mut flag_eth_token, &result.details);

					// NFT
					let param = GetNFTHoldingsParam {
						account_address: "0x49AD262C49C7aA708Cc2DF262eD53B64A17Dd5EE".into(),
						token_type: "ERC721".into(),
						page: 1,
						page_size: 2,
					};

					let res_nft = self
						.eth_client
						.get_nft_holdings(&param)
						.map_err(|e| e.into_error_detail())?;
					let details = res_nft.details;
					Self::update_holding_flag(&mut flag_eth_nft, &details)
				}
			}
		}

		Ok(SummaryHoldings::construct(&flag_bsc_token, &flag_eth_token, &flag_eth_nft))
	}

	fn update_holding_flag<'a, T>(flag_array: &mut [bool], result: &[T])
	where
		T: Serialize + Deserialize<'a>,
	{
        for (index, f) in flag_array.iter().enumerate() {
            if *f == false {

            }
        }
	}
}

#[cfg(test)]
mod tests {
    use super::SummaryHoldings;

	#[test]
	fn summary_construct_works() {
		let flag_bsc_token = [false, true, true, true, true, true, true, true, true, true, true, true, true, true, true];
		let flag_eth_token = [false, false, false,false,false,false,false,false,false,false,false,false,false,false,false];
		let flag_eth_nft = [true,  false, false,false,false,false,false,false,false,false,false,false,false,false,false];

		let summary = SummaryHoldings::construct(&flag_bsc_token, &flag_eth_token, &flag_eth_nft);
		println!(">> {}", serde_json::to_string(&summary).unwrap());
		assert!(!summary.is_empty());
	}
}