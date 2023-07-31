// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::*;
use lc_data_providers::achainable::{AchainableClient, Params};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::AchainableParams;

use self::{
	achainable_amount::build_amount, achainable_amount_holding::build_amount_holding,
	achainable_amount_token::build_amount_token, achainable_amounts::build_amounts,
	achainable_basic::build_basic, achainable_between_percents::build_between_percents,
	achainable_class_of_year::build_class_of_year, achainable_date::build_date,
	achainable_date_interval::build_date_interval, achainable_date_percent::build_date_percent,
	achainable_token::build_token,
};

pub mod achainable_amount;
pub mod achainable_amount_holding;
pub mod achainable_amount_token;
pub mod achainable_amounts;
pub mod achainable_basic;
pub mod achainable_between_percents;
pub mod achainable_class_of_year;
pub mod achainable_date;
pub mod achainable_date_interval;
pub mod achainable_date_percent;
pub mod achainable_token;

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
	}
}

pub fn request_achainable(addresses: Vec<String>, param: Params) -> Result<bool> {
	let mut client: AchainableClient = AchainableClient::new();

	let mut flag = false;
	for address in &addresses {
		if flag {
			break
		}

		let ret = client.query_system_label(address, param.clone());
		match ret {
			Ok(r) => flag = r,
			Err(e) => error!("Request achainable failed {:?}", e),
		}
	}

	Ok(flag)
}
