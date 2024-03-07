#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use hex_sgx as hex;
	pub use threadpool_sgx as threadpool;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use codec::{Decode, Encode};
use frame_support::{ensure, sp_runtime::traits::One};
use ita_sgx_runtime::{pallet_imt::get_eligible_identities, BlockNumber, Hash, Runtime};
#[cfg(not(feature = "production"))]
use ita_stf::helpers::ensure_alice;
use ita_stf::{
	aes_encrypt_default,
	helpers::ensure_self,
	trusted_call_result::{RequestVCResult, RequestVcResultOrError},
	Getter, OpaqueCall, TrustedCall, TrustedCallSigned, TrustedCallVerification, TrustedOperation,
	H256,
};
use itp_enclave_metrics::EnclaveMetric;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata, NodeMetadataTrait,
};
use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_storage::{storage_map_key, storage_value_key, StorageHasher};
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	parentchain::ParentchainId, AccountId, BlockNumber as SidechainBlockNumber, ShardIdentifier,
};
use lc_stf_task_receiver::{handler::assertion::create_credential_str, StfTaskContext};
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{init_vc_task_sender_storage, VCRequest};
use litentry_macros::if_production_or;
use litentry_primitives::{Assertion, DecryptableRequest, Identity, ParentchainBlockNumber};
use log::*;
use pallet_identity_management_tee::{identity_context::sort_id_graph, IdentityContext};
use sp_core::blake2_256;
use std::{
	boxed::Box,
	collections::HashSet,
	format,
	string::{String, ToString},
	sync::{
		mpsc::{channel, Sender},
		Arc,
	},
	thread,
	time::Instant,
	vec::Vec,
};
use threadpool::ThreadPool;

pub fn run_vc_handler_runner<ShieldingKeyRepository, A, S, H, O, Z, N>(
	context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
) where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let vc_task_receiver = init_vc_task_sender_storage();
	let n_workers = 4;
	let pool = ThreadPool::new(n_workers);

	let (tc_sender, tc_receiver) = channel::<(ShardIdentifier, TrustedCall)>();

	// Spawn thread to handle received tasks, to serialize the nonce increase even if multiple threads
	// are submitting trusted calls simultaneously
	let context_cloned = context.clone();
	thread::spawn(move || loop {
		if let Ok((shard, call)) = tc_receiver.recv() {
			info!("Submitting trusted call to the pool");
			if let Err(e) = context_cloned.submit_trusted_call(&shard, None, &call) {
				error!("Submit Trusted Call failed: {:?}", e);
			}
		}
	});

	while let Ok(mut req) = vc_task_receiver.recv() {
		let context_pool = context.clone();
		let extrinsic_factory_pool = extrinsic_factory.clone();
		let node_metadata_repo_pool = node_metadata_repo.clone();
		let tc_sender_pool = tc_sender.clone();

		pool.execute(move || {
			handle_vc_request(
				&mut req,
				context_pool,
				extrinsic_factory_pool,
				node_metadata_repo_pool,
				tc_sender_pool,
			);
		})
	}

	pool.join();
	warn!("vc_task_receiver loop terminated");
}

