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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use super::{BalanceRange, BalanceRangeIndex};
use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};

// (type, description)
const VC_HOLDING_AMOUNT_INFOS: [(&str, &str); 1] =
	[("Token holding amount", "The amount of a particular token you are holding")];

pub const ETH_HOLDING_AMOUNT_RANGE: [f64; 10] =
	[0.0, 0.01, 0.5, 1.0, 5.0, 10.0, 50.0, 100.0, 500.0, 1000.0];

pub trait LitentryProfileHoldingAmount {
	fn update_eth_holding_amount(&mut self, balance: f64);
}

impl LitentryProfileHoldingAmount for Credential {
	fn update_eth_holding_amount(&mut self, balance: f64) {
		self.add_subject_info(VC_HOLDING_AMOUNT_INFOS[0].1, VC_HOLDING_AMOUNT_INFOS[0].0);

		update_assertion(balance, self);
	}
}

fn update_assertion(balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	let assertion_content = get_assertion_content();
	let index = BalanceRange::index(&ETH_HOLDING_AMOUNT_RANGE, balance);
	match index {
		Some(index) => {
			let min = format!("{}", ETH_HOLDING_AMOUNT_RANGE[index]);
			let max = format!("{}", ETH_HOLDING_AMOUNT_RANGE[index + 1]);
			let min_item = AssertionLogic::new_item(assertion_content, Op::GreaterEq, &min);
			let max_item = AssertionLogic::new_item(assertion_content, Op::LessThan, &max);

			assertion = assertion.add_item(min_item);
			assertion = assertion.add_item(max_item);

			credential.credential_subject.values.push(index != 0);
		},
		None => {
			let min_item = AssertionLogic::new_item(
				assertion_content,
				Op::GreaterEq,
				&format!("{}", ETH_HOLDING_AMOUNT_RANGE.last().unwrap_or(&1000.0)),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

fn get_assertion_content() -> &'static str {
	"$eth_holding_amount"
}
