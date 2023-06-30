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
	command_utils::{get_accountid_from_str, get_worker_api_direct},
	trusted_cli::TrustedCli,
	trusted_command_utils::get_identifiers,
	Cli,
};
use codec::Decode;
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{RpcResponse, RpcReturnValue};
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use log::*;

#[derive(Parser)]
pub struct NonceCommand {
	/// AccountId in ss58check format
	account: String,
}

impl NonceCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) {
		let (_mrenclave, shard) = get_identifiers(trusted_cli);
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce_ret =
			worker_api_direct.get_next_nonce(shard, get_accountid_from_str(&self.account));
		info!("nonce_ret {:?} ", nonce_ret);
		let nonce_val = nonce_ret.unwrap();
		info!("nonce_val {:?} ", nonce_val);
		let rpc_response: RpcResponse = serde_json::from_str(&nonce_val).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result).unwrap();
		if rpc_return_value.status == DirectRequestStatus::Error {
			println!("[Error] {}", String::decode(&mut rpc_return_value.value.as_slice()).unwrap());
			worker_api_direct.close().unwrap();
			return
		}

		worker_api_direct.close().unwrap();
		let nonce: u32 = Decode::decode(&mut rpc_return_value.value.as_slice()).unwrap_or_default();

		println!("{}", nonce);
	}
}
