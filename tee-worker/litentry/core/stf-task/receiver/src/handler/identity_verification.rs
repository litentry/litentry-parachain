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
use lc_stf_task_sender::RequestType;
use log::error;
use parachain_core_primitives::IMPError;
use std::{sync::Arc, vec::Vec};

pub(crate) struct IdentityVerificationHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: RequestType,
	pub(crate) context: Arc<StfTaskContext<K, O, C, M, A, S, H>>,
}

impl<K, O, C, M, A, S, H> TaskHandler for IdentityVerificationHandler<K, O, C, M, A, S, H>
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
	type Error = IMPError;
	type Result = (Vec<u8>, Vec<u8>);

	fn on_process(&self) -> Result<(Vec<u8>, Vec<u8>), Self::Error> {
		match self.req.clone() {
			RequestType::Web2IdentityVerification(ref req) =>
				lc_identity_verification::web2::verify(req)
					.map(|_| (req.encoded_shard.clone(), req.encoded_callback.clone())),

			RequestType::Web3IdentityVerification(ref req) =>
				lc_identity_verification::web3::verify(
					req.who.clone(),
					req.identity.clone(),
					req.challenge_code,
					req.validation_data.clone(),
				)
				.map(|_| (req.encoded_shard.clone(), req.encoded_callback.clone())),
			_ => {
				unimplemented!()
			},
		}
	}

	fn on_success(&self, result: Self::Result) {
		let (shard, callback) = result;
		match self.context.decode_and_submit_trusted_call(shard, callback) {
			Ok(_) => {},
			Err(e) => {
				error!("decode_and_submit_trusted_call failed. Due to: {:?}", e);
			},
		}
	}

	fn on_failure(&self, error: Self::Error) {
		match self
			.context
			.node_metadata
			.get_from_metadata(|m| IMPCallIndexes::some_error_call_indexes(m))
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
		}
	}
}
