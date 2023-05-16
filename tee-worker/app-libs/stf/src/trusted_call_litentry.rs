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
	AssertionBuildRequest, IdentityVerificationRequest, RequestType,
};
use litentry_primitives::{
	Assertion, ChallengeCode, ErrorDetail, Identity, ParentchainBlockNumber, UserShieldingKeyType,
	ValidationData,
};
use log::*;
use std::{format, vec::Vec};

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

	pub fn remove_identity_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::RemoveIdentityFailed(ErrorDetail::UnauthorisedSender)
		);
		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RemoveIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		IMTCall::remove_identity { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::RemoveIdentityFailed(e.into()))?;

		Ok(key)
	}

	pub fn verify_identity_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
		hash: H256,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::VerifyIdentityFailed(ErrorDetail::UnauthorisedSender)
		);
		let code = IdentityManagement::challenge_codes(&who, &identity)
			.ok_or(StfError::VerifyIdentityFailed(ErrorDetail::ChallengeCodeNotFound))?;
		let _ = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::VerifyIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

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
		StfRequestSender::new()
			.send_stf_request(request)
			.map_err(|_| StfError::VerifyIdentityFailed(ErrorDetail::SendStfRequestFailed))
	}

	pub fn request_vc_internal(
		signer: AccountId,
		who: AccountId,
		assertion: Assertion,
		bn: ParentchainBlockNumber,
		hash: H256,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorisedSender)
		);

		let _ = IdentityManagement::user_shielding_keys(&who).ok_or_else(|| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UserShieldingKeyNotFound)
		})?;

		let id_graph = IMT::get_id_graph(&who);
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
			bn,
			hash,
		}
		.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|e| {
			error!("[RequestVc] : {:?}", e);
			StfError::RequestVCFailed(assertion, ErrorDetail::SendStfRequestFailed)
		})
	}

	pub fn verify_identity_callback_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		bn: ParentchainBlockNumber,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer)
			.map_err(|_| StfError::VerifyIdentityFailed(ErrorDetail::UnauthorisedSender))?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::VerifyIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

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

		Ok(key)
	}

	pub fn request_vc_callback_internal(
		signer: AccountId,
		who: AccountId,
		assertion: Assertion,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer).map_err(|_| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UnauthorisedSender)
		})?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RequestVCFailed(assertion, ErrorDetail::UserShieldingKeyNotFound))?;

		Ok(key)
	}

	pub fn set_challenge_code_internal(
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
}
