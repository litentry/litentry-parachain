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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{AssertionId, SmartContractByteCode};
use codec::{Decode, Encode};
use lc_dynamic_assertion::AssertionLogicRepository;
use std::{
	collections::HashMap,
	format,
	string::{String, ToString},
	vec::Vec,
};

#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[cfg(feature = "std")]
use std::sync::Mutex;

// Assertion repository backed by sealed file. Contains only latest state from parachain storage.
#[allow(clippy::type_complexity)]
pub struct EvmAssertionRepository {
	path: String,
	state: Mutex<HashMap<AssertionId, (SmartContractByteCode, Vec<String>)>>,
}

impl EvmAssertionRepository {
	pub fn new(path: &str) -> Result<Self, String> {
		let mut state = HashMap::new();
		let unsealed = io::unseal(path);

		if !unsealed.is_empty() {
			let decoded: Vec<(AssertionId, (SmartContractByteCode, Vec<String>))> =
				Decode::decode(&mut unsealed.as_slice())
					.map_err(|e| format!("Could not decode assertions state, reason: {:?}", e))?;

			decoded.into_iter().for_each(|(id, byte_code)| {
				state.insert(id, byte_code);
			});
		}

		Ok(EvmAssertionRepository { state: state.into(), path: path.to_string() })
	}
}

impl AssertionLogicRepository for EvmAssertionRepository {
	type Id = AssertionId;
	type Value = SmartContractByteCode;

	fn get(&self, id: &Self::Id) -> Result<Option<(Self::Value, Vec<String>)>, String> {
		Ok(self
			.state
			.lock()
			.map_err(|e| format!("Could not acquire lock on inner state: {:?}", e))?
			.get(id)
			.cloned())
	}

	fn save(&self, id: Self::Id, value: Self::Value, secrets: Vec<String>) -> Result<(), String> {
		std::println!("Saving assertion id: {:?} with code: {:?}", id, value);
		self.state
			.lock()
			.map_err(|e| format!("Could not acquire lock on inner state: {:?}", e))?
			.insert(id, (value, secrets));
		// prepare data for encoding
		let to_encode: Vec<(AssertionId, (SmartContractByteCode, Vec<String>))> = self
			.state
			.lock()
			.map_err(|e| format!("Could not acquire lock on inner state: {:?}", e))?
			.iter()
			.map(|(key, val)| (*key, val.clone()))
			.collect();
		let encoded = to_encode.encode();
		io::seal(&self.path, encoded);
		Ok(())
	}
}

#[cfg(feature = "std")]
pub mod io {
	use std::{vec, vec::Vec};

	pub fn seal(_path: &str, _state: Vec<u8>) {}

	pub fn unseal(_path: &str) -> Vec<u8> {
		vec![]
	}
}

#[cfg(feature = "sgx")]
pub mod io {
	use itp_sgx_io::{seal as io_seal, unseal as io_unseal};
	use std::{vec, vec::Vec};

	pub fn seal(path: &str, state: Vec<u8>) {
		io_seal(&state, path).unwrap()
	}

	pub fn unseal(path: &str) -> Vec<u8> {
		io_unseal(path).unwrap()
	}
}
