use evm::{executor::stack::PrecompileFailure, ExitError};
use std::{borrow::Cow, string::String};

pub fn prepare_custom_failure(reason: String) -> PrecompileFailure {
	PrecompileFailure::Error { exit_status: ExitError::Other(Cow::Owned(reason)) }
}

#[macro_export]
macro_rules! http_get_precompile_fn {
	($name:ident, $token:ident, $parse_fn_name:ident) => {
		pub fn $name<T: SendHttpRequest>(
			input: Vec<u8>,
			client: T,
		) -> $crate::dynamic::precompiles::PrecompileResult {
			let decoded = ethabi::decode(&[ethabi::ParamType::String, ethabi::ParamType::String], &input).map_err(|e| {
				$crate::dynamic::precompiles::macros::prepare_custom_failure(format!(
					"Could not decode bytes {:?}, reason: {:?}", input, e
				))
			})?;
			// safe to unwrap
			let url = decoded.get(0).unwrap().clone().into_string().unwrap();
			let url = itc_rest_client::rest_client::Url::parse(&url).map_err(|e| {
				$crate::dynamic::precompiles::macros::prepare_custom_failure(format!(
					"Could not parse url {:?}, reason: {:?}",
					url, e
				))
			})?;
			// safe to unwrap
			let pointer = decoded.get(1).unwrap().clone().into_string().unwrap();
			let resp = client
				.send_request_raw(url, itc_rest_client::rest_client::Method::GET, None)
				.map_err(|e| {
				$crate::dynamic::precompiles::macros::prepare_custom_failure(format!(
					"Error while performing http call: {:?}", e
				))
			})?;
			let value: serde_json::Value = serde_json::from_slice(&resp.1).map_err(|e| {
				$crate::dynamic::precompiles::macros::prepare_custom_failure(format!(
					"Could not parse json {:?}, reason: {:?}",
					resp.1, e
				))
			})?;
			let result = match value.pointer(&pointer) {
				Some(v) => v,
				None =>
					return Err($crate::dynamic::precompiles::macros::prepare_custom_failure(
						format!("No value under given pointer: :{:?}", pointer),
					)),
			};
			let encoded = match result.$parse_fn_name() {
				Some(v) => ethabi::encode(&[ethabi::Token::$token(v.into())]),
				None =>
					return Err($crate::dynamic::precompiles::macros::prepare_custom_failure(
						format!("There is no value or it might be of different type, pointer: ${:?}", pointer)
					)),
			};
			Ok(evm::executor::stack::PrecompileOutput {
				exit_status: evm::ExitSucceed::Returned,
				output: encoded,
			})
		}
	};
}
