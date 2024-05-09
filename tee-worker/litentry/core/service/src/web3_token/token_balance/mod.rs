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

use crate::*;

mod bnb_balance;
mod btc_balance;
mod common;
mod eth_balance;
mod lit_balance;
mod sol_balance;

pub fn get_token_balance(
	token_type: Web3TokenType,
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	match token_type {
		Web3TokenType::Bnb => bnb_balance::get_balance(addresses, data_provider_config),
		Web3TokenType::Eth => eth_balance::get_balance(addresses, data_provider_config),
		Web3TokenType::Lit => lit_balance::get_balance(addresses, data_provider_config),
		Web3TokenType::Sol => sol_balance::get_balance(addresses, data_provider_config),
		Web3TokenType::Btc => btc_balance::get_balance(addresses, data_provider_config),
		_ => common::get_balance(addresses, token_type, data_provider_config),
	}
}
