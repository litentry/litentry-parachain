use super::*;
use base58::FromBase58;
use codec::Decode;
use itp_sgx_crypto::ed25519_derivation::DeriveEd25519;
use itp_stf_executor::mocks::StfEnclaveSignerMock;
use itp_test::mock::{
	handle_state_mock::HandleStateMock, onchain_mock::OnchainMock,
	shielding_crypto_mock::ShieldingCryptoMock,
};
// use itp_top_pool_author::mocks::AuthorApiMock;
use ita_stf::{hash::TrustedOperationOrHash, Getter, TrustedCall, TrustedGetterSigned};
use itp_top_pool::primitives::PoolFuture;
use itp_top_pool_author::{error::Result, traits::OnBlockImported};
use itp_types::AccountId;
use jsonrpc_core::{futures::future::ready, Error as RpcError};
use lazy_static::lazy_static;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest,
};
use litentry_primitives::Assertion;
use sgx_crypto_helper::{rsa3072::Rsa3072KeyPair, RsaKeyPair};
use sp_core::{blake2_256, ed25519::Pair as Ed25519Pair, sr25519, Pair};
use std::{
	collections::HashMap,
	marker::PhantomData,
	sync::{
		mpsc,
		mpsc::{Receiver, Sender},
		Mutex, RwLock,
	},
	thread,
	time::Duration,
	vec::Vec,
};

lazy_static! {
	static ref GLOBAL_MOCK_AUTHOR_API: Arc<Mutex<Option<Sender<Vec<u8>>>>> =
		Arc::new(Mutex::new(None));
}

pub const COMMON_SEED: &[u8] =
	b"crouch whisper apple ladder skull blouse ridge oven despair cloth pony";

pub fn init_global_mock_author_api() -> Result<Receiver<Vec<u8>>> {
	let (sender, receiver) = std::sync::mpsc::channel();
	let mut stf_task_storage = GLOBAL_MOCK_AUTHOR_API.lock().unwrap();
	*stf_task_storage = Some(sender);
	Ok(receiver)
}

#[test]
fn test_mock_context() {
	let shielding_key = ShieldingCryptoMock::default();
	let author_mock: AuthorApiMock<H256, H256> = AuthorApiMock::default();
	let stf_enclave_signer_mock = StfEnclaveSignerMock::default();
	let handle_state_mock = HandleStateMock::default();
	let onchain_mock = OnchainMock::default();
	let context = StfTaskContext::new(
		shielding_key.clone(),
		author_mock.into(),
		stf_enclave_signer_mock.into(),
		handle_state_mock.into(),
		onchain_mock.into(),
	);
	let handle = thread::spawn(move || {
		run_stf_task_receiver(Arc::new(context)).unwrap();
	});

	let sender = StfRequestSender::default();

	let s: String = String::from("751h9re4VmXYTEyFtsVPDm7H8PHgbz9D3guUSd1vKyUf");
	let s = s.from_base58().unwrap();
	let mrenclave: [u8; 32] = s.clone().try_into().unwrap();
	let shard: ShardIdentifier = ShardIdentifier::decode(&mut &s[..]).unwrap();

	let seed = blake2_256(COMMON_SEED).to_vec();
	let pair = sr25519::Pair::from_seed_slice(&seed)
		.expect("Failed to create a key pair from the provided seed");
	let public_id = pair.public();

	let request: RequestType = AssertionBuildRequest {
		shard,
		signer: public_id.into(),
		enclave_account: public_id.into(),
		who: public_id.into(),
		assertion: Assertion::A1,
		identities: vec![],
		top_hash: H256::zero(),
		req_ext_hash: H256::zero(),
	}
	.into();

	sender.send_stf_request(request).unwrap();

	let request: RequestType = AssertionBuildRequest {
		shard,
		signer: public_id.into(),
		enclave_account: public_id.into(),
		who: public_id.into(),
		assertion: Assertion::A6,
		identities: vec![],
		top_hash: H256::zero(),
		req_ext_hash: H256::zero(),
	}
	.into();

	sender.send_stf_request(request).unwrap();

	let mut expected_output: Vec<Assertion> = vec![Assertion::A1, Assertion::A6];

	let receiver = init_global_mock_author_api().unwrap();
	while let Ok(ext) = receiver.recv() {
		let decrypted = shielding_key.decrypt(&ext).unwrap();
		let decoded: TrustedOperation = Decode::decode(&mut decrypted.as_ref()).unwrap();
		match decoded {
			TrustedOperation::direct_call(trusted_call_signed) =>
				if let TrustedCall::request_vc_callback(_, _, assertion, ..) =
					trusted_call_signed.call
				{
					println!("Received Request VC Callback for: {:?}", assertion);
					assert_eq!(expected_output.remove(0), assertion);
				},
			_ => {
				// Do nothing
			},
		}
		if expected_output.len() == 0 {
			break
		}
		// println!("Trusted Operation: {:?}", decoded)
	}

	// handle.join().unwrap();
}

#[test]
fn test_run_stf_task_receiver() {}

// We cannot use the AuthorApiMock as it is because it doesn't implement watch_top,
// So we have to create our own AuthorApiMock
#[derive(Default)]
pub struct AuthorApiMock<Hash, BlockHash> {
	tops: RwLock<HashMap<ShardIdentifier, Vec<Vec<u8>>>>,
	_phantom: PhantomData<(Hash, BlockHash)>,
	pub remove_attempts: RwLock<usize>,
}

