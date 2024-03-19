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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use litentry_primitives::ErrorDetail as Error;
use std::vec::Vec;

pub enum AbortStrategy<F> {
	// Immediately terminate the loop
	FailFast,
	// Terminate the loop based on the predicate function
	ContinueUntil(F),
	// Continue the loop until the end of the items
	ContinueUntilEnd,
}

pub fn loop_with_abort_strategy<F, T>(
	items: Vec<T>,
	mut action: impl FnMut(&T) -> Result<bool, Error>, // Closure to perform action, returns a boolean indicating whether to exit the loop
	abort_strategy: AbortStrategy<F>,                  // Control when to abort the loop
) -> Result<(), Error>
where
	F: Fn(&T) -> bool, // Type of the predicate function, takes a parameter of type T and returns a boolean
{
	for (index, item) in items.iter().enumerate() {
		match action(item) {
			Ok(exit_loop) => {
				// If the action returns true, break the loop immediately
				if exit_loop {
					break
				}
			},
			Err(err) => match abort_strategy {
				AbortStrategy::FailFast => return Err(err), // If FailFast is chosen, return the error immediately
				AbortStrategy::ContinueUntil(ref predicate) => {
					// If ContinueUntil is chosen, decide whether to return the error based on the predicate function
					if predicate(item) {
						return Err(err)
					}
				},
				AbortStrategy::ContinueUntilEnd => {
					// If ContinueUntilEnd is chosen, return the error when processing the last item
					if index == items.len() - 1 {
						return Err(err)
					}
				},
			},
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use litentry_primitives::ErrorString;

	use super::*;

	#[test]
	fn test_break() {
		let test_array = vec!["item1", "item2", "item3", "item4"];
		let mut result = "";

		let loop_result = loop_with_abort_strategy(
			test_array,
			|item| {
				result = *item;
				if *item == "item2" {
					return Ok(true)
				}

				Ok(false)
			},
			AbortStrategy::FailFast::<fn(&_) -> bool>,
		);

		assert_eq!(loop_result.err(), None);
		assert_eq!(result, "item2");
	}

	#[test]
	fn test_fail_fast() {
		let test_array = vec!["item1", "item2", "item3", "item4"];
		let mut result = "";

		let loop_result = loop_with_abort_strategy(
			test_array,
			|item| {
				result = *item;
				if *item == "item2" {
					return Err(Error::DataProviderError(ErrorString::truncate_from(
						"test error".as_bytes().to_vec(),
					)))
				}

				Ok(false)
			},
			AbortStrategy::FailFast::<fn(&_) -> bool>,
		);

		assert_ne!(loop_result.err(), None);
		assert_eq!(result, "item2");
	}

	#[test]
	fn test_continue_until() {
		let test_array = vec!["item1", "item2", "item3", "item4"];
		let mut result = "";

		let loop_result = loop_with_abort_strategy(
			test_array,
			|item| {
				result = *item;

				Err(Error::DataProviderError(ErrorString::truncate_from(
					"test error".as_bytes().to_vec(),
				)))
			},
			AbortStrategy::ContinueUntil(|item: &&str| *item == "item3"),
		);

		assert_ne!(loop_result.err(), None);
		assert_eq!(result, "item3");
	}

	#[test]
	fn test_continue_until_end_success() {
		let test_array = vec!["item1", "item2", "item3", "item4"];
		let mut result = "";

		let loop_result = loop_with_abort_strategy(
			test_array,
			|item| {
				result = *item;

				if *item == "item4" {
					return Ok(false)
				}
				Err(Error::DataProviderError(ErrorString::truncate_from(
					"test error".as_bytes().to_vec(),
				)))
			},
			AbortStrategy::ContinueUntilEnd::<fn(&_) -> bool>,
		);

		assert_eq!(loop_result.err(), None);
		assert_eq!(result, "item4");
	}

	#[test]
	fn test_continue_until_end_failure() {
		let test_array = vec!["item1", "item2", "item3", "item4"];
		let mut result = "";

		let loop_result = loop_with_abort_strategy(
			test_array,
			|item| {
				result = *item;

				Err(Error::DataProviderError(ErrorString::truncate_from(
					"test error".as_bytes().to_vec(),
				)))
			},
			AbortStrategy::ContinueUntilEnd::<fn(&_) -> bool>,
		);

		assert_ne!(loop_result.err(), None);
		assert_eq!(result, "item4");
	}
}
