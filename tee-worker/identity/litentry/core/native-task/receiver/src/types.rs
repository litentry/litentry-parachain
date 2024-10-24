use codec::{Decode, Encode};
use ita_sgx_runtime::Hash;
use ita_stf::{Getter, TrustedCallSigned};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{provider::AccessNodeMetadata, NodeMetadataTrait};
use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning as StfEnclaveSigningTrait;
use itp_top_pool_author::traits::AuthorApi as AuthorApiTrait;
use lc_data_providers::DataProviderConfig;
use sp_core::ed25519::Pair as Ed25519Pair;
use std::{string::String, sync::Arc};

pub struct NativeTaskContext<
	ShieldingKeyRepository,
	AuthorApi,
	StfEnclaveSigning,
	OCallApi,
	ExtrinsicFactory,
	NodeMetadataRepo,
> where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt,
	AuthorApi: AuthorApiTrait<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	StfEnclaveSigning: StfEnclaveSigningTrait<TrustedCallSigned> + Send + Sync + 'static,
	OCallApi:
		EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	ExtrinsicFactory: CreateExtrinsics + Send + Sync + 'static,
	NodeMetadataRepo: AccessNodeMetadata + Send + Sync + 'static,
	NodeMetadataRepo::MetadataType: NodeMetadataTrait,
{
	pub shielding_key: Arc<ShieldingKeyRepository>,
	pub author_api: Arc<AuthorApi>,
	pub enclave_signer: Arc<StfEnclaveSigning>,
	pub enclave_account: Arc<Ed25519Pair>,
	pub ocall_api: Arc<OCallApi>,
	pub data_provider_config: Arc<DataProviderConfig>,
	pub extrinsic_factory: Arc<ExtrinsicFactory>,
	pub node_metadata_repo: Arc<NodeMetadataRepo>,
}

impl<
		ShieldingKeyRepository,
		AuthorApi,
		StfEnclaveSigning,
		OCallApi,
		ExtrinsicFactory,
		NodeMetadataRepo,
	>
	NativeTaskContext<
		ShieldingKeyRepository,
		AuthorApi,
		StfEnclaveSigning,
		OCallApi,
		ExtrinsicFactory,
		NodeMetadataRepo,
	> where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt,
	AuthorApi: AuthorApiTrait<Hash, Hash, TrustedCallSigned, Getter> + Send + Sync + 'static,
	StfEnclaveSigning: StfEnclaveSigningTrait<TrustedCallSigned> + Send + Sync + 'static,
	OCallApi:
		EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	ExtrinsicFactory: CreateExtrinsics + Send + Sync + 'static,
	NodeMetadataRepo: AccessNodeMetadata + Send + Sync + 'static,
	NodeMetadataRepo::MetadataType: NodeMetadataTrait,
{
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		shielding_key: Arc<ShieldingKeyRepository>,
		author_api: Arc<AuthorApi>,
		enclave_signer: Arc<StfEnclaveSigning>,
		enclave_account: Arc<Ed25519Pair>,
		ocall_api: Arc<OCallApi>,
		data_provider_config: Arc<DataProviderConfig>,
		extrinsic_factory: Arc<ExtrinsicFactory>,
		node_metadata_repo: Arc<NodeMetadataRepo>,
	) -> Self {
		Self {
			shielding_key,
			author_api,
			enclave_signer,
			enclave_account,
			ocall_api,
			data_provider_config,
			extrinsic_factory,
			node_metadata_repo,
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum NativeTaskError {
	UnexpectedCall(String),
	ShieldingKeyRetrievalFailed(String), // Stringified itp_sgx_crypto::Error
	RequestPayloadDecodingFailed,
	ParentchainDataRetrievalFailed(String), // Stringified itp_stf_state_handler::Error
	InvalidSignerAccount,
	UnauthorizedSigner,
	MissingAesKey,
	MrEnclaveRetrievalFailed,
	EnclaveSignerRetrievalFailed,
	SignatureVerificationFailed,
	ConnectionHashNotFound(String),
	MetadataRetrievalFailed(String), // Stringified itp_node_api_metadata_provider::Error
	InvalidMetadata(String),         // Stringified itp_node_api_metadata::Error
	TrustedCallSendingFailed(String), // Stringified mpsc::SendError<(H256, TrustedCall)>
	CallSendingFailed(String),
	ExtrinsicConstructionFailed(String), // Stringified itp_extrinsics_factory::Error
	ExtrinsicSendingFailed(String),      // Stringified sgx_status_t
}

pub enum NativeRequest {
	// TODO: Define the tasks
}
