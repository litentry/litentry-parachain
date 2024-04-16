use super::*;
use base58::FromBase58;
use codec::Decode;
use core::time::Duration;
use ita_stf::AesOutput;
use itp_extrinsics_factory::mock::ExtrinsicsFactoryMock;
use itp_node_api::metadata::{metadata_mocks::NodeMetadataMock, provider::NodeMetadataRepository};
use itp_sgx_crypto::{mocks::KeyRepositoryMock, ShieldingCryptoDecrypt, ToPubkey};
use itp_stf_executor::mocks::StfEnclaveSignerMock;
use itp_stf_primitives::{traits::TrustedCallSigning, types::KeyPair};
use itp_test::mock::{
	handle_state_mock::HandleStateMock, onchain_mock::OnchainMock,
	shielding_crypto_mock::ShieldingCryptoMock,
};
use itp_top_pool_author::{
	error::Result,
	mocks::{AuthorApiMock, GLOBAL_MOCK_AUTHOR_API, GLOBAL_MOCK_RPC_API},
};
use itp_types::MrEnclave;
use lazy_static::lazy_static;
use lc_data_providers::DataProviderConfig;
use lc_stf_task_sender::{AssertionBuildRequest, RequestType};
use litentry_primitives::{aes_decrypt, AesRequest, Assertion, RequestAesKey};
use sp_core::{blake2_256, sr25519, Pair};
use std::{sync::mpsc::Receiver, vec::Vec};

pub const COMMON_SEED: &[u8] =
	b"crouch whisper apple ladder skull blouse ridge oven despair cloth pony";
lazy_static! {
	pub static ref SHIELDING_KEY: ShieldingCryptoMock = ShieldingCryptoMock::default();
}

pub fn init_global_mock_author_api() -> Result<Receiver<Vec<u8>>> {
	let (sender, receiver) = std::sync::mpsc::channel();
	let mut stf_task_storage = GLOBAL_MOCK_AUTHOR_API.lock().unwrap();
	*stf_task_storage = Some(sender);
	Ok(receiver)
}

pub fn init_global_rpc_api() -> Result<Receiver<Vec<u8>>> {
	let (sender, receiver) = std::sync::mpsc::channel();
	let mut stf_task_storage = GLOBAL_MOCK_RPC_API.lock().unwrap();
	*stf_task_storage = Some(sender);
	Ok(receiver)
}

pub fn init_stf_context() -> StfTaskContext<
	KeyRepositoryMock<ShieldingCryptoMock>,
	AuthorApiMock<H256, H256, TrustedCallSigned, Getter>,
	StfEnclaveSignerMock,
	HandleStateMock,
	OnchainMock,
> {
	let shielding_key = SHIELDING_KEY.clone();
	let shielding_key_repository_mock = KeyRepositoryMock::new(shielding_key.clone());
	let author_mock = AuthorApiMock::default();
	let stf_enclave_signer_mock = StfEnclaveSignerMock::default();
	let handle_state_mock = HandleStateMock::from_shard(H256::zero()).unwrap();
	let onchain_mock = OnchainMock::default();
	let data_provider_conifg = DataProviderConfig::new().unwrap();

	let context = StfTaskContext::new(
		Arc::new(shielding_key_repository_mock),
		author_mock.into(),
		stf_enclave_signer_mock.into(),
		handle_state_mock.into(),
		onchain_mock.into(),
		data_provider_conifg.into(),
	);
	context
}

pub fn init_vc_task() {
	let stf_context = init_stf_context();
	let node_metadata = NodeMetadataRepository::<NodeMetadataMock>::new(NodeMetadataMock::new());
	let extrinsic_factory = ExtrinsicsFactoryMock::default();
	std::thread::spawn(move || {
		run_vc_handler_runner(
			Arc::new(stf_context),
			Arc::new(extrinsic_factory),
			Arc::new(node_metadata),
		);
	});
}

pub fn execute_with_vc_task<F>(process_fn: F)
where
	F: FnOnce(&Receiver<Vec<u8>>, &Receiver<Vec<u8>>),
{
	init_vc_task();
	let top_calls_receiver = init_global_mock_author_api().unwrap();
	let rpc_calls_receiver = init_global_rpc_api().unwrap();

	thread::sleep(Duration::from_secs(2));

	process_fn(&top_calls_receiver, &rpc_calls_receiver);
}

