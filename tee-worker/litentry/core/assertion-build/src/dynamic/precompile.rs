#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use core::time::Duration;
use ethabi::{encode, Token};
use evm::{
	executor::stack::{
		IsPrecompileResult, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet,
	},
	ExitSucceed,
};
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient, SendHttpRequest},
	rest_client::{Method, Url},
};
use primitive_types::{H160, U256};
use serde_json::Value;
use std::{
	result::Result as StdResult,
	string::{String, ToString},
	vec::Vec,
};

/* This is precompile contract for making http get requests.
It will send HTTP GET request to hardcoded URL, parse JSON response, extract value using JSON Pointer and pass it back to calle contract.
`input` can be used to customize URL, JSON pointer and authorization.
Currently this contract return only integers, but it may be possible to return any data as byte array but handling code on calling side
will be more complex. It may also require more flexible parsing or JSON handling in Solidity (jsmnSol) */
pub struct HttpGetI64Precompile;

impl HttpGetI64Precompile {
	// 256 bytes for url, 256 bytes for json pointer, total 512 bytes
	fn execute(input: Vec<u8>) -> PrecompileResult {
		let mut reader = InputReader::new(input);

		let url = reader.read_string();
		let pointer = reader.read_string();

		let client = HttpClient::new(DefaultSend, true, Some(Duration::from_secs(10)), None, None);
		let resp = client.send_request_raw(Url::parse(&url).unwrap(), Method::GET, None).unwrap();
		let value: Value = serde_json::from_slice(&resp.1).unwrap();
		let result = value.pointer(&pointer).unwrap();
		let encoded = encode(&[Token::Uint(result.as_i64().unwrap().into())]);
		Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: encoded })
	}
}

pub struct Precompiles();

impl PrecompileSet for Precompiles {
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			a if a == hash(2) => Some(HttpGetI64Precompile::execute(handle.input().to_vec())),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _remaining_gas: u64) -> IsPrecompileResult {
		match address {
			a if a == hash(2) => IsPrecompileResult::Answer { is_precompile: false, extra_cost: 0 },
			_ => IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
		}
	}
}

pub type PrecompileResult = StdResult<PrecompileOutput, PrecompileFailure>;

pub struct InputReader {
	input: Vec<u8>,
	position: usize,
}

impl InputReader {
	pub fn new(input: Vec<u8>) -> Self {
		Self { input, position: 0 }
	}

	pub fn read_string(&mut self) -> String {
		let word_size = 32;
		let str_len = self.read_string_len();
		let end = self.position + str_len;
		let value = String::from_utf8_lossy(&self.input[(self.position)..end]).to_string();
		self.position += ((str_len / word_size) + 1) * word_size;
		value
	}

	fn read_string_len(&mut self) -> usize {
		let word_size = 32;
		// first word contains information about string size,
		let end = self.position + word_size;
		let size: usize = U256::from_big_endian(&self.input[(self.position)..end])
			.try_into()
			.expect("Could not convert size");
		self.position += word_size;
		size
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
