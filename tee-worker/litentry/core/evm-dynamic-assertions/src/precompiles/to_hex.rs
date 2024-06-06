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

use crate::precompiles::PrecompileResult;
use std::{format, vec::Vec};

pub fn to_hex(input: Vec<u8>) -> PrecompileResult {
	let decoded = match ethabi::decode(&[ethabi::ParamType::Bytes], &input) {
		Ok(d) => d,
		Err(e) => {
			log::debug!("Could not decode bytes {:?}, reason: {:?}", input, e);
			let encoded = ethabi::encode(&[
				ethabi::Token::Bool(true),
				ethabi::Token::String(Default::default()),
			]);
			return Ok(evm::executor::stack::PrecompileOutput {
				exit_status: evm::ExitSucceed::Returned,
				output: encoded,
			})
		},
	};
	// safe to unwrap
	let bytes = decoded.get(0).unwrap().clone().into_bytes().unwrap();
	let hex_encoded = format!("0x{}", hex::encode(&bytes));
	let encoded = ethabi::encode(&[ethabi::Token::Bool(true), ethabi::Token::String(hex_encoded)]);
	Ok(evm::executor::stack::PrecompileOutput {
		exit_status: evm::ExitSucceed::Returned,
		output: encoded,
	})
}

#[cfg(test)]
pub mod test {
	use crate::precompiles::to_hex::to_hex;
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
			ethabi::encode(&[
				ethabi::Token::Bool(true),
				ethabi::Token::String("0x01020304".to_string())
			]),
			result.output
		)
	}
}
