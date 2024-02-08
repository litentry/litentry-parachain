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

use lc_common::web3_token::{TokenAddress, TokenDecimals};
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

	for address in addresses.iter() {
		let network = address.0;
		let account_address = address.1.clone();
		match network {
			Web3Network::Bsc | Web3Network::Ethereum => {
				let decimals = Web3TokenType::Lit.get_decimals(network);
				let param = GetTokenBalance20Param {
					contract_address: Web3TokenType::Lit
						.get_token_address(address.0)
						.unwrap_or_default()
						.into(),
					address: account_address,
					block_number: "latest".into(),
				};

				if let Some(mut client) =
					network.create_nodereal_jsonrpc_client(data_provider_config)
				{
					match client.get_token_balance_20(&param) {
						Ok(balance) => {
							total_balance += calculate_balance_with_decimals(balance, decimals);
						},
						Err(err) => return Err(err.into_error_detail()),
					}
				}
			},
			Web3Network::Litentry | Web3Network::Litmus => {
				let mut client = AchainableClient::new(data_provider_config)
					.map_err(|e| e.into_error_detail())?;

				let param =
					Params::ParamsBasicTypeWithAmountToken(ParamsBasicTypeWithAmountToken::new(
						AchainableNameAmountToken::BalanceOverAmount.name().into(),
						&network,
						"0".into(),
						None,
					));
				match client.holding_amount(vec![account_address], param) {
					Ok(balance) => match balance.parse::<f64>() {
						Ok(balance_value) => {
							total_balance += balance_value;
						},
						Err(_) => return Err(Error::ParseError),
					},
					Err(err) => return Err(err.into_error_detail()),
				}
			},
			_ => {},
		}
	}

	Ok(total_balance)
}
