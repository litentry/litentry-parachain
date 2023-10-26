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
mod vc_primitives;
mod vc_sender;

use crate::{vc_handling::VCRequestHandler, vc_primitives::VCResponse};
use ita_sgx_runtime::Hash;
use ita_stf::{aes_encrypt_default, IdentityManagement, OpaqueCall, VCMPCallIndexes, H256};
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
use lc_stf_task_sender::RequestType;
use litentry_primitives::{Assertion, Identity};
use std::sync::{
	mpsc::{channel, Sender},
	Arc,
};

pub fn run_vc_handler_runner<K, A, S, H, O, Z, N>(
	context: Arc<StfTaskContext<K, A, S, H, O>>,
	extrinsic_factory: Arc<Z>,
	node_metadata_repo: N,
) where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + 'static,
	Z: CreateExtrinsics,
	N: AccessNodeMetadata,
	N::MetadataType: NodeMetadataTrait,
{
	let receiver = vc_sender::init_vc_task_sender_storage();
	let (sender, receiver_z) = channel::<(VCResponse, Sender<Vec<u8>>)>();

	loop {
		let req = receiver.recv().unwrap();

		let context_pool = context.clone();
		let sender_pool = sender.clone();
		VCRequestHandler { req: req.clone(), context: context_pool.clone() }.start(sender_pool);
	}
}
