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

use codec::{Decode, Encode};
use ita_stf::TrustedCallSigned;
use itc_parentchain_indirect_calls_executor::{
	error::{Error, Result},
	IndirectDispatch,
};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_types::{MrEnclave, SidechainBlockNumber, WorkerType};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use log::*;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct SetScheduledEnclaveArgs {
	worker_type: WorkerType,
	sbn: SidechainBlockNumber,
	mrenclave: MrEnclave,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for SetScheduledEnclaveArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		debug!("execute indirect call: SetScheduledEnclave, worker_type: {:?}, sidechain_block_number: {:?}, mrenclave: {:?}", self.worker_type, self.sbn, self.mrenclave);
		if self.worker_type == WorkerType::BitAcross {
			GLOBAL_SCHEDULED_ENCLAVE.update(self.sbn, self.mrenclave)?;
		} else {
			warn!("Ignore SetScheduledEnclave due to wrong worker_type");
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RemoveScheduledEnclaveArgs {
	worker_type: WorkerType,
	sbn: SidechainBlockNumber,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for RemoveScheduledEnclaveArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		debug!(
			"execute indirect call: RemoveScheduledEnclave, worker_type: {:?}, sidechain_block_number: {:?}",
			self.worker_type,
			self.sbn
		);
		if self.worker_type == WorkerType::BitAcross {
			GLOBAL_SCHEDULED_ENCLAVE.remove(self.sbn)?;
		} else {
			warn!("Ignore RemoveScheduledEnclave due to wrong worker_type");
		}
		Ok(())
	}
}
