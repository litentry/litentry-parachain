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

mod vc_callback;
mod vc_handling;

use crate::{vc_callback::VCCallbackHandler, vc_handling::VCRequestHandler};
use codec::Decode;
pub use futures;
use futures::channel::oneshot;
use ita_sgx_runtime::Hash;
use ita_stf::{
	helpers::enclave_signer_account, Runtime, TrustedCall, TrustedOperation, UserShieldingKeys,
	H256, IMT,
};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{provider::AccessNodeMetadata, NodeMetadataTrait};
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lc_stf_task_receiver::StfTaskContext;
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{init_vc_task_sender_storage, ErrorCode, RpcError, VCRequest, VCResponse};
use litentry_primitives::{IdentityNetworkTuple, VCMPError};
use std::{
	format,
	string::{String, ToString},
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc,
	},
	vec::Vec,
};

pub type VCMPResponseSender =
	Sender<(Result<VCResponse, VCMPError>, oneshot::Sender<Result<Vec<u8>, RpcError>>)>;
pub type VCMPResponseReceiver =
	Receiver<(Result<VCResponse, VCMPError>, oneshot::Sender<Result<Vec<u8>, RpcError>>)>;

pub fn run_vc_handler_runner<K, A, S, H, O, Z, N>(
	context: Arc<StfTaskContext<K, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: Arc<N>,
) where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	let receiver = init_vc_task_sender_storage();
	let (sender, response_receiver) =
		channel::<(Result<VCResponse, VCMPError>, oneshot::Sender<Result<Vec<u8>, RpcError>>)>();

	// TODO: Use a builder pattern here
	let vc_callback_handler =
		VCCallbackHandler::new(context.clone(), extrinsic_factory, node_metadata_repo);
	let vc_callback_handler = Arc::new(vc_callback_handler);
	start_response_handler(vc_callback_handler, response_receiver);

	loop {
		let req = receiver.recv().unwrap();
		handle_jsonrpc_request(context.clone(), req, sender.clone());
	}
}
// TODO: Create a function that sends the error to via the one-shot channel
pub fn start_response_handler<K, A, S, H, O, Z, N>(
	vc_callback_handler: Arc<VCCallbackHandler<K, A, S, H, O, Z, N>>,
	response_receiver: VCMPResponseReceiver,
) where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	std::thread::spawn(move || loop {
		let vc_handler = vc_callback_handler.clone();
		let (vc_response, sender) = response_receiver.recv().unwrap();
		if let Err(e) = vc_response.clone() {
			send_rpc_error(format!("Failed to generate credential: {:?}", e), sender);
		} else {
			vc_handler.request_vc_callback(vc_response.clone().unwrap(), sender);
		}
	});
}

pub fn handle_jsonrpc_request<K, A, S, H, O>(
	context: Arc<StfTaskContext<K, A, S, H, O>>,
	req: VCRequest,
	sender: VCMPResponseSender,
) where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
{
	let decrypted_trusted_operation =
		match context.shielding_key.decrypt(&req.encrypted_trusted_call) {
			Ok(s) => s,
			Err(e) => {
				send_rpc_error(format!("Failed to decrypt trusted operation: {:?}", e), req.sender);
				return
			},
		};

	let trusted_operation =
		match TrustedOperation::decode(&mut decrypted_trusted_operation.as_slice()) {
			Ok(s) => s,
			Err(e) => {
				send_rpc_error(format!("Failed to decode trusted operation, {:?}", e), req.sender);
				return
			},
		};

	let trusted_call = match trusted_operation.to_call() {
		Some(s) => s,
		None => {
			send_rpc_error(
				String::from("Failed to convert trusted operation to trusted call"),
				req.sender,
			);
			return
		},
	};

	if let TrustedCall::request_vc(signer, who, assertion, maybe_key, _hash) =
		trusted_call.call.clone()
	{
		let (mut state, _) = match context.state_handler.load_cloned(&req.shard) {
			Ok(s) => s,
			Err(e) => {
				send_rpc_error(
					format!("Received error while trying to obtain sidechain state: {:?}", e),
					req.sender,
				);
				return
			},
		};
		state.execute_with(|| {
			let key = UserShieldingKeys::<Runtime>::contains_key(&who);
			if !key {
				send_rpc_error(
					String::from("UserShieldingKey has not been set by User"),
					req.sender,
				);
				return
			}
			let id_graph = IMT::get_id_graph(&who, usize::MAX);
			let assertion_networks = assertion.clone().get_supported_web3networks();
			let identities: Vec<IdentityNetworkTuple> = id_graph
				.into_iter()
				.filter(|item| item.1.is_active())
				.map(|item| {
					let mut networks = item.1.web3networks.to_vec();
					networks.retain(|n| assertion_networks.contains(n));
					(item.0, networks)
				})
				.collect();

			let signer = match signer.to_account_id() {
				Some(s) => s,
				None => {
					send_rpc_error(
						"Invalid signer account, failed to convert".to_string(),
						req.sender,
					);
					return
				},
			};

			let assertion_build: AssertionBuildRequest = AssertionBuildRequest {
				shard: req.shard,
				signer,
				enclave_account: enclave_signer_account(),
				who: who.clone(),
				assertion: assertion.clone(),
				identities,
				maybe_key,
				top_hash: H256::zero(),
				req_ext_hash: H256::zero(),
			};

			let context_pool = context.clone();
			let sender_pool = sender.clone();
			let vc_request_handler =
				VCRequestHandler { req: assertion_build, context: context_pool };
			let result = vc_request_handler.process();
			if let Err(e) = sender_pool.send((result, req.sender)) {
				log::warn!("Failed to send processed result to sequencer: {:?}", e);
			}
		});
	} else {
		send_rpc_error(
			"Invalid Trusted Operation send to VC Request handler".to_string(),
			req.sender,
		);
	}
}

pub fn send_rpc_error(message: String, sender: oneshot::Sender<Result<Vec<u8>, RpcError>>) {
	let mut error = RpcError::new(ErrorCode::InternalError);
	error.message = message;
	if let Err(e) = sender.send(Err(error)) {
		log::warn!("Failed to send messasge to channel: {:?}", e);
	}
}
