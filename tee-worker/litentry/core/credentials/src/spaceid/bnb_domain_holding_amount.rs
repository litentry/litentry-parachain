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

use super::match_domain_amount;

// VC type / info
const SPACE_ID_BNB_DOMAIN_HOLDING_AMOUNT_INFOS: (&str, &str) =
	("bnb domain holding amount", "The amount of .bnb domain you are holding");

// [x-y)
pub const SPACEID_BNB_DOMAIN_RANGE: [usize; 8] = [0, 1, 5, 10, 20, 50, 100, 200];

pub trait SpaceIDBnbDomainHoldingAmount {
	fn update_spaceid_bnb_holding_amount(&mut self, amount: usize);
}

impl SpaceIDBnbDomainHoldingAmount for Credential {
	fn update_spaceid_bnb_holding_amount(&mut self, amount: usize) {
		self.add_subject_info(
			SPACE_ID_BNB_DOMAIN_HOLDING_AMOUNT_INFOS.1,
			SPACE_ID_BNB_DOMAIN_HOLDING_AMOUNT_INFOS.0,
		);
		update_assertion(self, amount);
	}
}

fn update_assertion(credential: &mut Credential, amount: usize) {
	let index = match_domain_amount(SPACEID_BNB_DOMAIN_RANGE.as_ref(), amount);
	let items = match index {
		Some(index) => {
			let min = SPACEID_BNB_DOMAIN_RANGE[index];
			let max = SPACEID_BNB_DOMAIN_RANGE[index + 1];
			let min_item = AssertionLogic::new_item(
				"$bnb_domain_holding_amount",
				Op::GreaterEq,
				&format!("{}", min),
			);
			let max_item = AssertionLogic::new_item(
				"$bnb_domain_holding_amount",
				Op::LessThan,
				&format!("{}", max),
			);

			vec![min_item, max_item]
		},
		None => {
			// >= 200
			let min_item = AssertionLogic::new_item(
				"$bnb_domain_holding_amount",
				Op::GreaterEq,
				&format!("{}", 200),
			);
			vec![min_item]
		},
	};

	let mut assertion = AssertionLogic::new_and();
	for item in items {
		assertion = assertion.add_item(item);
	}

	credential.credential_subject.assertions.push(assertion);
	credential.credential_subject.values.push(index != Some(0));
}