fn send_vc_response<ShieldingKeyRepository, A, S, H, O>(
	sender: &Sender<Result<Vec<u8>, String>>,
	context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
	response: Result<Vec<u8>, String>,
	idx: u8,
	len: u8,
) where
	ShieldingKeyRepository: AccessKey + core::marker::Send + core::marker::Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
{
	let vc_res: RequestVcResultOrError = match response.clone() {
		Ok(payload) => RequestVcResultOrError { payload, is_error: false, idx, len },
		Err(e) =>
			RequestVcResultOrError { payload: e.as_bytes().to_vec(), is_error: true, idx, len },
	};

	if let Err(e) = sender.send(Ok(vc_res.encode())) {
		warn!("Unable to submit response back to the handler: {:?}", e);
	}
	if response.is_err() {
		if let Err(e) = context.ocall_api.update_metric(EnclaveMetric::FailedVCIssuance) {
			warn!("Failed to update metric for VC Issuance: {:?}", e);
		}
	} else if let Err(e) = context.ocall_api.update_metric(EnclaveMetric::SuccessfullVCIssuance) {
		warn!("Failed to update metric for VC Issuance: {:?}", e);
	}
}
fn handle_vc_request<ShieldingKeyRepository, A, S, H, O, Z, N>(
	vc_req: &mut VCRequest,
	context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
	tc_sender: Sender<(ShardIdentifier, TrustedCall)>,
) where
	ShieldingKeyRepository: AccessKey + core::marker::Send + core::marker::Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let request = &mut vc_req.request;
	let sender = &vc_req.sender;
	let enclave_shielding_key = match context.shielding_key.retrieve_key() {
		Ok(value) => value,
		Err(e) => {
			send_vc_response(
				sender,
				context,
				Err(format!("Failed to retrieve shielding key: {:?}", e)),
				0u8,
				1u8,
			);
			return
		},
	};
	let tcs = match request
		.decrypt(Box::new(enclave_shielding_key))
		.ok()
		.and_then(|v| TrustedOperation::<TrustedCallSigned, Getter>::decode(&mut v.as_slice()).ok())
		.and_then(|top| top.to_call().cloned())
	{
		Some(tcs) => tcs,
		None => {
			send_vc_response(
				sender,
				context,
				Err("Failed to decode request payload".to_string()),
				0u8,
				1u8,
			);
			return
		},
	};
	let mrenclave = match context.ocall_api.get_mrenclave_of_self() {
		Ok(m) => m.m,
		Err(_) => {
			send_vc_response(sender, context, Err("Failed to get mrenclave".to_string()), 0u8, 1u8);
			return
		},
	};
	if !tcs.verify_signature(&mrenclave, &request.shard) {
		send_vc_response(sender, context, Err("Failed to verify sig".to_string()), 0u8, 1u8);
		return
	}
	if let TrustedCall::request_vc(..) = tcs.call {
		let response = process_single_request(
			request.shard,
			context.clone(),
			extrinsic_factory,
			node_metadata_repo,
			tc_sender,
			tcs.call.clone(),
		);
		send_vc_response(sender, context, response, 0u8, 1u8);
	} else if let TrustedCall::request_batch_vc(signer, who, assertions, maybe_key, req_ext_hash) =
		tcs.call
	{
		// Filter out duplicate assertions
		let mut seen: HashSet<H256> = HashSet::new();
		let mut unique_assertions = Vec::new();
		for assertion in assertions.into_iter() {
			let hash = H256::from(blake2_256(&assertion.encode()));
			if seen.insert(hash) {
				unique_assertions.push(Some(assertion));
			} else {
				unique_assertions.push(None);
			}
		}

		let pool = ThreadPool::new(4);
		let len = unique_assertions.len() as u8;
		for (idx, assertion) in unique_assertions.iter().enumerate() {
			let context_pool = context.clone();
			let sender_clone = sender.clone();

			if let Some(assertion) = assertion {
				let new_call = TrustedCall::request_vc(
					signer.clone(),
					who.clone(),
					assertion.clone(),
					maybe_key,
					req_ext_hash,
				);
				let extrinsic_factory_pool = extrinsic_factory.clone();
				let node_metadata_repo_pool = node_metadata_repo.clone();
				let tc_sender_pool = tc_sender.clone();
				let shard = request.shard;

				pool.execute(move || {
					let response = process_single_request(
						shard,
						context_pool.clone(),
						extrinsic_factory_pool,
						node_metadata_repo_pool,
						tc_sender_pool,
						new_call,
					);

					send_vc_response(&sender_clone, context_pool, response, idx as u8, len);
				})
			} else {
				send_vc_response(
					&sender_clone,
					context_pool,
					Err("Duplicate assertion request".to_string()),
					idx as u8,
					len,
				);
			}
		}

		pool.join();
		debug!("request_batch_vc execution finished. In total {:?} assertions", len);
	} else {
		send_vc_response(
			sender,
			context,
			Err("Expect request_vc trusted call".to_string()),
			0u8,
			1u8,
		);
	}
}

