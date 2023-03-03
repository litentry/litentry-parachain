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

use crate::{handler::TaskHandler, StfTaskContext};
use ita_sgx_runtime::Hash;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_imp::IMPCallIndexes, pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata,
};
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{AccountId, OpaqueCall, H256};
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{aes_encrypt_default, Assertion, UserShieldingKeyType, VCMPError};
use log::*;
use sp_core::hashing::blake2_256;
use std::sync::Arc;

pub(crate) struct AssertionHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<K, O, C, M, A, S, H>>,
}

impl<K, O, C, M, A, S, H> TaskHandler for AssertionHandler<K, O, C, M, A, S, H>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	M::MetadataType: IMPCallIndexes + VCMPCallIndexes,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	type Error = VCMPError;
	type Result = Option<(Credential, AccountId)>;

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		match self.req.assertion.clone() {
			Assertion::A1 => lc_assertion_build::a1::build(
				self.req.vec_identity.clone(),
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A2(guild_id) => lc_assertion_build::a2::build(
				self.req.vec_identity.to_vec(),
				guild_id,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A3(guild_id, channel_id, role_id) => lc_assertion_build::a3::build(
				self.req.vec_identity.to_vec(),
				guild_id,
				channel_id,
				role_id,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A4(min_balance) => lc_assertion_build::a4::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A5(twitter_account, original_tweet_id) => lc_assertion_build::a5::build(
				self.req.vec_identity.to_vec(),
				twitter_account,
				original_tweet_id,
			)
			.map(|_| None),

			Assertion::A6 => lc_assertion_build::a6::build(
				self.req.vec_identity.to_vec(),
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A7(min_balance) => lc_assertion_build::a7::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A8(networks) => lc_assertion_build::a8::build(
				self.req.vec_identity.to_vec(),
				networks,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A10(min_balance) => lc_assertion_build::a10::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			Assertion::A11(min_balance) => lc_assertion_build::a11::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			)
			.map(|credential| Some((credential, self.req.who.clone()))),

			_ => {
				unimplemented!()
			},
		}
	}

	fn on_success(&self, result: Self::Result) {
		let (mut credential, who) = result.unwrap();
		let signer = self.context.enclave_signer.as_ref();

		let payload = credential.to_json().unwrap();
		let payload_hash = blake2_256(&payload.as_bytes());
		debug!("	[Assertion] payload: {}", payload);
		debug!("	[Assertion] payload_hash: {:?}", payload_hash);

		if let Ok((enclave_account, sig)) = signer.sign_vc_with_self(&payload_hash) {
			debug!("	[Assertion] Payload hash signature: {:?}", sig);

			credential.issuer.id = account_id_to_string(&enclave_account);
			credential.add_proof(&sig, credential.issuance_block_number, &enclave_account, H256::from(payload_hash));

			if credential.validate().is_err() {
				error!("failed to validate credential");
				return
			}

			let key: UserShieldingKeyType = self.req.key;
			if let Ok(vc_index) = credential.get_index() {
				let credential_str = credential.to_json().unwrap();
				debug!("on_success {}, length {}", credential_str, credential_str.len());

				let vc_hash = blake2_256(credential_str.as_bytes());
				let output = aes_encrypt_default(&key, credential_str.as_bytes());

				match self
					.context
					.node_metadata
					.get_from_metadata(|m| VCMPCallIndexes::vc_issued_call_indexes(m))
				{
					Ok(Ok(call_index)) => {
						let call =
							OpaqueCall::from_tuple(&(call_index, who, vc_index, vc_hash, output));
						self.context.submit_to_parentchain(call)
					},
					Ok(Err(e)) => {
						error!("failed to get metadata. Due to: {:?}", e);
					},
					Err(e) => {
						error!("failed to get metadata. Due to: {:?}", e);
					},
				};
			} else {
				error!("failed to decode credential id");
			}
		} else {
			error!("failed to sign credential");
		}
	}

	fn on_failure(&self, error: Self::Error) {
		log::error!("occur an error while building assertion, due to:{:?}", error);
		match self
			.context
			.node_metadata
			.get_from_metadata(|m| VCMPCallIndexes::vcmp_some_error_call_indexes(m))
		{
			Ok(Ok(call_index)) => {
				let call = OpaqueCall::from_tuple(&(call_index, error));
				self.context.submit_to_parentchain(call)
			},
			Ok(Err(e)) => {
				error!("failed to get metadata. Due to: {:?}", e);
			},
			Err(e) => {
				error!("failed to get metadata. Due to: {:?}", e);
			},
		};
	}
}
