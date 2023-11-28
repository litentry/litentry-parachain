// Copyright 2020-2023 Trust Computing GmbH.
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

use crate::{handler::TaskHandler, EnclaveOnChainOCallApi, StfTaskContext, TrustedCall};
use ita_sgx_runtime::Hash;
use ita_stf::H256;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::ShardIdentifier;
use lc_stf_task_sender::Web2IdentityVerificationRequest;
use litentry_primitives::IMPError;
use log::*;
use std::sync::Arc;

pub(crate) struct IdentityVerificationHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> {
	pub(crate) req: Web2IdentityVerificationRequest,
	pub(crate) context: Arc<StfTaskContext<K, A, S, H, O>>,
}

impl<K, A, S, H, O> TaskHandler for IdentityVerificationHandler<K, A, S, H, O>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi,
{
	type Error = IMPError;
	type Result = ();

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		lc_identity_verification::verify(&self.req, &self.context.data_provider_config)
	}

	fn on_success(
		&self,
		_result: Self::Result,
		sender: std::sync::mpsc::Sender<(ShardIdentifier, H256, TrustedCall)>,
	) {
		debug!("verify identity OK");
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::link_identity_callback(
				enclave_signer.into(),
				self.req.who.clone(),
				self.req.identity.clone(),
				self.req.web3networks.clone(),
				self.req.maybe_key,
				self.req.req_ext_hash,
			);
			if let Err(e) = sender.send((self.req.shard, self.req.top_hash, c)) {
				error!("Unable to send message to the trusted_call_receiver: {:?}", e);
			}
		} else {
			error!("can't get enclave signer");
		}
	}

	fn on_failure(
		&self,
		error: Self::Error,
		sender: std::sync::mpsc::Sender<(ShardIdentifier, H256, TrustedCall)>,
	) {
		error!("verify identity failed:{:?}", error);
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::handle_imp_error(
				enclave_signer.into(),
				Some(self.req.who.clone()),
				error,
				self.req.req_ext_hash,
			);
			if let Err(e) = sender.send((self.req.shard, self.req.top_hash, c)) {
				error!("Unable to send message to the trusted_call_receiver: {:?}", e);
			}
		} else {
			error!("can't get enclave signer");
		}
	}
}
