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
	indirect_calls::{
		InvokeArgs, RemoveScheduledEnclaveArgs, ShieldFundsArgs, UpdateScheduledEnclaveArgs,
	},
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
use itp_types::CallIndex;
use log::trace;
use sp_core::crypto::AccountId32;
use sp_std::vec::Vec;

/// The default indirect call (extrinsic-triggered) of the Integritee-Parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	#[codec(index = 0)]
	BitAcross(BitAcrossArgs),
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for IndirectCall
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		trace!("dispatching indirect call {:?}", self);
		match self {
			IndirectCall::BitAcross(bitacross_args) => bitacross_args.dispatch(executor, ()),
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

		if index == metadata.placeholder_call_indexes().ok()? {
			let args = decode_and_log_error::<BitAcrossArgs>(call_args)?;
			Some(IndirectCall::BitAcross(args))
		} else {
			None
		}
	}
}
