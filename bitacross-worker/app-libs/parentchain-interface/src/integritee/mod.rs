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
	extrinsic_parser::{ExtrinsicParser, ParseExtrinsic, SemiOpaqueExtrinsic},
	indirect_calls::{RemoveScheduledEnclaveArgs, SetScheduledEnclaveArgs},
};
use bc_enclave_registry::{EnclaveRegistryUpdater, GLOBAL_ENCLAVE_REGISTRY};
use bc_relayer_registry::{RelayerRegistryUpdater, GLOBAL_RELAYER_REGISTRY};
use bc_signer_registry::{SignerRegistryUpdater, GLOBAL_SIGNER_REGISTRY};
use codec::{Decode, Encode};
use core::str::from_utf8;
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
use litentry_primitives::{Address32, AttestationType, Identity, WorkerMode, WorkerType};
use log::*;
use sp_runtime::{traits::BlakeTwo256, MultiAddress};
use std::{string::ToString, vec::Vec};

pub type BlockNumber = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Signature = sp_runtime::MultiSignature;

/// Parses the extrinsics corresponding to the parentchain.
pub type ParentchainExtrinsicParser = ExtrinsicParser<ParentchainSignedExtra>;

/// The default indirect call (extrinsic-triggered) of the Integritee-Parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	#[codec(index = 0)]
	SetScheduledEnclave(SetScheduledEnclaveArgs),
	#[codec(index = 1)]
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
	#[codec(index = 2)]
	AddRelayer(AddRelayerArgs),
	#[codec(index = 3)]
	RemoveRelayer(RemoveRelayerArgs),
	#[codec(index = 4)]
	AddEnclave(RegisterEnclaveArgs),
	#[codec(index = 5)]
	RemoveEnclave(UnregisterEnclaveArgs),
	#[codec(index = 5)]
	SaveSigner(SaveSignerArgs),
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for IndirectCall
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		trace!("dispatching indirect call {:?}", self);
		match self {
			IndirectCall::SetScheduledEnclave(set_scheduled_enclave_args) =>
				set_scheduled_enclave_args.dispatch(executor, ()),
			IndirectCall::RemoveScheduledEnclave(remove_scheduled_enclave_args) =>
				remove_scheduled_enclave_args.dispatch(executor, ()),
			IndirectCall::AddRelayer(add_relayer_args) => add_relayer_args.dispatch(executor, ()),
			IndirectCall::RemoveRelayer(remove_relayer_args) =>
				remove_relayer_args.dispatch(executor, ()),
			IndirectCall::AddEnclave(args) => args.dispatch(executor, ()),
			IndirectCall::RemoveEnclave(args) => args.dispatch(executor, ()),
			IndirectCall::SaveSigner(args) => args.dispatch(executor, ()),
		}
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct AddRelayerArgs {
	account_id: Identity,
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
	account_id: Identity,
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

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RegisterEnclaveArgs {
	account_id: Address32,
	worker_url: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct BtcWalletGeneratedCall {
	pub_key: [u8; 33],
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct SaveSignerArgs {
	account_id: Address32,
	pub_key: [u8; 33],
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct TeebagRegisterEnclaveCall {
	worker_type: WorkerType,
	worker_mode: WorkerMode,
	attestation: Vec<u8>,
	worker_url: Vec<u8>,
	shielding_pubkey: Option<Vec<u8>>,
	vc_pubkey: Option<sp_core::ed25519::Public>,
	attestation_type: AttestationType,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for SaveSignerArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::info!("Save signer : {:?}, pub_key: {:?}", self.account_id, self.pub_key);
		GLOBAL_SIGNER_REGISTRY.update(self.account_id, self.pub_key).unwrap();
		Ok(())
	}
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for RegisterEnclaveArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::info!("Register enclave : {:?}", self.account_id);
		GLOBAL_ENCLAVE_REGISTRY
			.update(self.account_id, from_utf8(&self.worker_url).unwrap().to_string())
			.unwrap();
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct UnregisterEnclaveArgs {
	account_id: Address32,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for UnregisterEnclaveArgs
{
	type Args = ();
	fn dispatch(&self, _executor: &Executor, _args: Self::Args) -> Result<()> {
		log::info!("Unregister enclave : {:?}", self.account_id);
		GLOBAL_ENCLAVE_REGISTRY.remove(self.account_id).unwrap();
		Ok(())
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

		if index == metadata.set_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<SetScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::SetScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else if index == metadata.add_relayer_call_indexes().ok()? {
			info!("Got add relayer call");
			let args = decode_and_log_error::<AddRelayerArgs>(call_args)?;
			Some(IndirectCall::AddRelayer(args))
		} else if index == metadata.remove_relayer_call_indexes().ok()? {
			info!("Got remove relayer call");
			let args = decode_and_log_error::<RemoveRelayerArgs>(call_args)?;
			Some(IndirectCall::RemoveRelayer(args))
		} else if index == metadata.register_enclave_call_indexes().ok()? {
			info!("Got register enclave call");
			let address = get_xt_sender(xt).ok()?;
			let call_args = decode_and_log_error::<TeebagRegisterEnclaveCall>(call_args)?;
			let args =
				RegisterEnclaveArgs { account_id: address, worker_url: call_args.worker_url };
			Some(IndirectCall::AddEnclave(args))
		} else if index == metadata.unregister_enclave_call_indexes().ok()? {
			info!("Got unregister enclave call");
			let address = get_xt_sender(xt).ok()?;
			let args = UnregisterEnclaveArgs { account_id: address };
			Some(IndirectCall::RemoveEnclave(args))
		} else if index == metadata.btc_wallet_generated_indexes().ok()? {
			info!("Got btc wallet generated call");
			let address = get_xt_sender(xt).ok()?;
			let call_args = decode_and_log_error::<BtcWalletGeneratedCall>(call_args)?;
			let args = SaveSignerArgs { account_id: address, pub_key: call_args.pub_key };
			Some(IndirectCall::SaveSigner(args))
		} else {
			None
		}
	}
}

fn get_xt_sender(
	xt: SemiOpaqueExtrinsic<ParentchainSignedExtra>,
) -> std::result::Result<Address32, ()> {
	let sender = xt.signature.ok_or(())?.0;
	let address = match sender {
		MultiAddress::Address32(bytes) => Address32::try_from(bytes.as_slice())?,
		MultiAddress::Raw(bytes) => Address32::try_from(bytes.as_slice())?,
		MultiAddress::Id(account_id) => account_id.into(),
		_ => return Err(()),
	};
	Ok(address)
}
