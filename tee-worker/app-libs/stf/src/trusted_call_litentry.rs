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
	helpers::{ensure_enclave_signer_account, generate_challenge_code, is_authorised_signer},
	AccountId, IdentityManagement, MetadataOf, Runtime, StfError, StfResult,
};
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use ita_sgx_runtime::RuntimeOrigin;
use itp_stf_primitives::types::ShardIdentifier;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, IdentityVerificationRequest, MaxIdentityLength, RequestType,
};
use litentry_primitives::{
	Assertion, ChallengeCode, ErrorDetail, ErrorString, Identity, ParentchainBlockNumber,
	UserShieldingKeyType, ValidationData,
};
use log::*;
use sp_runtime::BoundedVec;
use std::{format, sync::Arc, vec, vec::Vec};

impl TrustedCallSigned {
	pub fn set_user_shielding_key_internal(
		signer: AccountId,
		who: AccountId,
		key: UserShieldingKeyType,
		parent_ss58_prefix: u16,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::SetUserShieldingKeyFailed(ErrorDetail::UnauthorisedSender)
		);
		IMTCall::set_user_shielding_key { who, key, parent_ss58_prefix }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_or_else(|e| Err(StfError::SetUserShieldingKeyFailed(e.error.into())), |_| Ok(key))
	}

	pub fn create_identity_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		metadata: Option<MetadataOf<Runtime>>,
		bn: ParentchainBlockNumber,
		parent_ss58_prefix: u16,
	) -> StfResult<(UserShieldingKeyType, ChallengeCode)> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::CreateIdentityFailed(ErrorDetail::UnauthorisedSender)
		);

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::CreateIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;
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

		Ok((key, code))
	}

	pub fn remove_identity_runtime<NodeMetadataRepository>(
		node_metadata_repo: Arc<NodeMetadataRepository>,
		calls: &mut Vec<OpaqueCall>,
		who: AccountId,
		identity: Identity,
		hash: H256,
	) -> StfResult<()>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType:
			TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
	{
		let account = SgxParentchainTypeConverter::convert(who.clone());
		if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
			match (IMTCall::remove_identity { who, identity: identity.clone() }
				.dispatch_bypass_filter(RuntimeOrigin::root())
				.map_err(|e| StfError::RemoveIdentityFailed(e.into())))
			{
				Ok(_) => {
					debug!("pushing identity_removed event ...");
					calls.push(OpaqueCall::from_tuple(&(
						node_metadata_repo
							.get_from_metadata(|m| m.identity_removed_call_indexes())??,
						account,
						aes_encrypt_default(&key, &identity.encode()),
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
				IMPError::RemoveIdentityFailed(ErrorDetail::UserShieldingKeyNotFound),
				hash,
			);
		}
		Ok(())
	}

	#[allow(clippy::too_many_arguments)]
	pub fn verify_identity_preflight<NodeMetadataRepository>(
		node_metadata_repo: Arc<NodeMetadataRepository>,
		calls: &mut Vec<OpaqueCall>,
		shard: &ShardIdentifier,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType:
			TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
	{
		if let Err(e) = Self::verify_identity_preflight_internal(
			shard,
			who.clone(),
			identity,
			validation_data,
			bn,
			hash,
		) {
			debug!("pushing error event ... error: {}", e);
			add_call_from_imp_error(
				calls,
				node_metadata_repo,
				Some(SgxParentchainTypeConverter::convert(who)),
				e.to_imp_error(),
				hash,
			);
		}
		Ok(())
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

	pub fn request_vc<NodeMetadataRepository>(
		node_metadata_repo: Arc<NodeMetadataRepository>,
		calls: &mut Vec<OpaqueCall>,
		shard: &ShardIdentifier,
		who: AccountId,
		assertion: Assertion,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()>
	where
		NodeMetadataRepository: AccessNodeMetadata,
		NodeMetadataRepository::MetadataType:
			TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
	{
		if let Err(e) = Self::request_vc_internal(shard, who.clone(), assertion, bn, hash) {
			add_call_from_vcmp_error(
				calls,
				node_metadata_repo,
				Some(SgxParentchainTypeConverter::convert(who)),
				e.to_vcmp_error(),
				hash,
			);
		}
		Ok(())
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

	fn verify_identity_preflight_internal(
		shard: &ShardIdentifier,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()> {
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

	fn request_vc_internal(
		shard: &ShardIdentifier,
		who: AccountId,
		assertion: Assertion,
		bn: ParentchainBlockNumber,
		hash: H256,
	) -> StfResult<()> {
		let id_graph = ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
		let mut vec_identity: BoundedVec<Identity, MaxIdentityLength> = vec![].try_into().unwrap();
		for id in &id_graph {
			if id.1.is_verified {
				vec_identity.try_push(id.0.clone()).map_err(|_| {
					let error_msg = "vec_identity exceeds max length".into();
					error!("[RequestVc] : {:?}", error_msg);
					StfError::RequestVCFailed(
						assertion.clone(),
						ErrorDetail::StfError(ErrorString::truncate_from(error_msg)),
					)
				})?;
			}
		}

		let key = IdentityManagement::user_shielding_keys(&who).ok_or_else(|| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UserShieldingKeyNotFound)
		})?;
		let request: RequestType = AssertionBuildRequest {
			shard: *shard,
			who,
			assertion: assertion.clone(),
			vec_identity,
			bn,
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
}
