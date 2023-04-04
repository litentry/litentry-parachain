// Copyright 2020-2023 Litentry Technologies GmbH.
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

#[cfg(feature = "sgx")]
extern crate sgx_tstd as std;

// TODO: maybe use parachain primitives for single source of truth
use itp_types::{MrEnclave, SidechainBlockNumber};
use sp_std::collections::btree_map::BTreeMap;

pub mod error;
use error::Result;
pub mod io;

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
	/// Global instance of a ScheduledEnclave
	pub static ref GLOBAL_SCHEDULED_ENCLAVE: Arc<ScheduledEnclave> = Default::default();
}

pub type ScheduledEnclaveMap = BTreeMap<SidechainBlockNumber, MrEnclave>;

#[derive(Default)]
pub struct ScheduledEnclave {
	pub registry: RwLock<ScheduledEnclaveMap>,
}

// all fn with &self as we mutate the state internally
pub trait ScheduledEnclaveUpdater {
	fn init(&self, mrenclave: MrEnclave) -> Result<()>;

	fn update(&self, sbn: SidechainBlockNumber, mrenclave: MrEnclave) -> Result<()>;

	fn remove(&self, sbn: SidechainBlockNumber) -> Result<()>;

	// given a SidechainBlockNumber, return the expected MRENCLAVE
	// For example, the registry is:
	// 0  -> 0xAA
	// 19 -> 0xBB
	// 21 -> 0xCC
	//
	// get_expected_mrenclave(0) -> 0xAA
	// get_expected_mrenclave(18) -> 0xAA
	// get_expected_mrenclave(19) -> 0xBB
	// get_expected_mrenclave(20) -> 0xBB
	// get_expected_mrenclave(21) -> 0xCC
	// get_expected_mrenclave(30) -> 0xCC
	fn get_expected_mrenclave(&self, sbn: SidechainBlockNumber) -> Result<MrEnclave>;
}

#[derive(Default)]
pub struct ScheduledEnclaveMock;

// todo!
impl ScheduledEnclaveUpdater for ScheduledEnclaveMock {
	fn init(&self, _mrenclave: MrEnclave) -> Result<()> {
		Ok(())
	}

	fn update(&self, _sbn: SidechainBlockNumber, _mrenclave: MrEnclave) -> Result<()> {
		Ok(())
	}

	fn remove(&self, _sbn: SidechainBlockNumber) -> Result<()> {
		Ok(())
	}

	fn get_expected_mrenclave(&self, _sbn: SidechainBlockNumber) -> Result<MrEnclave> {
		Ok(MrEnclave::default())
	}
}
