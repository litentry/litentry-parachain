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

use crate::{handler::TaskHandler, StfTaskContext, TrustedCall};
use ita_sgx_runtime::Hash;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_utils::stringify::account_id_to_string;
use lc_data_providers::G_DATA_PROVIDERS;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{
	aes_encrypt_default, AesOutput, Assertion, ErrorDetail, ErrorString, VCMPError,
};
use log::*;
use sp_core::hashing::blake2_256;
use std::{format, sync::Arc};

pub(crate) struct AssertionHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<K, A, S, H>>,
}

impl<K, A, S, H> TaskHandler for AssertionHandler<K, A, S, H>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	type Error = VCMPError;
	type Result = ([u8; 32], [u8; 32], AesOutput); // (vc_index, vc_hash, encrypted_vc_str)

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		// create the initial credential
		let mut credential = match self.req.assertion.clone() {
			Assertion::A1 => lc_assertion_build::a1::build(
				self.req.vec_identity.clone(),
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A2(guild_id) => lc_assertion_build::a2::build(
				self.req.vec_identity.to_vec(),
				guild_id,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A3(guild_id, channel_id, role_id) => lc_assertion_build::a3::build(
				self.req.vec_identity.to_vec(),
				guild_id,
				channel_id,
				role_id,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A4(min_balance) => lc_assertion_build::a4::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A5(original_tweet_id) => lc_assertion_build::a5::build(
				self.req.vec_identity.to_vec(),
				original_tweet_id,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A6 => lc_assertion_build::a6::build(
				self.req.vec_identity.to_vec(),
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A7(min_balance) => lc_assertion_build::a7::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A8(networks) => lc_assertion_build::a8::build(
				self.req.vec_identity.to_vec(),
				networks,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A10(min_balance) => lc_assertion_build::a10::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			Assertion::A11(min_balance) => lc_assertion_build::a11::build(
				self.req.vec_identity.to_vec(),
				min_balance,
				&self.req.shard,
				&self.req.who,
				self.req.bn,
			),

			_ => {
				unimplemented!()
			},
		}?;

		// post-process the credential
		let signer = self.context.enclave_signer.as_ref();
		let enclave_account = signer.get_enclave_account().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let credential_endpoint = G_DATA_PROVIDERS.read().unwrap().credential_endpoint.clone();
		credential.credential_subject.set_endpoint(credential_endpoint);

		credential.issuer.id = account_id_to_string(&enclave_account);
		let payload = credential.to_json().map_err(|_| {
			VCMPError::RequestVCFailed(self.req.assertion.clone(), ErrorDetail::ParseError)
		})?;
		debug!("Credential payload: {}", payload);
		let (enclave_account, sig) = signer.sign_vc_with_self(payload.as_bytes()).map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;
		debug!("Credential Payload signature: {:?}", sig);

		credential.add_proof(&sig, credential.issuance_block_number, &enclave_account);
		credential.validate().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let vc_index = credential.get_index().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;
		let credential_str = credential.to_json().map_err(|_| {
			VCMPError::RequestVCFailed(self.req.assertion.clone(), ErrorDetail::ParseError)
		})?;
		debug!("Credential: {}, length: {}", credential_str, credential_str.len());
		let vc_hash = blake2_256(credential_str.as_bytes());
		debug!("VC hash: {:?}", vc_hash);

		let output = aes_encrypt_default(&self.req.key, credential_str.as_bytes());
		Ok((vc_index, vc_hash, output))
	}

	fn on_success(&self, result: Self::Result) {
		debug!("Assertion build OK");
		// we shouldn't have the maximum text length limit in normal RSA3072 encryption, as the payload
		// using enclave's shielding key is encrypted in chunks
		let (vc_index, vc_hash, vc_payload) = result;
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::handle_vc_issued(
				enclave_signer,
				self.req.who.clone(),
				self.req.assertion.clone(),
				vc_index,
				vc_hash,
				vc_payload,
				self.req.hash,
			);
			let _ = self
				.context
				.submit_trusted_call(&self.req.shard, &c)
				.map_err(|e| error!("submit_trusted_call failed: {:?}", e));
		} else {
			error!("can't get enclave signer");
		}
	}

	fn on_failure(&self, error: Self::Error) {
		error!("Assertion build error: {error:?}");
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::handle_vcmp_error(
				enclave_signer,
				Some(self.req.who.clone()),
				error,
				self.req.hash,
			);
			let _ = self
				.context
				.submit_trusted_call(&self.req.shard, &c)
				.map_err(|e| error!("submit_trusted_call failed: {:?}", e));
		} else {
			error!("can't get enclave signer");
		}
	}
}
