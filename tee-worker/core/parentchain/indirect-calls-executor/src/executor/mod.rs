/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use crate::{error::Result, hash_of, ExecutionStatus, IndirectCallsExecutor};
use codec::{Decode, Encode};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
		pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_types::{extrinsics::ParentchainUncheckedExtrinsicWithStatus, H256};
use log::*;

pub mod call_worker;
pub mod litentry;
pub mod shield_funds;

pub(crate) trait Executor<R, S, T, N>
where
	N: AccessNodeMetadata,
{
	type Call: Decode + Encode + Clone;

	fn call_index(&self, call: &Self::Call) -> [u8; 2];

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]>;

	fn is_target_call(&self, call: &Self::Call, node_metadata_provider: &N) -> bool {
		node_metadata_provider
			.get_from_metadata(|m| match self.call_index_from_metadata(m) {
				Ok(call_index) => self.call_index(call) == call_index,
				Err(_e) => false,
			})
			.unwrap_or(false)
	}

	fn decode(
		&self,
		_context: &IndirectCallsExecutor<R, S, T, N>,
		input: &mut &[u8],
	) -> Result<ParentchainUncheckedExtrinsicWithStatus<Self::Call>> {
		ParentchainUncheckedExtrinsicWithStatus::<Self::Call>::decode(input).map_err(|e| e.into())
	}

	/// extrinisc in this function should execute successfully on parentchain
	fn execute(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()>;
}

pub(crate) trait DecorateExecutor<R, S, T, N> {
	fn decode_and_execute(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		input: &mut &[u8],
	) -> Result<ExecutionStatus<H256>>;
}

impl<E, R, S, T, N> DecorateExecutor<R, S, T, N> for E
where
	E: Executor<R, S, T, N>,
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
{
	fn decode_and_execute(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		input: &mut &[u8],
	) -> Result<ExecutionStatus<H256>> {
		if let Ok(ParentchainUncheckedExtrinsicWithStatus { xt, status }) =
			self.decode(context, input)
		{
			if self.is_target_call(&xt.function, context.node_metadata_provider.as_ref()) {
				if status {
					debug!(
						"found extrinsic(call index: {:?}) with status {}",
						self.call_index(&xt.function),
						status
					);
					self.execute(context, xt.clone())
						.map(|_| ExecutionStatus::Success(hash_of(&xt)))
				} else {
					warn!(
						"extrinsic(call index: {:?}) fail to execute on parentchain.",
						self.call_index(&xt.function)
					);
					Ok(ExecutionStatus::Skip)
				}
			} else {
				Ok(ExecutionStatus::NextExecutor)
			}
		} else {
			Ok(ExecutionStatus::NextExecutor)
		}
	}
}
