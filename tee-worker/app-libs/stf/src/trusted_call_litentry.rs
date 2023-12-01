// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use super::*;
use crate::{
	helpers::{
		enclave_signer_account, ensure_enclave_signer_account, ensure_enclave_signer_or_self,
		get_expected_raw_message, verify_web3_identity,
	},
	trusted_call_result::{LinkIdentityResult, TrustedCallResult},
	AccountId, ConvertAccountId, SgxParentchainTypeConverter, ShardIdentifier, StfError, StfResult,
	H256,
};
use codec::Encode;
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use ita_sgx_runtime::{RuntimeOrigin, System};
use itp_node_api::metadata::NodeMetadataTrait;
use itp_node_api_metadata::pallet_imp::IMPCallIndexes;
use itp_node_api_metadata_provider::AccessNodeMetadata;
use itp_utils::{if_production_or, stringify::account_id_to_string};
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, RequestType, Web2IdentityVerificationRequest,
};
use litentry_primitives::{
	Assertion, ErrorDetail, Identity, IdentityNetworkTuple, RequestAesKey, ValidationData,
	Web3Network,
};
use log::*;
use sp_core::blake2_256;
use std::{sync::Arc, vec::Vec};

#[cfg(not(feature = "production"))]
use crate::helpers::{ensure_alice, ensure_enclave_signer_or_alice};

