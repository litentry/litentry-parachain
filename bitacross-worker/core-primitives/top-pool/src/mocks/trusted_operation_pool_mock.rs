/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use std::sync::RwLock;

use crate::{
	base_pool::TrustedOperation,
	error::Error,
	primitives::{
		ImportNotificationStream, PoolFuture, PoolStatus, TrustedOperationPool,
		TrustedOperationSource, TxHash,
	},
};
use codec::Encode;
use core::{future::Future, pin::Pin};

use itp_types::{Block, ShardIdentifier, H256};
use jsonrpc_core::futures::future::ready;
use sp_runtime::{
	generic::BlockId,
	traits::{BlakeTwo256, Hash, NumberFor},
};
use std::{boxed::Box, collections::HashMap, string::String, sync::Arc, vec, vec::Vec};

/// Mock for the trusted operation pool
///
/// To be used in unit tests
pub struct TrustedOperationPoolMock<TOP: Encode + Clone + Send + Sync + 'static> {
	submitted_transactions: RwLock<HashMap<ShardIdentifier, TxPayload<TOP>>>,
}

/// Transaction payload
#[derive(Clone, PartialEq)]
pub struct TxPayload<TOP: Encode + Clone + Send + Sync + 'static> {
	pub block_id: BlockId<<TrustedOperationPoolMock<TOP> as TrustedOperationPool<TOP>>::Block>,
	pub source: TrustedOperationSource,
	pub xts: Vec<TOP>,
	pub shard: ShardIdentifier,
}

impl<TOP: Encode + Clone + Send + Sync + 'static> Default for TrustedOperationPoolMock<TOP> {
	fn default() -> Self {
		TrustedOperationPoolMock::<TOP> { submitted_transactions: RwLock::new(HashMap::new()) }
	}
}

impl<TOP: Encode + Clone + Send + Sync + 'static> TrustedOperationPoolMock<TOP> {
	pub fn get_last_submitted_transactions(&self) -> HashMap<ShardIdentifier, TxPayload<TOP>> {
		let transactions = self.submitted_transactions.read().unwrap();
		transactions.clone()
	}

	fn map_stf_top_to_tx(stf_top: &TOP) -> Arc<TrustedOperation<TOP>> {
		Arc::new(TrustedOperation::<TOP> {
			data: stf_top.clone(),
			bytes: 0,
			hash: hash_of_top(stf_top),
			priority: 0u64,
			valid_till: 0u64,
			requires: vec![],
			provides: vec![],
			propagate: false,
			source: TrustedOperationSource::External,
		})
	}
}

impl<TOP> TrustedOperationPool<TOP> for TrustedOperationPoolMock<TOP>
where
	TOP: Encode + Clone + Sync + Send + 'static,
{
	type Block = Block;
	type InPoolOperation = TrustedOperation<TOP>;
	type Error = Error;

	#[allow(clippy::type_complexity)]
	fn submit_at(
		&self,
		at: &BlockId<Self::Block>,
		source: TrustedOperationSource,
		xts: Vec<TOP>,
		shard: ShardIdentifier,
	) -> PoolFuture<Vec<Result<TxHash, Self::Error>>, Self::Error> {
		let mut transactions = self.submitted_transactions.write().unwrap();
		transactions.insert(shard, TxPayload { block_id: *at, source, xts: xts.clone(), shard });

		let top_hashes: Vec<Result<TxHash, Self::Error>> =
			xts.iter().map(|top| Ok(hash_of_top(top))).collect();

		Box::pin(ready(Ok(top_hashes)))
	}

	fn submit_one(
		&self,
		at: &BlockId<Self::Block>,
		source: TrustedOperationSource,
		xt: TOP,
		shard: ShardIdentifier,
	) -> PoolFuture<TxHash, Self::Error> {
		let mut transactions = self.submitted_transactions.write().unwrap();
		transactions
			.insert(shard, TxPayload { block_id: *at, source, xts: vec![xt.clone()], shard });

		let top_hash = hash_of_top(&xt);

		Box::pin(ready(Ok(top_hash)))
	}

	fn submit_and_watch(
		&self,
		at: &BlockId<Self::Block>,
		source: TrustedOperationSource,
		xt: TOP,
		shard: ShardIdentifier,
	) -> PoolFuture<TxHash, Self::Error> {
		self.submit_one(at, source, xt, shard)
	}

	#[allow(clippy::type_complexity)]
	fn ready_at(
		&self,
		_at: NumberFor<Self::Block>,
		_shard: ShardIdentifier,
	) -> Pin<
		Box<
			dyn Future<Output = Box<dyn Iterator<Item = Arc<Self::InPoolOperation>> + Send>> + Send,
		>,
	> {
		unimplemented!()
	}

	#[allow(clippy::type_complexity)]
	fn ready(
		&self,
		shard: ShardIdentifier,
	) -> Box<dyn Iterator<Item = Arc<Self::InPoolOperation>> + Send> {
		let transactions = self.submitted_transactions.read().unwrap();
		let ready_transactions = transactions
			.get(&shard)
			.map(|payload| payload.xts.iter().map(Self::map_stf_top_to_tx).collect())
			.unwrap_or_else(Vec::new);
		Box::new(ready_transactions.into_iter())
	}

	fn shards(&self) -> Vec<ShardIdentifier> {
		let transactions = self.submitted_transactions.read().unwrap();
		transactions.iter().map(|(shard, _)| *shard).collect()
	}

	fn remove_invalid(
		&self,
		_hashes: &[TxHash],
		_shard: ShardIdentifier,
		_inblock: bool,
	) -> Vec<Arc<Self::InPoolOperation>> {
		Vec::new()
	}

	fn status(&self, shard: ShardIdentifier) -> PoolStatus {
		let transactions = self.submitted_transactions.read().unwrap();
		transactions
			.get(&shard)
			.map(|payload| PoolStatus {
				ready: payload.xts.len(),
				ready_bytes: 0,
				future: 0,
				future_bytes: 0,
			})
			.unwrap_or_else(default_pool_status)
	}

	fn import_notification_stream(&self) -> ImportNotificationStream<TxHash> {
		unimplemented!()
	}

	fn on_broadcasted(&self, _propagations: HashMap<TxHash, Vec<String>>) {
		unimplemented!()
	}

	fn hash_of(&self, xt: &TOP) -> TxHash {
		hash_of_top(xt)
	}

	fn ready_transaction(
		&self,
		_hash: &TxHash,
		_shard: ShardIdentifier,
	) -> Option<Arc<Self::InPoolOperation>> {
		unimplemented!()
	}

	fn update_connection_state(&self, _updates: Vec<(TxHash, (Vec<u8>, bool))>) {}

	fn swap_rpc_connection_hash(&self, _old_hash: TxHash, _new_hash: TxHash) {}
}

fn default_pool_status() -> PoolStatus {
	PoolStatus { ready: 0, ready_bytes: 0, future: 0, future_bytes: 0 }
}

fn hash_of_top<TOP: Encode>(top: &TOP) -> H256 {
	top.using_encoded(|x| BlakeTwo256::hash(x))
}
