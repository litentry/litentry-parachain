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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use super::*;
use crate::{
	helpers::{ensure_enclave_signer_account, generate_challenge_code},
	is_root, AccountId, IdentityManagement, MetadataOf, Runtime, StfError, StfResult,
};
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use ita_sgx_runtime::RuntimeOrigin;
use itp_stf_primitives::types::ShardIdentifier;
use itp_utils::stringify::account_id_to_string;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, IdentityVerificationRequest, RequestType, SetUserShieldingKeyRequest,
};
use litentry_primitives::{
	Assertion, ChallengeCode, ErrorDetail, Identity, ParentchainBlockNumber, UserShieldingKeyType,
	ValidationData,
};
use log::*;
use std::{format, sync::Arc, vec::Vec};

type IMTCall = ita_sgx_runtime::IdentityManagementCall<Runtime>;

impl TrustedCallSigned {
	pub fn set_user_shielding_key_preflight(
		root: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		key: UserShieldingKeyType,
		hash: H256,
	) -> StfResult<()> {
		ensure!(is_root::<Runtime, AccountId>(&root), StfError::MissingPrivileges(root));
		let request = SetUserShieldingKeyRequest { shard: *shard, who, key, hash }.into();
		let sender = StfRequestSender::new();
		sender
			.send_stf_request(request)
			.map_err(|_| StfError::SetUserShieldingKeyFailed(ErrorDetail::SendStfRequestFailed))
	}

