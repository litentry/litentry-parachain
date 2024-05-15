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
use std::vec;

use lc_common::{
	abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls},
	web3_token::{TokenAddress, TokenDecimals},
};
use lc_data_providers::{
	achainable::{AchainableClient, HoldingAmount, Params, ParamsBasicTypeWithAmountToken},
	achainable_names::{AchainableNameAmountToken, GetAchainableName},
	nodereal_jsonrpc::{FungibleApiList, GetTokenBalance20Param, Web3NetworkNoderealJsonrpcClient},
};

use crate::*;

use super::common::calculate_balance_with_decimals;

pub fn get_balance(
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	let mut total_balance = 0_f64;

	loop_with_abort_strategy(
		addresses,
		|(network, address)| match network {
			Web3Network::Bsc | Web3Network::Ethereum => {
				let decimals = Web3TokenType::Lit.get_decimals(*network);
				let param = GetTokenBalance20Param {
					contract_address: Web3TokenType::Lit
						.get_token_address(*network)
						.unwrap_or_default()
						.into(),
					address: address.clone(),
					block_number: "latest".into(),
				};
				match network.create_nodereal_jsonrpc_client(data_provider_config) {
					Some(mut client) => match client.get_token_balance_20(&param, false) {
						Ok(balance) => {
							total_balance += calculate_balance_with_decimals(balance, decimals);
							Ok(LoopControls::Continue)
						},
						Err(err) => Err(err.into_error_detail()),
					},
					None => Ok(LoopControls::Continue),
				}
			},
			Web3Network::Litentry | Web3Network::Litmus => {
				let mut client = AchainableClient::new(data_provider_config);

				let param =
					Params::ParamsBasicTypeWithAmountToken(ParamsBasicTypeWithAmountToken::new(
						AchainableNameAmountToken::BalanceOverAmount.name().into(),
						network,
						"0".into(),
						None,
					));
				match client.holding_amount(vec![address.clone()], param) {
					Ok(balance) => match balance.parse::<f64>() {
						Ok(balance_value) => {
							total_balance += balance_value;
							Ok(LoopControls::Continue)
						},
						Err(_) => Err(Error::ParseError),
					},
					Err(err) => Err(err.into_error_detail()),
				}
			},
			_ => Ok(LoopControls::Continue),
		},
		AbortStrategy::FailFast::<fn(&_) -> bool>,
	)
	.map_err(|errors| errors[0].clone())?;

	Ok(total_balance)
}
