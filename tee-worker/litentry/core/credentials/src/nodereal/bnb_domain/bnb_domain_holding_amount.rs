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

use super::RangeCredentialDetail;
use crate::{assertion_logic::AssertionLogic, Credential};
use std::vec::Vec;

// VC type / info
const BNB_DOMAIN_HOLDING_AMOUNT_INFOS: (&str, &str) =
	("bnb domain holding amount", "The amount of .bnb domain you are holding");

// [x-y)
pub const BNB_DOMAIN_HOLDING_AMOUNT_RANGE: [usize; 8] = [0, 1, 5, 10, 20, 50, 100, 200];

pub struct BnbDomainHoldingAmount {
	pub amount: usize,
}

impl BnbDomainHoldingAmount {
	pub fn new(amount: usize) -> Self {
		Self { amount }
	}
}

impl RangeCredentialDetail for BnbDomainHoldingAmount {
	fn get_info(&self) -> (&'static str, &'static str) {
		BNB_DOMAIN_HOLDING_AMOUNT_INFOS
	}

	fn get_range(&self) -> Vec<usize> {
		BNB_DOMAIN_HOLDING_AMOUNT_RANGE.to_vec()
	}

	fn get_last_value(&self) -> usize {
		200
	}

	fn get_breakdown(&self) -> &'static str {
		"$bnb_domain_holding_amount"
	}
}

pub trait UpdateBnbDomainHoldingAmountCredential {
	fn update_bnb_holding_amount(&mut self, amount: usize);
}

impl UpdateBnbDomainHoldingAmountCredential for Credential {
	fn update_bnb_holding_amount(&mut self, amount: usize) {
		let bnb_amount = BnbDomainHoldingAmount::new(amount);
		let items = bnb_amount.get_assertion_items(amount);
		let mut assertion = AssertionLogic::new_and();
		for item in items {
			assertion = assertion.add_item(item);
		}

		self.credential_subject.assertions.push(assertion);
		// The credential value should be true if amount > ranges[0].
		self.credential_subject.values.push(amount > bnb_amount.get_range()[0]);

		let info = bnb_amount.get_info();
		self.add_subject_info(info.1, info.0);
	}
}
