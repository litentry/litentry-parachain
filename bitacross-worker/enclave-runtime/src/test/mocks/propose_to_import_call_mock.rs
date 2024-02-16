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

use codec::{Decode, Encode};
use itc_parentchain::primitives::ParentchainId;
use itp_ocall_api::{
	EnclaveMetricsOCallApi, EnclaveOnChainOCallApi, EnclaveSidechainOCallApi, Result,
};
use itp_types::{
	storage::StorageEntryVerified, BlockHash, Header as ParentchainHeader, ShardIdentifier,
	WorkerRequest, WorkerResponse, H256,
};
use sgx_types::SgxResult;
use sp_runtime::{traits::Header as ParentchainHeaderTrait, OpaqueExtrinsic};
use std::{string::String, sync::Arc, vec::Vec};

/// OCallApi mock that routes the proposed sidechain blocks directly to the importer,
/// short circuiting all the RPC calls.
#[derive(Clone)]
pub struct ProposeToImportOCallApi {
	parentchain_header: ParentchainHeader,
}

impl ProposeToImportOCallApi {
	pub fn new(parentchain_header: ParentchainHeader) -> Self {
		ProposeToImportOCallApi { parentchain_header }
	}
}

impl EnclaveOnChainOCallApi for ProposeToImportOCallApi {
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
		todo!()
	}

	fn get_storage_verified<H: ParentchainHeaderTrait<Hash = H256>, V: Decode>(
		&self,
		_storage_hash: Vec<u8>,
		_header: &H,
		_: &ParentchainId,
	) -> Result<StorageEntryVerified<V>> {
		todo!()
	}

	fn get_multiple_storages_verified<H: ParentchainHeaderTrait<Hash = H256>, V: Decode>(
		&self,
		_storage_hashes: Vec<Vec<u8>>,
		_header: &H,
		_: &ParentchainId,
	) -> Result<Vec<StorageEntryVerified<V>>> {
		todo!()
	}

	fn get_storage_keys(&self, _key_prefix: Vec<u8>) -> Result<Vec<Vec<u8>>> {
		todo!()
	}
}

impl EnclaveMetricsOCallApi for ProposeToImportOCallApi {
	fn update_metric<Metric: Encode>(&self, _metric: Metric) -> SgxResult<()> {
		Ok(())
	}
}
