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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::precompiles::{
	http_get::{http_get_bool, http_get_i64, http_get_string},
	to_hex::to_hex,
};
use ethabi::ethereum_types::H160;
use evm::executor::stack::{
	IsPrecompileResult, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet,
};
use itc_rest_client::http_client::HttpClient;
use std::result::Result as StdResult;

mod http_get;
mod macros;
mod to_hex;

#[cfg(test)]
mod mocks;

pub type PrecompileResult = StdResult<PrecompileOutput, PrecompileFailure>;

pub struct Precompiles();

impl PrecompileSet for Precompiles {
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let mut headers = itc_rest_client::rest_client::Headers::new();
		headers.insert(http::header::CONNECTION.as_str(), "close");
		let client = HttpClient::new(
			itc_rest_client::http_client::DefaultSend {},
			true,
			Some(core::time::Duration::from_secs(5)),
			Some(headers),
			None,
		);

		match handle.code_address() {
			a if a == hash(1000) => Some(http_get_i64(handle.input().to_vec(), client)),
			a if a == hash(1001) => Some(http_get_bool(handle.input().to_vec(), client)),
			a if a == hash(1002) => Some(http_get_string(handle.input().to_vec(), client)),
			a if a == hash(1051) => Some(to_hex(handle.input().to_vec())),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _remaining_gas: u64) -> IsPrecompileResult {
		match address {
			a if a == hash(1000) =>
				IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
			a if a == hash(1001) =>
				IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
			a if a == hash(1002) =>
				IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
			a if a == hash(1051) =>
				IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
			_ => IsPrecompileResult::Answer { is_precompile: false, extra_cost: 0 },
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
