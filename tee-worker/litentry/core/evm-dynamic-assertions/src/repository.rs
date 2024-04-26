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

use crate::{
	sealing::io::{seal_state, unseal_state},
	AssertionId, SmartContractByteCode,
};
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

pub type AssertionsMap = HashMap<AssertionId, (SmartContractByteCode, Vec<String>)>;

// Assertion repository backed by sealed file. Contains only latest state from parachain storage.
#[allow(clippy::type_complexity)]
pub struct EvmAssertionRepository {
	path: String,
	state: Mutex<HashMap<AssertionId, (SmartContractByteCode, Vec<String>)>>,
}

impl EvmAssertionRepository {
	pub fn new(path: &str) -> Result<Self, String> {
		let mut state = HashMap::new();
		let unsealed = unseal_state(path)
			.map_err(|e| format!("Could not unseal assertions state: {:?}", e))?;
		unsealed.into_iter().for_each(|(id, byte_code)| {
			state.insert(id, byte_code);
		});

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
		let unsealed_state: Vec<(AssertionId, (SmartContractByteCode, Vec<String>))> = self
			.state
			.lock()
			.map_err(|e| format!("Could not acquire lock on inner state: {:?}", e))?
			.iter()
			.map(|(key, val)| (*key, val.clone()))
			.collect();

		if let Err(e) = seal_state(&self.path, unsealed_state) {
			//clean up memory state
			self.state
				.lock()
				.map_err(|e| format!("Could not acquire lock on inner state: {:?}", e))?
				.remove(&id);
			return Err(format!("Could not seal assertions state: {:?}", e))
		}
		Ok(())
	}
}
