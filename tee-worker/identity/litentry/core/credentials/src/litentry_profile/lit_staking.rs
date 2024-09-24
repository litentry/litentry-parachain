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

// VC type / info
const LIT_STAKING_INFOS: (&str, &str) = ("LIT staking amount", "The amount of LIT you are staking");

// [x-y)
pub const LIT_STAKING_AMOUNT_RANGE: [usize; 10] = [0, 1, 50, 100, 200, 500, 800, 1200, 1600, 3000];

pub struct LITStakingAmount {
	pub amount: u128,
}

impl LITStakingAmount {
	pub fn new(amount: u128) -> Self {
		Self { amount }
	}
}

impl RangeCredentialDetail for LITStakingAmount {
	fn get_info(&self) -> (&'static str, &'static str) {
		LIT_STAKING_INFOS
	}

	fn get_range(&self) -> Vec<usize> {
		LIT_STAKING_AMOUNT_RANGE.to_vec()
	}

	fn get_last_value(&self) -> usize {
		3000
	}

	fn get_breakdown(&self) -> &'static str {
		"$lit_staking_amount"
	}
}

pub trait UpdateLITStakingAmountCredential {
	fn update_lit_staking_amount(&mut self, amount: u128);
}

impl UpdateLITStakingAmountCredential for Credential {
	fn update_lit_staking_amount(&mut self, amount: u128) {
		let lit_staking = LITStakingAmount::new(amount);
		let items = lit_staking.get_assertion_items(amount as usize);
		let mut assertion = AssertionLogic::new_and();
		for item in items {
			assertion = assertion.add_item(item);
		}

		self.credential_subject.assertions.push(assertion);
		// The credential value should be true if amount > ranges[0].
		self.credential_subject.values.push(amount > lit_staking.get_range()[0] as u128);

		let info = lit_staking.get_info();
		self.add_subject_info(info.1, info.0);
	}
}
