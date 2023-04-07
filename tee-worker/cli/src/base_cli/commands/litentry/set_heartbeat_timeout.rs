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
	command_utils::{get_accountid_from_str, get_chain_api, get_pair_from_str},
	Cli,
};

use itp_node_api::api_client::TEEREX;
use log::*;
use substrate_api_client::{compose_call, compose_extrinsic, UncheckedExtrinsicV4, XtStatus};

#[derive(Parser)]
pub struct SetHeartbeatTimeoutCommand {
	/// Heartbeat timeout
	timeout: u64,
}

impl SetHeartbeatTimeoutCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		let chain_api = get_chain_api(cli);

		let alice = get_pair_from_str("//Alice");
		let chain_api = chain_api.set_signer(alice.into());

		// set //Alice as enclave admin
		let call = compose_call!(
			chain_api.metadata,
			TEEREX,
			"set_enclave_admin",
			get_accountid_from_str("//Alice")
		);
		let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(chain_api, "Sudo", "sudo", call);
		let _ = chain_api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();

		// call set_heartbeat_timeout
		let xt: UncheckedExtrinsicV4<_, _> =
			compose_extrinsic!(chain_api, TEEREX, "set_heartbeat_timeout", self.timeout);
		let tx_hash = chain_api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();

		println!("[+] TrustedOperation got finalized. Hash: {:?}\n", tx_hash);
	}
}
