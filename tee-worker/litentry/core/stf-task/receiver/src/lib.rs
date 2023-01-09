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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use chrono_sgx as chrono;
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

use codec::{Decode, Encode};
use futures::executor;
use handler::{
	assertion::AssertionHandler, identity_verification::IdentityVerificationHandler, TaskHandler,
};
use ita_sgx_runtime::{Hash, IdentityManagement};
use ita_stf::{TrustedCall, TrustedOperation};
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

	pub fn decode_and_submit_trusted_call(
		&self,
		encoded_shard: Vec<u8>,
		encoded_callback: Vec<u8>,
	) -> Result<(), Error> {
		let shard = ShardIdentifier::decode(&mut encoded_shard.as_slice())
			.map_err(|e| Error::OtherError(format!("error decoding ShardIdentifier {:?}", e)))?;
		let callback = TrustedCall::decode(&mut encoded_callback.as_slice())
			.map_err(|e| Error::OtherError(format!("error decoding TrustedCall {:?}", e)))?;
		self.submit_trusted_call(&shard, &callback)
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

		let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = self
			.shielding_key
			.encrypt(&trusted_operation.encode())
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		let top_submit_future =
			async { self.author_api.submit_top(encrypted_trusted_call, *shard).await };
		executor::block_on(top_submit_future).map_err(|e| {
			Error::OtherError(format!("Error adding indirect trusted call to TOP pool: {:?}", e))
		})?;

		Ok(())
	}

	fn submit_to_parentchain(&self, call: OpaqueCall) {
		match self.create_extrinsics.create_extrinsics(vec![call].as_slice(), None) {
			Err(e) => {
				error!("failed to create extrinsics. Due to: {:?}", e);
			},
			Ok(xt) => {
				let _ = self.ocall_api.send_to_parentchain(xt);
			},
		}
	}

	// TODO: maybe add a wrapper to read the state and eliminate the public access to `state_handler`
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

	// TODO: When an error occurs, send the extrinsic (error message) to the parachain
	// TODO: error handling still incomplete, we only print logs but no error handling
	// TODO: we can further simplify the handling logic
	loop {
		let request_type = receiver
			.recv()
			.map_err(|e| Error::OtherError(format!("receiver error:{:?}", e)))?;

		match request_type.clone() {
			RequestType::Web2IdentityVerification(_) | RequestType::Web3IdentityVerification(_) => {
				IdentityVerificationHandler { req: request_type.clone(), context: context.clone() }
					.start();
			},
			RequestType::AssertionVerification(request) => {
				AssertionHandler { req: request.clone(), context: context.clone() }.start();
			},
			// only used for testing
			// demonstrate how to read the storage in the stf-task handling with the loaded state
			// in real cases we prefer to read the state ahead and sent the related storage as parameters in `Request`
			RequestType::SetUserShieldingKey(request) => {
				let shard = ShardIdentifier::decode(&mut request.encoded_shard.as_slice())
					.map_err(|e| {
						Error::OtherError(format!("error decoding ShardIdentifier {:?}", e))
					})?;

				let (mut state, _) = context
					.state_handler
					.load_cloned(&shard)
					.map_err(|e| Error::OtherError(format!("load state failed: {:?}", e)))?;

				let key =
					state.execute_with(|| IdentityManagement::user_shielding_keys(&request.who));

				debug!("in RequestType::SetUserShieldingKey read key is: {:?}", key);

				context.decode_and_submit_trusted_call(
					request.encoded_shard,
					request.encoded_callback,
				)?;
			},
		}
	}
}
