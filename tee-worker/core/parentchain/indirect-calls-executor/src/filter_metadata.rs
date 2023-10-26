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
	event_filter::{FilterEvents, MockEvents},
	indirect_calls::{
		ActivateIdentityArgs, DeactivateIdentityArgs, InvokeArgs, LinkIdentityArgs,
		RemoveScheduledEnclaveArgs, RequestVCArgs, SetUserShieldingKeyArgs, ShieldFundsArgs,
		TransferToAliceShieldsFundsArgs, UpdateScheduledEnclaveArgs, ALICE_ACCOUNT_ID,
	},
	parentchain_parser::ParseExtrinsic,
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use itp_api_client_types::{Events, Metadata};
use itp_node_api::metadata::{
	pallet_balances::BalancesCallIndexes, NodeMetadata, NodeMetadataTrait,
};
use itp_types::{CallIndex, H256};
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;
use sp_std::{vec, vec::Vec};

pub trait EventsFromMetadata<NodeMetadata> {
	type Output: FilterEvents;

	fn create_from_metadata(
		metadata: NodeMetadata,
		block_hash: H256,
		events: &[u8],
	) -> Option<Self::Output>;
}

pub struct EventCreator;

impl<NodeMetadata: TryInto<Metadata> + Clone> EventsFromMetadata<NodeMetadata> for EventCreator {
	type Output = Events<H256>;

	fn create_from_metadata(
		metadata: NodeMetadata,
		block_hash: H256,
		events: &[u8],
	) -> Option<Self::Output> {
		let raw_metadata: Metadata = metadata.try_into().ok()?;
		Some(Events::<H256>::new(raw_metadata, block_hash, events.to_vec()))
	}
}

pub struct TestEventCreator;

impl<NodeMetadata> EventsFromMetadata<NodeMetadata> for TestEventCreator {
	type Output = MockEvents;

	fn create_from_metadata(
		_metadata: NodeMetadata,
		_block_hash: H256,
		_events: &[u8],
	) -> Option<Self::Output> {
		Some(MockEvents)
	}
}

/// Trait to filter an indirect call and decode into it, where the decoding
/// is based on the metadata provided.
pub trait FilterIntoDataFrom<NodeMetadata> {
	/// Type to decode into.
	type Output;

	/// Knows how to parse the parentchain metadata.
	type ParseParentchainMetadata;

	/// Filters some bytes and returns `Some(Self::Output)` if the filter matches some criteria.
	fn filter_into_from_metadata(
		encoded_data: &[u8],
		metadata: &NodeMetadata,
	) -> Option<Self::Output>;
}

/// Indirect calls filter denying all indirect calls.
pub struct DenyAll;

/// Simple demo filter for testing.
///
/// A transfer to Alice will issue the corresponding balance to Alice in the enclave.
/// It does not do anything else.
pub struct TransferToAliceShieldsFundsFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}
/// Default filter we use for the Integritee-Parachain.
pub struct ShieldFundsAndInvokeFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterIntoDataFrom<NodeMetadata>
	for ShieldFundsAndInvokeFilter<ExtrinsicParser>
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
					"[ShieldFundsAndInvokeFilter] Could not parse parentchain extrinsic: {:?}",
					e
				);
				return None
			},
		};

		let address = if let Some(signature) = xt.signature { Some(signature.0) } else { None };

		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		log::trace!(
			"[ShieldFundsAndInvokeFilter] attempting to execute indirect call with index {:?}",
			index
		);

		if index == metadata.shield_funds_call_indexes().ok()? {
			let args = decode_and_log_error::<ShieldFundsArgs>(call_args)?;
			Some(IndirectCall::ShieldFunds(args))
		} else if index == metadata.invoke_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			Some(IndirectCall::Invoke(args))
		}
		// Litentry
		else if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			let hashed_extrinsic = xt.hashed_extrinsic;
			Some(IndirectCall::SetUserShieldingKey(args, address, hashed_extrinsic))
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
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.batch_all_call_indexes().ok()? {
			parse_batch_all(call_args, metadata, address, xt.hashed_extrinsic)
		} else {
			log::debug!("executing no call");
			None
		}
	}
}

