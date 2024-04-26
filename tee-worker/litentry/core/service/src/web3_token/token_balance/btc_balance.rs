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
	web3_token::TokenDecimals,
};
use lc_data_providers::blockchain_info::{BlockChainInfoClient, BlockChainInfoDataApi};

use crate::*;

use super::common::calculate_balance_with_decimals;

pub fn get_balance(
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	let mut total_balance = 0_f64;

	loop_with_abort_strategy(
		addresses,
		|(network, address)| {
			let decimals = Web3TokenType::Btc.get_decimals(*network);
			let mut client = BlockChainInfoClient::new(data_provider_config);

			match client.get_single_address(address.clone(), false) {
				Ok(response) => {
					total_balance +=
						calculate_balance_with_decimals(response.final_balance, decimals);
					Ok(LoopControls::Continue)
				},
				Err(err) => Err(err.into_error_detail()),
			}
		},
		AbortStrategy::FailFast::<fn(&_) -> bool>,
	)?;

	Ok(total_balance)
}
