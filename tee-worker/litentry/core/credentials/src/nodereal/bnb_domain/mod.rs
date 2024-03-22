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

use crate::assertion_logic::{AssertionLogic, Op};
use core::fmt::{Debug, Display};
use std::vec::Vec;

pub mod bnb_digit_domain_club_amount;
pub mod bnb_domain_holding_amount;

pub trait RangeCredentialDetail {
	fn get_info(&self) -> (&'static str, &'static str);
	fn get_range(&self) -> Vec<usize>;
	fn get_last_value(&self) -> usize;
	fn get_breakdown(&self) -> &'static str;

	fn get_index(&self, amount: usize) -> Option<usize> {
		let range = self.get_range();
		let index = match_range_index(range.as_ref(), amount);
		index
	}

	fn get_assertion_items(&self, amount: usize) -> Vec<AssertionLogic> {
		let breakdown = self.get_breakdown();
		let range = self.get_range();
		let index = self.get_index(amount);
		match index {
			Some(index) => {
				let min = range[index - 1];
				let max = range[index];
				let min_item =
					AssertionLogic::new_item(breakdown, Op::GreaterEq, &format!("{}", min));
				let max_item =
					AssertionLogic::new_item(breakdown, Op::LessThan, &format!("{}", max));

				vec![min_item, max_item]
			},
			None => {
				// >= last value
				let min_item = AssertionLogic::new_item(
					breakdown,
					Op::GreaterEq,
					&format!("{}", self.get_last_value()),
				);
				vec![min_item]
			},
		}
	}
}

pub fn match_range_index<T>(range: &[T], amount: T) -> Option<usize>
where
	T: std::cmp::PartialOrd + Display + Debug + Copy,
{
	range.iter().position(|item| amount < *item)
}

#[cfg(test)]
mod tests {
	use super::{bnb_domain_holding_amount::BNB_DOMAIN_HOLDING_AMOUNT_RANGE, *};

	#[test]
	fn match_domain_amount_0_works() {
		let amount = 0;
		let index = match_range_index(&BNB_DOMAIN_HOLDING_AMOUNT_RANGE.to_vec(), amount);
		assert_eq!(index.unwrap(), 1);
	}

	#[test]
	fn match_domain_amount_mid_works() {
		let amount = 20;
		let index = match_range_index(&BNB_DOMAIN_HOLDING_AMOUNT_RANGE.to_vec(), amount);
		assert_eq!(index.unwrap(), 5);
	}

	#[test]
	fn match_domain_amount_last_works() {
		let amount = 300;
		let index = match_range_index(&BNB_DOMAIN_HOLDING_AMOUNT_RANGE.to_vec(), amount);
		assert_eq!(index, None);
	}
}
