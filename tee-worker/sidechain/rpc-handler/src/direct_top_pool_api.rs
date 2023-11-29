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

#[cfg(feature = "std")]
use rust_base58::base58::FromBase58;

#[cfg(feature = "sgx")]
use base58::FromBase58;

#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

use codec::{Decode, Encode};
use futures::{channel::oneshot, FutureExt};
use itp_rpc::RpcReturnValue;
use itp_stf_primitives::types::AccountId;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{DirectRequestStatus, RsaRequest, ShardIdentifier, TrustedOperationStatus};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use jsonrpc_core::{futures::executor, serde_json::json, Error as RpcError, IoHandler, Params};
use lazy_static::lazy_static;
use lc_vc_task_sender::{VCRequest, VcRequestSender};
use litentry_primitives::{AesOutput, AesRequest};
use log::*;
use std::{
	borrow::ToOwned,
	collections::HashMap,
	format,
	string::{String, ToString},
	sync::Arc,
	time::{Duration, Instant},
	vec,
	vec::Vec,
};

type Hash = sp_core::H256;

struct RateLimiter {
	requests: RwLock<HashMap<Vec<u8>, Instant>>,
}

impl RateLimiter {
	fn should_allow(&self, request_key: Vec<u8>) -> bool {
		let requests = self.requests.read().unwrap();
		if let Some(&last_instant) = requests.get(&request_key) {
			if last_instant.elapsed() < Duration::from_secs(300) {
				return false // Reject if within 5 minutes
			}
		}
		drop(requests); // Drop read lock

		let mut requests = self.requests.write().unwrap();
		requests.insert(request_key, Instant::now()); // Update with current Instant
		true
	}
}

// Global instance using lazy_static
lazy_static! {
	static ref GLOBAL_RATE_LIMITER: RateLimiter =
		RateLimiter { requests: RwLock::new(HashMap::new()) };
}

pub fn add_top_pool_direct_rpc_methods<R>(
	top_pool_author: Arc<R>,
	mut io_handler: IoHandler,
) -> IoHandler
where
	R: AuthorApi<Hash, Hash> + Send + Sync + 'static,
{
	// author_submitAndWatchRsaRequest
	let author_submit_and_watch_rsa_request_name: &str = "author_submitAndWatchRsaRequest";
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(author_submit_and_watch_rsa_request_name, move |params: Params| {
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
	let author_submit_and_watch_broadcasted_rsa_request: &str =
		"author_submitAndWatchBroadcastedRsaRequest";
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(
		author_submit_and_watch_broadcasted_rsa_request,
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
	let author_submit_extrinsic_name: &str = "author_submitRsaRequest";
	let submit_author = top_pool_author.clone();
	io_handler.add_sync_method(author_submit_extrinsic_name, move |params: Params| {
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

	let author_submit_vc_request_name: &str = "author_submitVCRequest";
	io_handler.add_method(author_submit_vc_request_name, move |params: Params| {
		async move {
			let hex_encoded_params = params.parse::<Vec<String>>().unwrap();
			let request = AesRequest::from_hex(&hex_encoded_params[0].clone()).unwrap();
			let shard: ShardIdentifier = request.shard;
			let key = request.key;
			let encrypted_trusted_call: AesOutput = request.payload;
			if !GLOBAL_RATE_LIMITER.should_allow(encrypted_trusted_call.encode()) {
				return Ok(json!(compute_hex_encoded_return_error("Request exceeded limit")))
			}
			let request_sender = VcRequestSender::new();
			let (sender, receiver) = oneshot::channel::<Result<Vec<u8>, String>>();
			let vc_request = VCRequest { encrypted_trusted_call, sender, shard, key };
			if let Err(e) = request_sender.send_vc_request(vc_request) {
				return Ok(json!(compute_hex_encoded_return_error(&e)))
			}
			match receiver.await {
				Ok(Ok(response)) => {
					let json_value = RpcReturnValue {
						do_watch: false,
						value: response,
						status: DirectRequestStatus::Ok,
					};
					Ok(json!(json_value.to_hex()))
				},
				Ok(Err(e)) => {
					log::error!("Received error in jsonresponse: {:?} ", e);
					Ok(json!(compute_hex_encoded_return_error(&e)))
				},
				Err(_) => {
					// This case will only happen if the sender has been dropped
					Ok(json!(compute_hex_encoded_return_error("The sender has been dropped")))
				},
			}
		}
		.boxed()
	});
	// Litentry: a morphling of `author_submitAndWatchRsaRequest`
	// a different name is used to highlight the request type
	let author_submit_and_watch_aes_request_name: &str = "author_submitAndWatchAesRequest";
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(author_submit_and_watch_aes_request_name, move |params: Params| {
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

	let author_submit_and_watch_broadcasted_aes_request_name: &str =
		"author_submitAndWatchBroadcastedAesRequest";
	let watch_author = top_pool_author.clone();
	io_handler.add_sync_method(
		author_submit_and_watch_broadcasted_aes_request_name,
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
	let author_pending_extrinsic_name: &str = "author_pendingExtrinsics";
	let pending_author = top_pool_author.clone();
	io_handler.add_sync_method(author_pending_extrinsic_name, move |params: Params| {
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

	// author_pendingTrustedCallsFor
	let author_pending_trusted_calls_for_name: &str = "author_pendingTrustedCallsFor";
	let pending_author = top_pool_author;
	io_handler.add_sync_method(author_pending_trusted_calls_for_name, move |params: Params| {
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
							format!("Could not retrieve pending trusted calls due to: {}", msg);
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

fn author_submit_extrinsic_inner<R: AuthorApi<Hash, Hash> + Send + Sync + 'static>(
	author: Arc<R>,
	params: Params,
	json_rpc_method: Option<String>,
) -> Result<Hash, String> {
	debug!("Author submit and watch trusted operation..");

	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	info!("Got request hex: {:?}", &hex_encoded_params[0]);
	std::println!("Got request hex: {:?}", &hex_encoded_params[0]);

	let request =
		RsaRequest::from_hex(&hex_encoded_params[0].clone()).map_err(|e| format!("{:?}", e))?;

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

fn author_submit_aes_request_inner<R: AuthorApi<Hash, Hash> + Send + Sync + 'static>(
	author: Arc<R>,
	params: Params,
	json_rpc_method: Option<String>,
) -> Result<Hash, String> {
	debug!("Author submit and watch AesRequest..");

	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	info!("Got request hex: {:?}", &hex_encoded_params[0]);
	std::println!("Got request hex: {:?}", &hex_encoded_params[0]);

	let request =
		AesRequest::from_hex(&hex_encoded_params[0].clone()).map_err(|e| format!("{:?}", e))?;

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
