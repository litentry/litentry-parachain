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

use crate::{
	error::Result,
	indirect_calls::{
		CallWorkerArgs, CreateIdentityArgs, RemoveIdentityArgs, RemoveScheduledEnclaveArgs,
		RequestVCArgs, SetUserShieldingKeyArgs, ShiedFundsArgs, UpdateScheduledEnclaveArgs,
		VerifyIdentityArgs,
	},
	parentchain_extrinsic_parser::ParseExtrinsic,
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use itp_node_api::metadata::{NodeMetadata, NodeMetadataTrait};
use itp_types::{CallIndex, H256};
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;
use sp_std::{vec, vec::Vec};

/// Trait to filter an indirect call and decode into it, where the decoding
/// is based on the metadata provided.
pub trait FilterCalls<NodeMetadata> {
	/// Call enum we try to decode into.
	type Call;

	/// Knows how to parse the parentchain extrinsics.
	type ParseParentchainExtrinsic;

	/// Filters some bytes and returns `Some(Self::Call)` if the filter matches some criteria.
	fn filter_into_with_metadata(call: &[u8], metadata: &NodeMetadata) -> Option<Self::Call>;
}

/// Indirect calls filter denying all indirect calls.
pub struct DenyAll;

impl FilterCalls<NodeMetadata> for DenyAll {
	type Call = ();
	type ParseParentchainExtrinsic = ();

	fn filter_into_with_metadata(_: &[u8], _: &NodeMetadata) -> Option<Self::Call> {
		None
	}
}

/// Default filter we use for the Integritee-Parachain.
pub struct ShieldFundsAndCallWorkerFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterCalls<NodeMetadata>
	for ShieldFundsAndCallWorkerFilter<ExtrinsicParser>
where
	ExtrinsicParser: ParseExtrinsic,
{
	type Call = IndirectCall;
	type ParseParentchainExtrinsic = ExtrinsicParser;

	fn filter_into_with_metadata(call: &[u8], metadata: &NodeMetadata) -> Option<Self::Call> {
		let call_mut = &mut &call[..];

		// Todo: the filter should not need to parse, only filter. This should directly be configured
		// in the indirect executor.
		let xt = match Self::ParseParentchainExtrinsic::parse(call_mut) {
			Ok(xt) => xt,
			Err(e) => {
				log::error!("Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};
		let address = if let Some(signature) = xt.signature { Some(signature.0) } else { None };

		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];

		if index == metadata.shield_funds_call_indexes().ok()? {
			let args = decode_and_log_error::<ShiedFundsArgs>(call_args)?;
			Some(IndirectCall::ShieldFunds(args))
		} else if index == metadata.call_worker_call_indexes().ok()? {
			let args = decode_and_log_error::<CallWorkerArgs>(call_args)?;
			Some(IndirectCall::CallWorker(args))
		} else if index == metadata.create_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<CreateIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::CreateIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.remove_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::RemoveIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::RequestVC(args, address, hashed_extrinsic))
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::SetUserShieldingKey(args, address, hashed_extrinsic))
		} else if index == metadata.verify_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<VerifyIdentityArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::VerifyIdentity(args, address, hashed_extrinsic))
		} else if index == metadata.batch_all_call_indexes().ok()? {
			parse_batch_all(call_args, metadata, address, xt.hashed_extrinsic)
		} else {
			None
		}
	}
}

/// The default indirect call of the Integritee-Parachain.
///
/// Todo: Move or provide a template in app-libs such that users
/// can implemeent their own indirect call there.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	ShieldFunds(ShiedFundsArgs),
	CallWorker(CallWorkerArgs),
	CreateIdentity(CreateIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	RemoveIdentity(RemoveIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	RequestVC(RequestVCArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	UpdateScheduledEnclave(UpdateScheduledEnclaveArgs),
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
	SetUserShieldingKey(SetUserShieldingKeyArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	VerifyIdentity(VerifyIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	BatchAll(Vec<IndirectCall>),
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for IndirectCall {
	type Args = u32;
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		let block = args;
		match self {
			IndirectCall::ShieldFunds(shieldfunds) => shieldfunds.dispatch(executor, ()),
			IndirectCall::CallWorker(call_worker) => call_worker.dispatch(executor, ()),
			IndirectCall::CreateIdentity(create_identity, address, hash) =>
				create_identity.dispatch(executor, (address.clone(), block, *hash)),
			IndirectCall::RemoveIdentity(remove_ideentity, address, hash) =>
				remove_ideentity.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::RequestVC(requestvc, address, hash) =>
				requestvc.dispatch(executor, (address.clone(), *hash, block)),
			IndirectCall::UpdateScheduledEnclave(update_enclave_args) =>
				update_enclave_args.dispatch(executor, ()),
			IndirectCall::RemoveScheduledEnclave(remove_enclave_args) =>
				remove_enclave_args.dispatch(executor, ()),
			IndirectCall::SetUserShieldingKey(set_shied, address, hash) =>
				set_shied.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::VerifyIdentity(verify_id, address, hash) =>
				verify_id.dispatch(executor, (address.clone(), *hash, block)),
			IndirectCall::BatchAll(calls) => {
				for x in calls.clone() {
					if let Err(e) = x.dispatch(executor, block) {
						log::warn!("Failed to execute indirect call in batch all due to: {:?}", e);
						continue
					}
				}
				Ok(())
			},
		}
	}
}

fn decode_and_log_error<V: Decode>(encoded: &mut &[u8]) -> Option<V> {
	match V::decode(encoded) {
		Ok(v) => Some(v),
		Err(e) => {
			log::warn!("Could not decode. {:?}", e);
			None
		},
	}
}

fn parse_batch_all<NodeMetadata: NodeMetadataTrait>(
	call_args: &mut &[u8],
	metadata: &NodeMetadata,
	address: Option<MultiAddress<AccountId32, ()>>,
	hash: H256,
) -> Option<IndirectCall> {
	let call_count: sp_std::vec::Vec<()> = Decode::decode(call_args).ok()?;
	let mut calls: Vec<IndirectCall> = vec![];
	log::debug!("Received Batch All Call for This Many call: {}", call_count.len());
	for _i in 0..call_count.len() {
		let index: CallIndex = Decode::decode(call_args).ok()?;
		if index == metadata.shield_funds_call_indexes().ok()? {
			let args = decode_and_log_error::<ShiedFundsArgs>(call_args)?;
			calls.push(IndirectCall::ShieldFunds(args))
		} else if index == metadata.call_worker_call_indexes().ok()? {
			let args = decode_and_log_error::<CallWorkerArgs>(call_args)?;
			calls.push(IndirectCall::CallWorker(args))
		} else if index == metadata.create_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<CreateIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::CreateIdentity(args, address.clone(), hashed_extrinsic));
		} else if index == metadata.remove_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::RemoveIdentity(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::RequestVC(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::SetUserShieldingKey(args, address.clone(), hashed_extrinsic))
		} else if index == metadata.verify_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<VerifyIdentityArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::VerifyIdentity(args, address.clone(), hashed_extrinsic))
		}
	}
	Some(IndirectCall::BatchAll(calls))
}