fn process_single_request<ShieldingKeyRepository, A, S, H, O, Z, N>(
	shard: H256,
	context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
	tc_sender: Sender<(ShardIdentifier, TrustedCall)>,
	call: TrustedCall,
) -> Result<Vec<u8>, String>
where
	ShieldingKeyRepository: AccessKey + core::marker::Send + core::marker::Sync,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let start_time = Instant::now();
	// The `call` should always be `TrustedCall:request_vc`
	if let TrustedCall::request_vc(signer, who, assertion, maybe_key, req_ext_hash) = call {
		let (mut id_graph, is_already_linked, parachain_block_number, sidechain_block_number) =
			context
				.state_handler
				.execute_on_current(&shard, |state, _| {
					let storage_key = storage_map_key(
						"IdentityManagement",
						"IDGraphs",
						&who,
						&StorageHasher::Blake2_128Concat,
					);

					// `None` means empty IDGraph, thus `unwrap_or_default`
					let mut id_graph: Vec<(Identity, IdentityContext<Runtime>)> = state
						.iter_prefix::<Identity, IdentityContext<Runtime>>(&storage_key)
						.unwrap_or_default();

					// Sorts the IDGraph in place
					sort_id_graph::<Runtime>(&mut id_graph);

					let storage_key = storage_map_key(
						"IdentityManagement",
						"LinkedIdentities",
						&who,
						&StorageHasher::Blake2_128Concat,
					);

					// should never be `None`, but use `unwrap_or_default` to not panic
					let parachain_block_number = state
						.get(&storage_value_key("Parentchain", "Number"))
						.and_then(|v| ParentchainBlockNumber::decode(&mut v.as_slice()).ok())
						.unwrap_or_default();
					let sidechain_block_number = state
						.get(&storage_value_key("System", "Number"))
						.and_then(|v| SidechainBlockNumber::decode(&mut v.as_slice()).ok())
						.unwrap_or_default();

					(
						id_graph,
						state.contains_key(&storage_key),
						parachain_block_number,
						sidechain_block_number,
					)
				})
				.map_err(|e| format!("Failed to fetch sidechain data due to: {:?}", e))?;

		let mut should_create_id_graph = false;
		if id_graph.is_empty() {
			info!("IDGraph is empty, will pre-create one");
			// To create IDGraph upon first vc request (see P-410), there're two options:
			//
			// 1. synchronous creation:
			// we delegate the vc handling to the STF version, which only returns when the IDGraph is actually created (= InSidechainBlock state).
			// The downside of this method is that the first vc_request processing time is limited to the sidechain block interval.
			//
			// 2. asynchronous creation (this implementation):
			// we check if an IDGraph **could** be created and then process the VC request right away, meanwhile, we submit a trusted call to
			// top pool. So the IDGraph will be created async: in the next sidechain block. In the `RequestVCResult` we return the pre-calculated
			// `mutated_id_graph` and `id_graph_hash`.
			//
			// Corner case: there's a small chance that some IDGraph mutation was injectd in between. For example, a client sends `request_vc` which
			// is closely followed by a `link_identity` request. In this case, the IDGrpah creation resulting from `vc_request` would fail, as
			// the IDGraph would have been created already by that time. But this is OK as long as it reaches the desired state eventually.
			//
			// However, `RequestVCResult` might carry with outdated `mutated_id_graph` and `id_graph_hash` if it lands later than `LinkIdentityResult`.
			// So we call the fields `pre_mutated_id_graph` and `pre_id_graph_hash` to show they are pre-calculated.
			// The client should take proper actions against it, e.g., only use the value when the local IDGraph is empty.
			//
			// Please note we can't mutate the state inside vc-task-receiver via `load_for_mutation` even
			// though it's lock guarded, because: a) it intereferes with the block import on another thread, which eventually
			// cause state mismatch before/after applying the state diff b) it's not guaranteed to be broadcasted to other workers.
			//
			ensure!(!is_already_linked, "Identity already exists in other IDGraph".to_string());
			// we are safe to use `default_web3networks` and `Active` as IDGraph would be non-empty otherwise
			id_graph.push((
				who.clone(),
				IdentityContext::new(BlockNumber::one(), who.default_web3networks()),
			));
			should_create_id_graph = true;
		}
		info!("should_create_id_graph: {}", should_create_id_graph);

		let id_graph_hash = H256::from(blake2_256(&id_graph.encode()));
		let assertion_networks = assertion.get_supported_web3networks();
		let identities = get_eligible_identities(id_graph.as_ref(), assertion_networks);
		ensure!(!identities.is_empty(), "No eligible identity".to_string());

		let signer_account = signer
			.to_account_id()
			.ok_or_else(|| "Invalid signer account, failed to convert".to_string())?;

		match assertion {
			// the signer will be checked inside A13, as we don't seem to have access to ocall_api here
			Assertion::A13(_) => (),
			_ => if_production_or!(
				ensure!(ensure_self(&signer, &who), "Unauthorized signer",),
				ensure!(
					ensure_self(&signer, &who) || ensure_alice(&signer_account),
					"Unauthorized signer",
				)
			),
		}

		let req = AssertionBuildRequest {
			shard,
			signer: signer_account,
			who: who.clone(),
			assertion: assertion.clone(),
			identities,
			top_hash: H256::zero(),
			parachain_block_number,
			sidechain_block_number,
			maybe_key,
			should_create_id_graph,
			req_ext_hash,
		};

		let credential_str = create_credential_str(&req, &context)
			.map_err(|e| format!("Failed to build assertion due to: {:?}", e))?;

		let call_index = node_metadata_repo
			.get_from_metadata(|m| m.vc_issued_call_indexes())
			.map_err(|_| "Failed to get vc_issued_call_indexes".to_string())?
			.map_err(|_| "Failed to get metadata".to_string())?;

		let key = maybe_key.ok_or_else(|| "Invalid aes key".to_string())?;
		let call = OpaqueCall::from_tuple(&(
			call_index,
			who.clone(),
			assertion.clone(),
			id_graph_hash,
			req_ext_hash,
		));

		let mutated_id_graph = if should_create_id_graph { id_graph } else { Default::default() };

		let res = RequestVCResult {
			vc_payload: aes_encrypt_default(&key, &credential_str),
			pre_mutated_id_graph: aes_encrypt_default(&key, &mutated_id_graph.encode()),
			pre_id_graph_hash: id_graph_hash,
		};

		// submit TrustedCall::maybe_create_id_graph to the reciever thread
		let enclave_signer: AccountId = context
			.enclave_signer
			.get_enclave_account()
			.map_err(|_| "Failed to get enclave signer".to_string())?;
		let c = TrustedCall::maybe_create_id_graph(enclave_signer.into(), who);
		tc_sender
			.send((shard, c))
			.map_err(|e| format!("Failed to send trusted call: {}", e))?;

		// this internally fetches nonce from a mutex and then updates it thereby ensuring ordering
		let xt = extrinsic_factory
			.create_extrinsics(&[call], None)
			.map_err(|e| format!("Failed to construct extrinsic for parentchain: {:?}", e))?;

		context
			.ocall_api
			.send_to_parentchain(xt, &ParentchainId::Litentry, false)
			.map_err(|e| format!("Unable to send extrinsic to parentchain: {:?}", e))?;

		if let Err(e) = context.ocall_api.update_metric(EnclaveMetric::VCBuildTime(
			format!("{:?}", assertion),
			start_time.elapsed(),
		)) {
			warn!("Failed to update metric for vc build time: {:?}", e);
		}

		Ok(res.encode())
	} else {
		Err("Expect request_vc trusted call".to_string())
	}
}
