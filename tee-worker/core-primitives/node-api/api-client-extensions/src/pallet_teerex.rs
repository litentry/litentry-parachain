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

use crate::ApiResult;
use itp_api_client_types::{traits::GetStorage, storage_key, Api, Config, Request};
use itp_types::{parentchain::Hash, AccountId, Enclave, IpfsHash, MrEnclave, ShardIdentifier};
use sp_core::storage::StorageKey;
// use substrate_api_client::{
// 	rpc::Request, storage_key, Api, GetStorage,
// };

pub const TEEREX: &str = "Teerex";
pub const SIDECHAIN: &str = "Sidechain";

/// ApiClient extension that enables communication with the `teerex` pallet.
// Todo: make generic over `Config` type instead?
pub trait PalletTeerexApi {
	type Hash;

	fn enclave(&self, index: u64, at_block: Option<Self::Hash>) -> ApiResult<Option<Enclave>>;
	fn enclave_count(&self, at_block: Option<Self::Hash>) -> ApiResult<u64>;
	fn all_enclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<Enclave>>;
	fn worker_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>>;
	fn latest_ipfs_hash(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<IpfsHash>>;

	// litentry
	fn all_scheduled_mrenclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<MrEnclave>>;
}

impl<RuntimeConfig, Client> PalletTeerexApi for Api<RuntimeConfig, Client>
where
	RuntimeConfig: Config,
	Client: Request,
{
	type Hash = RuntimeConfig::Hash;

	fn enclave(&self, index: u64, at_block: Option<Self::Hash>) -> ApiResult<Option<Enclave>> {
		self.get_storage_map(TEEREX, "EnclaveRegistry", index, at_block)
	}

	fn enclave_count(&self, at_block: Option<Self::Hash>) -> ApiResult<u64> {
		Ok(self.all_enclaves(at_block)?.len() as u64)
	}

	fn all_enclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<Enclave>> {
		let count = self.enclave_count(at_block)?;
		let mut enclaves = Vec::with_capacity(count as usize);
		for n in 1..=count {
			enclaves.push(self.enclave(n, at_block)?.expect("None enclave"))
		}
		Ok(enclaves)
	}

	fn worker_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>> {
		self.get_storage_map(SIDECHAIN, "WorkerForShard", shard, at_block)?
			.map_or_else(|| Ok(None), |w_index| self.enclave(w_index, at_block))
	}

	fn latest_ipfs_hash(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<IpfsHash>> {
		self.get_storage_map(TEEREX, "LatestIPFSHash", shard, at_block)
	}

	fn all_scheduled_mrenclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<MrEnclave>> {
		let keys: Vec<_> = self
			.get_keys(storage_key(TEEREX, "ScheduledEnclave"), at_block)?
			.unwrap_or_default()
			.iter()
			.map(|key| {
				let key = key.strip_prefix("0x").unwrap_or(key);
				let raw_key = hex::decode(key).unwrap();
				self.get_storage_by_key::<MrEnclave>(StorageKey(raw_key).into(), at_block)
			})
			.filter(|enclave| matches!(enclave, Ok(Some(_))))
			.map(|enclave| enclave.unwrap().unwrap())
			.collect();
		Ok(keys)
	}
}
