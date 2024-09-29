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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use core::result::Result;

use lc_common::{
	abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls},
	web3_token::{TokenAddress, TokenDecimals},
};
use lc_data_providers::{
	moralis::{BalanceApiList, MoralisClient},
	nodereal_jsonrpc::{FungibleApiList, GetTokenBalance20Param, Web3NetworkNoderealJsonrpcClient},
};
use log::*;

use crate::*;

use super::common::calculate_balance_with_decimals;

pub fn get_balance(
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	let mut total_balance = 0_f64;

	loop_with_abort_strategy(
		addresses,
		|address| {
			let network = address.0;

			match network {
				Web3Network::Bsc | Web3Network::Ethereum => {
					let decimals = Web3TokenType::Sol.get_decimals(network);
					match network.create_nodereal_jsonrpc_client(data_provider_config) {
						Some(mut client) => {
							let param = GetTokenBalance20Param {
								contract_address: Web3TokenType::Sol
									.get_token_address(network)
									.unwrap_or_default()
									.into(),
								address: address.1.clone(),
								block_number: "latest".into(),
							};
							let result = client.get_token_balance_20(&param, false);
							match result {
								Ok(balance) => {
									total_balance +=
										calculate_balance_with_decimals(balance, decimals);
									Ok(LoopControls::Continue)
								},
								Err(err) => Err(err.into_error_detail()),
							}
						},
						None => Ok(LoopControls::Continue),
					}
				},
				Web3Network::Solana => {
					let mut client = MoralisClient::new(data_provider_config);
					let result =
						client.get_solana_native_balance_by_wallet(address.1.clone(), false);
					match result {
						Ok(response) => match response.solana.parse::<f64>() {
							Ok(balance) => {
								total_balance += balance;
								Ok(LoopControls::Continue)
							},
							Err(err) => {
								error!("Failed to parse {} to f64: {}", response.solana, err);
								Err(Error::ParseError)
							},
						},
						Err(err) => Err(err.into_error_detail()),
					}
				},
				_ => Ok(LoopControls::Continue),
			}
		},
		AbortStrategy::FailFast::<fn(&_) -> bool>,
	)
	.map_err(|errors| errors[0].clone())?;

	Ok(total_balance)
}
