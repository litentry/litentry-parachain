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

use lc_common::web3_token::TokenDecimals;
use lc_data_providers::blockchain_info::{BlockChainInfoClient, BlockChainInfoDataApi};

use crate::*;

use super::common::calculate_balance_with_decimals;

pub fn get_balance(
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	let decimals = Web3TokenType::Btc.get_decimals(addresses[0].0);
	let mut client = BlockChainInfoClient::new(data_provider_config);
	let address_vec: Vec<String> = addresses.into_iter().map(|(_, address)| address).collect();
	let response = client
		.get_multi_addresses(address_vec, false)
		.map_err(|err| err.into_error_detail())?;
	let total_balance = calculate_balance_with_decimals(response.wallet.final_balance, decimals);

	Ok(total_balance)
}
