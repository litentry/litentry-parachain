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

use crate::{
	command_utils::{get_shielding_key, get_worker_api_direct},
	trusted_cli::TrustedCli,
	trusted_operation::read_shard,
	Cli,
};
use codec::{Decode, Encode, Input};
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{Id, RpcRequest, RpcResponse, RpcReturnValue};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::error::StfError;
use itp_types::{parentchain::Hash, DirectRequestStatus, TrustedOperationStatus};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use lc_direct_call::DirectCallSigned;
use litentry_primitives::{
	aes_encrypt_default, AesRequest, RequestAesKey, ShardIdentifier, REQUEST_AES_KEY_LEN,
};
use log::{debug, error};
use std::sync::mpsc::channel;

pub fn random_aes_key() -> RequestAesKey {
	let random: Vec<u8> = (0..REQUEST_AES_KEY_LEN).map(|_| rand::random::<u8>()).collect();
	random[0..REQUEST_AES_KEY_LEN].try_into().unwrap()
}

pub fn send_direct_request(
	cli: &Cli,
	trusted_args: &TrustedCli,
	call: DirectCallSigned,
	key: RequestAesKey,
) -> Result<String, String> {
	let encryption_key = get_shielding_key(cli).unwrap();
	let shard = read_shard(trusted_args, cli).unwrap();
	let jsonrpc_call: String = get_bitacross_json_request(shard, call, encryption_key, key);
	let direct_api = get_worker_api_direct(cli);
	direct_api.get(&jsonrpc_call).map_err(|e| e.to_string())
}

pub fn send_direct_request_and_watch<T: Decode>(
	cli: &Cli,
	trusted_args: &TrustedCli,
	call: DirectCallSigned,
	key: RequestAesKey,
) -> Result<T, String> {
	let encryption_key = get_shielding_key(cli).unwrap();
	let shard = read_shard(trusted_args, cli).unwrap();
	let jsonrpc_call: String = get_bitacross_json_request(shard, call, encryption_key, key);
	let direct_api = get_worker_api_direct(cli);

	let (sender, receiver) = channel();
	direct_api.watch(jsonrpc_call, sender);

	debug!("waiting for rpc response");
	loop {
		match receiver.recv() {
			Ok(response) => {
				debug!("received response");
				let response: RpcResponse = serde_json::from_str(&response).unwrap();
				if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
					match return_value.status {
						DirectRequestStatus::Error => {
							debug!("request status is error");
							if let Ok(value) = String::decode(&mut return_value.value.as_slice()) {
								error!("{}", value);
							}
							direct_api.close().unwrap();
							return Err("[Error] DirectRequestStatus::Error".to_string())
						},
						DirectRequestStatus::TrustedOperationStatus(status, top_hash) => {
							debug!("request status is: {:?}, top_hash: {:?}", status, top_hash);

							if matches!(status, TrustedOperationStatus::Invalid) {
								let error = StfError::decode(&mut return_value.value.as_slice())
									.map_err(|e| {
										format!("Could not decode error value: {:?}", e)
									})?;
								return Err(format!(
									"[Error] Error occurred while executing trusted call: {:?}",
									error
								))
							}
							if let Ok(value) = Hash::decode(&mut return_value.value.as_slice()) {
								println!("Trusted call {:?} is {:?}", value, status);
							}
							if !return_value.do_watch {
								direct_api.close().unwrap();
								let value =
									decode_response_value(&mut return_value.value.as_slice())?;
								return Ok(value)
							}
						},
						DirectRequestStatus::Processing(_hash) => {
							println!("Request is processing...");
						},
						DirectRequestStatus::Ok => {
							debug!("request status is ignored");
							direct_api.close().unwrap();
							return Err("Unexpected status: DirectRequestStatus::Ok".to_string())
						},
					}
				};
			},
			Err(e) => {
				error!("failed to receive rpc response: {:?}", e);
				direct_api.close().unwrap();
				return Err("failed to receive rpc response".to_string())
			},
		};
	}
}

pub fn get_bitacross_json_request(
	shard: ShardIdentifier,
	call: DirectCallSigned,
	shielding_pubkey: sgx_crypto_helper::rsa3072::Rsa3072PubKey,
	key: RequestAesKey,
) -> String {
	let encrypted_key = shielding_pubkey.encrypt(&key).unwrap();
	let encrypted_top = aes_encrypt_default(&key, &call.encode());

	// compose jsonrpc call
	let request = AesRequest { shard, key: encrypted_key, payload: encrypted_top };
	RpcRequest::compose_jsonrpc_call(
		Id::Number(1),
		"bitacross_submitRequest".to_string(),
		vec![request.to_hex()],
	)
	.unwrap()
}

fn decode_response_value<T: Decode, I: Input>(value: &mut I) -> Result<T, String> {
	T::decode(value).map_err(|e| format!("Could not decode result value: {:?}", e))
}
