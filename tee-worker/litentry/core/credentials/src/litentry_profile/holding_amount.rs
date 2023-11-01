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

use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};

use super::match_balance;

// (type, description)
const VC_HOLDING_AMOUNT_INFOS: [(&str, &str); 1] =
	[("REPLACE_ME_ETH_HOLDING_AMOUNT_TYPE", "REPLACE_ME_ETH_HOLDING_AMOUNT_DESC")];

// ETH Holding Amount Range [0.01-0.5)
pub const ETH_HOLDING_AMOUNT_RANGE: [(&str, &str); 9] = [
	("0.0", "0.01"), // False
	("0.01", "0.5"),
	("0.5", "1.0"),
	("1.0", "5.0"),
	("5.0", "10.0"),
	("10.0", "50.0"),
	("50.0", "100.0"),
	("100.0", "500.0"),
	("500.0", "1000.0"),
];

pub trait LitentryProfileHoldingAmount {
	fn update_eth_holding_amount(&mut self, balance: &str);
}

impl LitentryProfileHoldingAmount for Credential {
	fn update_eth_holding_amount(&mut self, balance: &str) {
		let mut assertion = AssertionLogic::new_and();

		let index = match_balance(ETH_HOLDING_AMOUNT_RANGE.to_vec(), balance);
		match ETH_HOLDING_AMOUNT_RANGE.get(index) {
			Some((min, max)) => {
				let min_item = AssertionLogic::new_item("$eth_holding_amount", Op::GreaterEq, min);
				let max_item = AssertionLogic::new_item("$eth_holding_amount", Op::LessThan, max);

				assertion = assertion.add_item(min_item);
				assertion = assertion.add_item(max_item);
			},
			// >= 1000.0
			None => {
				let min_item = AssertionLogic::new_item(
					"$eth_holding_amount",
					Op::GreaterEq,
					&format!("{}", 1000.0),
				);
				assertion = assertion.add_item(min_item);
			},
		}

		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(index != 0);

		self.add_subject_info(VC_HOLDING_AMOUNT_INFOS[0].1, VC_HOLDING_AMOUNT_INFOS[0].0);
	}
}
