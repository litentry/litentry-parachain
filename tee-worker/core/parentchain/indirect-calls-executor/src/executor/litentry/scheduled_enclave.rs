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

use crate::{error::Result, executor::Executor, IndirectCallsExecutor};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_types::{RemoveScheduledEnclaveFn, UpdateScheduledEnclaveFn};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use log::*;

pub(crate) struct UpdateScheduledEnclave;
pub(crate) struct RemoveScheduledEnclave;

impl UpdateScheduledEnclave {
	fn execute_internal<R, S, T, N>(
		&self,
		extrinsic: ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
	) -> Result<()>
	where
		N: AccessNodeMetadata,
		N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
	{
		let (_, sbn, mrenclave) = extrinsic.function;
		debug!("execute indirect call: UpdateScheduledEnclave, sidechain_block_number: {}, mrenclave: {:?}", sbn, mrenclave);
		GLOBAL_SCHEDULED_ENCLAVE.update(sbn, mrenclave)?;
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for UpdateScheduledEnclave
where
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
{
	type Call = UpdateScheduledEnclaveFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]> {
		metadata_type.update_scheduled_enclave().map_err(|e| e.into())
	}

	fn execute(
		&self,
		_context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		self.execute_internal::<R, S, T, N>(extrinsic)
	}
}

impl RemoveScheduledEnclave {
	fn execute_internal<R, S, T, N>(
		&self,
		extrinsic: ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
	) -> Result<()>
	where
		N: AccessNodeMetadata,
		N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
	{
		let (_, sbn) = extrinsic.function;
		debug!("execute indirect call: RemoveScheduledEnclave, sidechain_block_number: {}", sbn);
		GLOBAL_SCHEDULED_ENCLAVE.remove(sbn)?;
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for RemoveScheduledEnclave
where
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
{
	type Call = RemoveScheduledEnclaveFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]> {
		metadata_type.remove_scheduled_enclave().map_err(|e| e.into())
	}

	fn execute(
		&self,
		_context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		self.execute_internal::<R, S, T, N>(extrinsic)
	}
}
