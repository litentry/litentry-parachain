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
	format_assertion_to_date, Credential,
};
use litentry_primitives::AmountHoldingTimeType;
use std::string::{String, ToString};

const VC_AMOUNT_HOLDING_TIME_DESCRIPTIONS: &str = "The length of time a user continues to hold a particular token (with particular threshold of token amount)";
const VC_AMOUNT_HOLDING_TIME_TYPES: [&str; 4] =
	["LIT Holding Time", "DOT Holding Time", "WBTC Holding Time", "ETH Holding Time"];

pub trait GetAmountHoldingTimeInfo {
	fn get_info(&self) -> (String, String);
}

impl GetAmountHoldingTimeInfo for AmountHoldingTimeType {
	fn get_info(&self) -> (String, String) {
		let index = match self {
			AmountHoldingTimeType::LIT => 0,
			AmountHoldingTimeType::DOT => 1,
			AmountHoldingTimeType::WBTC => 2,
			AmountHoldingTimeType::ETH => 3,
		};

		(
			VC_AMOUNT_HOLDING_TIME_DESCRIPTIONS.to_string(),
			VC_AMOUNT_HOLDING_TIME_TYPES[index].to_string(),
		)
	}
}

pub trait AchainableAmountHoldingTimeUpdate {
	fn update_amount_holding_time_credential(
		&mut self,
		holding_type: &AmountHoldingTimeType,
		value: bool,
		minimum_amount: &str,
		from_date: &str,
	);
}

impl AchainableAmountHoldingTimeUpdate for Credential {
	fn update_amount_holding_time_credential(
		&mut self,
		holding_type: &AmountHoldingTimeType,
		value: bool,
		minimum_amount: &str,
		from_date: &str,
	) {
		// from_date's Op is ALWAYS Op::LessThan
		let from_date_logic = AssertionLogic::new_item("$from_date", Op::LessThan, from_date);

		// minimum_amount' Op is ALWAYS Op::Equal
		let minimum_amount_logic =
			AssertionLogic::new_item("$minimum_amount", Op::Equal, minimum_amount);

		// to_date's Op is ALWAYS Op::GreaterEq
		let to_date = format_assertion_to_date();
		let to_date_logic = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);

		let assertion = AssertionLogic::new_and()
			.add_item(minimum_amount_logic)
			.add_item(from_date_logic)
			.add_item(to_date_logic);

		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		self.add_subject_info(&holding_type.get_info().0, &holding_type.get_info().1);
	}
}
