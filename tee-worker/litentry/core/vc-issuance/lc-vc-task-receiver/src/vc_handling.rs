#![allow(clippy::result_large_err)]

use crate::{Getter, TrustedCallSigned};
use ita_sgx_runtime::Hash;
pub use ita_stf::aes_encrypt_default;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lc_data_providers::DataProviderConfig;
use lc_stf_task_receiver::{handler::assertion::create_credential_str, StfTaskContext};
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::VCResponse;
use litentry_primitives::{
	AmountHoldingTimeType, Assertion, ErrorDetail, ErrorString, Identity, ParameterString,
	VCMPError,
};
use std::{format, string::ToString, sync::Arc};

pub(crate) struct VCRequestHandler<
	ShieldingKeyRepository,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + 'static,
{
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O>>,
}

impl<ShieldingKeyRepository, A, S, H, O> VCRequestHandler<ShieldingKeyRepository, A, S, H, O>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType:
		ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi,
{
	pub fn process(self) -> Result<VCResponse, VCMPError> {
		let credential_str = create_credential_str(&self.req, &self.context)?;
		let vc_response = VCResponse { vc_payload: credential_str };

		Ok(vc_response)
	}
}

fn build_holding_time(
	req: &AssertionBuildRequest,
	htype: AmountHoldingTimeType,
	min_balance: ParameterString,
	data_provider_config: &DataProviderConfig,
) -> Result<lc_credentials::Credential, VCMPError> {
	lc_assertion_build::holding_time::build(req, htype, min_balance, data_provider_config)
}
