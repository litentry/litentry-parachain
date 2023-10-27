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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use hex_sgx as hex;
	pub use thiserror_sgx as thiserror;
	pub use threadpool_sgx as threadpool;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

pub mod handler;

use codec::Encode;
use frame_support::sp_tracing::warn;
use futures::executor;
use handler::{
	assertion::AssertionHandler, identity_verification::IdentityVerificationHandler, TaskHandler,
};
use ita_sgx_runtime::Hash;
use ita_stf::{hash::Hash as TopHash, TrustedCall, TrustedOperation};
use itp_enclave_metrics::EnclaveMetric;
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{ShardIdentifier, H256};
use lc_stf_task_sender::{stf_task_sender, RequestType};
use log::{debug, error, info};
use std::{boxed::Box, format, string::String, sync::Arc};
use threadpool::ThreadPool;

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
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> {
	pub shielding_key: K,
	author_api: Arc<A>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
}

impl<
		K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
		A: AuthorApi<Hash, Hash>,
		S: StfEnclaveSigning,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
	> StfTaskContext<K, A, S, H, O>
where
	H::StateT: SgxExternalitiesTrait,
{
	pub fn new(
		shielding_key: K,
		author_api: Arc<A>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
	) -> Self {
		Self { shielding_key, author_api, enclave_signer, state_handler, ocall_api }
	}

	fn submit_trusted_call(
		&self,
		shard: &ShardIdentifier,
		old_top_hash: &H256,
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
		if self
			.author_api
			.get_pending_trusted_calls_for(
				*shard,
				&trusted_call.sender_identity().to_account_id().ok_or_else(|| {
					Error::OtherError(format!(
						"Not a valid account: {:?}",
						trusted_call.sender_identity()
					))
				})?,
			)
			.into_iter()
			.any(|t| t.hash() == top.hash())
		{
			// skip the submission if some top with the same hash already exists, return Ok(())
			warn!("Skip submit_trusted_call because top with the same hash exists");
			return Ok(())
		}

		// swap the hash in the rpc connection registry to make sure furthre RPC responses go to
		// the right channel
		self.author_api.swap_rpc_connection_hash(*old_top_hash, top.hash());

		let encrypted_trusted_call = self
			.shielding_key
			.encrypt(&top.encode())
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		debug!(
			"submit encrypted trusted call: {} bytes, original encoded top: {} bytes",
			encrypted_trusted_call.len(),
			top.encode().len()
		);
		executor::block_on(self.author_api.watch_top(encrypted_trusted_call, *shard)).map_err(
			|e| Error::OtherError(format!("error submitting trusted call to top pool: {:?}", e)),
		)?;

		Ok(())
	}
}

// lifetime elision: StfTaskContext is guaranteed to outlive the fn
pub fn run_stf_task_receiver<K, A, S, H, O>(
	context: Arc<StfTaskContext<K, A, S, H, O>>,
) -> Result<(), Error>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
{
	let receiver = stf_task_sender::init_stf_task_sender_storage()
		.map_err(|e| Error::OtherError(format!("read storage error:{:?}", e)))?;

	let (sender, to_receiver) = std::sync::mpsc::channel::<(ShardIdentifier, H256, TrustedCall)>();

	// Spawn thread to handle received tasks
	let context_for_thread = context.clone();
	std::thread::spawn(move || loop {
		if let Ok((shard, hash, call)) = to_receiver.recv() {
			info!("Submitting trusted call to the pool");
			if let Err(e) = context_for_thread.submit_trusted_call(&shard, &hash, &call) {
				error!("Submit Trusted Call failed: {:?}", e);
			}
		}
	});

	// The total number of threads that will be used to spawn tasks in the ThreadPool
	let n_workers = 4;
	let pool = ThreadPool::new(n_workers);

	loop {
		let req = receiver
			.recv()
			.map_err(|e| Error::OtherError(format!("receiver error:{:?}", e)))?;

		let context_pool = context.clone();
		let sender_pool = sender.clone();

		pool.execute(move || {
			let start_time = std::time::Instant::now();

			match &req {
				RequestType::IdentityVerification(req) =>
					IdentityVerificationHandler { req: req.clone(), context: context_pool.clone() }
						.start(sender_pool),
				RequestType::AssertionVerification(req) =>
					AssertionHandler { req: req.clone(), context: context_pool.clone() }
						.start(sender_pool),
			}

			if let Err(e) =
				context_pool.ocall_api.update_metric(EnclaveMetric::StfTaskExecutionTime(
					Box::new(req),
					start_time.elapsed().as_secs_f64(),
				)) {
				warn!("Failed to update metric for stf execution: {:?}", e);
			}
		});
	}
}
