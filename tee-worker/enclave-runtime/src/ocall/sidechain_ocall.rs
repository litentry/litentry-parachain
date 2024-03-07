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

use crate::ocall::{ffi, OcallApi};
use codec::{Decode, Encode};
use frame_support::ensure;
use itp_ocall_api::EnclaveSidechainOCallApi;
use itp_types::{BlockHash, ShardIdentifier};
use log::*;
use sgx_types::error::*;
use std::{string::String, vec::Vec};

impl EnclaveSidechainOCallApi for OcallApi {
	fn propose_sidechain_blocks<SignedSidechainBlock: Encode>(
		&self,
		signed_blocks: Vec<SignedSidechainBlock>,
	) -> SgxResult<()> {
		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let signed_blocks_encoded = signed_blocks.encode();

		let res = unsafe {
			ffi::ocall_propose_sidechain_blocks(
				&mut rt as *mut SgxStatus,
				signed_blocks_encoded.as_ptr(),
				signed_blocks_encoded.len() as u32,
			)
		};

		ensure!(rt == SgxStatus::Success, rt);
		ensure!(res == SgxStatus::Success, res);

		Ok(())
	}

	fn store_sidechain_blocks<SignedSidechainBlock: Encode>(
		&self,
		signed_blocks: Vec<SignedSidechainBlock>,
	) -> SgxResult<()> {
		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let signed_blocks_encoded = signed_blocks.encode();

		let res = unsafe {
			ffi::ocall_store_sidechain_blocks(
				&mut rt as *mut SgxStatus,
				signed_blocks_encoded.as_ptr(),
				signed_blocks_encoded.len() as u32,
			)
		};

		ensure!(rt == SgxStatus::Success, rt);
		ensure!(res == SgxStatus::Success, res);

		Ok(())
	}

	fn fetch_sidechain_blocks_from_peer<SignedSidechainBlock: Decode>(
		&self,
		last_imported_block_hash: BlockHash,
		maybe_until_block_hash: Option<BlockHash>,
		shard_identifier: ShardIdentifier,
	) -> SgxResult<Vec<SignedSidechainBlock>> {
		const BLOCK_BUFFER_SIZE: usize = 262144; // Buffer size for sidechain blocks in bytes (256KB).

		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let last_imported_block_hash_encoded = last_imported_block_hash.encode();
		let maybe_until_block_hash_encoded = maybe_until_block_hash.encode();
		let shard_identifier_encoded = shard_identifier.encode();

		// We have to pre-allocate the vector and hope it's large enough (see GitHub issue #621).
		let mut signed_blocks_encoded: Vec<u8> = vec![0; BLOCK_BUFFER_SIZE];

		let res = unsafe {
			ffi::ocall_fetch_sidechain_blocks_from_peer(
				&mut rt as *mut SgxStatus,
				last_imported_block_hash_encoded.as_ptr(),
				last_imported_block_hash_encoded.len() as u32,
				maybe_until_block_hash_encoded.as_ptr(),
				maybe_until_block_hash_encoded.len() as u32,
				shard_identifier_encoded.as_ptr(),
				shard_identifier_encoded.len() as u32,
				signed_blocks_encoded.as_mut_ptr(),
				signed_blocks_encoded.len() as u32,
			)
		};

		ensure!(rt == SgxStatus::Success, rt);
		ensure!(res == SgxStatus::Success, res);

		let decoded_signed_blocks: Vec<SignedSidechainBlock> =
			Decode::decode(&mut signed_blocks_encoded.as_slice()).map_err(|e| {
				error!("Failed to decode WorkerResponse: {}", e);
				SgxStatus::Unexpected
			})?;

		Ok(decoded_signed_blocks)
	}

	fn get_trusted_peers_urls(&self) -> SgxResult<Vec<String>> {
		let mut rt: SgxStatus = SgxStatus::Unexpected;
		const BLOCK_BUFFER_SIZE: usize = 262144; // Buffer size for sidechain blocks in bytes (256KB).

		// We have to pre-allocate the vector and hope it's large enough (see GitHub issue #621).
		let mut peers_encoded: Vec<u8> = vec![0; BLOCK_BUFFER_SIZE];

		let res = unsafe {
			ffi::ocall_get_trusted_peers_urls(
				&mut rt as *mut SgxStatus,
				peers_encoded.as_mut_ptr(),
				peers_encoded.len() as u32,
			)
		};

		ensure!(rt == SgxStatus::Success, rt);
		ensure!(res == SgxStatus::Success, res);

		let decoded_peers: Vec<String> =
			Decode::decode(&mut peers_encoded.as_slice()).map_err(|e| {
				error!("Failed to decode peers list: {}", e);
				SgxStatus::Unexpected
			})?;

		Ok(decoded_peers)
	}
}