pub fn construct_assertion_request(assertion: Assertion) -> RequestType {
	let s: String = String::from("751h9re4VmXYTEyFtsVPDm7H8PHgbz9D3guUSd1vKyUf");
	let s = s.from_base58().unwrap();
	let shard: ShardIdentifier = ShardIdentifier::decode(&mut &s[..]).unwrap();

	let seed = blake2_256(COMMON_SEED).to_vec();
	let pair = sr25519::Pair::from_seed_slice(&seed)
		.expect("Failed to create a key pair from the provided seed");
	let public_id = pair.public();

	let mut key = RequestAesKey::default();
	hex::decode_to_slice(
		"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
		&mut key,
	)
	.expect("decoding shielding_key failed");

	let request: RequestType = AssertionBuildRequest {
		shard,
		signer: public_id.into(),
		who: public_id.into(),
		assertion,
		identities: vec![],
		maybe_key: Some(key),
		parachain_block_number: 0u32,
		sidechain_block_number: 0u32,
		top_hash: H256::zero(),
		should_create_id_graph: false,
		req_ext_hash: H256::zero(),
	}
	.into();
	request
}

pub fn create_vc_request(
	assertion: Vec<Assertion>,
	mrenclave: [u8; 32],
	shard: ShardIdentifier,
) -> AesRequest {
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
	let s: String = String::from("751h9re4VmXYTEyFtsVPDm7H8PHgbz9D3guUSd1vKyUf");
	let s = s.from_base58().unwrap();

	let seed = blake2_256(COMMON_SEED).to_vec();
	let pair = sr25519::Pair::from_seed_slice(&seed)
		.expect("Failed to create a key pair from the provided seed");
	let public_id = pair.public();

	let shielding_key = SHIELDING_KEY.clone().key.pubkey().unwrap();
	let mut key = RequestAesKey::default();
	hex::decode_to_slice(
		"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
		&mut key,
	)
	.expect("decoding shielding_key failed");

	let tcs = TrustedCall::request_batch_vc(
		alice.public().into(),
		alice.public().into(),
		assertion.try_into().unwrap(),
		Some(key),
		Default::default(),
	)
	.sign(&KeyPair::Sr25519(Box::new(alice)), 0, &mrenclave, &shard);

	let top = tcs.clone().into_trusted_operation(true);

	encrypt_trusted_operation(H256::zero(), &top, shielding_key, key)
}

pub fn create_noop_trusted_call() -> AesRequest {
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
	let s: String = String::from("751h9re4VmXYTEyFtsVPDm7H8PHgbz9D3guUSd1vKyUf");
	let s = s.from_base58().unwrap();

	let seed = blake2_256(COMMON_SEED).to_vec();
	let pair = sr25519::Pair::from_seed_slice(&seed)
		.expect("Failed to create a key pair from the provided seed");
	let public_id = pair.public();

	let mrenclave = [0_u8; 32];
	let shard = H256::zero();

	let shielding_key = SHIELDING_KEY.clone().key.pubkey().unwrap();
	let mut key = RequestAesKey::default();
	hex::decode_to_slice(
		"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
		&mut key,
	)
	.expect("decoding shielding_key failed");

	let tcs = TrustedCall::noop(alice.public().into()).sign(
		&KeyPair::Sr25519(Box::new(alice)),
		0,
		&mrenclave,
		&shard,
	);

	let top = tcs.clone().into_trusted_operation(true);

	encrypt_trusted_operation(H256::zero(), &top, shielding_key, key)
}

pub fn encrypt_trusted_operation(
	shard: ShardIdentifier,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
	shielding_pubkey: sgx_crypto_helper::rsa3072::Rsa3072PubKey,
	key: RequestAesKey,
) -> AesRequest {
	let encrypted_key = shielding_pubkey.encrypt(&key).unwrap();
	let encrypted_top = aes_encrypt_default(&key, &top.encode());
	AesRequest { shard, key: encrypted_key, payload: encrypted_top }
}

pub fn assert_is_err(error: String, payload: Vec<u8>) -> bool {
	let res = RequestVcResultOrError::decode(&mut payload.as_slice()).unwrap();

	let mut key = RequestAesKey::default();
	hex::decode_to_slice(
		"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
		&mut key,
	)
	.expect("decoding shielding_key failed");

	if res.is_error {
		println!("received one error: {:?}", String::from_utf8(res.payload.clone()).unwrap());
		String::from_utf8(res.payload).unwrap() == error
	} else {
		false
	}
}
