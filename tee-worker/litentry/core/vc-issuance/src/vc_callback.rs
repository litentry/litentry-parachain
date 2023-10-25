use ita_sgx_runtime::Hash;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lc_stf_task_receiver::StfTaskContext;
use std::sync::Arc;

pub(crate) struct VCCallbackHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	Z: CreateExtrinsics,
> {
	pub(crate) context: Arc<StfTaskContext<K, A, S, H, O>>,
	pub(crate) extrinsic_factory: Arc<Z>,
}
