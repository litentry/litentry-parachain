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

use crate::{
	assertion_logic::AssertionLogic, nodereal::bnb_domain::RangeCredentialDetail, Credential,
};
use std::vec::Vec;

// Type / Info
const LIT_HOLDING_AMOUNT_INFO: (&str, &str) =
	("Token holding amount", "The amount of a particular token you are holding");
const LIT_HOLDING_AMOUNT_BREAKDOWN: &str = "$lit_holding_amount";
const LIT_BALANCE_RANGE: [usize; 10] = [0, 1, 50, 100, 200, 500, 800, 1200, 1600, 3000];

pub struct LitHoldingAmount {
	pub amount: usize,
}

impl LitHoldingAmount {
	pub fn new(amount: usize) -> Self {
		Self { amount }
	}
}

impl RangeCredentialDetail for LitHoldingAmount {
	fn get_info(&self) -> (&'static str, &'static str) {
		LIT_HOLDING_AMOUNT_INFO
	}

	fn get_range(&self) -> Vec<usize> {
		LIT_BALANCE_RANGE.to_vec()
	}

	fn get_last_value(&self) -> usize {
		3000
	}

	fn get_breakdown(&self) -> &'static str {
		LIT_HOLDING_AMOUNT_BREAKDOWN
	}
}

pub trait AchainableLitHoldingAmountUpdate {
	fn update_lit_holding_amount(&mut self, balance: usize);
}

impl AchainableLitHoldingAmountUpdate for Credential {
	fn update_lit_holding_amount(&mut self, amount: usize) {
		let lit_holding_amount = LitHoldingAmount::new(amount);
		let items = lit_holding_amount.get_assertion_items(amount);
		let mut assertion = AssertionLogic::new_and();
		for item in items {
			assertion = assertion.add_item(item);
		}

		self.credential_subject.assertions.push(assertion);
		// The credential value should be true if amount > ranges[0].
		self.credential_subject.values.push(amount > lit_holding_amount.get_range()[0]);

		let info = lit_holding_amount.get_info();
		self.add_subject_info(info.1, info.0);
	}
}
