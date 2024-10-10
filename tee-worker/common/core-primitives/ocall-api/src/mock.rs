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
use crate::{EnclaveOnChainOCallApi, Error as OCallApiError};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use codec::{Decode, Encode};
use core::fmt::Debug;
use itp_storage::Error::StorageValueUnavailable;
use itp_types::{
	parentchain::ParentchainId, storage::StorageEntryVerified, AccountId, BlockHash,
	ShardIdentifier, WorkerRequest, WorkerResponse, WorkerType,
};
use sgx_types::*;
use sp_core::H256;
use sp_runtime::{traits::Header as HeaderTrait, OpaqueExtrinsic};
use sp_std::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct OnchainMock {
	inner: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl OnchainMock {
	pub fn get_at_header<Header: HeaderTrait<Hash = H256>>(
		&self,
		header: &Header,
		key: &[u8],
	) -> Option<&Vec<u8>> {
		let key_with_header = (header, key).encode();
		self.inner.get(&key_with_header)
	}
}

impl EnclaveOnChainOCallApi for OnchainMock {
	fn send_to_parentchain(
		&self,
		_extrinsics: Vec<OpaqueExtrinsic>,
		_: &ParentchainId,
		_: bool,
	) -> SgxResult<()> {
		Ok(())
	}

	fn worker_request<V: Encode + Decode>(
		&self,
		_req: Vec<WorkerRequest>,
		_: &ParentchainId,
	) -> SgxResult<Vec<WorkerResponse<V>>> {
		Ok(Vec::new())
	}

	fn get_storage_verified<Header: HeaderTrait<Hash = H256>, V: Decode>(
		&self,
		storage_hash: Vec<u8>,
		header: &Header,
		parentchain_id: &ParentchainId,
	) -> Result<StorageEntryVerified<V>, OCallApiError> {
		self.get_multiple_storages_verified(vec![storage_hash], header, parentchain_id)?
			.into_iter()
			.next()
			.ok_or_else(|| OCallApiError::Storage(StorageValueUnavailable))
	}

	fn get_multiple_storages_verified<Header: HeaderTrait<Hash = H256>, V: Decode>(
		&self,
		storage_hashes: Vec<Vec<u8>>,
		header: &Header,
		_: &ParentchainId,
	) -> Result<Vec<StorageEntryVerified<V>>, OCallApiError> {
		let mut entries = Vec::with_capacity(storage_hashes.len());
		for hash in storage_hashes.into_iter() {
			let value = self
				.get_at_header(header, &hash)
				.map(|val| Decode::decode(&mut val.as_slice()))
				.transpose()
				.map_err(OCallApiError::Codec)?;

			entries.push(StorageEntryVerified::new(hash, value))
		}
		Ok(entries)
	}

	fn get_storage_keys<H: HeaderTrait<Hash = H256>>(
		&self,
		_key_prefix: Vec<u8>,
		_header: Option<&H>,
	) -> Result<Vec<Vec<u8>>, OCallApiError> {
		Ok(Default::default())
	}
}