impl<Hash, BlockHash> AuthorApiMock<Hash, BlockHash> {
	fn decode_trusted_operation(mut encoded_operation: &[u8]) -> Option<TrustedOperation> {
		TrustedOperation::decode(&mut encoded_operation).ok()
	}

	fn decode_trusted_getter_signed(mut encoded_operation: &[u8]) -> Option<TrustedGetterSigned> {
		TrustedGetterSigned::decode(&mut encoded_operation).ok()
	}

	fn remove_top(
		&self,
		bytes_or_hash: Vec<TrustedOperationOrHash<H256>>,
		shard: ShardIdentifier,
		_inblock: bool,
	) -> Result<Vec<H256>> {
		let hashes = bytes_or_hash
			.into_iter()
			.map(|x| match x {
				TrustedOperationOrHash::Hash(h) => h,
				TrustedOperationOrHash::OperationEncoded(bytes) => {
					let top: TrustedOperation =
						TrustedOperation::decode(&mut bytes.as_slice()).unwrap();
					top.hash()
				},
				TrustedOperationOrHash::Operation(op) => op.hash(),
			})
			.collect::<Vec<_>>();

		let mut tops_lock = self.tops.write().unwrap();

		// Note: Not important for the test
		// match tops_lock.get_mut(&shard) {
		// 	Some(tops_encoded) => {
		// 		let removed_tops = tops_encoded
		// 			.drain_filter(|t| hashes.contains(&blake2_256(t).into()))
		// 			.map(|t| blake2_256(&t).into())
		// 			.collect::<Vec<_>>();
		// 		Ok(removed_tops)
		// 	},
		// 	None => Ok(Vec::new()),
		// }
		Ok(Vec::new())
	}
}

impl AuthorApi<H256, H256> for AuthorApiMock<H256, H256> {
	fn submit_top(&self, extrinsic: Vec<u8>, shard: ShardIdentifier) -> PoolFuture<H256, RpcError> {
		let mut write_lock = self.tops.write().unwrap();
		let extrinsics = write_lock.entry(shard).or_default();
		extrinsics.push(extrinsic);
		Box::pin(ready(Ok(H256::default())))
	}

	fn hash_of(&self, xt: &TrustedOperation) -> H256 {
		xt.hash()
	}

	fn pending_tops(&self, shard: ShardIdentifier) -> Result<Vec<Vec<u8>>> {
		let extrinsics = self.tops.read().unwrap().get(&shard).cloned();
		Ok(extrinsics.unwrap_or_default())
	}

	fn get_pending_trusted_getters(&self, shard: ShardIdentifier) -> Vec<TrustedOperation> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_getters: Vec<TrustedOperation> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Some(g) = Self::decode_trusted_getter_signed(encoded_operation) {
						trusted_getters.push(TrustedOperation::get(Getter::trusted(g)));
					}
				}
				trusted_getters
			})
			.unwrap_or_default()
	}

	fn get_pending_trusted_calls(&self, shard: ShardIdentifier) -> Vec<TrustedOperation> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_operations: Vec<TrustedOperation> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Some(o) = Self::decode_trusted_operation(encoded_operation) {
						trusted_operations.push(o);
					}
				}
				trusted_operations
			})
			.unwrap_or_default()
	}

	fn get_pending_trusted_calls_for(
		&self,
		shard: ShardIdentifier,
		account: &AccountId,
	) -> Vec<TrustedOperation> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_operations: Vec<TrustedOperation> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Some(o) = Self::decode_trusted_operation(encoded_operation) {
						if o.signed_caller_account().as_ref() == Some(account) {
							trusted_operations.push(o);
						}
					}
				}
				trusted_operations
			})
			.unwrap_or_default()
	}

	fn get_shards(&self) -> Vec<ShardIdentifier> {
		self.tops.read().unwrap().keys().cloned().collect()
	}

	fn remove_calls_from_pool(
		&self,
		shard: ShardIdentifier,
		executed_calls: Vec<(TrustedOperationOrHash<H256>, bool)>,
	) -> Vec<TrustedOperationOrHash<H256>> {
		let mut remove_attempts_lock = self.remove_attempts.write().unwrap();
		*remove_attempts_lock += 1;

		let mut failed_to_remove = Vec::new();
		for (executed_call, inblock) in executed_calls {
			if self.remove_top(vec![executed_call.clone()], shard, inblock).is_err() {
				failed_to_remove.push(executed_call);
			}
		}
		failed_to_remove
	}

	fn watch_top(&self, ext: Vec<u8>, _shard: ShardIdentifier) -> PoolFuture<H256, RpcError> {
		let sender_guard = GLOBAL_MOCK_AUTHOR_API.lock().unwrap();
		let sender = &*sender_guard;
		sender.as_ref().expect("Not yet initialized").send(ext).unwrap();
		Box::pin(ready(Ok([0u8; 32].into())))
	}

	fn update_connection_state(&self, _updates: Vec<(H256, (Vec<u8>, bool))>) {}

	fn swap_rpc_connection_hash(&self, _old_hash: H256, _new_hash: H256) {}
}

impl OnBlockImported for AuthorApiMock<H256, H256> {
	type Hash = H256;

	fn on_block_imported(&self, _hashes: &[Self::Hash], _block_hash: H256) {}
}
