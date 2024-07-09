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

use crate::{
	attestation::{
		generate_dcap_ra_extrinsic_from_quote_internal,
		generate_ias_ra_extrinsic_from_der_cert_internal,
	},
	utils::get_validator_accessor_from_integritee_solo_or_parachain,
};
use codec::Encode;
use core::result::Result;
use ita_sgx_runtime::{Runtime, System};
use ita_stf::{aes_encrypt_default, AesOutput, Getter, TrustedCallSigned};
use itc_parentchain::light_client::{concurrent_access::ValidatorAccess, ExtrinsicSender};
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_primitives_cache::{GetPrimitives, GLOBAL_PRIMITIVES_CACHE};
use itp_rpc::RpcReturnValue;
use itp_sgx_crypto::{
	ed25519_derivation::DeriveEd25519,
	key_repository::{AccessKey, AccessPubkey},
	ShieldingCryptoDecrypt, ShieldingCryptoEncrypt,
};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::getter_executor::ExecuteGetter;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{DirectRequestStatus, Index, RsaRequest, ShardIdentifier, H256};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use its_primitives::types::block::SignedBlock;
use its_sidechain::rpc_handler::{
	direct_top_pool_api, direct_top_pool_api::decode_shard_from_base58, import_block_api,
};
use jsonrpc_core::{serde_json::json, IoHandler, Params, Value};
use lc_data_providers::DataProviderConfig;
use lc_identity_verification::web2::twitter;
use litentry_macros::{if_development, if_development_or};
use litentry_primitives::{aes_decrypt, AesRequest, DecryptableRequest, Identity};
use log::debug;
use sgx_crypto::rsa::Rsa3072PublicKey;
use sp_core::Pair;
use sp_runtime::OpaqueExtrinsic;
use std::{borrow::ToOwned, boxed::Box, format, str, string::String, sync::Arc, vec::Vec};

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

