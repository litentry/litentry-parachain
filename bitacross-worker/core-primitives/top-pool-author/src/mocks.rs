/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;
use core::fmt::Debug;

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use std::sync::RwLock;

use crate::{
	error::Result,
	traits::{AuthorApi, OnBlockImported},
};
use codec::{Decode, Encode};
use itp_stf_primitives::{
	traits::TrustedCallVerification,
	types::{AccountId, TrustedOperation as StfTrustedOperation, TrustedOperationOrHash},
};
use itp_top_pool::primitives::{PoolFuture, PoolStatus};
use itp_types::{DecryptableRequest, ShardIdentifier};
use jsonrpc_core::{futures::future::ready, Error as RpcError};
use lazy_static::lazy_static;
use sp_core::{blake2_256, H256};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	boxed::Box,
	collections::HashMap,
	marker::PhantomData,
	sync::{mpsc::Sender, Arc},
	vec,
	vec::Vec,
};

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(feature = "std")]
use std::sync::Mutex;

lazy_static! {
	pub static ref GLOBAL_MOCK_AUTHOR_API: Arc<Mutex<Option<Sender<Vec<u8>>>>> =
		Arc::new(Mutex::new(None));
}

#[derive(Default)]
pub struct AuthorApiMock<Hash, BlockHash, TCS, G>
where
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + TrustedCallVerification,
	G: PartialEq + Encode + Decode + Debug + Send + Sync,
{
	tops: RwLock<HashMap<ShardIdentifier, Vec<Vec<u8>>>>,
	_phantom: PhantomData<(Hash, BlockHash, TCS, G)>,
	pub remove_attempts: RwLock<usize>,
}

impl<TCS, G> AuthorApi<H256, H256, TCS, G> for AuthorApiMock<H256, H256, TCS, G>
where
	TCS: PartialEq + Encode + Decode + Debug + Clone + TrustedCallVerification + Send + Sync,
	G: PartialEq + Encode + Decode + Debug + Clone + Send + Sync,
{
	fn submit_top<R: DecryptableRequest>(&self, req: R) -> PoolFuture<H256, RpcError> {
		let mut write_lock = self.tops.write().unwrap();
		let extrinsics = write_lock.entry(req.shard()).or_default();
		extrinsics.push(req.payload().to_vec());
		Box::pin(ready(Ok(H256::default())))
	}

	fn hash_of(&self, xt: &StfTrustedOperation<TCS, G>) -> H256 {
		xt.hash()
	}

	fn pending_tops(&self, shard: ShardIdentifier) -> Result<Vec<Vec<u8>>> {
		let extrinsics = self.tops.read().unwrap().get(&shard).cloned();
		Ok(extrinsics.unwrap_or_default())
	}

	fn get_pending_getters(&self, shard: ShardIdentifier) -> Vec<StfTrustedOperation<TCS, G>> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_getters: Vec<StfTrustedOperation<TCS, G>> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Ok(g) = G::decode(&mut encoded_operation.as_slice()) {
						trusted_getters.push(StfTrustedOperation::<TCS, G>::get(g));
					}
				}
				trusted_getters
			})
			.unwrap_or_default()
	}

	fn get_pending_trusted_calls(
		&self,
		shard: ShardIdentifier,
	) -> Vec<StfTrustedOperation<TCS, G>> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_operations: Vec<StfTrustedOperation<TCS, G>> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Ok(o) = StfTrustedOperation::decode(&mut encoded_operation.as_slice()) {
						trusted_operations.push(o);
					}
				}
				trusted_operations
			})
			.unwrap_or_default()
	}

	fn get_status(&self, shard: ShardIdentifier) -> PoolStatus {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_operations: Vec<StfTrustedOperation<TCS, G>> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Ok(o) = StfTrustedOperation::decode(&mut encoded_operation.as_slice()) {
						trusted_operations.push(o);
					}
				}
				PoolStatus {
					ready: trusted_operations.len(),
					ready_bytes: trusted_operations.encode().len(),
					future: 0,
					future_bytes: 0,
				}
			})
			.unwrap_or_default()
	}

	fn get_pending_trusted_calls_for(
		&self,
		shard: ShardIdentifier,
		account: &AccountId,
	) -> Vec<StfTrustedOperation<TCS, G>> {
		self.tops
			.read()
			.unwrap()
			.get(&shard)
			.map(|encoded_operations| {
				let mut trusted_operations: Vec<StfTrustedOperation<TCS, G>> = Vec::new();
				for encoded_operation in encoded_operations {
					if let Ok(top) = StfTrustedOperation::decode(&mut encoded_operation.as_slice())
					{
						if top.signed_caller_account().as_ref() == Some(account) {
							trusted_operations.push(top);
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

	fn list_handled_shards(&self) -> Vec<ShardIdentifier> {
		//dummy
		self.tops.read().unwrap().keys().cloned().collect()
	}

	fn watch_top<R: DecryptableRequest>(&self, request: R) -> PoolFuture<H256, RpcError> {
		// Note: The below implementation is specific for litentry/core/stf-task/receiver/test.rs
		let sender_guard = GLOBAL_MOCK_AUTHOR_API.lock().unwrap();
		let sender = &*sender_guard;
		sender
			.as_ref()
			.expect("Not yet initialized")
			.send(request.payload().to_vec())
			.unwrap();
		Box::pin(ready(Ok([0u8; 32].into())))
	}

	fn update_connection_state(&self, _updates: Vec<(H256, (Vec<u8>, bool))>) {}

	fn swap_rpc_connection_hash(&self, _old_hash: H256, _new_hash: H256) {}
}

impl<TCS, G> OnBlockImported for AuthorApiMock<H256, H256, TCS, G>
where
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + TrustedCallVerification,
	G: PartialEq + Encode + Decode + Debug + Send + Sync,
{
	type Hash = H256;

	fn on_block_imported(&self, _hashes: &[Self::Hash], _block_hash: H256) {}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::test_fixtures::shard_id;
	use codec::Encode;
	use futures::executor::block_on;
	use itp_test::mock::stf_mock::{
		mock_top_indirect_trusted_call_signed, GetterMock, TrustedCallSignedMock,
	};
	use itp_types::RsaRequest;
	use std::vec;

	#[test]
	fn submitted_tops_can_be_removed_again() {
		let author = AuthorApiMock::<H256, H256, TrustedCallSignedMock, GetterMock>::default();
		let shard = shard_id();
		let trusted_operation = mock_top_indirect_trusted_call_signed();

		let _ = block_on(author.submit_top(RsaRequest::new(shard, trusted_operation.encode())))
			.unwrap();

		assert_eq!(1, author.pending_tops(shard).unwrap().len());
		assert_eq!(1, author.get_pending_trusted_calls(shard).len());
		assert_eq!(0, author.get_pending_getters(shard).len());

		let trusted_operation_or_hash =
			TrustedOperationOrHash::<TrustedCallSignedMock, GetterMock>::from_top(
				trusted_operation.clone(),
			);
		let removed_tops = author.remove_top(vec![trusted_operation_or_hash], shard, true).unwrap();

		assert_eq!(1, removed_tops.len());
		assert!(author.tops.read().unwrap().get(&shard).unwrap().is_empty());
	}
}
