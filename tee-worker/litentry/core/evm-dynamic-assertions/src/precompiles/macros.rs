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
		) -> $crate::precompiles::PrecompileResult {
			let decoded = ethabi::decode(
				&[
					ethabi::ParamType::String,
					ethabi::ParamType::String,
					ethabi::ParamType::Array(
						ethabi::ParamType::Tuple(vec![
							ethabi::ParamType::String,
							ethabi::ParamType::String,
						])
						.into(),
					),
				],
				&input,
			)
			.map_err(|e| {
				$crate::precompiles::macros::prepare_custom_failure(std::format!(
					"Could not decode bytes {:?}, reason: {:?}",
					input, e
				))
			})?;
			// safe to unwrap
			let url = decoded.get(0).unwrap().clone().into_string().unwrap();
			let url = itc_rest_client::rest_client::Url::parse(&url).map_err(|e| {
				$crate::precompiles::macros::prepare_custom_failure(std::format!(
					"Could not parse url {:?}, reason: {:?}",
					url, e
				))
			})?;

			// safe to unwrap
			let pointer = decoded.get(1).unwrap().clone().into_string().unwrap();
			let http_headers: Vec<(String, String)> = decoded
				.get(2)
				.unwrap()
				.clone()
				.into_array()
				.unwrap()
				.iter()
				.map(|v| {
					let name = v
						.clone()
						.into_tuple()
						.unwrap()
						.get(0)
						.unwrap()
						.clone()
						.into_string()
						.unwrap();
					let value = v
						.clone()
						.into_tuple()
						.unwrap()
						.get(1)
						.unwrap()
						.clone()
						.into_string()
						.unwrap();

					(name, value)
				})
				.collect();
			let resp = client
				.send_request_raw(
					url,
					itc_rest_client::rest_client::Method::GET,
					None,
					http_headers,
				)
				.map_err(|e| {
					$crate::precompiles::macros::prepare_custom_failure(std::format!(
						"Error while performing http call: {:?}",
						e
					))
				})?;
			let value: serde_json::Value = serde_json::from_slice(&resp.1).map_err(|e| {
				$crate::precompiles::macros::prepare_custom_failure(std::format!(
					"Could not parse json {:?}, reason: {:?}",
					resp.1, e
				))
			})?;
			let result = match value.pointer(&pointer) {
				Some(v) => v,
				None =>
					return Err($crate::precompiles::macros::prepare_custom_failure(std::format!(
						"No value under given pointer: :{:?}",
						pointer
					))),
			};

			let encoded = match result.$parse_fn_name() {
				Some(v) => ethabi::encode(&[ethabi::Token::$token(v.into())]),
				None =>
					return Err($crate::precompiles::macros::prepare_custom_failure(std::format!(
						"There is no value or it might be of different type, pointer: ${:?}",
						pointer
					))),
			};
			Ok(evm::executor::stack::PrecompileOutput {
				exit_status: evm::ExitSucceed::Returned,
				output: encoded,
			})
		}
	};
}
