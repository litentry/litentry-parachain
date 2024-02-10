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
use litentry_primitives::AesRequest;
use log::*;
use std::{
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

use ita_stf::TrustedCallSigned;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct BitAcrossTaskContext<
	ShieldingKeyRepository,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub shielding_key: Arc<ShieldingKeyRepository>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
}

impl<
		ShieldingKeyRepository,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
	> BitAcrossTaskContext<ShieldingKeyRepository, S, H, O>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
{
	pub fn new(
		shielding_key: Arc<ShieldingKeyRepository>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
	) -> Self {
		Self { shielding_key, enclave_signer, state_handler, ocall_api }
	}
}

pub fn run_bit_across_handler_runner<ShieldingKeyRepository, S, H, O>(
	_context: Arc<BitAcrossTaskContext<ShieldingKeyRepository, S, H, O>>,
) where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
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
