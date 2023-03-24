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

use crate::error::{Error, Result};
use itc_parentchain_indirect_calls_executor::executor::litentry::get_scheduled_enclave::GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES;
use itp_component_container::ComponentGetter;
use itp_enclave_scheduled::ScheduledEnclaveHandle;
use itp_types::{MrEnclave, SidechainBlockNumber};

/// Trait to suspend the production of sidechain blocks.
pub trait UpdaterTrait {
	/// current mr enclave
	fn current_mr_enclave(&self) -> MrEnclave {
		MrEnclave::default()
	}

	/// Trait to query if sidechain block production is suspended.
	fn is_block_suspended(&self, _current_sidechain_num: SidechainBlockNumber) -> Result<bool> {
		Ok(false)
	}

	/// check if need to merge return next mrenclave
	fn get_merge_mr_enclave(
		&self,
		_current_sidechain_num: SidechainBlockNumber,
	) -> Result<Option<MrEnclave>> {
		Ok(None)
	}
}

/// Implementation for suspending and resuming sidechain block production.
#[derive(Default, Clone)]
pub struct Updater {
	current_mr_enclave: MrEnclave,
}

impl Updater {
	pub fn new(current_mr_enclave: MrEnclave) -> Self {
		Updater { current_mr_enclave }
	}
}

impl UpdaterTrait for Updater {
	fn current_mr_enclave(&self) -> MrEnclave {
		self.current_mr_enclave
	}

	fn is_block_suspended(&self, current_sidechain_num: SidechainBlockNumber) -> Result<bool> {
		let scheduled_enclaves = GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES
			.get()
			.map_err(|_| Error::GetScheduledEnclavesFailed)?;
		if let Some(mr_enclave_should_used) =
			scheduled_enclaves.get_current_scheduled_enclave(current_sidechain_num)
		{
			Ok(self.current_mr_enclave == mr_enclave_should_used.mr_enclave)
		} else {
			// by default, if the all the scheduled mr_enclaves is empty
			// we assume that current mr_enclave is ok
			Ok(false)
		}
	}

	/// return the next MrEnclave
	fn get_merge_mr_enclave(
		&self,
		current_sidechain_num: SidechainBlockNumber,
	) -> Result<Option<MrEnclave>> {
		let scheduled_enclaves = GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES
			.get()
			.map_err(|_| Error::GetScheduledEnclavesFailed)?;
		if let Some(scheduled_enclave) =
			scheduled_enclaves.get_next_scheduled_enclave(current_sidechain_num)
		{
			if current_sidechain_num + 1 == scheduled_enclave.sidechain_block_number
				&& self.current_mr_enclave != scheduled_enclave.mr_enclave
			{
				return Ok(Some(scheduled_enclave.mr_enclave))
			}
		}
		Ok(None)
	}
}
