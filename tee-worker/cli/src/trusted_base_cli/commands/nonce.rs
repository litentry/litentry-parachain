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
	command_utils::get_worker_api_direct,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	Cli, CliResult, CliResultOk,
};
use itc_rpc_client::direct_client::DirectApi;
use sp_core::Pair;

#[derive(Parser)]
pub struct NonceCommand {
	/// AccountId in ss58check format
	account: String,
}

impl NonceCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) -> CliResult {
		let (_mrenclave, shard) = get_identifiers(trusted_args);
		let who = get_pair_from_str(trusted_args, &self.account);
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce_ret = worker_api_direct.get_next_nonce(&shard, &(who.public().into()));
		let nonce = nonce_ret.expect("get nonce error!");
		println!("{}", nonce);
		worker_api_direct.close().unwrap();
		Ok(CliResultOk::None)
	}
}
