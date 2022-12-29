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

use codec::{Decode, Encode};
use futures::executor;
use ita_sgx_runtime::Hash;
use ita_stf::{TrustedCall, TrustedOperation};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{pallet_imp::IMPCallIndexes, provider::AccessNodeMetadata};
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{OpaqueCall, ShardIdentifier};
use litentry_primitives::IMPError;
use log::error;
use std::{format, string::String, sync::Arc, vec, vec::Vec};

mod assertion;
mod identity_verification;
pub mod stf_task_receiver;

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
	M::MetadataType: IMPCallIndexes,
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

	// TODO: maybe add a wrapper to read the state and eliminate the public access to `state_handler`
}

trait TaskHandler {
	type Error;
	type Result;
	fn start(&self) {
		match self.on_process() {
			Ok(r) => self.on_success(r),
			Err(e) => self.on_failure(e),
		}
	}
	fn on_process(&self) -> Result<Self::Result, Self::Error>;
	fn on_success(&self, r: Self::Result);
	fn on_failure(&self, e: Self::Error);
}

pub(crate) fn submit_error_extrinsics<O, C, M>(
	error: IMPError,
	ocall_api: Arc<O>,
	create_extrinsics: Arc<C>,
	node_metadata: Arc<M>,
) where
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	M::MetadataType: IMPCallIndexes,
{
	match node_metadata.get_from_metadata(|m| m.some_error_call_indexes()) {
		Ok(Ok(call_index)) => {
			let call = OpaqueCall::from_tuple(&(call_index, error).encode());
			match create_extrinsics.create_extrinsics(vec![call].as_slice(), None) {
				Err(e) => {
					error!("create_extrinsics failed. Due to: {:?}", e);
				},
				Ok(xt) => {
					let _ = ocall_api.send_to_parentchain(xt);
				},
			}
		},
		Ok(Err(e)) => {
			error!("get metadata failed. Due to: {:?}", e);
		},
		Err(e) => {
			error!("get metadata failed. Due to: {:?}", e);
		},
	}
}
