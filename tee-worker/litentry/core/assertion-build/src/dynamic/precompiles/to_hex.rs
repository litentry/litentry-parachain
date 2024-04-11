use crate::dynamic::precompiles::{macros::prepare_custom_failure, PrecompileResult};
use std::{format, vec::Vec};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate hex_sgx as hex;

pub fn to_hex(input: Vec<u8>) -> PrecompileResult {
	let decoded = ethabi::decode(&[ethabi::ParamType::Bytes], &input).map_err(|e| {
		prepare_custom_failure(format!("Could not decode bytes {:?}, reason: {:?}", input, e))
	})?;

	// safe to unwrap
	let bytes = decoded.get(0).unwrap().clone().into_bytes().unwrap();

	let hex_encoded = format!("0x{}", hex::encode(&bytes));
	let encoded = ethabi::encode(&[ethabi::Token::String(hex_encoded)]);
	Ok(evm::executor::stack::PrecompileOutput {
		exit_status: evm::ExitSucceed::Returned,
		output: encoded[32..encoded.len()].to_vec(),
	})
}

#[cfg(test)]
pub mod test {
	use crate::dynamic::precompiles::to_hex::to_hex;
	use evm::ExitSucceed;

	#[test]
	pub fn test_to_hex() {
		// given
		let bytes = [1, 2, 3, 4];
		let encoded = ethabi::encode(&[ethabi::Token::Bytes(bytes.to_vec())]);

		// when
		let result = to_hex(encoded).unwrap();

		//then
		assert!(matches!(result.exit_status, ExitSucceed::Returned));
		assert_eq!(
			ethabi::encode(&[ethabi::Token::String("0x01020304".to_string())])[32..].to_vec(),
			result.output
		)
	}
}
