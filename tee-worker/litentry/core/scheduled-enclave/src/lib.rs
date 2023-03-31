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

use codec::{Decode, Encode};
use itp_types::{MrEnclave, SidechainBlockNumber};
use sp_std::collections::btree_map::BTreeMap;

pub mod error;
use error::Result;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod io;

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct ScheduledEnclave {
	pub scheduled_enclave: BTreeMap<SidechainBlockNumber, MrEnclave>,
}
pub trait ScheduledEnclaveUpd {
	fn update_scheduled_enclave(
		&mut self,
		sbn: SidechainBlockNumber,
		mrenclave: MrEnclave,
	) -> Result<()>;

	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> Result<()>;

	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo>;
}

impl ScheduledEnclaveHandle for ScheduledEnclaves {
	fn add_scheduled_enclave(
		&mut self,
		scheduled_enclave: ScheduledEnclaveInfo,
	) -> IOResult<Option<ScheduledEnclaveInfo>> {
		let old_enclave = self
			.scheduled_enclaves
			.insert(scheduled_enclave.sidechain_block_number, scheduled_enclave);
		self.sync_to_static_file()?;
		Ok(old_enclave)
	}

	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> IOResult<Option<ScheduledEnclaveInfo>> {
		let enclave_info = self.scheduled_enclaves.remove(&sidechain_block_number);
		if enclave_info.is_some() {
			self.sync_to_static_file()?;
		}
		Ok(enclave_info)
	}

	/// get the next scheduled enclave
	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo> {
		self.scheduled_enclaves
			.iter()
			.filter_map(
				|(k, v)| {
					if k > &current_side_chain_number {
						Some(v.clone())
					} else {
						None
					}
				},
			)
			.min_by_key(|v| v.sidechain_block_number)
	}
}
