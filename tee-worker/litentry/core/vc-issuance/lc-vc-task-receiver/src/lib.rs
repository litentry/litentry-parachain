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

use crate::vc_handling::VCRequestHandler;
use codec::{Decode, Encode};
use frame_support::ensure;
pub use futures;
use ita_sgx_runtime::{pallet_imt::get_eligible_identities, Hash, Runtime};
use ita_stf::{
	aes_encrypt_default,
	helpers::{ensure_alice, ensure_enclave_signer_or_self},
	trusted_call_result::RequestVCResult,
	Getter, OpaqueCall, TrustedCallSigned, H256,
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
use itp_utils::if_production_or;
use lc_stf_task_receiver::StfTaskContext;
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{init_vc_task_sender_storage, VCRequest};
use litentry_primitives::{Assertion, Identity, ParentchainBlockNumber};
use log::*;
use pallet_identity_management_tee::{identity_context::sort_id_graph, IdentityContext};
use sp_core::blake2_256;
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

	while let Ok(request) = receiver.recv() {
		let context_pool = context.clone();
		let extrinsic_factory_pool = extrinsic_factory.clone();
		let node_metadata_repo_pool = node_metadata_repo.clone();
		pool.execute(move || {
			if let Err(e) = request.sender.send(handle_request(
				&request,
				context_pool,
				extrinsic_factory_pool,
				node_metadata_repo_pool,
			)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	info!("Terminate the vc handler loop");
}

pub fn handle_request<K, A, S, H, O, Z, N>(
	request: &VCRequest,
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
	let (id_graph, parachain_block_number, sidechain_block_number) = context
		.state_handler
		.execute_on_current(&request.shard, |state, _| {
			let prefix_key = storage_map_key(
				"IdentityManagement",
				"IDGraphs",
				&request.who,
				&StorageHasher::Blake2_128Concat,
			);

			// `None` means empty IDGraph, thus `unwrap_or_default`
			let mut id_graph: Vec<(Identity, IdentityContext<Runtime>)> = state
				.iter_prefix::<Identity, IdentityContext<Runtime>>(&prefix_key)
				.unwrap_or_default();

			// Sorts the IDGraph in place
			sort_id_graph::<Runtime>(&mut id_graph);

			// should never be `None`, but use `unwrap_or_default` to not panic
			let parachain_block_number = state
				.get(&storage_value_key("Parentchain", "Number"))
				.and_then(|v| ParentchainBlockNumber::decode(&mut v.as_slice()).ok())
				.unwrap_or_default();
			let sidechain_block_number = state
				.get(&storage_value_key("System", "Number"))
				.and_then(|v| SidechainBlockNumber::decode(&mut v.as_slice()).ok())
				.unwrap_or_default();

			(id_graph, parachain_block_number, sidechain_block_number)
		})
		.map_err(|e| format!("Failed to fetch sidechain data due to: {:?}", e))?;

	// an empty id graph is unexpected, it should have landed in the top pool handling
	ensure!(!id_graph.is_empty(), "Unexpected empty IDGraph".to_string());

	let id_graph_hash = H256::from(blake2_256(&id_graph.encode()));
	let assertion_networks = request.assertion.get_supported_web3networks();
	let identities = get_eligible_identities(id_graph, assertion_networks);
	ensure!(!identities.is_empty(), "No eligible identity".to_string());

	let signer = request
		.signer
		.to_account_id()
		.ok_or_else(|| "Invalid signer account, failed to convert".to_string())?;

	match request.assertion {
		// the signer will be checked inside A13, as we don't seem to have access to ocall_api here
		Assertion::A13(_) => (),
		_ => if_production_or!(
			ensure!(
				ensure_enclave_signer_or_self(&signer, request.who.to_account_id()),
				"Unauthorized signer",
			),
			ensure!(
				ensure_enclave_signer_or_self(&signer, request.who.to_account_id())
					|| ensure_alice(&signer),
				"Unauthorized signer",
			)
		),
	}

	let req = AssertionBuildRequest {
		shard: request.shard,
		signer,
		who: request.who.clone(),
		assertion: request.assertion.clone(),
		identities,
		top_hash: H256::zero(),
		parachain_block_number,
		sidechain_block_number,
		maybe_key: request.maybe_key,
		should_create_id_graph: false, // non-empty id graph
		req_ext_hash: request.req_ext_hash,
	};

	let vc_request_handler = VCRequestHandler { req: req.clone(), context: context.clone() };
	let res = vc_request_handler
		.process()
		.map_err(|e| format!("Failed to build assertion due to: {:?}", e))?;

	let call_index = node_metadata_repo
		.get_from_metadata(|m| m.vc_issued_call_indexes())
		.map_err(|_| "Failed to get vc_issued_call_indexes".to_string())?
		.map_err(|_| "Failed to get metadata".to_string())?;

	let key = request.maybe_key.ok_or_else(|| "Invalid aes key".to_string())?;
	let result = aes_encrypt_default(&key, &res.vc_payload);
	let call = OpaqueCall::from_tuple(&(
		call_index,
		req.who,
		req.assertion,
		res.vc_index,
		res.vc_hash,
		req.req_ext_hash,
	));

	// for non-empty id graph, no mutation has happended, and id_graph_hash is guaranteed to be `Some(..)`
	let empty_mutated_id_graph = Vec::<(Identity, IdentityContext<Runtime>)>::new();
	let res = RequestVCResult {
		vc_index: res.vc_index,
		vc_hash: res.vc_hash,
		vc_payload: result,
		mutated_id_graph: aes_encrypt_default(&key, &empty_mutated_id_graph.encode()),
		id_graph_hash,
	};
	// this internally fetches nonce from a mutex and then updates it thereby ensuring ordering
	let xt = extrinsic_factory
		.create_extrinsics(&[call], None)
		.map_err(|e| format!("Failed to construct extrinsic for parentchain: {:?}", e))?;
	context
		.ocall_api
		.send_to_parentchain(xt, &ParentchainId::Litentry, false)
		.map_err(|e| format!("Unable to send extrinsic to parentchain: {:?}", e))?;

	Ok(res.encode())
}
