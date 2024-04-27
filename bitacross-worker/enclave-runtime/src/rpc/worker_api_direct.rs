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
	initialization::global_components::{
		EnclaveBitcoinKeyRepository, EnclaveEthereumKeyRepository, EnclaveSigningKeyRepository,
	},
	std::string::ToString,
	utils::{
		get_stf_enclave_signer_from_solo_or_parachain,
		get_validator_accessor_from_integritee_solo_or_parachain,
	},
};
use bc_musig2_ceremony::{generate_aggregated_public_key, PublicKey};
use bc_signer_registry::SignerRegistryLookup;
use bc_task_sender::{BitAcrossProcessingResult, BitAcrossRequest, BitAcrossRequestSender};
use codec::Encode;
use core::result::Result;
use futures_sgx::channel::oneshot;
use ita_sgx_runtime::Runtime;
use ita_stf::{Getter, TrustedCallSigned};
use itc_parentchain::light_client::{concurrent_access::ValidatorAccess, ExtrinsicSender};
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_primitives_cache::{GetPrimitives, GLOBAL_PRIMITIVES_CACHE};
use itp_rpc::RpcReturnValue;
use itp_sgx_crypto::{
	ed25519_derivation::DeriveEd25519,
	key_repository::{AccessKey, AccessPubkey},
	ShieldingCryptoDecrypt, ShieldingCryptoEncrypt,
};
use itp_stf_executor::{getter_executor::ExecuteGetter, traits::StfShardVaultQuery};
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{DirectRequestStatus, RsaRequest, ShardIdentifier, H256};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use jsonrpc_core::{serde_json::json, IoHandler, Params, Value};
#[cfg(feature = "development")]
use lc_scheduled_enclave::ScheduledEnclaveUpdater;
use lc_scheduled_enclave::GLOBAL_SCHEDULED_ENCLAVE;
use litentry_macros::if_development;
use litentry_primitives::{AesRequest, DecryptableRequest};
use log::debug;
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::crypto::Pair;
use sp_runtime::OpaqueExtrinsic;
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

