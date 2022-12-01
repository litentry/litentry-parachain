// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate::{
	helpers::{enclave_signer_account, generate_challenge_code},
	AccountId, Encode, IdentityManagement, MetadataOf, Runtime, ShardIdentifier, StfError,
	StfResult, TrustedCall, TrustedCallSigned,
};
use frame_support::dispatch::UnfilteredDispatchable;
use itp_utils::stringify::account_id_to_string;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, MaxIdentityLength, RequestType, SetUserShieldingKeyRequest,
	Web2IdentityVerificationRequest, Web3IdentityVerificationRequest,
};
use litentry_primitives::{
	Assertion, ChallengeCode, Identity, ParentchainBlockNumber, UserShieldingKeyType,
	ValidationData,
};
use log::*;
use sp_runtime::BoundedVec;
use std::{format, string::ToString, vec};

impl TrustedCallSigned {
	pub fn set_user_shielding_key_preflight(
		shard: &ShardIdentifier,
		who: AccountId,
		key: UserShieldingKeyType,
	) -> StfResult<()> {
		debug!("who.str = {:?}, key = {:?}", account_id_to_string(&who), key.clone());
		let encoded_callback =
			TrustedCall::set_user_shielding_key_runtime(enclave_signer_account(), who.clone(), key)
				.encode();
		let encoded_shard = shard.encode();
		let request = SetUserShieldingKeyRequest { encoded_shard, who, encoded_callback }.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|_| StfError::VerifyIdentityFailed)
	}

	pub fn set_user_shielding_key_runtime(
		who: AccountId,
		key: UserShieldingKeyType,
	) -> StfResult<()> {
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_user_shielding_key { who, key }
			.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}

	pub fn create_identity_runtime(
		who: AccountId,
		identity: Identity,
		metadata: Option<MetadataOf<Runtime>>,
		bn: ParentchainBlockNumber,
	) -> StfResult<ChallengeCode> {
		debug!(
			"who.str = {:?}, identity = {:?}, metadata = {:?}, bn = {:?}",
			account_id_to_string(&who),
			identity,
			metadata,
			bn
		);

		ita_sgx_runtime::IdentityManagementCall::<Runtime>::create_identity {
			who: who.clone(),
			identity: identity.clone(),
			metadata,
			linking_request_block: bn,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		// generate challenge code
		let code = generate_challenge_code();
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_challenge_code {
			who,
			identity,
			code,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		Ok(code)
	}

	pub fn remove_identity_runtime(who: AccountId, identity: Identity) -> StfResult<()> {
		debug!("who.str = {:?}, identity = {:?}", account_id_to_string(&who), identity,);
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::remove_identity { who, identity }
			.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}

	pub fn verify_identity_preflight(
		shard: &ShardIdentifier,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		let code = IdentityManagement::challenge_codes(&who, &identity)
			.ok_or_else(|| StfError::Dispatch("code not found".to_string()))?;

		debug!("who:{:?}, identity:{:?}, code:{:?}", who, identity, code);

		let encoded_callback = TrustedCall::verify_identity_runtime(
			enclave_signer_account(),
			who.clone(),
			identity.clone(),
			bn,
		)
		.encode();
		let encoded_shard = shard.encode();
		let request: RequestType = match validation_data {
			ValidationData::Web2(web2) => Web2IdentityVerificationRequest {
				encoded_shard,
				who,
				identity,
				challenge_code: code,
				validation_data: web2,
				bn,
				encoded_callback,
			}
			.into(),
			ValidationData::Web3(web3) => Web3IdentityVerificationRequest {
				encoded_shard,
				who,
				identity,
				challenge_code: code,
				validation_data: web3,
				bn,
				encoded_callback,
			}
			.into(),
		};

		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|_| StfError::VerifyIdentityFailed)
	}

	pub fn verify_identity_runtime(
		who: AccountId,
		identity: Identity,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		debug!(
			"who.str = {:?}, identity = {:?}, bn = {:?}",
			account_id_to_string(&who),
			identity,
			bn
		);
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::verify_identity {
			who: who.clone(),
			identity: identity.clone(),
			verification_request_block: bn,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		// remove challenge code
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::remove_challenge_code { who, identity }
			.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		Ok(())
	}

	pub fn build_assertion(
		shard: &ShardIdentifier,
		who: AccountId,
		assertion: Assertion,
	) -> StfResult<()> {
		let v_identity_context =
			ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_identity_and_identity_context(&who);

		let mut vec_identity: BoundedVec<Identity, MaxIdentityLength> = vec![].try_into().unwrap();

		for identity_ctx in &v_identity_context {
			if identity_ctx.1.is_verified {
				vec_identity
					.try_push(identity_ctx.0.clone())
					.map_err(|_| StfError::AssertionBuildFail)?;
			}
		}

		let encoded_shard = shard.encode();
		let request: RequestType =
			AssertionBuildRequest { encoded_shard, who, assertion, vec_identity }.into();

		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|_| StfError::AssertionBuildFail)
	}

	pub fn set_challenge_code_runtime(
		who: AccountId,
		identity: Identity,
		code: ChallengeCode,
	) -> StfResult<()> {
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_challenge_code {
			who,
			identity,
			code,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}
}
