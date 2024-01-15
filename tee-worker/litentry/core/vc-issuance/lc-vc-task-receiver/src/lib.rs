#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use hex_sgx as hex;
	pub use threadpool_sgx as threadpool;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use crate::vc_handling::VCRequestHandler;
use codec::{Decode, Encode};
use frame_support::{ensure, sp_runtime::traits::One};
pub use futures;
use ita_sgx_runtime::{pallet_imt::get_eligible_identities, BlockNumber, Hash, Runtime};
use ita_stf::{
	aes_encrypt_default, trusted_call_result::RequestVCResult, Getter, OpaqueCall, TrustedCall,
	TrustedCallSigned, TrustedOperation, H256,
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
use itp_storage::{storage_map_key, storage_value_key, StorageHasher};
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{parentchain::ParentchainId, BlockNumber as SidechainBlockNumber};
use lc_stf_task_receiver::StfTaskContext;
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::init_vc_task_sender_storage;
use litentry_primitives::{
	aes_decrypt, AesOutput, Identity, ParentchainBlockNumber, RequestAesKey, ShardIdentifier,
};
use log::*;
use pallet_identity_management_tee::{identity_context::sort_id_graph, IdentityContext};
use std::{
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use threadpool::ThreadPool;

mod vc_handling;

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
				warn!("Unable to submit response back to the handler: {:?}", e);
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

	let decrypted_trusted_operation = aes_decrypt(&aes_key, &mut encrypted_trusted_call)
		.ok_or_else(|| "Failed to decrypt trusted operation".to_string())?;

	let trusted_operation = TrustedOperation::<TrustedCallSigned, Getter>::decode(
		&mut decrypted_trusted_operation.as_slice(),
	)
	.map_err(|e| format!("Failed to decode trusted operation, {:?}", e))?;

	let trusted_call: &TrustedCallSigned = trusted_operation
		.to_call()
		.ok_or_else(|| "Failed to convert trusted operation to trusted call".to_string())?;

	if let TrustedCall::request_vc(signer, who, assertion, maybe_key, req_ext_hash) =
		trusted_call.call.clone()
	{
		let key = maybe_key.ok_or_else(|| "User shielding key not provided".to_string())?;
		let (identities, parachain_block_number, sidechain_block_number) = context
			.state_handler
			.execute_on_current(&shard, |state, _| {
				let prefix_key = storage_map_key(
					"IdentityManagement",
					"IDGraphs",
					&who,
					&StorageHasher::Blake2_128Concat,
				);

				// `None` means empty IDGraph, thus `unwrap_or_default`
				let mut id_graph = state
					.iter_prefix::<Identity, IdentityContext<Runtime>>(&prefix_key)
					.unwrap_or_default();

				// Sorts the IDGraph in place
				sort_id_graph::<Runtime>(&mut id_graph);

				if id_graph.is_empty() {
					// we are safe to use `default_web3networks` and `Active` as IDGraph would be non-empty otherwise
					id_graph.push((
						who.clone(),
						IdentityContext::new(BlockNumber::one(), who.default_web3networks()),
					));
				}

				// should never be `None`, but use `unwrap_or_default` to not panic
				let parachain_block_number = state
					.get(&storage_value_key("Parentchain", "Number"))
					.and_then(|v| ParentchainBlockNumber::decode(&mut v.as_slice()).ok())
					.unwrap_or_default();
				let sidechain_block_number = state
					.get(&storage_value_key("System", "Number"))
					.and_then(|v| SidechainBlockNumber::decode(&mut v.as_slice()).ok())
					.unwrap_or_default();

				let assertion_networks = assertion.clone().get_supported_web3networks();
				(
					get_eligible_identities(id_graph, assertion_networks),
					parachain_block_number,
					sidechain_block_number,
				)
			})
			.map_err(|e| format!("Failed to fetch sidechain data due to: {:?}", e))?;

		ensure!(!identities.is_empty(), "No eligible identity".to_string());

		let signer = signer
			.to_account_id()
			.ok_or_else(|| "Invalid signer account, failed to convert".to_string())?;

		let req = AssertionBuildRequest {
			shard,
			signer,
			who,
			assertion,
			identities,
			top_hash: H256::zero(),
			parachain_block_number,
			sidechain_block_number,
			maybe_key,
			req_ext_hash,
		};

		let vc_request_handler = VCRequestHandler { req, context: context.clone() };
		let response = vc_request_handler
			.process()
			.map_err(|e| format!("Failed to build assertion due to: {:?}", e))?;

		let call_index = node_metadata_repo
			.get_from_metadata(|m| m.vc_issued_call_indexes())
			.unwrap()
			.unwrap();
		let result = aes_encrypt_default(&key, &response.vc_payload);
		let call = OpaqueCall::from_tuple(&(
			call_index,
			response.assertion_request.who,
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
			.send_to_parentchain(xt, &ParentchainId::Litentry, false)
			.map_err(|e| format!("Unable to send extrinsic to parentchain: {:?}", e))?;

		Ok(res.encode())
	} else {
		Err("Invalid Trusted Operation send to VC Request handler".to_string())
	}
}