#[allow(clippy::too_many_arguments)]
pub fn public_api_rpc_handler<Author, GetterExecutor, AccessShieldingKey, OcallApi, SR>(
	top_pool_author: Arc<Author>,
	getter_executor: Arc<GetterExecutor>,
	shielding_key: Arc<AccessShieldingKey>,
	ocall_api: Arc<OcallApi>,
	signing_key_repository: Arc<EnclaveSigningKeyRepository>,
	bitcoin_key_repository: Arc<EnclaveBitcoinKeyRepository>,
	ethereum_key_repository: Arc<EnclaveEthereumKeyRepository>,
	signer_lookup: Arc<SR>,
) -> IoHandler
where
	Author: AuthorApi<H256, H256, TrustedCallSigned, Getter> + Send + Sync + 'static,
	GetterExecutor: ExecuteGetter + Send + Sync + 'static,
	AccessShieldingKey: AccessPubkey<KeyType = Rsa3072PubKey> + AccessKey + Send + Sync + 'static,
	<AccessShieldingKey as AccessKey>::KeyType:
		ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + DeriveEd25519 + Send + Sync + 'static,
	OcallApi: EnclaveAttestationOCallApi + Send + Sync + 'static,
	SR: SignerRegistryLookup + Send + Sync + 'static,
{
	let mut io = IoHandler::new();

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

	// author_getEnclaveSignerAccount
	let rsa_pubkey_name: &str = "author_getEnclaveSignerAccount";
	io.add_sync_method(rsa_pubkey_name, move |_: Params| {
		let enclave_signer_public_key = match shielding_key
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

	// Submit BitAcross Request
	io.add_method("bitacross_submitRequest", move |params: Params| {
		debug!("worker_api_direct rpc was called: bitacross_submitRequest");
		async move {
			let json_value = match request_bit_across_inner(params).await {
				Ok(value) => value.to_hex(),
				Err(error) => RpcReturnValue {
					value: error,
					do_watch: false,
					status: DirectRequestStatus::Error,
				}
				.to_hex(),
			};
			Ok(json!(json_value))
		}
	});

	io.add_sync_method("bitacross_aggregatedPublicKey", move |_: Params| {
		debug!("worker_api_direct rpc was called: bitacross_aggregatedPublicKey");
		if let Ok(keys) = signer_lookup
			.get_all()
			.iter()
			.map(|(_, pub_key)| PublicKey::from_sec1_bytes(pub_key))
			.collect()
		{
			let key_bytes = generate_aggregated_public_key(keys).to_sec1_bytes().to_vec();
			let json_value = RpcReturnValue::new(key_bytes, false, DirectRequestStatus::Ok);
			Ok(json!(json_value.to_hex()))
		} else {
			Ok(json!(compute_hex_encoded_return_error("Could not produce aggregate key")))
		}
	});

	io.add_sync_method("bitacross_getPublicKeys", move |_: Params| {
		debug!("worker_api_direct rpc was called: bitacross_getPublicKeys");

		let signer = match signing_key_repository.retrieve_key() {
			Ok(pair) => pair.public().0.to_hex(),
			Err(_e) => compute_hex_encoded_return_error("Can not obtain signer key"),
		};

		let bitcoin_key = match bitcoin_key_repository.retrieve_key() {
			Ok(pair) => pair.public_bytes().to_hex(),
			Err(_e) => compute_hex_encoded_return_error("Can not obtain bitcoin key"),
		};

		let ethereum_key = match ethereum_key_repository.retrieve_key() {
			Ok(pair) => pair.public_bytes().to_hex(),
			Err(_e) => compute_hex_encoded_return_error("Can not obtain ethereum key"),
		};

		Ok(json!({
			"signer": signer,
			"bitcoin_key": bitcoin_key,
			"ethereum_key": ethereum_key
		}))
	});

	io.add_sync_method("state_getScheduledEnclave", move |_: Params| {
		debug!("worker_api_direct rpc was called: state_getScheduledEnclave");
		let json_value = match GLOBAL_SCHEDULED_ENCLAVE.registry.read() {
			Ok(registry) => {
				let mut serialized_registry = vec![];
				for (block_number, mrenclave) in registry.iter() {
					serialized_registry.push((*block_number, *mrenclave));
				}
				RpcReturnValue::new(serialized_registry.encode(), false, DirectRequestStatus::Ok)
					.to_hex()
			},
			Err(_err) => compute_hex_encoded_return_error("Poisoned registry storage"),
		};
		Ok(json!(json_value))
	});

	let local_top_pool_author = top_pool_author.clone();
	io.add_sync_method("author_getShardVault", move |_: Params| {
		debug!("worker_api_direct rpc was called: author_getShardVault");
		let shard =
			local_top_pool_author.list_handled_shards().first().copied().unwrap_or_default();
		if let Ok(stf_enclave_signer) = get_stf_enclave_signer_from_solo_or_parachain() {
			if let Ok(vault) = stf_enclave_signer.get_shard_vault(&shard) {
				let json_value =
					RpcReturnValue::new(vault.encode(), false, DirectRequestStatus::Ok);
				Ok(json!(json_value.to_hex()))
			} else {
				Ok(json!(compute_hex_encoded_return_error("failed to get shard vault").to_hex()))
			}
		} else {
			Ok(json!(compute_hex_encoded_return_error(
				"failed to get stf_enclave_signer to get shard vault"
			)
			.to_hex()))
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
		debug!("worker_api_direct rpc was called: tate_getMetadata");
		let metadata = Runtime::metadata();
		let json_value = RpcReturnValue::new(metadata.into(), false, DirectRequestStatus::Ok);
		Ok(json!(json_value.to_hex()))
	});

	io.add_sync_method("state_getRuntimeVersion", |_: Params| {
		debug!("worker_api_direct rpc was called: state_getRuntimeVersion");
		let parsed = "world";
		Ok(Value::String(format!("hello, {}", parsed)))
	});

	io.add_sync_method("state_executeGetter", move |params: Params| {
		debug!("worker_api_direct rpc was called: state_executeGetter");
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
		use itp_types::{MrEnclave, SidechainBlockNumber};
		// state_setScheduledEnclave, params: sidechainBlockNumber, hex encoded mrenclave
		io.add_sync_method("state_setScheduledEnclave", move |params: Params| {
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

	let rpc_methods_string = get_all_rpc_methods_string(&io);
	io.add_sync_method("rpc_methods", move |_: Params| {
		debug!("worker_api_direct rpc was called: rpc_methods");
		Ok(Value::String(rpc_methods_string.to_owned()))
	});

	io
}

// Litentry: TODO - we still use `RsaRequest` for trusted getter, as the result
// in unencrypted, see P-183
fn execute_getter_inner<GE: ExecuteGetter>(
	getter_executor: &GE,
	params: Params,
) -> Result<Option<Vec<u8>>, String> {
	let hex_encoded_params = params.parse::<Vec<String>>().map_err(|e| format!("{:?}", e))?;

	let param = &hex_encoded_params.get(0).ok_or("Could not get first param")?;
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

	let param = &hex_encoded_params.get(0).ok_or("Could not get first param")?;
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

	let param = &hex_encoded_params.get(0).ok_or("Could not get first param")?;
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

pub enum BitacrossRequestError {
	DirectCallError(Vec<u8>),
	Other(Vec<u8>),
}

async fn request_bit_across_inner(params: Params) -> Result<RpcReturnValue, Vec<u8>> {
	let payload = get_request_payload(params)?;
	let request = AesRequest::from_hex(&payload)
		.map_err(|e| format!("AesRequest construction error: {:?}", e))?;

	let bit_across_request_sender = BitAcrossRequestSender::new();
	let (sender, receiver) = oneshot::channel::<Result<BitAcrossProcessingResult, Vec<u8>>>();

	bit_across_request_sender.send(BitAcrossRequest { sender, request })?;

	// we only expect one response, hence no loop
	match receiver.await {
		Ok(Ok(response)) => match response {
			BitAcrossProcessingResult::Ok(response_payload) => {
				println!("BitAcrossProcessingResult::Ok");

				Ok(RpcReturnValue {
					do_watch: false,
					value: response_payload,
					status: DirectRequestStatus::Ok,
				})
			},
			BitAcrossProcessingResult::Submitted(hash) => {
				println!("BitAcrossProcessingResult::Submitted");
				Ok(RpcReturnValue {
					do_watch: true,
					value: vec![],
					status: DirectRequestStatus::Processing(hash.into()),
				})
			},
		},
		Ok(Err(e)) => {
			println!("Got Ok(Err)");

			Err(e)
		},
		Err(_) => {
			println!("Got Err");
			// This case will only happen if the sender has been dropped
			Err(vec![])
		},
	}
}

// we expect our `params` to be "by-position array"
// see https://www.jsonrpc.org/specification#parameter_structures
fn get_request_payload(params: Params) -> Result<String, String> {
	let s_vec = params.parse::<Vec<String>>().map_err(|e| format!("{}", e))?;

	let s = s_vec.get(0).ok_or_else(|| "Empty params".to_string())?;
	debug!("Request payload: {}", s);
	Ok(s.to_owned())
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
