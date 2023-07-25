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

use codec::Encode;
use core::result::Result;
use ita_sgx_runtime::{Runtime, System};
use itp_primitives_cache::{GetPrimitives, GLOBAL_PRIMITIVES_CACHE};
use itp_rpc::RpcReturnValue;
use itp_sgx_crypto::Rsa3072Seal;
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::getter_executor::ExecuteGetter;
use itp_stf_primitives::types::AccountId;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	DirectRequestStatus, Index, MrEnclave, Request, ShardIdentifier, SidechainBlockNumber, H256,
};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use its_primitives::types::block::SignedBlock;
use its_sidechain::rpc_handler::{
	direct_top_pool_api, direct_top_pool_api::decode_shard_from_base58, import_block_api,
};
use jsonrpc_core::{serde_json::json, IoHandler, Params, Value};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use log::debug;
use std::{borrow::ToOwned, format, str, string::String, sync::Arc, vec::Vec};

fn compute_hex_encoded_return_error(error_msg: &str) -> String {
	RpcReturnValue::from_error_message(error_msg).to_hex()
}

fn get_all_rpc_methods_string(io_handler: &IoHandler) -> String {
	let method_string = io_handler
		.iter()
		.map(|rp_tuple| rp_tuple.0.to_owned())
		.collect::<Vec<String>>()
		.join(", ");

	format!("methods: [{}]", method_string)
}

