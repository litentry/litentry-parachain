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

use codec::{Decode, Encode};
use ita_stf::{Getter, TrustedCall, TrustedCallSigned, TrustedCallVerification, TrustedOperation};
use itp_rpc::RpcReturnValue;
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_primitives::types::AccountId;
use itp_stf_state_handler::handle_state::HandleState;
use itp_storage::{storage_map_key, StorageHasher::Blake2_128Concat};
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	DirectRequestStatus, MrEnclave, RsaRequest, ShardIdentifier, TrustedOperationStatus,
};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use jsonrpc_core::{futures::executor, serde_json::json, Error as RpcError, IoHandler, Params};
use lc_vc_task_sender::{VCRequest, VcRequestSender};
use litentry_primitives::{AesRequest, DecryptableRequest};
use log::*;
use std::{
	borrow::ToOwned,
	boxed::Box,
	format,
	string::{String, ToString},
	sync::{mpsc::channel, Arc},
	vec,
	vec::Vec,
};

type Hash = sp_core::H256;

pub fn add_top_pool_direct_rpc_methods<R, TCS, G, S, K>(
	top_pool_author: Arc<R>,
	mut io_handler: IoHandler,
	mrenclave: Option<MrEnclave>,
	state: Option<Arc<S>>,
	shielding_key_repository: Option<Arc<K>>,
) -> IoHandler
where
	R: AuthorApi<Hash, Hash, TCS, G> + Send + Sync + 'static,
	TCS: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
	G: PartialEq + Encode + Decode + Debug + Send + Sync + 'static,
	S: HandleState + Send + Sync + 'static,
	S::StateT: SgxExternalitiesTrait,
	K: AccessKey + Send + Sync + 'static,
	<K as AccessKey>::KeyType: ShieldingCryptoDecrypt + Send + Sync + 'static,
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

	// TODO: optimise the error handling
	let author_cloned = top_pool_author.clone();
	io_handler.add_sync_method("author_requestVc", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_requestVc");
		let (state, shielding_key_repository) =
			match (state.clone(), shielding_key_repository.clone()) {
				(Some(s), Some(k)) => (s, k),
				_ =>
					return Ok(json!(compute_hex_encoded_return_error(
						"Empty state or shielding_key_repository"
					))),
			};

		// try to decrypt the request
		let payload = match get_request_payload(params.clone()) {
			Ok(v) => v,
			Err(e) =>
				return Ok(json!(compute_hex_encoded_return_error(
					format!("Failed to get request payload: {}", e).as_str()
				))),
		};
		let mut request = match AesRequest::from_hex(&payload) {
			Ok(v) => v,
			Err(e) =>
				return Ok(json!(compute_hex_encoded_return_error(
					format!("Failed to construct AesRequest: {:?}", e).as_str()
				))),
		};
		let tcs = match shielding_key_repository
			.retrieve_key()
			.ok()
			.and_then(|k| request.decrypt(Box::new(k)).ok())
			.and_then(|v| {
				TrustedOperation::<TrustedCallSigned, Getter>::decode(&mut v.as_slice()).ok()
			})
			.and_then(|top| top.to_call().cloned())
		{
			Some(v) => v,
			None =>
				return Ok(json!(compute_hex_encoded_return_error(
					"Failed to decode TrustedCallSigned"
				))),
		};

		let mrenclave = match mrenclave {
			Some(v) => v,
			None => return Ok(json!(compute_hex_encoded_return_error("Invalid mrenclave"))),
		};

		if !tcs.verify_signature(&mrenclave, &request.shard) {
			return Ok(json!(compute_hex_encoded_return_error(
				"Failed to verify trusted call signature"
			)))
		}

		if let TrustedCall::request_vc(signer, who, assertion, maybe_key, req_ext_hash) = tcs.call {
			let id_graph_is_empty = match state.execute_on_current(&request.shard, |s, _| {
				let storage_key =
					storage_map_key("IdentityManagement", "IDGraphLens", &who, &Blake2_128Concat);

				// `None` means empty, thus `unwrap_or_default`
				let id_graph_len = s
					.get(&storage_key)
					.and_then(|v| u32::decode(&mut v.as_slice()).ok())
					.unwrap_or_default();
				id_graph_len == 0
			}) {
				Ok(v) => v,
				Err(e) =>
					return Ok(json!(compute_hex_encoded_return_error(format!("{:?}", e).as_str()))),
			};

			// If id_graph is empty, we delegate the vc handling to the STF version, otherwise we use the threaded version which
			// doesn't need to wait for a sidechain block. The downside of this method is that the first vc_request processing time
			// is limited to the sidechain block interval, but it ensures the correctness.
			//
			// An alternative is to check if an IDGraph **could** be created and then process the VC request right away. The actual
			// creation of IDGraph is done async (by submitting a trusted call to top pool). This should work most of the time, but
			// there's a small chance that some IDGraph mutation was injectd before that (e.g. link_identity) so that creation of IDGraph
			// would fail afterwards. The abortion isn't so critical per se, but `RequestVCResult` will carry with wrong `mutated_id_graph`
			// and `id_graph_hash` which are pre-filled when building VCs.
			//
			// So the current impl weights correctness over performance. If the client doesn't see a problem with the performant variant,
			// we can go for it too.
			//
			// Please note we can't mutate the state inside vc-task-receiver via `load_for_mutation` even
			// though it's lock guarded, because: a) it intereferes with the block import on another thread, which eventually
			// cause state mismatch before/after applying the state diff b) it's not guaranteed to be broadcasted to other workers

			if id_graph_is_empty {
				info!("IDGraph is empty, signer = {}", signer.to_did().unwrap());
				let json_value = match author_submit_aes_request_inner(
					author_cloned.clone(),
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
				return Ok(json!(json_value))
			} else {
				info!("IDGraph is not empty, signer = {}", signer.to_did().unwrap());
				let vc_request_sender = VcRequestSender::new();
				let (sender, receiver) = channel::<Result<Vec<u8>, String>>();
				if let Err(e) = vc_request_sender.send(VCRequest {
					sender,
					shard: request.shard,
					signer,
					who,
					assertion,
					maybe_key,
					req_ext_hash,
				}) {
					return Ok(json!(compute_hex_encoded_return_error(&e)))
				}

				// we only expect one response, hence no loop
				match receiver.recv() {
					Ok(Ok(response)) => {
						let json_value = RpcReturnValue {
							do_watch: false,
							value: response.encode(),
							status: DirectRequestStatus::Ok,
						};
						return Ok(json!(json_value.to_hex()))
					},
					Ok(Err(e)) => {
						log::error!("Received error in jsonresponse: {:?} ", e);
						return Ok(json!(compute_hex_encoded_return_error(&e)))
					},
					Err(_) => {
						// This case will only happen if the sender has been dropped
						return Ok(json!(compute_hex_encoded_return_error(
							"The sender has been dropped"
						)))
					},
				}
			}
		}

		Ok(json!(compute_hex_encoded_return_error("Only request_vc trusted call is allowed")))
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
