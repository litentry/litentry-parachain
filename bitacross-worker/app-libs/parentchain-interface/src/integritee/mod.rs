/*
	Copyright 2021 Integritee AG

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

mod event_filter;
mod event_handler;
mod extrinsic_parser;

use crate::{
	decode_and_log_error,
	indirect_calls::{RemoveScheduledEnclaveArgs, UpdateScheduledEnclaveArgs},
	integritee::extrinsic_parser::ParseExtrinsic,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
pub use event_filter::FilterableEvents;
pub use event_handler::ParentchainEventHandler;
pub use extrinsic_parser::ParentchainExtrinsicParser;
use ita_stf::TrustedCallSigned;
use itc_parentchain_indirect_calls_executor::{
	error::{Error, Result},
	filter_metadata::FilterIntoDataFrom,
	IndirectDispatch,
};
use itp_node_api::metadata::NodeMetadataTrait;
use itp_stf_primitives::traits::IndirectExecutor;
use litentry_primitives::{RelayerRegistryUpdater, GLOBAL_RELAYER_REGISTRY};
use log::trace;
use sp_core::crypto::AccountId32;

/// The default indirect call (extrinsic-triggered) of the Integritee-Parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	#[codec(index = 0)]
	BitAcross(BitAcrossArgs),
	#[codec(index = 1)]
	UpdateScheduledEnclave(UpdateScheduledEnclaveArgs),
	#[codec(index = 2)]
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
	#[codec(index = 3)]
	AddRelayer(AddRelayerArgs),
	#[codec(index = 4)]
	RemoveRelayer(RemoveRelayerArgs),
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for IndirectCall
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		trace!("dispatching indirect call {:?}", self);
		match self {
			IndirectCall::BitAcross(bitacross_args) => bitacross_args.dispatch(executor, ()),
			IndirectCall::UpdateScheduledEnclave(update_scheduled_enclave_args) =>
				update_scheduled_enclave_args.dispatch(executor, ()),
			IndirectCall::RemoveScheduledEnclave(remove_scheduled_enclave_args) =>
				remove_scheduled_enclave_args.dispatch(executor, ()),
			IndirectCall::AddRelayer(add_relayer_args) => add_relayer_args.dispatch(executor, ()),
			IndirectCall::RemoveRelayer(remove_relayer_args) =>
				remove_relayer_args.dispatch(executor, ()),
		}
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct BitAcrossArgs {
	account_id: AccountId32,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for BitAcrossArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::error!("Not yet implemented");
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct AddRelayerArgs {
	account_id: AccountId32,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for AddRelayerArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::info!("Adding Relayer Account to Registry: {:?}", self.account_id);
		GLOBAL_RELAYER_REGISTRY.update(self.account_id.clone()).unwrap();
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RemoveRelayerArgs {
	account_id: AccountId32,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for RemoveRelayerArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::info!("Remove Relayer Account from Registry: {:?}", self.account_id);
		GLOBAL_RELAYER_REGISTRY.remove(self.account_id.clone()).unwrap();
		Ok(())
	}
}

/// Default filter we use for the Integritee-Parachain.
pub struct BitAcrossIndirectCallsFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterIntoDataFrom<NodeMetadata>
	for BitAcrossIndirectCallsFilter<ExtrinsicParser>
where
	ExtrinsicParser: ParseExtrinsic,
{
	type Output = IndirectCall;
	type ParseParentchainMetadata = ExtrinsicParser;

	fn filter_into_from_metadata(
		encoded_data: &[u8],
		metadata: &NodeMetadata,
	) -> Option<Self::Output> {
		let call_mut = &mut &encoded_data[..];

		// Todo: the filter should not need to parse, only filter. This should directly be configured
		// in the indirect executor.
		let xt = match Self::ParseParentchainMetadata::parse(call_mut) {
			Ok(xt) => xt,
			Err(e) => {
				log::error!(
					"[BitAcrossIndirectCallsFilter] Could not parse parentchain extrinsic: {:?}",
					e
				);
				return None
			},
		};
		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		log::error!("Parsing the call indexes");

		if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.add_relayer_call_indexes().ok()? {
			log::error!("Received Add Relayer indirect call");
			let args = decode_and_log_error::<AddRelayerArgs>(call_args)?;
			Some(IndirectCall::AddRelayer(args))
		} else if index == metadata.remove_relayer_call_indexes().ok()? {
			log::error!("Processing Remove Relayer Call");
			let args = decode_and_log_error::<RemoveRelayerArgs>(call_args)?;
			Some(IndirectCall::RemoveRelayer(args))
		} else {
			None
		}
	}
}
