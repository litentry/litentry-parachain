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

use crate::{command_utils::get_worker_api_direct, trusted_cli::TrustedCli, Cli};
use itc_rpc_client::direct_client::DirectApi;

#[derive(Parser)]
pub struct NonceCommand {
	/// AccountId in ss58check format
	account: String,
}

impl NonceCommand {
	pub(crate) fn run(&self, cli: &Cli, _trusted_args: &TrustedCli) {
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce = worker_api_direct.get_next_nonce().unwrap();
		println!("{}", nonce);
	}
}
