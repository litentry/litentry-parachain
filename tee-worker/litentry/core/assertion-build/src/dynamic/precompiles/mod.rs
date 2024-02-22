use crate::dynamic::precompiles::http_get::{HttpGetBoolPrecompile, HttpGetI64Precompile};
use evm::executor::stack::{
	IsPrecompileResult, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet,
};
use primitive_types::{H160, U256};
use std::{
	result::Result as StdResult,
	string::{String, ToString},
	vec::Vec,
};

mod http_get;
mod macros;

pub type PrecompileResult = StdResult<PrecompileOutput, PrecompileFailure>;

pub struct Precompiles();

impl PrecompileSet for Precompiles {
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			a if a == hash(2) => Some(HttpGetI64Precompile::execute(handle.input().to_vec())),
			a if a == hash(3) => Some(HttpGetBoolPrecompile::execute(handle.input().to_vec())),
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

#[cfg(test)]
pub mod test {}
