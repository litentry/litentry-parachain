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

use std::vec::Vec;
pub mod holding_amount;
pub mod mirror;
pub mod token_balance;

fn match_balance(range: Vec<(&str, &str)>, balance: &str) -> usize {
	let balance = balance.parse::<f64>().unwrap_or_default();
	let mut r_index = range.len();
	for (index, item) in range.iter().enumerate() {
		let low = item.0.parse::<f64>().unwrap();
		let high = item.1.parse::<f64>().unwrap();

		if balance >= low && balance < high {
			r_index = index;

			break
		}
	}

	r_index
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::litentry_profile::holding_amount::ETH_HOLDING_AMOUNT_RANGE;

	#[test]
	fn match_balance_eth_holding_false_works() {
		let balance = "0";
		let index = match_balance(ETH_HOLDING_AMOUNT_RANGE.to_vec(), balance);
		assert_eq!(index, 0);
	}

	#[test]
	fn match_balance_eth_holding_true_works() {
		let balance = "10";
		let index = match_balance(ETH_HOLDING_AMOUNT_RANGE.to_vec(), balance);
		assert_eq!(index, 5);
	}

	#[test]
	fn match_balance_eth_holding_max_works() {
		let balance = "1000000";
		let index = match_balance(ETH_HOLDING_AMOUNT_RANGE.to_vec(), balance);
		assert_eq!(index, ETH_HOLDING_AMOUNT_RANGE.len());
	}
}
