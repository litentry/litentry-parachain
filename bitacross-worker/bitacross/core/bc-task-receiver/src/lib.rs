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
use codec::Decode;
use frame_support::ensure;
use litentry_primitives::{AesRequest, DirectCall};
use log::*;
use std::{
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec,
	vec::Vec,
};
use threadpool::ThreadPool;

use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;

use ita_stf::TrustedCallSigned;
use itp_sgx_crypto::ecdsa::{ecdsa_sign, Pair};
use litentry_primitives::{DecryptableRequest, DirectCallSigned};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Assertion error: {0}")]
	AssertionError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct BitAcrossTaskContext<
	ShieldingKeyRepository,
	EthereumKeyRepository,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> where
	ShieldingKeyRepository: AccessKey,
	EthereumKeyRepository: AccessKey<KeyType = Pair>,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub shielding_key: Arc<ShieldingKeyRepository>,
	pub ethereum_key_repository: Arc<EthereumKeyRepository>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
}

impl<
		ShieldingKeyRepository,
		EthereumKeyRepository,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
	> BitAcrossTaskContext<ShieldingKeyRepository, EthereumKeyRepository, S, H, O>
where
	ShieldingKeyRepository: AccessKey,
	EthereumKeyRepository: AccessKey<KeyType = Pair>,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
{
	pub fn new(
		shielding_key: Arc<ShieldingKeyRepository>,
		ethereum_key_repository: Arc<EthereumKeyRepository>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
	) -> Self {
		Self { shielding_key, ethereum_key_repository, enclave_signer, state_handler, ocall_api }
	}
}

pub fn run_bit_across_handler_runner<ShieldingKeyRepository, EthereumKeyRepository, S, H, O>(
	context: Arc<BitAcrossTaskContext<ShieldingKeyRepository, EthereumKeyRepository, S, H, O>>,
) where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	EthereumKeyRepository: AccessKey<KeyType = Pair> + Send + Sync + 'static,
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
		let context_pool = context.clone();
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(&mut req.request, context_pool)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

pub fn handle_request<ShieldingKeyRepository, EthereumKeyRepository, S, H, O>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<ShieldingKeyRepository, EthereumKeyRepository, S, H, O>>,
) -> Result<Vec<u8>, String>
where
	ShieldingKeyRepository: AccessKey,
	EthereumKeyRepository: AccessKey<KeyType = Pair>,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
{
	let enclave_shielding_key = context
		.shielding_key
		.retrieve_key()
		.map_err(|e| format!("Failed to retrieve shielding key: {:?}", e))?;
	let dc = request
		.decrypt(Box::new(enclave_shielding_key))
		.ok()
		.and_then(|v| DirectCallSigned::decode(&mut v.as_slice()).ok())
		.ok_or_else(|| "Failed to decode payload".to_string())?;

	let mrenclave = match context.ocall_api.get_mrenclave_of_self() {
		Ok(m) => m.m,
		Err(_) => return Err("Failed to get mrenclave".to_string()),
	};
	ensure!(dc.verify_signature(&mrenclave, &request.shard), "Failed to verify sig".to_string());
	match dc.call {
		DirectCall::SignBitcoin(_, _payload) => Ok(vec![]),
		DirectCall::SignEthereum(_, payload) => {
			let key = context.ethereum_key_repository.retrieve_key().unwrap();
			let signature = ecdsa_sign(key, &payload).unwrap();
			Ok(signature.to_vec())
		},
	}
}