	pub fn set_user_shielding_key_runtime<NodeMetadataRepository>(
		node_metadata_repo: Arc<NodeMetadataRepository>,
		calls: &mut Vec<OpaqueCall>,
		who: AccountId,
		key: UserShieldingKeyType,
		hash: H256,
	) -> StfResult<()>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType:
			TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
	{
		debug!("set user shielding key runtime, who = {:?}", account_id_to_string(&who));
		let account = SgxParentchainTypeConverter::convert(who.clone());
		let parent_ss58_prefix =
			node_metadata_repo.get_from_metadata(|m| m.system_ss58_prefix())??;
		match (IMTCall::set_user_shielding_key { who: who.clone(), key, parent_ss58_prefix }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::SetUserShieldingKeyFailed(e.error.into())))
		{
			Ok(_) =>
				if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
					debug!("pushing user_shielding_key_set event ...");
					let id_graph =
						ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph_with_max_len(
							&who,
							RETURNED_IDGRAPH_MAX_LEN,
						);
					calls.push(OpaqueCall::from_tuple(&(
						node_metadata_repo
							.get_from_metadata(|m| m.user_shielding_key_set_call_indexes())??,
						account,
						aes_encrypt_default(&key, &id_graph.encode()),
						hash,
					)));
				} else {
					debug!("pushing error event ... error: UserShieldingKeyNotFound");
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(account),
						IMPError::SetUserShieldingKeyFailed(ErrorDetail::UserShieldingKeyNotFound),
						hash,
					);
				},
			Err(e) => {
				debug!("pushing error event ... error: {}", e);
				add_call_from_imp_error(
					calls,
					node_metadata_repo,
					Some(account),
					e.to_imp_error(),
					hash,
				);
			},
		}
		Ok(())
	}

	pub fn create_identity_runtime<NodeMetadataRepository>(
		node_metadata_repo: Arc<NodeMetadataRepository>,
		calls: &mut Vec<OpaqueCall>,
		who: AccountId,
		identity: Identity,
		metadata: Option<MetadataOf<Runtime>>,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType:
			TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
	{
		let account = SgxParentchainTypeConverter::convert(who.clone());
		if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
			let parent_ss58_prefix =
				node_metadata_repo.get_from_metadata(|m| m.system_ss58_prefix())??;

			match Self::create_identity_runtime_internal(
				who,
				identity.clone(),
				metadata,
				bn,
				parent_ss58_prefix,
			) {
				Ok(code) => {
					debug!("pushing identity_created event ...");
					calls.push(OpaqueCall::from_tuple(&(
						node_metadata_repo
							.get_from_metadata(|m| m.identity_created_call_indexes())??,
						account,
						aes_encrypt_default(&key, &identity.encode()),
						aes_encrypt_default(&key, &code.encode()),
						hash,
					)));
				},
				Err(e) => {
					debug!("pushing error event ... error: {}", e);
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(account),
						e.to_imp_error(),
						hash,
					);
				},
			}
		} else {
			debug!("pushing error event ... error: UserShieldingKeyNotFound");
			add_call_from_imp_error(
				calls,
				node_metadata_repo,
				Some(account),
				IMPError::CreateIdentityFailed(ErrorDetail::UserShieldingKeyNotFound),
				hash,
			);
		}
		Ok(())
	}

	pub fn remove_identity_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
	) -> StfResult<()> {
		ensure_enclave_signer_account(&enclave_account)?;
		IMTCall::remove_identity { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::RemoveIdentityFailed(e.into()))?;
		Ok(())
	}

	pub fn verify_identity_preflight(
		enclave_account: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()> {
		ensure_enclave_signer_account(&enclave_account)?;
		let code = IdentityManagement::challenge_codes(&who, &identity)
			.ok_or(StfError::VerifyIdentityFailed(ErrorDetail::ChallengeCodeNotFound))?;

		let request: RequestType = IdentityVerificationRequest {
			shard: *shard,
			who,
			identity,
			challenge_code: code,
			validation_data,
			bn,
			hash,
		}
		.into();
		let sender = StfRequestSender::new();
		sender
			.send_stf_request(request)
			.map_err(|_| StfError::VerifyIdentityFailed(ErrorDetail::SendStfRequestFailed))
	}

	pub fn verify_identity_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		ensure_enclave_signer_account(&enclave_account)?;
		IMTCall::verify_identity {
			who: who.clone(),
			identity: identity.clone(),
			verification_request_block: bn,
		}
		.dispatch_bypass_filter(RuntimeOrigin::root())
		.map_err(|e| StfError::VerifyIdentityFailed(e.into()))?;

		// remove challenge code
		IMTCall::remove_challenge_code { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::VerifyIdentityFailed(e.into()))?;

		Ok(())
	}

	pub fn request_vc(
		enclave_account: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		assertion: Assertion,
		hash: H256,
	) -> StfResult<()> {
		ensure_enclave_signer_account(&enclave_account)?;
		let key = IdentityManagement::user_shielding_keys(&who).ok_or_else(|| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UserShieldingKeyNotFound)
		})?;
		let id_graph = ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
		let vec_identity: Vec<Identity> = id_graph
			.into_iter()
			.filter(|item| item.1.is_verified)
			.map(|item| item.0)
			.collect();
		let request: RequestType = AssertionBuildRequest {
			shard: *shard,
			who,
			assertion: assertion.clone(),
			vec_identity,
			key,
			hash,
		}
		.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|e| {
			error!("[RequestVc] : {:?}", e);
			StfError::RequestVCFailed(assertion, ErrorDetail::SendStfRequestFailed)
		})
	}

	pub fn set_challenge_code_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
		code: ChallengeCode,
	) -> StfResult<()> {
		ensure_enclave_signer_account(&enclave_account)?;
		// only used in tests, we don't care about the error
		IMTCall::set_challenge_code { who, identity, code }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}

	// internal helper fn
	fn create_identity_runtime_internal(
		who: AccountId,
		identity: Identity,
		metadata: Option<MetadataOf<Runtime>>,
		bn: ParentchainBlockNumber,
		parent_ss58_prefix: u16,
	) -> StfResult<ChallengeCode> {
		IMTCall::create_identity {
			who: who.clone(),
			identity: identity.clone(),
			metadata,
			creation_request_block: bn,
			parent_ss58_prefix,
		}
		.dispatch_bypass_filter(RuntimeOrigin::root())
		.map_err(|e| StfError::CreateIdentityFailed(e.into()))?;

		// generate challenge code
		let code = generate_challenge_code();

		IMTCall::set_challenge_code { who, identity, code }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::CreateIdentityFailed(e.into()))?;

		Ok(code)
	}
}