impl<ExtrinsicParser, NodeMetadata: BalancesCallIndexes> FilterIntoDataFrom<NodeMetadata>
	for TransferToAliceShieldsFundsFilter<ExtrinsicParser>
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
				log::error!("[TransferToAliceShieldsFundsFilter] Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};
		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		log::trace!("[TransferToAliceShieldsFundsFilter] attempting to execute indirect call with index {:?}", index);
		if index == metadata.transfer_call_index().ok()?
			|| index == metadata.transfer_allow_death_call_index().ok()?
		{
			log::debug!("found `transfer` or `transfer_allow_death` call.");
			let args = decode_and_log_error::<TransferToAliceShieldsFundsArgs>(call_args)?;
			if args.destination == ALICE_ACCOUNT_ID.into() {
				Some(IndirectCall::TransferToAliceShieldsFunds(args))
			} else {
				log::debug!("Parentchain transfer was not for Alice; ignoring...");
				// No need to put it into the top pool if it isn't executed in the first place.
				None
			}
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
	ShieldFunds(ShieldFundsArgs),
	Invoke(InvokeArgs),
	TransferToAliceShieldsFunds(TransferToAliceShieldsFundsArgs),
	// Litentry
	SetUserShieldingKey(SetUserShieldingKeyArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	LinkIdentity(LinkIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	DeactivateIdentity(DeactivateIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	ActivateIdentity(ActivateIdentityArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	RequestVC(RequestVCArgs, Option<MultiAddress<AccountId32, ()>>, H256),
	UpdateScheduledEnclave(UpdateScheduledEnclaveArgs),
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
	BatchAll(Vec<IndirectCall>),
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for IndirectCall {
	type Args = ();

	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		match self {
			IndirectCall::ShieldFunds(shieldfunds_args) => shieldfunds_args.dispatch(executor, ()),
			IndirectCall::Invoke(invoke_args) => invoke_args.dispatch(executor, ()),
			IndirectCall::TransferToAliceShieldsFunds(trans_args) =>
				trans_args.dispatch(executor, ()),
			// Litentry
			IndirectCall::SetUserShieldingKey(set_shied, address, hash) =>
				set_shied.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::LinkIdentity(verify_id, address, hash) =>
				verify_id.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::DeactivateIdentity(deactivate_identity, address, hash) =>
				deactivate_identity.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::ActivateIdentity(activate_identity, address, hash) =>
				activate_identity.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::RequestVC(request_vc, address, hash) =>
				request_vc.dispatch(executor, (address.clone(), *hash)),
			IndirectCall::UpdateScheduledEnclave(update_enclave_args) =>
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

fn decode_and_log_error<V: Decode>(encoded: &mut &[u8]) -> Option<V> {
	match V::decode(encoded) {
		Ok(v) => Some(v),
		Err(e) => {
			log::warn!("Could not decode. {:?}", e);
			None
		},
	}
}

mod seal {
	use super::*;

	/// Stub struct for the `DenyAll` filter that never executes anything.
	#[derive(Debug, Encode)]
	pub struct CantExecute;

	impl FilterIntoDataFrom<NodeMetadata> for DenyAll {
		type Output = CantExecute;
		type ParseParentchainMetadata = ();

		fn filter_into_from_metadata(_: &[u8], _: &NodeMetadata) -> Option<CantExecute> {
			None
		}
	}

	impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for CantExecute {
		type Args = ();
		fn dispatch(&self, _: &Executor, _args: Self::Args) -> Result<()> {
			// We should never get here because `CantExecute` is in a private module and the trait
			// implementation is sealed and always returns `None` instead of a `CantExecute` instance.
			// Regardless, we never want the enclave to panic, this is why we take this extra safety
			// measure.
			log::warn!(
				"Executed indirect dispatch for 'CantExecute'\
			 	this means there is some logic error."
			);
			Ok(())
		}
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
	log::debug!("Received BatchAll including {} calls", call_count.len());
	for _i in 0..call_count.len() {
		let index: CallIndex = Decode::decode(call_args).ok()?;
		if index == metadata.shield_funds_call_indexes().ok()? {
			let args = decode_and_log_error::<ShieldFundsArgs>(call_args)?;
			calls.push(IndirectCall::ShieldFunds(args))
		} else if index == metadata.invoke_call_indexes().ok()? {
			let args = decode_and_log_error::<InvokeArgs>(call_args)?;
			calls.push(IndirectCall::Invoke(args))
		} else if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			let hashed_extrinsic = hash;
			calls.push(IndirectCall::SetUserShieldingKey(args, address.clone(), hashed_extrinsic))
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
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			calls.push(IndirectCall::RemoveScheduledEnclave(args))
		}
	}
	Some(IndirectCall::BatchAll(calls))
}
