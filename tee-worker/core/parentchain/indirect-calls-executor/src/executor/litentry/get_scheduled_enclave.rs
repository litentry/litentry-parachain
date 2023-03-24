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
use itp_component_container::{ComponentContainer, ComponentGetter, ComponentSetter};
use itp_enclave_scheduled::{ScheduledEnclaveHandle, ScheduledEnclaveInfo, ScheduledEnclaves};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_types::{CallRemoveScheduledEnclaveFn, CallUpdateScheduledEnclaveFn};
use litentry_primitives::ParentchainBlockNumber;
use log::*;
use std::sync::Arc;

pub(crate) struct ScheduledEnclaveUpdate {
	pub(crate) block_number: ParentchainBlockNumber,
}

pub(crate) struct ScheduledEnclaveRemove;

/// sidechain enclave info
pub static GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES: ComponentContainer<ScheduledEnclaves> =
	ComponentContainer::new("sidechain_sheduled_enclaves");

impl ScheduledEnclaveUpdate {
	fn execute_internal<R, S, T, N>(
		&self,
		extrinsic: ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
	) -> Result<()>
	where
		N: AccessNodeMetadata,
		N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
	{
		let (_, sidechain_block_number, mr_enclave) = extrinsic.function;
		debug!("execute indirect call: ScheduledEnclaveUpdate, sidechain_block_number: {}, mr_enclave: {:?}", sidechain_block_number, mr_enclave);

		let scheduled_enclave = ScheduledEnclaveInfo {
			parachain_block_number: self.block_number,
			sidechain_block_number,
			mr_enclave,
		};
		let old_enclaves = GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES.get()?;
		// unwrap is safe here, because GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES is initialized in `init_enclave()`
		let mut scheduled_enclaves = Arc::<ScheduledEnclaves>::try_unwrap(old_enclaves).unwrap();
		scheduled_enclaves.add_scheduled_enclave(scheduled_enclave)?;
		GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES.set(Arc::new(scheduled_enclaves));
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for ScheduledEnclaveUpdate
where
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
{
	type Call = CallUpdateScheduledEnclaveFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]> {
		metadata_type.update_scheduled_encalve().map_err(|e| e.into())
	}

	fn execute(
		&self,
		_context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		self.execute_internal::<R, S, T, N>(extrinsic)
	}
}

impl ScheduledEnclaveRemove {
	fn execute_internal<R, S, T, N>(
		&self,
		extrinsic: ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
	) -> Result<()>
	where
		N: AccessNodeMetadata,
		N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
	{
		let (_, sidechain_block_number) = extrinsic.function;
		debug!(
			"execute indirect call: ScheduledEnclaveRemove, sidechain_block_number: {}",
			sidechain_block_number
		);

		let old_enclaves = GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES.get()?;
		// `unwrap()` is safe here, because GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES is initialized in `init_enclave()`
		let mut scheduled_enclaves = Arc::<ScheduledEnclaves>::try_unwrap(old_enclaves).unwrap();
		scheduled_enclaves.remove_scheduled_enclave(sidechain_block_number)?;
		GLOBAL_SIDECHAIN_SCHEDULED_ENCLABES.set(Arc::new(scheduled_enclaves));
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for ScheduledEnclaveRemove
where
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
{
	type Call = CallRemoveScheduledEnclaveFn;

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
