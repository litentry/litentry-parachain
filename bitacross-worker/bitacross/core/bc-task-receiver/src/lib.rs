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
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use bc_task_sender::init_bit_across_task_sender_storage;
use futures::executor;
use litentry_primitives::AesRequest;
use log::*;
use std::{
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use threadpool::ThreadPool;

use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::ShardIdentifier;

use codec::Encode;
use ita_sgx_runtime::Hash;
use ita_stf::{Getter, TrustedCall, TrustedCallSigned, TrustedOperation, H256};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{provider::AccessNodeMetadata, NodeMetadataTrait};
use itp_types::RsaRequest;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Assertion error: {0}")]
	AssertionError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct StfTaskContext<
	ShieldingKeyRepository,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub shielding_key: Arc<ShieldingKeyRepository>,
	author_api: Arc<A>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
}

impl<
		ShieldingKeyRepository,
		A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
	> StfTaskContext<ShieldingKeyRepository, A, S, H, O>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
{
	pub fn new(
		shielding_key: Arc<ShieldingKeyRepository>,
		author_api: Arc<A>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
	) -> Self {
		Self { shielding_key, author_api, enclave_signer, state_handler, ocall_api }
	}

	pub fn submit_trusted_call(
		&self,
		shard: &ShardIdentifier,
		maybe_old_top_hash: Option<H256>,
		trusted_call: &TrustedCall,
	) -> Result<(), Error> {
		let signed_trusted_call = self
			.enclave_signer
			.sign_call_with_self(trusted_call, shard)
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		let top: TrustedOperation<TrustedCallSigned, Getter> =
			TrustedOperation::direct_call(signed_trusted_call);

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
		if let Some(old_hash) = maybe_old_top_hash {
			self.author_api.swap_rpc_connection_hash(old_hash, top.hash());
		}

		let shielding_key = self
			.shielding_key
			.retrieve_key()
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		let encrypted_trusted_call = shielding_key
			.encrypt(&top.encode())
			.map_err(|e| Error::OtherError(format!("{:?}", e)))?;

		debug!(
			"submit encrypted trusted call: {} bytes, original encoded top: {} bytes",
			encrypted_trusted_call.len(),
			top.encode().len()
		);
		executor::block_on(self.author_api.watch_and_broadcast_top(
			RsaRequest::new(*shard, encrypted_trusted_call),
			"author_submitAndWatchBroadcastedRsaRequest".to_string(),
		))
		.map_err(|e| {
			Error::OtherError(format!("error submitting trusted call to top pool: {:?}", e))
		})?;

		Ok(())
	}
}

pub fn run_bit_across_handler_runner<ShieldingKeyRepository, A, S, H, O>(
	_context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
) where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
{
	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let n_workers = 2;
	let pool = ThreadPool::new(n_workers);

	while let Ok(mut req) = bit_across_task_receiver.recv() {
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(&mut req.request)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

pub fn handle_request(_request: &mut AesRequest) -> Result<Vec<u8>, String>
where
{
	Err("Not Implemented".to_string())
}
