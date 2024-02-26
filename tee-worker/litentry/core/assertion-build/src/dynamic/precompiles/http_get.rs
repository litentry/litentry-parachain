use crate::*;
use itc_rest_client::http_client::SendHttpRequest;

http_get_precompile_fn!(http_get_bool, Bool, as_bool);
http_get_precompile_fn!(http_get_i64, Uint, as_i64);
http_get_precompile_fn!(http_get_string, String, as_str);

#[cfg(test)]
pub mod test {
	use crate::dynamic::precompiles::{
		http_get::{http_get_bool, http_get_i64, http_get_string},
		mocks::MockedHttpClient,
		PrecompileResult,
	};
	use evm::{executor::stack::PrecompileFailure, ExitError, ExitSucceed};
	use primitive_types::U256;

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
		assert_exit_status_reason(&result, "Could not read string len, start: 0 end 32");
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
		let url_bytes = url.as_bytes();
		let pointer_bytes = pointer.as_bytes();

		let mut bytes = [0; 128];
		U256::try_from(url_bytes.len()).unwrap().to_big_endian(&mut bytes[0..32]);
		bytes[32..64][0..url_bytes.len()].copy_from_slice(url_bytes);
		U256::try_from(pointer_bytes.len()).unwrap().to_big_endian(&mut bytes[64..96]);
		bytes[96..128][0..pointer_bytes.len()].copy_from_slice(pointer_bytes);

		bytes.to_vec()
	}

	//test cases for:
	//reader cannot panic
}
