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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use hex_sgx as hex;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

mod handler;

use codec::Encode;
use frame_support::sp_tracing::warn;
use futures::executor;
use handler::{
	assertion::AssertionHandler, identity_verification::IdentityVerificationHandler, TaskHandler,
};
use ita_sgx_runtime::{Hash, IdentityManagement};
use ita_stf::{hash::Hash as TopHash, TrustedCall, TrustedOperation};
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
use itp_types::{OpaqueCall, ShardIdentifier};
use lc_stf_task_sender::{stf_task_sender, RequestType};
use log::{debug, error};
use std::{format, string::String, sync::Arc, vec, vec::Vec};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Assertion error: {0}")]
	AssertionError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

#[allow(dead_code)]
pub struct StfTaskContext<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	shielding_key: K,
	ocall_api: Arc<O>,
	create_extrinsics: Arc<C>,
	node_metadata: Arc<M>,
	author_api: Arc<A>,
	enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
}

impl<
		K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
		O: EnclaveOnChainOCallApi,
		C: CreateExtrinsics,
		M: AccessNodeMetadata,
		A: AuthorApi<Hash, Hash>,
		S: StfEnclaveSigning,
		H: HandleState,
	> StfTaskContext<K, O, C, M, A, S, H>
where
	H::StateT: SgxExternalitiesTrait,
	M::MetadataType: IMPCallIndexes + VCMPCallIndexes,
{
	pub fn new(
		shielding_key: K,
		ocall_api: Arc<O>,
		create_extrinsics: Arc<C>,
		node_metadata: Arc<M>,
		author_api: Arc<A>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
	) -> Self {
		Self {
			shielding_key,
			ocall_api,
			create_extrinsics,
			node_metadata,
			author_api,
			enclave_signer,
			state_handler,
		}
	}

	fn submit_trusted_call(
		&self,
		shard: &ShardIdentifier,
		trusted_call: &TrustedCall,
	) -> Result<(), Error> {
		let signed_trusted_call = self
			.enclave_signer
			.sign_call_with_self(trusted_call, shard)
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		let top = TrustedOperation::direct_call(signed_trusted_call);

		// find out if we have any trusted operation which has the same hash in the pool already.
		// The hash can be used to de-duplicate a trusted operation for a certain request, as the
		// `trusted_call` in this fn always contains the req_ext_hash, which is unique for each request.
		let filtered_top: Vec<TrustedOperation> = self
			.author_api
			.get_pending_trusted_calls_for(*shard, trusted_call.sender_account())
			.into_iter()
			.filter(|t| t.hash() == top.hash())
			.collect();

		// skip the submission if filtered_top is non empty, return Ok(())
		if !filtered_top.is_empty() {
			warn!("Skip submit_trusted_call because top with the same hash exists");
			return Ok(())
		}

		let encrypted_trusted_call = self
			.shielding_key
			.encrypt(&top.encode())
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		executor::block_on(self.author_api.submit_top(encrypted_trusted_call, *shard)).map_err(
			|e| Error::OtherError(format!("error submitting trusted call to top pool: {:?}", e)),
		)?;

		Ok(())
	}

	fn submit_to_parentchain(&self, call: OpaqueCall) {
		match self.create_extrinsics.create_extrinsics(vec![call].as_slice(), None) {
			Err(e) => {
				error!("create extrinsic failed: {:?}", e);
			},
			Ok(xt) => {
				let _ = self.ocall_api.send_to_parentchain(xt);
			},
		}
	}
}

// lifetime elision: StfTaskContext is guaranteed to outlive the fn
pub fn run_stf_task_receiver<K, O, C, M, A, S, H>(
	context: Arc<StfTaskContext<K, O, C, M, A, S, H>>,
) -> Result<(), Error>
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
	let receiver = stf_task_sender::init_stf_task_sender_storage()
		.map_err(|e| Error::OtherError(format!("read storage error:{:?}", e)))?;

	loop {
		let req = receiver
			.recv()
			.map_err(|e| Error::OtherError(format!("receiver error:{:?}", e)))?;

		match &req {
			RequestType::IdentityVerification(req) =>
				IdentityVerificationHandler { req: req.clone(), context: context.clone() }.start(),
			RequestType::AssertionVerification(req) =>
				AssertionHandler { req: req.clone(), context: context.clone() }.start(),
			// only for demo purpose
			// it shows how to read the storage in the stf-task handling with the loaded state. However,
			// in real cases it's preferred to read the state ahead and sent it as parameter in `Request`
			// please note you are not supposed to write any state back - it will cause state mistmatch
			RequestType::SetUserShieldingKey(req) => {
				let (mut state, _) = context
					.state_handler
					.load_cloned(&req.shard)
					.map_err(|e| Error::OtherError(format!("load state failed: {:?}", e)))?;

				let current_key =
					state.execute_with(|| IdentityManagement::user_shielding_keys(&req.who));

				debug!("RequestType::SetUserShieldingKey, key: {:?}", current_key);

				let c = TrustedCall::set_user_shielding_key_runtime(
					context.enclave_signer.get_enclave_account().map_err(|e| {
						Error::OtherError(format!("error get enclave account {:?}", e))
					})?,
					req.who.clone(),
					req.key,
					req.hash,
				);
				context.submit_trusted_call(&req.shard, &c)?;
			},
		}
	}
}
