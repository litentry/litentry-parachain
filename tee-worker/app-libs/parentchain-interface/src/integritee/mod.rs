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
	indirect_calls::{
		ActivateIdentityArgs, DeactivateIdentityArgs, InvokeArgs, LinkIdentityArgs,
		RemoveScheduledEnclaveArgs, RequestVCArgs, SetScheduledEnclaveArgs, ShieldFundsArgs,
	},
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
use sp_core::crypto::AccountId32;
use sp_runtime::{traits::BlakeTwo256, MultiAddress};
use sp_std::vec::Vec;

pub type BlockNumber = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Signature = sp_runtime::MultiSignature;

/// Parses the extrinsics corresponding to the parentchain.
pub type ParentchainExtrinsicParser = ExtrinsicParser<ParentchainSignedExtra>;

/// The default indirect call (extrinsic-triggered) of Litentry parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	#[codec(index = 0)]
	ShieldFunds(ShieldFundsArgs),
	#[codec(index = 1)]
	Invoke(InvokeArgs),
	// Litentry
	// we ignore `TimestampSet` for now, see unused `TrustedCall::timestamp_set`
	#[codec(index = 2)]
	LinkIdentity(LinkIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	#[codec(index = 3)]
	DeactivateIdentity(DeactivateIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	#[codec(index = 4)]
	ActivateIdentity(ActivateIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	#[codec(index = 5)]
	RequestVC(RequestVCArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	#[codec(index = 6)]
	SetScheduledEnclave(SetScheduledEnclaveArgs),
	#[codec(index = 7)]
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
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
			IndirectCall::ShieldFunds(shieldfunds_args) => shieldfunds_args.dispatch(executor, ()),
			IndirectCall::Invoke(invoke_args) => invoke_args.dispatch(executor, ()),
			// Litentry
			IndirectCall::LinkIdentity(verify_id, address, hash) =>
				verify_id.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::DeactivateIdentity(deactivate_identity, address, hash) =>
				deactivate_identity.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::ActivateIdentity(activate_identity, address, hash) =>
				activate_identity.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::RequestVC(request_vc, address, hash) =>
				request_vc.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::SetScheduledEnclave(update_enclave_args) =>
				update_enclave_args.dispatch(executor, ()),
			IndirectCall::RemoveScheduledEnclave(remove_enclave_args) =>
				remove_enclave_args.dispatch(executor, ()),
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
		let address = if let Some(signature) = xt.signature { Some(signature.0) } else { None };
		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		trace!("ExtrinsicFilter: attempting to execute indirect call with index {:?}", index);
		if index == metadata.post_opaque_task_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			Some(IndirectCall::Invoke(args))
		// Litentry
		} else if index == metadata.link_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<LinkIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::LinkIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.deactivate_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<DeactivateIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::DeactivateIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.activate_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<ActivateIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::ActivateIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::RequestVC(args, address, hashed_extrinsic))
		} else if index == metadata.set_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<SetScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::SetScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.batch_all_call_indexes().ok()? {
			parse_batch_all(call_args, metadata, address, xt.hashed_extrinsic)
		} else {
			None
		}
	}
}

fn parse_batch_all<NodeMetadata: NodeMetadataTrait>(
	call_args: &mut &[u8],
	metadata: &NodeMetadata,
	address: Option<MultiAddress<AccountId32, ()>>,
	hash: H256,
) -> Option<IndirectCall> {
	let call_count: Vec<()> = Decode::decode(call_args).ok()?;
	let mut calls: Vec<IndirectCall> = Vec::new();
	log::debug!("Received BatchAll including {} calls", call_count.len());
	for _i in 0..call_count.len() {
		let index: CallIndex = Decode::decode(call_args).ok()?;
		if index == metadata.post_opaque_task_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			calls.push(IndirectCall::Invoke(args))
		} else if index == metadata.link_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<LinkIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::LinkIdentity(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.deactivate_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<DeactivateIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::DeactivateIdentity(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.activate_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<ActivateIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::ActivateIdentity(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::RequestVC(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.set_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<SetScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::SetScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::RemoveScheduledEnclave(args))
		}
	}
	Some(IndirectCall::BatchAll(calls))
}
