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

use super::match_balance;
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

// USDC/USDT TOKEN_BALANCE Range [0.01-0.5)
// Including >= 1200
const USDT_C_TOKEN_BALANCE_RANGE: [(&str, &str); 9] = [
	("0", "5"), // FALSE
	("5", "10"),
	("10", "50"),
	("50", "100"),
	("100", "150"),
	("150", "300"),
	("300", "500"),
	("500", "800"),
	("800", "1200"),
];

// LIT TOKEN_BALANCE Range [0.01-0.5)
// Including >= 3000
const LIT_TOKEN_BALANCE_RANGE: [(&str, &str); 7] = [
	("0", "100"), // FALSE
	("100", "200"),
	("200", "500"),
	("500", "800"),
	("800", "1200"),
	("1200", "1600"),
	("1600", "3000"),
];

// WBTC
// Including >= 1000
const WBTC_TOKEN_BALANCE_RANGE: [(&str, &str); 8] = [
	("0", "0.001"), // FALSE
	("0.05", "0.1"),
	("0.1", "0.5"),
	("5.0", "10"),
	("10", "50"),
	("50", "100"),
	("100", "500"),
	("500", "1000"),
];

pub trait TokenBalanceInfo {
	fn update_token_balance(&mut self, token: ETokenAddress, balance: &str);
}

impl TokenBalanceInfo for Credential {
	fn update_token_balance(&mut self, token: ETokenAddress, balance: &str) {
		let info = get_token_info(&token);
		self.add_subject_info(info.1, info.0);

		update_assertion(token, balance, self);
	}
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

fn get_token_range(token: &ETokenAddress) -> Vec<(&'static str, &'static str)> {
	match token {
		ETokenAddress::Lit => LIT_TOKEN_BALANCE_RANGE.to_vec(),
		ETokenAddress::Usdc | ETokenAddress::Usdt => USDT_C_TOKEN_BALANCE_RANGE.to_vec(),
		ETokenAddress::Wbtc => WBTC_TOKEN_BALANCE_RANGE.to_vec(),
		_ => {
			vec![]
		},
	}
}

fn get_token_range_last(token: &ETokenAddress) -> &'static str {
	match token {
		ETokenAddress::Lit => LIT_TOKEN_BALANCE_RANGE.last().unwrap().1,
		ETokenAddress::Usdc => USDT_C_TOKEN_BALANCE_RANGE.last().unwrap().1,
		ETokenAddress::Usdt => USDT_C_TOKEN_BALANCE_RANGE.last().unwrap().1,
		ETokenAddress::Wbtc => WBTC_TOKEN_BALANCE_RANGE.last().unwrap().1,
		_ => "Unknown",
	}
}

fn update_assertion(token: ETokenAddress, balance: &str, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	let content = get_assertion_content(&token);
	let range = get_token_range(&token);
	let index = match_balance(range.clone(), balance);
	match range.get(index) {
		Some((min, max)) => {
			let min_item = AssertionLogic::new_item(content, Op::GreaterEq, min);
			let max_item = AssertionLogic::new_item(content, Op::LessThan, max);

			assertion = assertion.add_item(min_item);
			assertion = assertion.add_item(max_item);
		},
		None => {
			let min_item =
				AssertionLogic::new_item(content, Op::GreaterEq, get_token_range_last(&token));
			assertion = assertion.add_item(min_item);
		},
	}

	credential.credential_subject.assertions.push(assertion);
	credential.credential_subject.values.push(index != 0);
}
