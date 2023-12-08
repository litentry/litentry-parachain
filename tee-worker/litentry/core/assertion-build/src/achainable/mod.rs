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

use self::{
	amount::build_amount, amount_holding::build_amount_holding, amount_token::build_amount_token,
	amounts::build_amounts, basic::build_basic, between_percents::build_between_percents,
	class_of_year::build_class_of_year, date::build_date, date_interval::build_date_interval,
	date_percent::build_date_percent, mirror::build_on_mirror, token::build_token,
};
use crate::*;
use lc_data_providers::{
	achainable::{
		AchainableClient, AchainableTagDeFi, HoldingAmount, Params, ParamsBasicTypeWithAmountToken,
	},
	achainable_names::{AchainableNameAmountToken, GetAchainableName},
	DataProviderConfigReader, ReadDataProviderConfig, LIT_TOKEN_ADDRESS,
};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::AchainableParams;

pub mod amount;
pub mod amount_holding;
pub mod amount_token;
pub mod amounts;
pub mod basic;
pub mod between_percents;
pub mod class_of_year;
pub mod date;
pub mod date_interval;
pub mod date_percent;
pub mod mirror;
pub mod token;

pub fn build(req: &AssertionBuildRequest, param: AchainableParams) -> Result<Credential> {
	match param {
		AchainableParams::AmountHolding(param) => build_amount_holding(req, param),
		AchainableParams::AmountToken(param) => build_amount_token(req, param),
		AchainableParams::Amount(param) => build_amount(req, param),
		AchainableParams::Amounts(param) => build_amounts(req, param),
		AchainableParams::Basic(param) => build_basic(req, param),
		AchainableParams::BetweenPercents(param) => build_between_percents(req, param),
		AchainableParams::ClassOfYear(param) => build_class_of_year(req, param),
		AchainableParams::DateInterval(param) => build_date_interval(req, param),
		AchainableParams::DatePercent(param) => build_date_percent(req, param),
		AchainableParams::Date(param) => build_date(req, param),
		AchainableParams::Token(param) => build_token(req, param),
		AchainableParams::Mirror(param) => build_on_mirror(req, param),
	}
}

pub fn request_achainable(addresses: Vec<String>, param: AchainableParams) -> Result<bool> {
	let request_param = Params::try_from(param.clone())?;

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::Achainable(param.clone()), e))?;
	let mut client: AchainableClient = AchainableClient::new(&data_provider_config);

	for address in &addresses {
		let ret = client.query_system_label(address, request_param.clone()).map_err(|e| {
			error!("Request achainable failed {:?}", e);

			Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
		})?;

		if ret {
			return Ok(true)
		}
	}

	Ok(false)
}

pub fn request_uniswap_v2_or_v3_user(
	addresses: Vec<String>,
	param: AchainableParams,
) -> Result<(bool, bool)> {
	let _request_param = Params::try_from(param.clone())?;
	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::Achainable(param.clone()), e))?;

	let mut client: AchainableClient = AchainableClient::new(&data_provider_config);

	let mut v2_user = false;
	let mut v3_user = false;
	for address in &addresses {
		v2_user |= client.uniswap_v2_user(address).map_err(|e| {
			Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
		})?;

		v3_user |= client.uniswap_v3_user(address).map_err(|e| {
			Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
		})?
	}

	Ok((v2_user, v3_user))
}

const INVALID_CLASS_OF_YEAR: &str = "Invalid";
pub fn request_achainable_classofyear(
	addresses: Vec<String>,
	param: AchainableParams,
) -> Result<(bool, String)> {
	let request_param = Params::try_from(param.clone())?;

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::Achainable(param.clone()), e))?;
	let mut client: AchainableClient = AchainableClient::new(&data_provider_config);

	let mut longest_created_year = INVALID_CLASS_OF_YEAR.into();
	for address in &addresses {
		let year = client.query_class_of_year(address, request_param.clone()).map_err(|e| {
			Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
		})?;

		// In some cases,the metadata field TDF will return null, so if there is a parsing error, we need to continue requesting the next address
		if year.parse::<u32>().is_err() {
			continue
		}

		if year < longest_created_year {
			longest_created_year = year;
		}
	}

	Ok((longest_created_year.parse::<u32>().is_ok(), longest_created_year))
}

pub fn request_achainable_balance(
	addresses: Vec<String>,
	param: AchainableParams,
) -> Result<String> {
	let request_param = Params::try_from(param.clone())?;

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::Achainable(param.clone()), e))?;
	let mut client: AchainableClient = AchainableClient::new(&data_provider_config);
	let balance = client.holding_amount(addresses, request_param).map_err(|e| {
		Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
	})?;

	Ok(balance)
}

pub fn query_lit_holding_amount(
	aparam: &AchainableParams,
	identities: &Vec<(Web3Network, Vec<String>)>,
) -> Result<usize> {
	let mut total_lit_balance = 0;

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::Achainable(aparam.clone()), e))?;
	let mut client: AchainableClient = AchainableClient::new(&data_provider_config);

	for (network, addresses) in identities {
		let q_param = if *network == Web3Network::Ethereum {
			let param = ParamsBasicTypeWithAmountToken::new(
				AchainableNameAmountToken::ERC20BalanceOverAmount.name().to_string(),
				&Web3Network::Ethereum,
				"0".to_string(),
				Some(LIT_TOKEN_ADDRESS.to_string()),
			);
			param
		} else if *network == Web3Network::Bsc {
			let param = ParamsBasicTypeWithAmountToken::new(
				AchainableNameAmountToken::BEP20BalanceOverAmount.name().to_string(),
				&Web3Network::Bsc,
				"0".to_string(),
				Some(LIT_TOKEN_ADDRESS.to_string()),
			);

			param
		} else if *network == Web3Network::Litentry {
			let param = ParamsBasicTypeWithAmountToken::new(
				AchainableNameAmountToken::BalanceOverAmount.name().to_string(),
				&Web3Network::Litentry,
				"0".to_string(),
				None,
			);

			param
		} else if *network == Web3Network::Litmus {
			let param = ParamsBasicTypeWithAmountToken::new(
				AchainableNameAmountToken::BalanceOverAmount.name().to_string(),
				&Web3Network::Litmus,
				"0".to_string(),
				None,
			);

			param
		} else {
			continue
		};

		let params = Params::ParamsBasicTypeWithAmountToken(q_param);
		let balance = client
			.holding_amount(addresses.clone(), params)
			.map_err(|e| {
				Error::RequestVCFailed(Assertion::Achainable(aparam.clone()), e.into_error_detail())
			})?
			.parse::<usize>()
			.map_err(|_| {
				Error::RequestVCFailed(
					Assertion::Achainable(aparam.clone()),
					ErrorDetail::ParseError,
				)
			})?;

		total_lit_balance += balance;
	}

	Ok(total_lit_balance)
}