impl TrustedCallSigned {
	#[allow(clippy::too_many_arguments)]
	pub fn link_identity_internal(
		shard: &ShardIdentifier,
		signer: AccountId,
		who: Identity,
		identity: Identity,
		validation_data: ValidationData,
		web3networks: Vec<Web3Network>,
		top_hash: H256,
		maybe_key: Option<RequestAesKey>,
		req_ext_hash: H256,
	) -> StfResult<bool> {
		ensure!(
			ensure_enclave_signer_or_self(&signer, who.to_account_id()),
			StfError::LinkIdentityFailed(ErrorDetail::UnauthorizedSigner)
		);

		// note it's the signer's nonce, not `who`
		// we intentionally use `System::account_nonce - 1` to make up for the increment at the
		// beginning of STF execution, otherwise it might be unexpected that we were hoping
		// (current nonce + 1) when verifying the validation data.
		let sidechain_nonce = System::account_nonce(&signer) - 1;

		let raw_msg = get_expected_raw_message(&who, &identity, sidechain_nonce);

		match validation_data {
			ValidationData::Web2(data) => {
				ensure!(
					identity.is_web2(),
					StfError::LinkIdentityFailed(ErrorDetail::InvalidIdentity)
				);
				let request: RequestType = Web2IdentityVerificationRequest {
					shard: *shard,
					who,
					identity,
					raw_msg,
					validation_data: data,
					web3networks,
					top_hash,
					maybe_key,
					req_ext_hash,
				}
				.into();
				StfRequestSender::new()
					.send_stf_request(request)
					.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::SendStfRequestFailed))?;
				Ok(false)
			},
			ValidationData::Web3(data) => {
				ensure!(
					identity.is_web3(),
					StfError::LinkIdentityFailed(ErrorDetail::InvalidIdentity)
				);
				verify_web3_identity(&identity, &raw_msg, &data)?;
				Ok(true)
			},
		}
	}

	pub fn deactivate_identity_internal(
		signer: AccountId,
		who: Identity,
		identity: Identity,
	) -> StfResult<()> {
		ensure!(
			ensure_enclave_signer_or_self(&signer, who.to_account_id()),
			StfError::DeactivateIdentityFailed(ErrorDetail::UnauthorizedSigner)
		);

		IMTCall::deactivate_identity { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::DeactivateIdentityFailed(e.into()))?;

		Ok(())
	}

	pub fn activate_identity_internal(
		signer: AccountId,
		who: Identity,
		identity: Identity,
	) -> StfResult<()> {
		ensure!(
			ensure_enclave_signer_or_self(&signer, who.to_account_id()),
			StfError::ActivateIdentityFailed(ErrorDetail::UnauthorizedSigner)
		);

		IMTCall::activate_identity { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::ActivateIdentityFailed(e.into()))?;

		Ok(())
	}

	pub fn request_vc_internal(
		signer: AccountId,
		who: Identity,
		assertion: Assertion,
		top_hash: H256,
		req_ext_hash: H256,
		maybe_key: Option<RequestAesKey>,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		match assertion {
			// the signer will be checked inside A13, as we don't seem to have access to ocall_api here
			Assertion::A13(_) => (),
			_ => if_production_or!(
				ensure!(
					ensure_enclave_signer_or_self(&signer, who.to_account_id()),
					StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorizedSigner)
				),
				ensure!(
					ensure_enclave_signer_or_self(&signer, who.to_account_id())
						|| ensure_alice(&signer),
					StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorizedSigner)
				)
			),
		}

		let id_graph = IMT::get_id_graph(&who);
		let assertion_networks = assertion.get_supported_web3networks();
		let identities: Vec<IdentityNetworkTuple> = id_graph
			.into_iter()
			.filter(|item| item.1.is_active())
			.map(|item| {
				let mut networks = item.1.web3networks.to_vec();
				// filter out the web3networks which are not supported by this specific `assertion`.
				// We do it here before every request sending because:
				// - it's a common step for all assertion buildings, for those assertions which only
				//   care about web2 identities, this step will empty `IdentityContext.web3networks`
				// - it helps to reduce the request size a bit
				networks.retain(|n| assertion_networks.contains(n));
				(item.0, networks)
			})
			.collect();
		let assertion_build: RequestType = AssertionBuildRequest {
			shard: *shard,
			signer,
			enclave_account: enclave_signer_account(),
			who,
			assertion: assertion.clone(),
			identities,
			top_hash,
			maybe_key,
			req_ext_hash,
		}
		.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(assertion_build).map_err(|e| {
			error!("[RequestVc] : {:?}", e);
			StfError::RequestVCFailed(assertion, ErrorDetail::SendStfRequestFailed)
		})
	}

	pub fn link_identity_callback_internal(
		signer: AccountId,
		who: Identity,
		identity: Identity,
		web3networks: Vec<Web3Network>,
	) -> StfResult<()> {
		if_production_or!(
			{
				// In prod: the signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
				ensure_enclave_signer_account(&signer)
					.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::UnauthorizedSigner))?;
			},
			{
				// In non-prod: we allow to use `Alice` as the dummy signer
				ensure!(
					ensure_enclave_signer_or_alice(&signer),
					StfError::LinkIdentityFailed(ErrorDetail::UnauthorizedSigner)
				);
			}
		);
		IMTCall::link_identity { who, identity, web3networks }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::LinkIdentityFailed(e.into()))?;

		Ok(())
	}

	pub fn request_vc_callback_internal(signer: AccountId, assertion: Assertion) -> StfResult<()> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer)
			.map_err(|_| StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorizedSigner))?;

		Ok(())
	}

	// common handler for both web2 and web3 identity verification
	#[allow(clippy::too_many_arguments)]
	pub fn handle_link_identity_callback<NodeMetadataRepository>(
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
		signer: Identity,
		who: Identity,
		identity: Identity,
		web3networks: Vec<Web3Network>,
		maybe_key: Option<RequestAesKey>,
		hash: H256,
	) -> StfResult<TrustedCallResult>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType: NodeMetadataTrait,
	{
		debug!("link_identity_callback, who: {}", account_id_to_string(&who));
		let account = SgxParentchainTypeConverter::convert(
			who.to_account_id().ok_or(StfError::InvalidAccount)?,
		);

		Self::link_identity_callback_internal(
			signer.to_account_id().ok_or(StfError::InvalidAccount)?,
			who.clone(),
			identity,
			web3networks,
		)
		.map_err(|e| {
			debug!("pushing error event ... error: {}", e);
			push_call_imp_some_error(
				calls,
				node_metadata_repo.clone(),
				Some(account.clone()),
				e.to_imp_error(),
				hash,
			);
			e
		})?;

		debug!("pushing identity_linked event ...");
		let id_graph = IMT::get_id_graph(&who);
		// push `identity_linked` call
		let call_index =
			node_metadata_repo.get_from_metadata(|m| m.identity_linked_call_indexes())??;
		calls.push(OpaqueCall::from_tuple(&(call_index, account.clone(), hash)));
		// push `update_id_graph_hash` call
		push_call_imp_update_id_graph_hash(
			calls,
			node_metadata_repo,
			account,
			blake2_256(&id_graph.encode()).into(),
		);

		if let Some(key) = maybe_key {
			Ok(TrustedCallResult::LinkIdentity(LinkIdentityResult {
				id_graph: aes_encrypt_default(&key, &id_graph.encode()),
			}))
		} else {
			Ok(TrustedCallResult::Empty)
		}
	}
}
