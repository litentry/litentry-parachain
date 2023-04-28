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
use lc_stf_task_sender::IdentityVerificationRequest;
use litentry_primitives::IMPError;
use log::*;
use std::sync::Arc;

pub(crate) struct IdentityVerificationHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: IdentityVerificationRequest,
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
	type Result = ();

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		lc_identity_verification::verify(&self.req)
	}

	fn on_success(&self, _result: Self::Result) {
		let _ = self
			.context
			.decode_and_submit_trusted_call(
				self.req.encoded_shard.clone(),
				self.req.encoded_callback.clone(),
			)
			.map_err(|e| error!("decode_and_submit_trusted_call failed: {:?}", e));
	}

	fn on_failure(&self, error: Self::Error) {
		error!("verify identity failed:{:?}", error);
		match self
			.context
			.node_metadata
			.get_from_metadata(|m| IMPCallIndexes::imp_some_error_call_indexes(m))
		{
			Ok(Ok(call_index)) => {
				debug!("Sending imp_some_error event to parachain ...");
				let call = OpaqueCall::from_tuple(&(
					call_index,
					Some(self.req.who.clone()),
					error,
					self.req.hash,
				));

				self.context.submit_to_parentchain(call)
			},
			Ok(Err(e)) => {
				error!("get metadata failed: {:?}", e);
			},
			Err(e) => {
				error!("get metadata failed: {:?}", e);
			},
		}
	}
}
