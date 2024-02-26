use crate::dynamic::precompiles::http_get::{http_get_bool, http_get_i64, http_get_string};
use evm::executor::stack::{
	IsPrecompileResult, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet,
};
use itc_rest_client::http_client::HttpClient;
use primitive_types::{H160, U256};
use std::{
	result::Result as StdResult,
	string::{String, ToString},
	vec::Vec,
};

mod http_get;
mod macros;

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
			a if a == hash(1000) => Some(http_get_bool(handle.input().to_vec(), client)),
			a if a == hash(1001) => Some(http_get_i64(handle.input().to_vec(), client)),
			a if a == hash(1002) => Some(http_get_string(handle.input().to_vec(), client)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _remaining_gas: u64) -> IsPrecompileResult {
		match address {
			a if a == hash(2) => IsPrecompileResult::Answer { is_precompile: false, extra_cost: 0 },
			a if a == hash(3) => IsPrecompileResult::Answer { is_precompile: false, extra_cost: 0 },
			_ => IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 },
		}
	}
}

pub struct InputReader {
	input: Vec<u8>,
	position: usize,
}

impl InputReader {
	pub fn new(input: Vec<u8>) -> Self {
		Self { input, position: 0 }
	}

	pub fn read_string(&mut self) -> Result<String, String> {
		let word_size = 32;
		let str_len = self.read_string_len()?;
		let end = self.position + str_len;
		if let Some(bytes) = &self.input.get((self.position)..end) {
			let value = String::from_utf8_lossy(bytes).to_string();
			self.position += ((str_len / word_size) + 1) * word_size;
			Ok(value)
		} else {
			Err(format!("Could not read string, start: {:?} end {:?}", self.position, end)
				.to_string())
		}
	}

	fn read_string_len(&mut self) -> Result<usize, String> {
		let word_size = 32;
		// first word contains information about string size,
		let end = self.position + word_size;

		if let Some(bytes) = &self.input.get((self.position)..end) {
			let size: usize =
				U256::from_big_endian(bytes).try_into().expect("Could not convert size");
			self.position += word_size;
			Ok(size)
		} else {
			Err(format!("Could not read string len, start: {:?} end {:?}", self.position, end)
				.to_string())
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[cfg(test)]
pub mod test {
	use crate::dynamic::precompiles::InputReader;
	use primitive_types::U256;

	#[test]
	pub fn test_input_reader() {
		// given
		let url = "https://www.litentry.com/".to_string();
		let pointer = "/privacy/is/there/0";
		let url_bytes = url.as_bytes();
		let pointer_bytes = pointer.as_bytes();

		let mut bytes = [0; 128];
		U256::try_from(url_bytes.len()).unwrap().to_big_endian(&mut bytes[0..32]);
		bytes[32..64][0..url_bytes.len()].copy_from_slice(url_bytes);
		U256::try_from(pointer_bytes.len()).unwrap().to_big_endian(&mut bytes[64..96]);
		bytes[96..128][0..pointer_bytes.len()].copy_from_slice(pointer_bytes);

		// when
		let mut reader = InputReader::new(bytes.to_vec());

		// then
		assert_eq!(url, reader.read_string().unwrap());
		assert_eq!(pointer, reader.read_string().unwrap());
	}

	#[test]
	pub fn test_input_reader_2() {
		// given
		let bytes = [0u8; 11];

		// when
		let mut reader = InputReader::new(bytes.to_vec());

		// then
		match reader.read_string() {
			Ok(_) => panic!("expected error!"),
			Err(e) => assert_eq!("Could not read string len, start: 0 end 32", e),
		}
	}
}
