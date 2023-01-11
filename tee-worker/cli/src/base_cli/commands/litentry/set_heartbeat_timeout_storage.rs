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
	command_utils::{get_chain_api, *},
	Cli,
};

use itp_node_api::api_client::TEEREX;
use log::*;
use sp_core::sr25519 as sr25519_core;
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus};

#[derive(Parser)]
pub struct SetHeartbeatTimeoutStorageCommand {
	/// Sender's parentchain AccountId in ss58check format
	account: String,

	/// Heartbeat timeout
	timeout: u64,
}

impl SetHeartbeatTimeoutStorageCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		let chain_api = get_chain_api(cli);

		// get the sender
		let from = get_pair_from_str(&self.account);
		let chain_api = chain_api.set_signer(sr25519_core::Pair::from(from));

		// compose the extrinsic
		let xt: UncheckedExtrinsicV4<_, _> =
			compose_extrinsic!(chain_api, TEEREX, "set_heartbeat_timeout_storage", self.timeout);

		let tx_hash = chain_api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();
		println!("[+] TrustedOperation got finalized. Hash: {:?}\n", tx_hash);
	}
}
