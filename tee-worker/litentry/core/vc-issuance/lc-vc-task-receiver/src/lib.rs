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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

mod vc_handling;

use crate::vc_handling::VCRequestHandler;
use codec::{Decode, Encode};
pub use futures;
use ita_sgx_runtime::Hash;
use ita_stf::{
	aes_encrypt_default, helpers::enclave_signer_account, trusted_call_result::RequestVCResult,
	ConvertAccountId, OpaqueCall, SgxParentchainTypeConverter, VCMPCallIndexes, H256, IMT,
};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata, NodeMetadataTrait,
};
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::parentchain::ParentchainId;
use lazy_static::lazy_static;
use lc_stf_task_receiver::StfTaskContext;
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{init_vc_task_sender_storage, TrustedVCRequestSigned};
use litentry_primitives::{
	aes_decrypt, AesOutput, IdentityNetworkTuple, RequestAesKey, ShardIdentifier,
};
use std::{
	collections::HashMap,
	format,
	string::{String, ToString},
	sync::Arc,
	time::{Duration, Instant},
	vec::Vec,
};
use threadpool::ThreadPool;

#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

struct RateLimiter {
	requests: RwLock<HashMap<String, Instant>>,
}

impl RateLimiter {
	fn should_allow(&self, request_key: String) -> Result<(), String> {
		let requests = self.requests.read().unwrap();
		log::error!("Request key: {:?}", request_key);
		if let Some(&last_instant) = requests.get(&request_key) {
			if last_instant.elapsed() < Duration::from_secs(10) {
				return Err("Request limit reached".to_string()) // Reject if within 5 minutes
			}
		}
		drop(requests); // Drop read lock

		log::error!("Writing to the hashmap");
		let mut requests = self.requests.write().unwrap();
		requests.insert(request_key, Instant::now()); // Update with current Instant
		Ok(())
	}
}

// Global instance using lazy_static
lazy_static! {
	static ref GLOBAL_RATE_LIMITER: RateLimiter =
		RateLimiter { requests: RwLock::new(HashMap::new()) };
}

pub fn run_vc_handler_runner<K, A, S, H, O, Z, N>(
	context: Arc<StfTaskContext<K, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
) where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let receiver = init_vc_task_sender_storage();
	let n_workers = 4;
	let pool = ThreadPool::new(n_workers);

	while let Ok(req) = receiver.recv() {
		let context_pool = context.clone();
		let extrinsic_factory_pool = extrinsic_factory.clone();
		let node_metadata_repo_pool = node_metadata_repo.clone();
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(
				req.key,
				req.encrypted_trusted_call,
				req.shard,
				context_pool,
				extrinsic_factory_pool,
				node_metadata_repo_pool,
			)) {
				log::warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}
}

pub fn handle_request<K, A, S, H, O, Z, N>(
	key: Vec<u8>,
	mut encrypted_trusted_call: AesOutput,
	shard: ShardIdentifier,
	context: Arc<StfTaskContext<K, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
) -> Result<Vec<u8>, String>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let aes_key: RequestAesKey = context
		.shielding_key
		.decrypt(&key)
		.map_err(|e| format!("Failed to decrypted AES Key: {:?}", e))?
		.try_into()
		.map_err(|e| format!("Failed to convert to UserShieldingKeyType: {:?}", e))?;

	let decrypted_trusted_operation = match aes_decrypt(&aes_key, &mut encrypted_trusted_call) {
		Some(s) => s,
		None => return Err("Failed to decrypted trusted operation".to_string()),
	};

	let signed_request =
		TrustedVCRequestSigned::decode(&mut decrypted_trusted_operation.as_slice())
			.map_err(|e| format!("Failed to decode trusted operation, {:?}", e))?;

	let vc_request = signed_request.vc_request;

	GLOBAL_RATE_LIMITER.should_allow(vc_request.signer.to_did().unwrap())?;
	let (mut state, _) = context
		.state_handler
		.load_cloned(&shard)
		.map_err(|e| format!("Received error while trying to obtain sidechain state: {:?}", e))?;

	state.execute_with(|| {
		let id_graph = IMT::get_id_graph(&vc_request.who);
		let assertion_networks = vc_request.assertion.clone().get_supported_web3networks();
		let identities: Vec<IdentityNetworkTuple> = id_graph
			.into_iter()
			.filter(|item| item.1.is_active())
			.map(|item| {
				let mut networks = item.1.web3networks.to_vec();
				networks.retain(|n| assertion_networks.contains(n));
				(item.0, networks)
			})
			.collect();

		let signer = match vc_request.signer.to_account_id() {
			Some(s) => s,
			None => return Err("Invalid signer account, failed to convert".to_string()),
		};

		let assertion_build: AssertionBuildRequest = AssertionBuildRequest {
			shard,
			signer,
			enclave_account: enclave_signer_account(),
			who: vc_request.who.clone(),
			assertion: vc_request.assertion.clone(),
			identities,
			maybe_key: Some(aes_key),
			top_hash: H256::zero(),
			req_ext_hash: H256::zero(),
		};

		let vc_request_handler =
			VCRequestHandler { req: assertion_build, context: context.clone() };
		let response = vc_request_handler
			.process()
			.map_err(|e| format!("Failed to build assertion due to: {:?}", e))?;

		let call_index = node_metadata_repo
			.get_from_metadata(|m| m.vc_issued_call_indexes())
			.unwrap()
			.unwrap();
		let result = aes_encrypt_default(&aes_key, &response.vc_payload);
		let account = SgxParentchainTypeConverter::convert(
			match response.assertion_request.who.to_account_id() {
				Some(s) => s,
				None => return Err("Failed to convert account".to_string()),
			},
		);
		let call = OpaqueCall::from_tuple(&(
			call_index,
			account,
			response.assertion_request.assertion,
			response.vc_index,
			response.vc_hash,
			H256::zero(),
		));
		let res = RequestVCResult {
			vc_index: response.vc_index,
			vc_hash: response.vc_hash,
			vc_payload: result,
		};
		// This internally fetches nonce from a Mutex and then updates it thereby ensuring ordering
		let xt = extrinsic_factory
			.create_extrinsics(&[call], None)
			.map_err(|e| format!("Failed to construct extrinsic for parentchain: {:?}", e))?;
		context
			.ocall_api
			.send_to_parentchain(xt, &ParentchainId::Litentry)
			.map_err(|e| format!("Unable to send extrinsic to parentchain: {:?}", e))?;

		Ok(res.encode())
	})
}
