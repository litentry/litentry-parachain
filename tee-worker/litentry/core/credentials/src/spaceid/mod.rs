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

use core::fmt::{Debug, Display};

pub mod bnb_domain_holding_amount;

pub fn match_domain_amount<T>(range: &[T], amount: T) -> Option<usize>
where
	T: std::cmp::PartialOrd + Display + Debug,
{
	let mut index = None;

	for (idx, item) in range.iter().enumerate() {
		if amount < *item {
			index = Some(idx - 1);
			break
		}
	}

	index
}

#[cfg(test)]
mod tests {
	use super::{bnb_domain_holding_amount::SPACEID_BNB_DOMAIN_RANGE, *};

	#[test]
	fn match_domain_amount_0_works() {
		let amount = 0;
		let index = match_domain_amount(&SPACEID_BNB_DOMAIN_RANGE.to_vec(), amount);
		assert_eq!(index.unwrap(), 0);
	}

	#[test]
	fn match_domain_amount_mid_works() {
		let amount = 20;
		let index = match_domain_amount(&SPACEID_BNB_DOMAIN_RANGE.to_vec(), amount);
		assert_eq!(index.unwrap(), 4);
	}

	#[test]
	fn match_domain_amount_last_works() {
		let amount = 300;
		let index = match_domain_amount(&SPACEID_BNB_DOMAIN_RANGE.to_vec(), amount);
		assert_eq!(index, None);
	}
}
