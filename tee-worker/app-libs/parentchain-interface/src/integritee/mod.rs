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

use crate::{
	decode_and_log_error,
	extrinsic_parser::{ExtrinsicParser, ParseExtrinsic},
	indirect_calls::InvokeArgs,
};
use codec::{Decode, Encode};
pub use event_filter::FilterableEvents;
pub use event_handler::ParentchainEventHandler;
use ita_stf::TrustedCallSigned;
use itc_parentchain_indirect_calls_executor::{
	error::{Error, Result},
	filter_metadata::FilterIntoDataFrom,
	IndirectDispatch,
};
use itp_api_client_types::ParentchainSignedExtra;
use itp_node_api::metadata::NodeMetadataTrait;
use itp_stf_primitives::traits::IndirectExecutor;
pub use itp_types::{
	parentchain::{AccountId, Balance, Hash},
	CallIndex, H256,
};
use log::*;
use sp_runtime::traits::BlakeTwo256;
use sp_std::vec::Vec;

pub type BlockNumber = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Signature = sp_runtime::MultiSignature;

/// Parses the extrinsics corresponding to the parentchain.
pub type ParentchainExtrinsicParser = ExtrinsicParser<ParentchainSignedExtra>;

/// The default indirect call (extrinsic-triggered) of Litentry parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	#[codec(index = 1)]
	Invoke(InvokeArgs),
	#[codec(index = 8)]
	BatchAll(Vec<IndirectCall>),
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for IndirectCall
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		trace!("dispatching indirect call {:?}", self);
		match self {
			IndirectCall::Invoke(invoke_args) => invoke_args.dispatch(executor, ()),
			IndirectCall::BatchAll(calls) => {
				for x in calls.clone() {
					if let Err(e) = x.dispatch(executor, ()) {
						log::warn!("Failed to execute indirect call in batch all due to: {:?}", e);
						continue
					}
				}
				Ok(())
			},
		}
	}
}

/// Default filter we use for Litentry parachain.
pub struct ExtrinsicFilter {}

impl<NodeMetadata: NodeMetadataTrait> FilterIntoDataFrom<NodeMetadata> for ExtrinsicFilter {
	type Output = IndirectCall;
	type ParseParentchainMetadata = ParentchainExtrinsicParser;

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
				error!("ExtrinsicFilter: Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};
		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		trace!("ExtrinsicFilter: attempting to execute indirect call with index {:?}", index);
		if index == metadata.post_opaque_task_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			Some(IndirectCall::Invoke(args))
		} else if index == metadata.batch_all_call_indexes().ok()? {
			parse_batch_all(call_args, metadata)
		} else {
			None
		}
	}
}

fn parse_batch_all<NodeMetadata: NodeMetadataTrait>(
	call_args: &mut &[u8],
	metadata: &NodeMetadata,
) -> Option<IndirectCall> {
	let call_count: Vec<()> = Decode::decode(call_args).ok()?;
	let mut calls: Vec<IndirectCall> = Vec::new();
	log::debug!("Received BatchAll including {} calls", call_count.len());
	for _i in 0..call_count.len() {
		let index: CallIndex = Decode::decode(call_args).ok()?;
		if index == metadata.post_opaque_task_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			calls.push(IndirectCall::Invoke(args))
		}
	}
	Some(IndirectCall::BatchAll(calls))
}
