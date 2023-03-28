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
use itp_types::OpaqueCall;
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

			// TODO: A5 not supported yet
			Assertion::A5(..) => Err(VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from("Not supported".into())),
			)),

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
		// it might seem a bit verbose with many `map_err`, but it's due to the fact that
		// the original error can of different type
		// TODO: maybe we can tidy up the original errors - some are chaotic and confusing
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
		debug!("[BuildAssertion] VC payload: {}", payload);
		let (enclave_account, sig) = signer.sign_vc_with_self(payload.as_bytes()).map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;
		debug!("[BuildAssertion] Payload hash signature: {:?}", sig);

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
		debug!("[BuildAssertion] Credential: {}, length: {}", credential_str, credential_str.len());
		let vc_hash = blake2_256(credential_str.as_bytes());
		debug!("[BuildAssertion] VC hash: {:?}", vc_hash);

		let output = aes_encrypt_default(&self.req.key, credential_str.as_bytes());
		Ok((vc_index, vc_hash, output))
	}

	fn on_success(&self, result: Self::Result) {
		let (vc_index, vc_hash, output) = result;
		match self
			.context
			.node_metadata
			.get_from_metadata(|m| VCMPCallIndexes::vc_issued_call_indexes(m))
		{
			Ok(Ok(call_index)) => {
				let call = OpaqueCall::from_tuple(&(
					call_index,
					self.req.who.clone(),
					self.req.assertion.clone(),
					vc_index,
					vc_hash,
					output,
				));
				self.context.submit_to_parentchain(call)
			},
			Ok(Err(e)) => error!("[BuildAssertion] failed to get metadata: {:?}", e),
			Err(e) => error!("[BuildAssertion] failed to get metadata: {:?}", e),
		};
	}

	fn on_failure(&self, error: Self::Error) {
		error!("[BuildAssertion] on_failure: {error:?}");

		match self
			.context
			.node_metadata
			.get_from_metadata(|m| VCMPCallIndexes::vcmp_some_error_call_indexes(m))
		{
			Ok(Ok(call_index)) => {
				let call = OpaqueCall::from_tuple(&(call_index, error));
				self.context.submit_to_parentchain(call)
			},
			Ok(Err(e)) => error!("failed to get metadata. Due to: {:?}", e),
			Err(e) => error!("failed to get metadata. Due to: {:?}", e),
		};
	}
}
