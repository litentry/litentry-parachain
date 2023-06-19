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
	helpers::{ensure_enclave_signer_account, is_authorised_signer},
	AccountId, IdentityManagement, Runtime, StfError, StfResult, UserShieldingKeys,
};
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use ita_sgx_runtime::{IdentityStatus, RuntimeOrigin};
use itp_stf_primitives::types::ShardIdentifier;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, IdentityVerificationRequest, RequestType,
};
use litentry_primitives::{Assertion, ErrorDetail, Identity, UserShieldingKeyType, ValidationData};
use log::*;
use std::vec::Vec;

impl TrustedCallSigned {
	pub fn set_user_shielding_key_internal(
		signer: AccountId,
		who: AccountId,
		key: UserShieldingKeyType,
		parent_ss58_prefix: u16,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::SetUserShieldingKeyFailed(ErrorDetail::UnauthorisedSigner)
		);
		IMTCall::set_user_shielding_key { who, key, parent_ss58_prefix }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_or_else(|e| Err(StfError::SetUserShieldingKeyFailed(e.error.into())), |_| Ok(key))
	}

	#[allow(clippy::too_many_arguments)]
	pub fn link_identity_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		nonce: UserShieldingKeyNonceType,
		hash: H256,
		shard: &ShardIdentifier,
		parent_ss58_prefix: u16,
	) -> StfResult<()> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::LinkIdentityFailed(ErrorDetail::UnauthorisedSigner)
		);

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::LinkIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		// note it's the signer's nonce, not `who`
		// we intentionally use `System::account_nonce - 1` to make up for the increment at the
		// beginning of STF execution, otherwise it might be unexpected that we were hoping
		// (current nonce + 1) when verifying the validation data.
		let sidechain_nonce = System::account_nonce(&signer) - 1;

		let request: RequestType = IdentityVerificationRequest {
			shard: *shard,
			who,
			identity,
			validation_data,
			sidechain_nonce,
			key_nonce: nonce,
			key,
			parent_ss58_prefix,
			hash,
		}
		.into();
		StfRequestSender::new()
			.send_stf_request(request)
			.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::SendStfRequestFailed))
	}

	pub fn remove_identity_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		parent_ss58_prefix: u16,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::RemoveIdentityFailed(ErrorDetail::UnauthorisedSigner)
		);
		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RemoveIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		IMTCall::remove_identity { who, identity, parent_ss58_prefix }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::RemoveIdentityFailed(e.into()))?;

		Ok(key)
	}

	pub fn request_vc_internal(
		signer: AccountId,
		who: AccountId,
		assertion: Assertion,
		hash: H256,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		ensure!(
			is_authorised_signer(&signer, &who),
			StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorisedSigner)
		);
		ensure!(
			UserShieldingKeys::<Runtime>::contains_key(&who),
			StfError::RequestVCFailed(assertion, ErrorDetail::UserShieldingKeyNotFound)
		);

		let id_graph = IMT::get_id_graph(&who);
		let vec_identity: Vec<Identity> = id_graph
			.into_iter()
			.filter(|item| item.1.status == IdentityStatus::Active)
			.map(|item| item.0)
			.collect();
		let request: RequestType = AssertionBuildRequest {
			shard: *shard,
			who,
			assertion: assertion.clone(),
			vec_identity,
			hash,
		}
		.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|e| {
			error!("[RequestVc] : {:?}", e);
			StfError::RequestVCFailed(assertion, ErrorDetail::SendStfRequestFailed)
		})
	}

	pub fn link_identity_callback_internal(
		signer: AccountId,
		who: AccountId,
		identity: Identity,
		parent_ss58_prefix: u16,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer)
			.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::UnauthorisedSigner))?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::LinkIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		IMTCall::link_identity { who, identity, parent_ss58_prefix }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::LinkIdentityFailed(e.into()))?;

		Ok(key)
	}

	pub fn request_vc_callback_internal(
		signer: AccountId,
		who: AccountId,
		assertion: Assertion,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer).map_err(|_| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UnauthorisedSigner)
		})?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RequestVCFailed(assertion, ErrorDetail::UserShieldingKeyNotFound))?;

		Ok(key)
	}
}
