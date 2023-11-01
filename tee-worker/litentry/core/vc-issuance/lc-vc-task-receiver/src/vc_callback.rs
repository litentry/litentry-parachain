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
use itp_types::ShardIdentifier;
use lc_stf_task_receiver::StfTaskContext;
use lc_vc_task_sender::VCResponse;
use litentry_primitives::{Assertion, Identity};
use std::sync::Arc;

pub(crate) struct VCCallbackHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	O: EnclaveOnChainOCallApi + Send + Sync + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
> {
	pub(crate) context: Arc<StfTaskContext<K, A, S, H, O>>,
	pub(crate) extrinsic_factory: Arc<Z>,
	pub(crate) node_metadata_repo: Arc<N>,
}

// We need to have some form of responder to the JSONRPC sender. (TODO: P-188)
impl<K, A, S, H, O, Z, N> VCCallbackHandler<K, A, S, H, O, Z, N>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone + Send + Sync + 'static,
	A: AuthorApi<Hash, Hash> + Send + Sync + 'static,
	S: StfEnclaveSigning + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + Send + Sync + 'static,
	Z: CreateExtrinsics + Send + Sync + 'static,
	N: AccessNodeMetadata + Send + Sync + 'static,
	N::MetadataType: NodeMetadataTrait,
{
	pub fn new(
		context: Arc<StfTaskContext<K, A, S, H, O>>,
		extrinsic_factory: Arc<Z>,
		node_metadata_repo: Arc<N>,
	) -> Self {
		Self { context, extrinsic_factory, node_metadata_repo }
	}

	pub fn request_vc_callback(&self, response: VCResponse) {
		let (mut state, _) = self
			.context
			.state_handler
			.load_cloned(&response.assertion_request.shard)
			.unwrap();
		state.execute_with(|| {
			let key =
				IdentityManagement::user_shielding_keys(&response.assertion_request.who).unwrap();
			let call_index = self
				.node_metadata_repo
				.get_from_metadata(|m| m.vc_issued_call_indexes())
				.unwrap()
				.unwrap();
			let result = aes_encrypt_default(&key, &response.vc_payload);
			let call = OpaqueCall::from_tuple(&(
				call_index,
				response.assertion_request.who.to_account_id().unwrap(),
				response.assertion_request.assertion,
				response.vc_index,
				response.vc_hash,
				aes_encrypt_default(&key, &response.vc_payload),
				H256::zero(),
			));
			let xt = self.extrinsic_factory.create_extrinsics(&[call], None).unwrap();
			self.context.ocall_api.send_to_parentchain(xt).unwrap();
		});
	}
}
