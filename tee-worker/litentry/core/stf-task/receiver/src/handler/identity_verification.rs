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
use lc_stf_task_sender::IdentityVerificationRequest;
use litentry_primitives::IMPError;
use log::*;
use std::sync::Arc;

pub(crate) struct IdentityVerificationHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: IdentityVerificationRequest,
	pub(crate) context: Arc<StfTaskContext<K, A, S, H>>,
}

impl<K, A, S, H> TaskHandler for IdentityVerificationHandler<K, A, S, H>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
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
		debug!("verify identity OK");
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::verify_identity_runtime(
				enclave_signer,
				self.req.who.clone(),
				self.req.identity.clone(),
				self.req.bn,
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
		error!("verify identity failed:{:?}", error);
		if let Ok(enclave_signer) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::handle_imp_error(
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
