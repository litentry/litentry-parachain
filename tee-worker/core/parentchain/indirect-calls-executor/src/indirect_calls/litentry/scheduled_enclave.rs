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

use crate::{error::Result, IndirectDispatch, IndirectExecutor};
use codec::{Decode, Encode};

use itp_types::{MrEnclave, SidechainBlockNumber};

use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};

use log::debug;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct UpdateScheduledEnclaveArgs {
	sbn: codec::Compact<SidechainBlockNumber>,
	mrenclave: MrEnclave,
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for UpdateScheduledEnclaveArgs {
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		debug!("execute indirect call: UpdateScheduledEnclave, sidechain_block_number: {:?}, mrenclave: {:?}", self.sbn, self.mrenclave);
		GLOBAL_SCHEDULED_ENCLAVE.update(self.sbn.into(), self.mrenclave)?;
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RemoveScheduledEnclaveArgs {
	sbn: codec::Compact<SidechainBlockNumber>,
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for RemoveScheduledEnclaveArgs {
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		debug!(
			"execute indirect call: RemoveScheduledEnclave, sidechain_block_number: {:?}",
			self.sbn
		);
		GLOBAL_SCHEDULED_ENCLAVE.remove(self.sbn.into())?;
		Ok(())
	}
}
