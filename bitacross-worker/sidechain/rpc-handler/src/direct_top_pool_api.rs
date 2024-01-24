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

#[cfg(feature = "std")]
use rust_base58::base58::FromBase58;

#[cfg(feature = "sgx")]
use base58::FromBase58;

use ba_itp_utils::{FromHexPrefixed, ToHexPrefixed};
use codec::{Decode, Encode};
use itp_rpc::RpcReturnValue;
use itp_stf_primitives::types::AccountId;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{DirectRequestStatus, RsaRequest, ShardIdentifier, TrustedOperationStatus};
use jsonrpc_core::{futures::executor, serde_json::json, Error as RpcError, IoHandler, Params};
use litentry_primitives::AesRequest;
use log::*;
use std::{
	borrow::ToOwned,
	format,
	string::{String, ToString},
	sync::Arc,
	vec,
	vec::Vec,
};

type Hash = sp_core::H256;

pub fn add_top_pool_direct_rpc_methods<R, TCS, G>(
	top_pool_author: Arc<R>,
	mut io_handler: IoHandler,
) -> IoHandler
where
	R: AuthorApi<Hash, Hash, TCS, G> + Send + Sync + 'static,
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
	G: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
{
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method("author_submitAndWatchRsaRequest", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_submitAndWatchRsaRequest");
		let json_value = match author_submit_extrinsic_inner(
			watch_author.clone(),
			params,
			Some("author_submitAndWatchBroadcastedRsaRequest".to_owned()),
		) {
			Ok(hash_value) => RpcReturnValue {
				do_watch: true,
				value: vec![],
				status: DirectRequestStatus::TrustedOperationStatus(
					TrustedOperationStatus::Submitted,
					hash_value,
				),
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};
		Ok(json!(json_value))
	});

	// author_submitAndWatchBroadcastedRsaRequest
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(
		"author_submitAndWatchBroadcastedRsaRequest",
		move |params: Params| {
			let json_value = match author_submit_extrinsic_inner(watch_author.clone(), params, None)
			{
				Ok(hash_value) => {
					RpcReturnValue {
						do_watch: true,
						value: vec![],
						status: DirectRequestStatus::TrustedOperationStatus(
							TrustedOperationStatus::Submitted,
							hash_value,
						),
					}
				}
				.to_hex(),
				Err(error) => compute_hex_encoded_return_error(error.as_str()),
			};
			Ok(json!(json_value))
		},
	);

	// author_submitRsaRequest
	let submit_author = top_pool_author.clone();
	io_handler.add_sync_method("author_submitRsaRequest", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_submitRsaRequest");
		let json_value = match author_submit_extrinsic_inner(submit_author.clone(), params, None) {
			Ok(hash_value) => RpcReturnValue {
				do_watch: false,
				value: vec![],
				status: DirectRequestStatus::TrustedOperationStatus(
					TrustedOperationStatus::Submitted,
					hash_value,
				),
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};
		Ok(json!(json_value))
	});

	// Litentry: a morphling of `author_submitAndWatchRsaRequest`
	// a different name is used to highlight the request type
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method("author_submitAndWatchAesRequest", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_submitAndWatchAesRequest");
		let json_value = match author_submit_aes_request_inner(
			watch_author.clone(),
			params,
			Some("author_submitAndWatchBroadcastedAesRequest".to_owned()),
		) {
			Ok(hash_value) => RpcReturnValue {
				do_watch: true,
				value: vec![],
				status: DirectRequestStatus::TrustedOperationStatus(
					TrustedOperationStatus::Submitted,
					hash_value,
				),
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};
		Ok(json!(json_value))
	});

	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(
		"author_submitAndWatchBroadcastedAesRequest",
		move |params: Params| {
			let json_value =
				match author_submit_aes_request_inner(watch_author.clone(), params, None) {
					Ok(hash_value) => RpcReturnValue {
						do_watch: true,
						value: vec![],
						status: DirectRequestStatus::TrustedOperationStatus(
							TrustedOperationStatus::Submitted,
							hash_value,
						),
					}
					.to_hex(),
					Err(error) => compute_hex_encoded_return_error(error.as_str()),
				};
			Ok(json!(json_value))
		},
	);

	// author_pendingExtrinsics
	let pending_author = top_pool_author.clone();
	io_handler.add_sync_method("author_pendingExtrinsics", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_pendingExtrinsics");
		match params.parse::<Vec<String>>() {
			Ok(shards) => {
				let mut retrieved_operations = vec![];
				for shard_base58 in shards.iter() {
					let shard = match decode_shard_from_base58(shard_base58.as_str()) {
						Ok(id) => id,
						Err(msg) => {
							let error_msg: String =
								format!("Could not retrieve pending calls due to: {}", msg);
							return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					};
					if let Ok(vec_of_operations) = pending_author.pending_tops(shard) {
						retrieved_operations.push(vec_of_operations);
					}
				}
				let json_value = RpcReturnValue {
					do_watch: false,
					value: retrieved_operations.encode(),
					status: DirectRequestStatus::Ok,
				};
				Ok(json!(json_value.to_hex()))
			},
			Err(e) => {
				let error_msg: String = format!("Could not retrieve pending calls due to: {}", e);
				Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		}
	});

	let pending_author = top_pool_author;
	io_handler.add_sync_method("author_pendingTrustedCallsFor", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_pendingTrustedCallsFor");
		match params.parse::<(String, String)>() {
			Ok((shard_base58, account_hex)) => {
				let shard = match decode_shard_from_base58(shard_base58.as_str()) {
					Ok(id) => id,
					Err(msg) => {
						let error_msg: String =
							format!("Could not retrieve pending trusted calls due to: {}", msg);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};
				let account = match AccountId::from_hex(account_hex.as_str()) {
					Ok(acc) => acc,
					Err(msg) => {
						let error_msg: String =
							format!("Could not retrieve pending trusted calls due to: {:?}", msg);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};
				let trusted_calls = pending_author.get_pending_trusted_calls_for(shard, &account);
				let json_value = RpcReturnValue {
					do_watch: false,
					value: trusted_calls.encode(),
					status: DirectRequestStatus::Ok,
				};
				Ok(json!(json_value.to_hex()))
			},
			Err(e) => {
				let error_msg: String =
					format!("Could not retrieve pending trusted calls due to: {}", e);
				Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		}
	});

	io_handler
}

// converts the rpc methods vector to a string and adds commas and brackets for readability
pub fn decode_shard_from_base58(shard_base58: &str) -> Result<ShardIdentifier, String> {
	let shard_vec = match shard_base58.from_base58() {
		Ok(vec) => vec,
		Err(_) => return Err("Invalid base58 format of shard id".to_owned()),
	};
	let shard = match ShardIdentifier::decode(&mut shard_vec.as_slice()) {
		Ok(hash) => hash,
		Err(_) => return Err("Shard ID is not of type H256".to_owned()),
	};
	Ok(shard)
}

fn compute_hex_encoded_return_error(error_msg: &str) -> String {
	RpcReturnValue::from_error_message(error_msg).to_hex()
}

// we expect our `params` to be "by-position array"
// see https://www.jsonrpc.org/specification#parameter_structures
fn get_request_payload(params: Params) -> Result<String, String> {
	let s_vec = params.parse::<Vec<String>>().map_err(|e| format!("{}", e))?;

	let s = s_vec.get(0).ok_or_else(|| "Empty params".to_string())?;
	debug!("Request payload: {}", s);
	Ok(s.to_owned())
}

fn author_submit_extrinsic_inner<R, TCS, G>(
	author: Arc<R>,
	params: Params,
	json_rpc_method: Option<String>,
) -> Result<Hash, String>
where
	R: AuthorApi<Hash, Hash, TCS, G> + Send + Sync + 'static,
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
	G: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
{
	let payload = get_request_payload(params)?;
	let request = RsaRequest::from_hex(&payload).map_err(|e| format!("{:?}", e))?;

	let response: Result<Hash, RpcError> = if let Some(method) = json_rpc_method {
		executor::block_on(async { author.watch_and_broadcast_top(request, method).await })
	} else {
		executor::block_on(async { author.watch_top(request).await })
	};

	match &response {
		Ok(h) => debug!("Trusted operation submitted successfully ({:?})", h),
		Err(e) => warn!("Submitting trusted operation failed: {:?}", e),
	}

	response.map_err(|e| format!("{:?}", e))
}

fn author_submit_aes_request_inner<R, TCS, G>(
	author: Arc<R>,
	params: Params,
	json_rpc_method: Option<String>,
) -> Result<Hash, String>
where
	R: AuthorApi<Hash, Hash, TCS, G> + Send + Sync + 'static,
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
	G: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
{
	let payload = get_request_payload(params)?;
	let request = AesRequest::from_hex(&payload).map_err(|e| format!("{:?}", e))?;

	let response: Result<Hash, RpcError> = if let Some(method) = json_rpc_method {
		executor::block_on(async { author.watch_and_broadcast_top(request, method).await })
	} else {
		executor::block_on(async { author.watch_top(request).await })
	};

	match &response {
		Ok(h) => debug!("AesRequest submitted successfully ({:?})", h),
		Err(e) => warn!("Submitting AesRequest failed: {:?}", e),
	}

	response.map_err(|e| format!("{:?}", e))
}