pub fn public_api_rpc_handler<R, G, S>(
	top_pool_author: Arc<R>,
	getter_executor: Arc<G>,
	state: Option<Arc<S>>,
) -> IoHandler
where
	R: AuthorApi<H256, H256> + Send + Sync + 'static,
	G: ExecuteGetter + Send + Sync + 'static,
	S: HandleState + Send + Sync + 'static,
	S::StateT: SgxExternalitiesTrait,
{
	let io = IoHandler::new();
	let pool_author = top_pool_author.clone();

	// Add direct TOP pool rpc methods
	let mut io = direct_top_pool_api::add_top_pool_direct_rpc_methods(top_pool_author, io);

	// author_getShieldingKey
	let rsa_pubkey_name: &str = "author_getShieldingKey";
	io.add_sync_method(rsa_pubkey_name, move |_: Params| {
		let rsa_pubkey = match Rsa3072Seal::unseal_pubkey() {
			Ok(key) => key,
			Err(status) => {
				let error_msg: String = format!("Could not get rsa pubkey due to: {}", status);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		};

		let rsa_pubkey_json = match serde_json::to_string(&rsa_pubkey) {
			Ok(k) => k,
			Err(x) => {
				let error_msg: String =
					format!("[Enclave] can't serialize rsa_pubkey {:?} {}", rsa_pubkey, x);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		};
		let json_value =
			RpcReturnValue::new(rsa_pubkey_json.encode(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	// author_getNextNonce
	let state_storage = state.clone();

	let author_get_next_nonce: &str = "author_getNextNonce";
	io.add_sync_method(author_get_next_nonce, move |params: Params| {
		let state_nonce = state.clone();
		if state_nonce.is_none() {
			return Ok(json!(compute_hex_encoded_return_error(
				"author_getNextNonce is not avaiable"
			)))
		}
		#[allow(clippy::unwrap_used)]
		let state_nonce_unwrap = state_nonce.unwrap();
		match params.parse::<(String, String)>() {
			Ok((shard_base58, account_hex)) => {
				let shard = match decode_shard_from_base58(shard_base58.as_str()) {
					Ok(id) => id,
					Err(msg) => {
						let error_msg: String =
							format!("Could not retrieve author_getNextNonce calls due to: {}", msg);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};
				let account = match AccountId::from_hex(account_hex.as_str()) {
					Ok(acc) => acc,
					Err(msg) => {
						let error_msg: String =
							format!("Could not retrieve author_getNextNonce calls due to: {}", msg);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};

				match state_nonce_unwrap.load_cloned(&shard) {
					Ok((mut state, _hash)) => {
						let trusted_calls =
							pool_author.get_pending_trusted_calls_for(shard, &account);
						let pending_tx_count = trusted_calls.len();
						#[allow(clippy::unwrap_used)]
						let pending_tx_count = Index::try_from(pending_tx_count).unwrap();
						let nonce = state.execute_with(|| System::account_nonce(&account));
						let json_value = RpcReturnValue {
							do_watch: false,
							value: (nonce.saturating_add(pending_tx_count)).encode(),
							status: DirectRequestStatus::Ok,
						};
						Ok(json!(json_value.to_hex()))
					},
					Err(e) => {
						let error_msg = format!("load shard failure due to: {:?}", e);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				}
			},
			Err(e) => {
				let error_msg: String =
					format!("Could not retrieve author_getNextNonce calls due to: {}", e);
				Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		}
	});

	let mu_ra_url_name: &str = "author_getMuRaUrl";
	io.add_sync_method(mu_ra_url_name, move |_: Params| {
		let url = match GLOBAL_PRIMITIVES_CACHE.get_mu_ra_url() {
			Ok(url) => url,
			Err(status) => {
				let error_msg: String = format!("Could not get mu ra url due to: {}", status);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		};

		let json_value = RpcReturnValue::new(url.encode(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	let untrusted_url_name: &str = "author_getUntrustedUrl";
	io.add_sync_method(untrusted_url_name, move |_: Params| {
		let url = match GLOBAL_PRIMITIVES_CACHE.get_untrusted_worker_url() {
			Ok(url) => url,
			Err(status) => {
				let error_msg: String = format!("Could not get untrusted url due to: {}", status);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		};

		let json_value = RpcReturnValue::new(url.encode(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	// chain_subscribeAllHeads
	let chain_subscribe_all_heads_name: &str = "chain_subscribeAllHeads";
	io.add_sync_method(chain_subscribe_all_heads_name, |_: Params| {
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// state_getMetadata
	let state_get_metadata_name: &str = "state_getMetadata";
	io.add_sync_method(state_get_metadata_name, |_: Params| {
		let metadata = Runtime::metadata();
		let json_value = RpcReturnValue::new(metadata.into(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	// state_getRuntimeVersion
	let state_get_runtime_version_name: &str = "state_getRuntimeVersion";
	io.add_sync_method(state_get_runtime_version_name, |_: Params| {
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// state_executeGetter
	let state_execute_getter_name: &str = "state_executeGetter";
	io.add_sync_method(state_execute_getter_name, move |params: Params| {
		let json_value = match execute_getter_inner(getter_executor.as_ref(), params) {
			Ok(state_getter_value) => RpcReturnValue {
				do_watch: false,
				value: state_getter_value.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};
		Ok(json!(json_value))
	});

	// state_getMrenclave
	let state_get_mrenclave_name: &str = "state_getMrenclave";
	io.add_sync_method(state_get_mrenclave_name, |_: Params| {
		let json_value = match GLOBAL_SCHEDULED_ENCLAVE.get_current_mrenclave() {
			Ok(mrenclave) => RpcReturnValue {
				do_watch: false,
				value: mrenclave.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex(),
			Err(error) => {
				let error_msg: String =
					format!("Could not get current mrenclave due to: {}", error);
				compute_hex_encoded_return_error(error_msg.as_str())
			},
		};
		Ok(json!(json_value))
	});

	if cfg!(not(feature = "production")) {
		// state_updateScheduledEnclave
		// params: sidechainBlockNumber, hex encoded mrenclave
		let mrenclave_update_scheduled_name: &str = "state_updateScheduledEnclave";
		io.add_sync_method(mrenclave_update_scheduled_name, move |params: Params| {
			match params.parse::<(SidechainBlockNumber, String)>() {
				Ok((bn, mrenclave)) =>
					return match hex::decode(&mrenclave) {
						Ok(mrenclave) => {
							let mut enclave_to_set: MrEnclave = [0u8; 32];
							if mrenclave.len() != enclave_to_set.len() {
								return Ok(json!(compute_hex_encoded_return_error(
									"mrenclave len mismatch, expected 32 bytes long"
								)))
							}

							enclave_to_set.copy_from_slice(&mrenclave);
							return match GLOBAL_SCHEDULED_ENCLAVE.update(bn, enclave_to_set) {
								Ok(()) => Ok(json!(RpcReturnValue::new(
									vec![],
									false,
									DirectRequestStatus::Ok
								)
								.to_hex())),
								Err(e) => {
									let error_msg =
										format!("Failed to set scheduled mrenclave {:?}", e);
									Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
								},
							}
						},
						Err(e) => {
							let error_msg = format!("Failed to decode mrenclave {:?}", e);
							Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					},
				Err(_) => Ok(json!(compute_hex_encoded_return_error("parse error"))),
			}
		});

		// state_getStorage
		let state_get_storage = "state_getStorage";
		io.add_sync_method(state_get_storage, move |params: Params| {
			if state_storage.is_none() {
				return Ok(json!(compute_hex_encoded_return_error(
					"state_getStorage is not avaiable"
				)))
			}

			#[allow(clippy::unwrap_used)]
			let state_storage = state_storage.clone().unwrap();
			match params.parse::<(String, String)>() {
				Ok((shard_str, key_hash)) => {
					let key_hash = if key_hash.starts_with("0x") {
						#[allow(clippy::unwrap_used)]
						key_hash.strip_prefix("0x").unwrap()
					} else {
						key_hash.as_str()
					};
					let key_hash = match hex::decode(key_hash) {
						Ok(key_hash) => key_hash,
						Err(_) =>
							return Ok(json!(compute_hex_encoded_return_error("docode key error"))),
					};

					let shard: ShardIdentifier = match decode_shard_from_base58(shard_str.as_str())
					{
						Ok(id) => id,
						Err(msg) => {
							let error_msg = format!("decode shard failure due to: {}", msg);
							return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					};
					match state_storage.load_cloned(&shard) {
						Ok((state_storage, _hash)) => {
							// Get storage by key hash
							let value =
								state_storage.get(key_hash.as_slice()).cloned().unwrap_or_default();
							debug!("query storage value:{:?}", &value);
							let json_value =
								RpcReturnValue::new(value, false, DirectRequestStatus::Ok);
							Ok(json!(json_value.to_hex()))
						},
						Err(e) => {
							let error_msg = format!("load shard failure due to: {:?}", e);
							return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					}
				},
				Err(_err) => Ok(json!(compute_hex_encoded_return_error("parse error"))),
			}
		});
	}

	// system_health
	let state_health_name: &str = "system_health";
	io.add_sync_method(state_health_name, |_: Params| {
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// system_name
	let state_name_name: &str = "system_name";
	io.add_sync_method(state_name_name, |_: Params| {
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// system_version
	let state_version_name: &str = "system_version";
	io.add_sync_method(state_version_name, |_: Params| {
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// returns all rpcs methods
	let rpc_methods_string = get_all_rpc_methods_string(&io);
	io.add_sync_method("rpc_methods", move |_: Params| {
		Ok(Value::String(rpc_methods_string.to_owned()))
	});

	io
}

fn execute_getter_inner<G: ExecuteGetter>(
	getter_executor: &G,
	params: Params,
) -> Result<Option<Vec<u8>>, String> {
	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	let param = &hex_encoded_params.get(0).ok_or("Could not get first param")?;
	let request = Request::from_hex(param).map_err(|e| format!("{:?}", e))?;

	let shard: ShardIdentifier = request.shard;
	let encoded_trusted_getter: Vec<u8> = request.cyphertext;

	let getter_result = getter_executor
		.execute_getter(&shard, encoded_trusted_getter)
		.map_err(|e| format!("{:?}", e))?;

	Ok(getter_result)
}

pub fn sidechain_io_handler<ImportFn, Error>(import_fn: ImportFn) -> IoHandler
where
	ImportFn: Fn(SignedBlock) -> Result<(), Error> + Sync + Send + 'static,
	Error: std::fmt::Debug,
{
	let io = IoHandler::new();
	import_block_api::add_import_block_rpc_method(import_fn, io)
}

#[cfg(feature = "test")]
pub mod tests {
	use super::*;
	use std::string::ToString;

	pub fn test_given_io_handler_methods_then_retrieve_all_names_as_string() {
		let mut io = IoHandler::new();
		let method_names: [&str; 4] = ["method1", "another_method", "fancy_thing", "solve_all"];

		for method_name in method_names.iter() {
			io.add_sync_method(method_name, |_: Params| Ok(Value::String("".to_string())));
		}

		let method_string = get_all_rpc_methods_string(&io);

		for method_name in method_names.iter() {
			assert!(method_string.contains(method_name));
		}
	}
}
