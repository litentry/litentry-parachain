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

mod vc_callback;
mod vc_handling;

use crate::{vc_callback::VCCallbackHandler, vc_handling::VCRequestHandler};
use codec::Decode;
#[cfg(feature = "std")]
use futures::channel::oneshot;
use ita_sgx_runtime::{ConvertAccountId, Hash};
use ita_stf::{
	aes_encrypt_default, helpers::enclave_signer_account, IdentityManagement, OpaqueCall, Runtime,
	SgxParentchainTypeConverter, TrustedCall, TrustedOperation, UserShieldingKeys, VCMPCallIndexes,
	H256, IMT,
};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_teerex::TeerexCallIndexes, provider::AccessNodeMetadata, NodeMetadataTrait,
};
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lc_stf_task_receiver::{handler::TaskHandler, StfTaskContext};
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{init_vc_task_sender_storage, RpcError, VCResponse};
use litentry_primitives::{Assertion, Identity, IdentityNetworkTuple, VCMPError};
use std::{
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc,
	},
	vec::Vec,
};

#[cfg(feature = "sgx")]
use futures_sgx::channel::oneshot;

#[cfg(feature = "sgx")]
use sgx_tstd::format;

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
	let vc_callback_handler = VCCallbackHandler::new(
		context.clone(),
		extrinsic_factory.clone(),
		node_metadata_repo.clone(),
	);
	let vc_callback_handler = Arc::new(vc_callback_handler);
	start_response_handler(vc_callback_handler, response_receiver);

	loop {
		let req = receiver.recv().unwrap();

		// TODO: Error handling
		let decrypted_trusted_operation =
			context.shielding_key.decrypt(&req.encrypted_trusted_call).unwrap();
		// TODO: Error handling
		let trusted_operation =
			TrustedOperation::decode(&mut decrypted_trusted_operation.as_slice()).unwrap();

		// TODO: Error handling
		let trusted_call = trusted_operation.to_call().unwrap();

		// TODO: Move this to a function that returns Result or gives an error
		if let TrustedCall::request_vc(signer, who, assertion, hash) = trusted_call.call.clone() {
			let (mut state, hash) = context.state_handler.load_cloned(&req.shard).unwrap();
			state.execute_with(|| {
				let key = UserShieldingKeys::<Runtime>::contains_key(&who);
				log::error!("This is the result of key: {:?}", key);
				let id_graph = IMT::get_id_graph(&who, usize::MAX);
				log::error!("Result of IMT Get Id Graph: {:?}", id_graph);
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

				// TODO: Understand this parameters properly and avoid the unwrap here
				let assertion_build: AssertionBuildRequest = AssertionBuildRequest {
					shard: req.shard.clone(),
					signer: signer.clone().to_account_id().unwrap(),
					enclave_account: enclave_signer_account(),
					who: who.clone().into(),
					assertion: assertion.clone(),
					identities,
					top_hash: H256::zero(),
					req_ext_hash: H256::zero(),
				};

				let context_pool = context.clone();
				let sender_pool = sender.clone();
				let vc_request_handler = VCRequestHandler {
					req: assertion_build.clone(),
					context: context_pool.clone(),
				};
				let result = vc_request_handler.process();
				sender_pool.send((result, req.sender));
			});
		}
	}
}
// TODO: Create a function that sends the error to via the one-shot channel
pub fn start_response_handler<K, A, S, H, O, Z, N>(
	vc_callback_handler: Arc<VCCallbackHandler<K, A, S, H, O, Z, N>>,
	response_receiver: Receiver<(
		Result<VCResponse, VCMPError>,
		oneshot::Sender<Result<Vec<u8>, RpcError>>,
	)>,
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
		// TODO: Error handling here
		let (vc_response, sender) = response_receiver.recv().unwrap();
		if let Err(e) = vc_response.clone() {
			if let Err(err) = sender.send(Err(RpcError::invalid_params(format!("{:?}", e)))) {
				log::warn!("Unable to send message to the RPC Handler: {:?}", err);
			}
		} else {
			log::error!("Received VC Request and we succesfully compiled it");
			vc_handler.request_vc_callback(vc_response.clone().unwrap());
			if let Err(err) = sender.send(Ok(vc_response.unwrap().vc_payload)) {
				log::warn!("Unable to send message to the RPC Handler: {:?}", err);
			}
		}
	});
}
