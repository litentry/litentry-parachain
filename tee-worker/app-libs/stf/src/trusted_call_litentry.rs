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
	helpers::{ensure_enclave_signer_account, is_authorized_signer},
	AccountId, IdentityManagement, Runtime, StfError, StfResult, UserShieldingKeys,
};
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use ita_sgx_runtime::{IdentityStatus, RuntimeOrigin};
use itp_stf_primitives::types::ShardIdentifier;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, IdentityVerificationRequest, RequestType,
};
use litentry_primitives::{
	Assertion, BoundedWeb3Network, ErrorDetail, Identity, IdentityNetworkTuple,
	UserShieldingKeyType, ValidationData, Web3Network,
};
use log::*;
use std::vec::Vec;

impl TrustedCallSigned {
	pub fn set_user_shielding_key_internal(
		signer: AccountId,
		who: Identity,
		key: UserShieldingKeyType,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorized_signer(&signer, who.to_account_id()),
			StfError::SetUserShieldingKeyFailed(ErrorDetail::UnauthorizedSigner)
		);
		IMTCall::set_user_shielding_key { who, key }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_or_else(|e| Err(StfError::SetUserShieldingKeyFailed(e.error.into())), |_| Ok(key))
	}

	#[allow(clippy::too_many_arguments)]
	pub fn link_identity_internal(
		signer: AccountId,
		who: Identity,
		identity: Identity,
		validation_data: ValidationData,
		web3networks: Vec<Web3Network>,
		nonce: UserShieldingKeyNonceType,
		hash: H256,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		ensure!(
			is_authorized_signer(&signer, who.to_account_id()),
			StfError::LinkIdentityFailed(ErrorDetail::UnauthorizedSigner)
		);

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::LinkIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		let bounded_web3networks = web3networks
			.try_into()
			.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::Web3NetworkOutOfBounds))?;

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
			bounded_web3networks,
			sidechain_nonce,
			key_nonce: nonce,
			key,
			hash,
		}
		.into();
		StfRequestSender::new()
			.send_stf_request(request)
			.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::SendStfRequestFailed))
	}

	pub fn remove_identity_internal(
		signer: AccountId,
		who: Identity,
		identity: Identity,
	) -> StfResult<UserShieldingKeyType> {
		ensure!(
			is_authorized_signer(&signer, who.to_account_id()),
			StfError::RemoveIdentityFailed(ErrorDetail::UnauthorizedSigner)
		);
		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RemoveIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		IMTCall::remove_identity { who, identity }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::RemoveIdentityFailed(e.into()))?;

		Ok(key)
	}

	pub fn request_vc_internal(
		signer: AccountId,
		who: Identity,
		assertion: Assertion,
		hash: H256,
		shard: &ShardIdentifier,
	) -> StfResult<()> {
		ensure!(
			is_authorized_signer(&signer, who.to_account_id()),
			StfError::RequestVCFailed(assertion, ErrorDetail::UnauthorizedSigner)
		);
		ensure!(
			UserShieldingKeys::<Runtime>::contains_key(&who),
			StfError::RequestVCFailed(assertion, ErrorDetail::UserShieldingKeyNotFound)
		);

		let id_graph = IMT::get_id_graph(&who, usize::MAX);
		let assertion_networks = assertion.get_supported_web3networks();
		let vec_identity: Vec<IdentityNetworkTuple> = id_graph
			.into_iter()
			.filter(|item| item.1.status == IdentityStatus::Active)
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
		who: Identity,
		identity: Identity,
		web3networks: BoundedWeb3Network,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer)
			.map_err(|_| StfError::LinkIdentityFailed(ErrorDetail::UnauthorizedSigner))?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::LinkIdentityFailed(ErrorDetail::UserShieldingKeyNotFound))?;

		IMTCall::link_identity { who, identity, web3networks }
			.dispatch_bypass_filter(RuntimeOrigin::root())
			.map_err(|e| StfError::LinkIdentityFailed(e.into()))?;

		Ok(key)
	}

	pub fn request_vc_callback_internal(
		signer: AccountId,
		who: Identity,
		assertion: Assertion,
	) -> StfResult<UserShieldingKeyType> {
		// important! The signer has to be enclave_signer_account, as this TrustedCall can only be constructed internally
		ensure_enclave_signer_account(&signer).map_err(|_| {
			StfError::RequestVCFailed(assertion.clone(), ErrorDetail::UnauthorizedSigner)
		})?;

		let key = IdentityManagement::user_shielding_keys(&who)
			.ok_or(StfError::RequestVCFailed(assertion, ErrorDetail::UserShieldingKeyNotFound))?;

		Ok(key)
	}
}
