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

use super::{BalanceRange, BalanceRangeIndex};
use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};
use lc_data_providers::ETokenAddress;
use std::vec::Vec;

// [LIT, USDC, USDT, WBTC]
// (type, description)
const VC_TOKEN_BALANCE_INFOS: [(&str, &str); 1] =
	[("Token holding amount", "The amount of a particular token you are holding")];

const USDT_C_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 5.0, 10.0, 50.0, 100.0, 150.0, 300.0, 500.0, 800.0, 1200.0];
const LIT_TOKEN_BALANCE_RANGE: [f64; 8] = [0.0, 100.0, 200.0, 500.0, 800.0, 1200.0, 1600.0, 3000.0];
const WBTC_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 0.001, 0.05, 0.1, 0.5, 10.0, 50.0, 100.0, 500.0, 1000.0];

pub trait TokenBalanceInfo {
	fn update_token_balance(&mut self, token: ETokenAddress, balance: f64);
}

impl TokenBalanceInfo for Credential {
	fn update_token_balance(&mut self, token: ETokenAddress, balance: f64) {
		let info = get_token_info(&token);
		self.add_subject_info(info.1, info.0);

		update_assertion(token, balance, self);
	}
}

fn update_assertion(token: ETokenAddress, balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	let content = get_assertion_content(&token);
	let range = get_balance_range(&token);
	let index = BalanceRange::index(&range, balance);
	match index {
		Some(index) => {
			let min = format!("{}", range[index]);
			let max = format!("{}", range[index + 1]);
			let min_item = AssertionLogic::new_item(content, Op::GreaterEq, &min);
			let max_item = AssertionLogic::new_item(content, Op::LessThan, &max);

			assertion = assertion.add_item(min_item);
			assertion = assertion.add_item(max_item);

			credential.credential_subject.values.push(index != 0);
		},
		None => {
			let min_item = AssertionLogic::new_item(
				content,
				Op::GreaterEq,
				&format!("{}", get_token_range_last(&token)),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

fn get_token_info(token: &ETokenAddress) -> (&'static str, &'static str) {
	match token {
		ETokenAddress::Lit | ETokenAddress::Usdc | ETokenAddress::Usdt | ETokenAddress::Wbtc =>
			VC_TOKEN_BALANCE_INFOS[0],
		_ => ("UnknownType", ("UnkonwDescription")),
	}
}

fn get_assertion_content(token: &ETokenAddress) -> &'static str {
	match token {
		ETokenAddress::Lit => "$lit_holding_amount",
		ETokenAddress::Usdc => "$usdc_holding_amount",
		ETokenAddress::Usdt => "$usdt_holding_amount",
		ETokenAddress::Wbtc => "$wbtc_holding_amount",
		_ => "Unknown",
	}
}

fn get_balance_range(token: &ETokenAddress) -> Vec<f64> {
	match token {
		ETokenAddress::Lit => LIT_TOKEN_BALANCE_RANGE.to_vec(),
		ETokenAddress::Usdc | ETokenAddress::Usdt => USDT_C_TOKEN_BALANCE_RANGE.to_vec(),
		ETokenAddress::Wbtc => WBTC_TOKEN_BALANCE_RANGE.to_vec(),
		_ => {
			vec![]
		},
	}
}

fn get_token_range_last(token: &ETokenAddress) -> f64 {
	match token {
		ETokenAddress::Lit => *LIT_TOKEN_BALANCE_RANGE.last().unwrap_or(&3000.0),
		ETokenAddress::Usdc | ETokenAddress::Usdt =>
			*USDT_C_TOKEN_BALANCE_RANGE.last().unwrap_or(&1200.0),
		ETokenAddress::Wbtc => *WBTC_TOKEN_BALANCE_RANGE.last().unwrap_or(&1000.0),
		_ => 0.0,
	}
}
