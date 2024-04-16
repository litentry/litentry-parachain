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

use crate::*;
use itc_rest_client::http_client::SendHttpRequest;

http_get_precompile_fn!(http_get_bool, Bool, as_bool);
http_get_precompile_fn!(http_get_i64, Uint, as_i64);
http_get_precompile_fn!(http_get_string, String, as_str);

#[cfg(test)]
pub mod test {
	use crate::precompiles::{
		http_get::{http_get_bool, http_get_i64, http_get_string},
		mocks::MockedHttpClient,
		PrecompileResult,
	};
	use ethabi::ethereum_types::U256;
	use evm::{executor::stack::PrecompileFailure, ExitError, ExitSucceed};

	#[test]
	pub fn test_get_bool() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("https://www.litentry.com/", "/bool");

		// when
		let result = http_get_bool(data, client).unwrap();

		// then
		assert!(matches!(result.exit_status, ExitSucceed::Returned));
		assert_eq!(ethabi::encode(&[ethabi::Token::Bool(true)]), result.output)
	}

	#[test]
	pub fn test_get_i64() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("https://www.litentry.com/", "/i64");

		// when
		let result = http_get_i64(data, client).unwrap();

		// then
		assert!(matches!(result.exit_status, ExitSucceed::Returned));
		assert_eq!(
			ethabi::encode(&[ethabi::Token::Uint(U256::try_from(10).unwrap())]),
			result.output
		)
	}

	#[test]
	pub fn test_get_string() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("https://www.litentry.com/", "/string");

		// when
		let result = http_get_string(data, client).unwrap();

		// then
		assert!(matches!(result.exit_status, ExitSucceed::Returned));
		assert_eq!(ethabi::encode(&[ethabi::Token::String("string".to_string())]), result.output)
	}

	#[test]
	pub fn returns_error_for_invalid_url() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("invalid_url", "/string");

		// when
		let result = http_get_string(data, client);

		// then
		assert_exit_status_reason(
			&result,
			"Could not parse url \"invalid_url\", reason: RelativeUrlWithoutBase",
		);
	}

	#[test]
	pub fn returns_error_for_invalid_json_pointer() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("https://www.litentry.com/", "invalid_pointer");

		// when
		let result = http_get_string(data, client);

		// then
		assert_exit_status_reason(&result, "No value under given pointer: :\"invalid_pointer\"");
	}

	#[test]
	pub fn returns_error_for_malformed_json() {
		// given
		let client = MockedHttpClient::malformed_json();
		let data = prepare_input_data("https://www.litentry.com/", "string");

		// when
		let result = http_get_string(data, client);

		// then
		assert_exit_status_reason(&result, "Could not parse json [123, 123], reason: Error(\"key must be a string\", line: 1, column: 2)");
	}

	#[test]
	pub fn returns_error_for_value_of_type_other_than_expected() {
		// given
		let client = MockedHttpClient::default();
		let data = prepare_input_data("https://www.litentry.com/", "/not_bool");

		// when
		let result = http_get_bool(data, client);

		// then
		assert_exit_status_reason(
			&result,
			"There is no value or it might be of different type, pointer: $\"/not_bool\"",
		);
	}

	#[test]
	pub fn returns_error_for_invalid_input_data() {
		// given
		let client = MockedHttpClient::default();
		let data = [0u8, 11];

		// when
		let result = http_get_bool(data.to_vec(), client);

		// then
		assert_exit_status_reason(&result, "Could not decode bytes [0, 11], reason: InvalidData");
	}

	#[test]
	pub fn returns_error_for_http_error() {
		// given
		let client = MockedHttpClient::http_error();
		let data = prepare_input_data("https://www.litentry.com/", "string");

		// when
		let result = http_get_string(data, client);

		// then
		assert_exit_status_reason(
			&result,
			"Error while performing http call: HttpError(404, \"Not found\")",
		);
	}

	fn assert_exit_status_reason(result: &PrecompileResult, expected_reason: &str) {
		match result {
			Err(e) => match e {
				PrecompileFailure::Error { exit_status } => match exit_status {
					ExitError::Other(reason) => {
						assert_eq!(reason.to_string(), expected_reason)
					},
					_ => panic!("Different exit status"),
				},
				_ => panic!("Different failure"),
			},
			_ => panic!("Expected err"),
		}
	}

	fn prepare_input_data(url: &str, pointer: &str) -> Vec<u8> {
		ethabi::encode(&[
			ethabi::Token::String(url.to_string()),
			ethabi::Token::String(pointer.to_string()),
			ethabi::Token::Array(vec![]),
		])
	}
}