pub fn public_api_rpc_handler<Author, GetterExecutor, AccessShieldingKey, OcallApi, State>(
	top_pool_author: Arc<Author>,
	getter_executor: Arc<GetterExecutor>,
	shielding_key: Arc<AccessShieldingKey>,
	ocall_api: Arc<OcallApi>,
	state: Option<Arc<State>>,
	data_provider_config: Arc<DataProviderConfig>,
) -> IoHandler
where
	Author: AuthorApi<H256, H256, TrustedCallSigned, Getter> + Send + Sync + 'static,
	GetterExecutor: ExecuteGetter + Send + Sync + 'static,
	AccessShieldingKey:
		AccessPubkey<KeyType = Rsa3072PublicKey> + AccessKey + Send + Sync + 'static,
	<AccessShieldingKey as AccessKey>::KeyType:
		ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + DeriveEd25519 + Send + Sync + 'static,
	OcallApi: EnclaveAttestationOCallApi + Send + Sync + 'static,
	State: HandleState + Send + Sync + 'static,
	State::StateT: SgxExternalitiesTrait,
{
	let mut io = direct_top_pool_api::add_top_pool_direct_rpc_methods(
		top_pool_author.clone(),
		IoHandler::new(),
	);

	let shielding_key_cloned = shielding_key.clone();
	io.add_sync_method("author_getShieldingKey", move |_: Params| {
		debug!("worker_api_direct rpc was called: author_getShieldingKey");
		let rsa_pubkey = match shielding_key_cloned.retrieve_pubkey() {
			Ok(key) => key,
			Err(status) => {
				let error_msg: String = format!("Could not get rsa pubkey due to: {}", status);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
		};

		let rsa_pubkey_json = match sgx_serialize::json::encode(&rsa_pubkey) {
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

	// author_getEnclaveSignerAccount
	let rsa_pubkey_name: &str = "author_getEnclaveSignerAccount";
	let shielding_key_cloned = shielding_key.clone();
	io.add_sync_method(rsa_pubkey_name, move |_: Params| {
		let enclave_signer_public_key = match shielding_key_cloned
			.retrieve_key()
			.and_then(|keypair| keypair.derive_ed25519().map(|keypair| keypair.public().to_hex()))
		{
			Err(e) => {
				let error_msg: String = format!("{:?}", e);
				return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
			},
			Ok(public_key) => public_key,
		};
		debug!("[Enclave] enclave_signer_public_key: {:?}", enclave_signer_public_key);

		let json_value = RpcReturnValue {
			do_watch: false,
			value: enclave_signer_public_key.encode(),
			status: DirectRequestStatus::Ok,
		};

		Ok(json!(json_value.to_hex()))
	});

	let local_top_pool_author = top_pool_author.clone();

	let local_state = if_development_or!(state.clone(), state);

	io.add_sync_method("author_getNextNonce", move |params: Params| {
		debug!("worker_api_direct rpc was called: author_getNextNonce");
		let local_state = match local_state.clone() {
			Some(s) => s,
			None =>
				return Ok(json!(compute_hex_encoded_return_error(
					"author_getNextNonce is not avaiable"
				))),
		};

		match params.parse::<(String, String)>() {
			Ok((shard_base58, identity_hex)) => {
				let shard = match decode_shard_from_base58(shard_base58.as_str()) {
					Ok(id) => id,
					Err(msg) => {
						let error_msg: String =
							format!("Could not retrieve author_getNextNonce calls due to: {}", msg);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};

				let account_id = match Identity::from_hex(identity_hex.as_str()) {
					Ok(identity) =>
						if let Some(account_id) = identity.to_account_id() {
							account_id
						} else {
							return Ok(json!(compute_hex_encoded_return_error(
								"Could not retrieve author_getNextNonce calls due to: invalid identity"
							)))
						},
					Err(msg) => {
						let error_msg: String = format!(
							"Could not retrieve author_getNextNonce calls due to: {:?}",
							msg
						);
						return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
					},
				};

				match local_state.load_cloned(&shard) {
					Ok((mut state, _hash)) => {
						let trusted_calls =
							local_top_pool_author.get_pending_trusted_calls_for(shard, &account_id);
						let pending_tx_count = trusted_calls.len();
						#[allow(clippy::unwrap_used)]
						let pending_tx_count = Index::try_from(pending_tx_count).unwrap();
						let nonce = state.execute_with(|| System::account_nonce(&account_id));
						let json_value = RpcReturnValue {
							do_watch: false,
							value: (nonce.saturating_add(pending_tx_count)).encode(),
							status: DirectRequestStatus::Ok,
						};
						Ok(json!(json_value.to_hex()))
					},
					Err(e) => {
						let error_msg = format!("load shard failure due to: {:?}", e);
						Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
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

	io.add_sync_method("author_getShard", move |_: Params| {
		debug!("worker_api_direct rpc was called: author_getShard");
		let shard = top_pool_author.list_handled_shards().first().copied().unwrap_or_default();
		let json_value = RpcReturnValue::new(shard.encode(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	io.add_sync_method("author_getMuRaUrl", move |_: Params| {
		debug!("worker_api_direct rpc was called: author_getMuRaUrl");
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

	io.add_sync_method("author_getUntrustedUrl", move |_: Params| {
		debug!("worker_api_direct rpc was called: author_getUntrustedUrl");
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

	io.add_sync_method("chain_subscribeAllHeads", |_: Params| {
		debug!("worker_api_direct rpc was called: chain_subscribeAllHeads");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	io.add_sync_method("state_getMetadata", |_: Params| {
		debug!("worker_api_direct rpc was called: state_getMetadata");
		let metadata = Runtime::metadata();
		let json_value = RpcReturnValue::new(metadata.into(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	io.add_sync_method("state_getRuntimeVersion", |_: Params| {
		debug!("worker_api_direct rpc was called: state_getRuntimeVersion");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	// TODO: deprecate
	let getter_executor_cloned = getter_executor.clone();
	io.add_sync_method("state_executeGetter", move |params: Params| {
		debug!("worker_api_direct rpc was called: state_executeGetter");
		#[allow(deprecated)]
		let json_value = match execute_rsa_getter_inner(getter_executor_cloned.as_ref(), params) {
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

	io.add_sync_method("state_executeAesGetter", move |params: Params| {
		debug!("worker_api_direct rpc was called: state_executeAesGetter");

		let shielding_key = match shielding_key.retrieve_key().map_err(|e| format!("{:?}", e)) {
			Ok(key) => key,
			Err(e) => return Ok(json!(compute_hex_encoded_return_error(&e))),
		};

		let return_value: Result<AesOutput, String> = (|| {
			let hex_encoded_params =
				params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;
			let param = &hex_encoded_params.first().ok_or("Could not get first param")?;
			let mut request = AesRequest::from_hex(param).map_err(|e| format!("{:?}", e))?;

			let aes_key = request
				.decrypt_aes_key(Box::new(shielding_key))
				.map_err(|_err: ()| "Could not decrypt request AES key")?;

			let encoded_trusted_getter = aes_decrypt(&aes_key, &mut request.payload)
				.ok_or(())
				.map_err(|_err: ()| "Could not decrypt getter request")?;

			let shard = request.shard();

			let state_getter_value = getter_executor
				.execute_getter(&shard, encoded_trusted_getter)
				.map_err(|e| format!("{:?}", e))?;

			Ok(aes_encrypt_default(&aes_key, state_getter_value.encode().as_slice()))
		})();

		match return_value {
			Ok(aes_output) => Ok(json!(RpcReturnValue {
				do_watch: false,
				value: aes_output.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex())),
			// FIXME: error not encrypted :sadpanda:
			Err(error) => Ok(json!(compute_hex_encoded_return_error(error.as_str()))),
		}
	});

	io.add_sync_method("attesteer_forwardDcapQuote", move |params: Params| {
		debug!("worker_api_direct rpc was called: attesteer_forwardDcapQuote");
		let json_value = match forward_dcap_quote_inner(params) {
			Ok(val) => RpcReturnValue {
				do_watch: false,
				value: val.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};

		Ok(json!(json_value))
	});

	io.add_sync_method("attesteer_forwardIasAttestationReport", move |params: Params| {
		debug!("worker_api_direct rpc was called: attesteer_forwardIasAttestationReport");
		let json_value = match attesteer_forward_ias_attestation_report_inner(params) {
			Ok(val) => RpcReturnValue {
				do_watch: false,
				value: val.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex(),
			Err(error) => compute_hex_encoded_return_error(error.as_str()),
		};
		Ok(json!(json_value))
	});

	// state_getMrenclave
	io.add_sync_method("state_getMrenclave", move |_: Params| {
		let json_value = match ocall_api.get_mrenclave_of_self() {
			Ok(m) => RpcReturnValue {
				do_watch: false,
				value: m.m.encode(),
				status: DirectRequestStatus::Ok,
			}
			.to_hex(),
			Err(e) => {
				let error_msg: String = format!("Could not get current mrenclave due to: {}", e);
				compute_hex_encoded_return_error(error_msg.as_str())
			},
		};
		Ok(json!(json_value))
	});

	if_development!({
		// state_getStorage
		io.add_sync_method("state_getStorage", move |params: Params| {
			let local_state = match state.clone() {
				Some(s) => s,
				None =>
					return Ok(json!(compute_hex_encoded_return_error(
						"state_getStorage is not avaiable"
					))),
			};
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
							return Ok(json!(compute_hex_encoded_return_error("decode key error"))),
					};

					let shard: ShardIdentifier = match decode_shard_from_base58(shard_str.as_str())
					{
						Ok(id) => id,
						Err(msg) => {
							let error_msg = format!("decode shard failure due to: {}", msg);
							return Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					};
					match local_state.load_cloned(&shard) {
						Ok((state, _)) => {
							// Get storage by key hash
							let value = state.get(key_hash.as_slice()).cloned().unwrap_or_default();
							debug!("query storage value:{:?}", &value);
							let json_value =
								RpcReturnValue::new(value, false, DirectRequestStatus::Ok);
							Ok(json!(json_value.to_hex()))
						},
						Err(e) => {
							let error_msg = format!("load shard failure due to: {:?}", e);
							Ok(json!(compute_hex_encoded_return_error(error_msg.as_str())))
						},
					}
				},
				Err(_err) => Ok(json!(compute_hex_encoded_return_error("parse error"))),
			}
		});
	});

	// system_health
	io.add_sync_method("system_health", |_: Params| {
		debug!("worker_api_direct rpc was called: system_health");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	io.add_sync_method("system_name", |_: Params| {
		debug!("worker_api_direct rpc was called: system_name");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	io.add_sync_method("system_version", |_: Params| {
		debug!("worker_api_direct rpc was called: system_version");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	io.add_sync_method("identity_getTwitterAuthorizeUrl", move |params: Params| {
		debug!("worker_api_direct rpc was called: identity_getTwitterAuthorizeUrl");

		match params.parse::<(String, String)>() {
			Ok((encoded_did, redirect_url)) => {
				let account_id = match Identity::from_did(encoded_did.as_str()) {
					Ok(identity) =>
						if let Some(account_id) = identity.to_account_id() {
							account_id
						} else {
							return Ok(json!(compute_hex_encoded_return_error("Invalid identity")))
						},
					Err(_) =>
						return Ok(json!(compute_hex_encoded_return_error(
							"Could not parse identity"
						))),
				};
				let authorize_data = twitter::get_authorize_data(
					&data_provider_config.twitter_client_id,
					&redirect_url,
				);
				match twitter::OAuthStore::save_data(
					account_id,
					authorize_data.code_verifier,
					authorize_data.state,
				) {
					Ok(_) => {
						let json_value = RpcReturnValue::new(
							authorize_data.authorize_url.encode(),
							false,
							DirectRequestStatus::Ok,
						);
						Ok(json!(json_value.to_hex()))
					},
					Err(_) =>
						Ok(json!(compute_hex_encoded_return_error("Could not save code verifier"))),
				}
			},

			Err(_) => Ok(json!(compute_hex_encoded_return_error("Could not parse params"))),
		}
	});

	let rpc_methods_string = get_all_rpc_methods_string(&io);
	io.add_sync_method("rpc_methods", move |_: Params| {
		debug!("worker_api_direct rpc was called: rpc_methods");
		Ok(Value::String(rpc_methods_string.to_owned()))
	});

	io
}

#[deprecated(note = "`state_executeAesGetter` should be preferred")]
fn execute_rsa_getter_inner<GE: ExecuteGetter>(
	getter_executor: &GE,
	params: Params,
) -> Result<Option<Vec<u8>>, String> {
	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	let param = &hex_encoded_params.first().ok_or("Could not get first param")?;
	let request = RsaRequest::from_hex(param).map_err(|e| format!("{:?}", e))?;

	let shard: ShardIdentifier = request.shard();
	let encoded_trusted_getter: Vec<u8> = request.payload().to_vec();

	let getter_result = getter_executor
		.execute_getter(&shard, encoded_trusted_getter)
		.map_err(|e| format!("{:?}", e))?;

	Ok(getter_result)
}

fn forward_dcap_quote_inner(params: Params) -> Result<OpaqueExtrinsic, String> {
	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	if hex_encoded_params.len() != 1 {
		return Err(format!(
			"Wrong number of arguments for IAS attestation report forwarding: {}, expected: {}",
			hex_encoded_params.len(),
			1
		))
	}

	let param = &hex_encoded_params.first().ok_or("Could not get first param")?;
	let encoded_quote_to_forward: Vec<u8> =
		litentry_hex_utils::decode_hex(param).map_err(|e| format!("{:?}", e))?;

	let url = String::new();
	let ext = generate_dcap_ra_extrinsic_from_quote_internal(
		url.as_bytes().to_vec(),
		&encoded_quote_to_forward,
	)
	.map_err(|e| format!("{:?}", e))?;

	let validator_access = get_validator_accessor_from_integritee_solo_or_parachain()
		.map_err(|e| format!("{:?}", e))?;
	validator_access
		.execute_mut_on_validator(|v| v.send_extrinsics(vec![ext.clone()]))
		.map_err(|e| format!("{:?}", e))?;

	Ok(ext)
}

fn attesteer_forward_ias_attestation_report_inner(
	params: Params,
) -> Result<OpaqueExtrinsic, String> {
	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	if hex_encoded_params.len() != 1 {
		return Err(format!(
			"Wrong number of arguments for IAS attestation report forwarding: {}, expected: {}",
			hex_encoded_params.len(),
			1
		))
	}

	let param = &hex_encoded_params.first().ok_or("Could not get first param")?;
	let ias_attestation_report =
		litentry_hex_utils::decode_hex(param).map_err(|e| format!("{:?}", e))?;

	let url = String::new();
	let ext = generate_ias_ra_extrinsic_from_der_cert_internal(
		url.as_bytes().to_vec(),
		&ias_attestation_report,
		false,
	)
	.map_err(|e| format!("{:?}", e))?;

	let validator_access = get_validator_accessor_from_integritee_solo_or_parachain()
		.map_err(|e| format!("{:?}", e))?;
	validator_access
		.execute_mut_on_validator(|v| v.send_extrinsics(vec![ext.clone()]))
		.map_err(|e| format!("{:?}", e))?;

	Ok(ext)
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
