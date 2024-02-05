// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use crate::ApiResult;
use itp_api_client_types::{storage_key, traits::GetStorage, Api, Config, Request};
use itp_types::{AccountId, Enclave, MrEnclave, ShardIdentifier, WorkerType};
use sp_core::storage::StorageKey;

pub const TEEBAG: &str = "Teebag";

/// ApiClient extension that enables communication with the `teebag` pallet.
pub trait PalletTeebagApi {
	type Hash;

	fn enclave(
		&self,
		account: &AccountId,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>>;
	fn enclave_count(
		&self,
		worker_type: WorkerType,
		at_block: Option<Self::Hash>,
	) -> ApiResult<u64>;
	fn primary_enclave_identifier_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<AccountId>>;
	fn primary_enclave_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>>;
	fn all_enclaves(
		&self,
		worker_type: WorkerType,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Vec<Enclave>>;
	fn all_scheduled_mrenclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<MrEnclave>>;
}

impl<RuntimeConfig, Client> PalletTeebagApi for Api<RuntimeConfig, Client>
where
	RuntimeConfig: Config,
	Client: Request,
{
	type Hash = RuntimeConfig::Hash;

	fn enclave(
		&self,
		account: &AccountId,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>> {
		self.get_storage_map(TEEBAG, "EnclaveRegistry", account, at_block)
	}

	fn enclave_count(
		&self,
		worker_type: WorkerType,
		at_block: Option<Self::Hash>,
	) -> ApiResult<u64> {
		// Vec<> and BoundedVec<> have the same encoding, thus they are used interchangeably
		let identifiers: Vec<AccountId> = self
			.get_storage_map(TEEBAG, "EnclaveIdentifier", worker_type, at_block)?
			.unwrap_or_default();
		Ok(identifiers.len() as u64)
	}

	fn primary_enclave_identifier_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<AccountId>> {
		self.get_storage_map(TEEBAG, "EnclaveIdentifierForShard", shard, at_block)
	}

	fn primary_enclave_for_shard(
		&self,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>> {
		self.primary_enclave_identifier_for_shard(shard, at_block)?
			.map_or_else(|| Ok(None), |account| self.enclave(&account, at_block))
	}

	fn all_enclaves(
		&self,
		worker_type: WorkerType,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Vec<Enclave>> {
		let identifiers: Vec<AccountId> = self
			.get_storage_map(TEEBAG, "EnclaveIdentifier", worker_type, at_block)?
			.unwrap_or_default();

		let enclaves = identifiers
			.into_iter()
			.filter_map(|account| self.enclave(&account, at_block).ok()?)
			.collect();
		Ok(enclaves)
	}

	fn all_scheduled_mrenclaves(&self, at_block: Option<Self::Hash>) -> ApiResult<Vec<MrEnclave>> {
		let keys: Vec<_> = self
			.get_keys(storage_key(TEEBAG, "ScheduledEnclave"), at_block)?
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
