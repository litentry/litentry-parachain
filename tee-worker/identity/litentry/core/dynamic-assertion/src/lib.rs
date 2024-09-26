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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

pub use litentry_primitives::{Identity, IdentityNetworkTuple, Web3Network};
use std::{string::String, vec::Vec};

// Used to retrieve assertion logic and secrets
pub trait AssertionLogicRepository {
	type Id;
	type Item;

	#[allow(clippy::type_complexity)]
	fn get(&self, id: &Self::Id) -> Result<Option<Self::Item>, String>;
	fn save(&self, id: Self::Id, item: Self::Item) -> Result<(), String>;
}

pub struct AssertionResult {
	pub description: String,
	pub assertion_type: String,
	pub assertions: Vec<String>,
	pub schema_url: String,
	pub meet: bool,
	pub contract_logs: Vec<String>,
}

pub trait AssertionExecutor<I, P> {
	fn execute(
		&self,
		assertion_id: I,
		assertion_params: P,
		identities: &[IdentityNetworkTuple],
	) -> Result<AssertionResult, String>;
}
