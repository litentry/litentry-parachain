// Copyright 2020-2023 Litentry Technologies GmbH.
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
	command_utils::{get_accountid_from_str, get_worker_api_direct},
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli,
};
use codec::Decode;
use ita_stf::{TrustedCall, TrustedOperation};
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{RpcResponse, RpcReturnValue};
use itp_stf_primitives::types::KeyPair;
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use litentry_primitives::UserShieldingKeyType;
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct SetUserShieldingKeyCommand {
	/// AccountId in ss58check format
	account: String,
	/// Shielding key in hex string
	key_hex: String,
}

impl SetUserShieldingKeyCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) {
		error!("SetUserShieldingKeyCommand account: {}", self.account);
		let who = get_pair_from_str(trusted_cli, &self.account);

		let (mrenclave, shard) = get_identifiers(trusted_cli);
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

		let mut key = UserShieldingKeyType::default();

		hex::decode_to_slice(&self.key_hex, &mut key).expect("decoding shielding_key failed");

		let top: TrustedOperation = TrustedCall::set_user_shielding_key(
			who.public().into(),
			who.public().into(),
			key,
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(who)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);
		perform_trusted_operation(cli, trusted_cli, &top);
	}
}
