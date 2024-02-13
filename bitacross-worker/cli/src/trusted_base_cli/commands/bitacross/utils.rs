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
use codec::Encode;
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{Id, RpcRequest};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_utils::ToHexPrefixed;
use lc_direct_call::DirectCallSigned;
use litentry_primitives::{
	aes_encrypt_default, AesRequest, RequestAesKey, ShardIdentifier, REQUEST_AES_KEY_LEN,
};

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
