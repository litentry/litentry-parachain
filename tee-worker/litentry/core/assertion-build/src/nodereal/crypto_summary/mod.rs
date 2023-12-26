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

mod crypto_summary;

use crate::*;
pub use crypto_summary::*;
use lc_credentials::nodereal::crypto_summary::{CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH, CRYPTO_SUMMARY_NFT_ADDRESSES_ETH_NFT};
use lc_data_providers::nodereal_jsonrpc::{NoderealJsonrpcClient, NoderealChain, FungibleApiList};
use litentry_primitives::{ErrorDetail, Assertion};
use serde::{Deserialize, Serialize};
use std::{
    string::String,
    vec::Vec,
};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct NameAndAddress {
    pub name: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub network: String,
    pub list: Vec<NameAndAddress>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAndNFT {
    pub token: Vec<Item>,
    pub nft: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryHoldings {
	#[serde(rename = "SUMMARRY")]
    pub summary: TokenAndNFT,
}

impl SummaryHoldings {
    pub fn is_empty(&self) -> bool {
        self.summary.token.is_empty() && self.summary.nft.is_empty()
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

        Self {
            eth_client,
            bsc_client,
        }
    }

    pub fn logic(&mut self, addresses: Vec<String>) -> core::result::Result<SummaryHoldings, ErrorDetail> {

        // Handle Ethererum network
        let response = self.eth_client.get_token_holdings(&address).map_err(|e| {
            Error::RequestVCFailed(
                Assertion::CryptoSummary,
                ErrorDetail::DataProviderError(e),
            )
        })?;

        // logic
        let _x = CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH;
        let _y = CRYPTO_SUMMARY_NFT_ADDRESSES_ETH_NFT;
        
        todo!()
    }
}