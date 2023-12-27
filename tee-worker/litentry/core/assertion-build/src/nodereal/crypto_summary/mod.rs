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
	CRYPTO_SUMMARY_NFT_ADDRESSES_ETH_NFT, CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH,
};
use lc_data_providers::nodereal_jsonrpc::{
	FungibleApiList, GetNFTHoldingsParam, NftApiList, NoderealChain, NoderealJsonrpcClient,
};
use litentry_primitives::{Assertion, ErrorDetail};
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
	pub network: String,
	pub list: Vec<NameAndAddress>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TokenAndNFT {
	pub token: Vec<Item>,
	pub nft: Vec<Item>,
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

	pub fn from_flags(bsc_token: &[bool], eth_token: &[bool], eth_nft: &[bool]) -> Self {
		// logic
		let _x = CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH;
		let _y = CRYPTO_SUMMARY_NFT_ADDRESSES_ETH_NFT;

		todo!()
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
		let mut flag_eth_nfg: [bool; 15] = [false; 15];

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
					Self::update_holding_flag(&mut flag_eth_nfg, &details)
				}
			}
		}

		Ok(SummaryHoldings::from_flags(&flag_bsc_token, &flag_eth_token, &flag_eth_nfg))
	}

	fn update_holding_flag<'a, T>(flag_array: &mut [bool], result: &[T])
	where
		T: Serialize + Deserialize<'a>,
	{

		todo!()
	}
}
