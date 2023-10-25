// Copyright 2020-2023 Trust Computing GmbH.
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

use crate::{command_utils::get_chain_api, Cli};

use crate::{CliResult, CliResultOk};
use itp_node_api::api_client::TEEREX;
use log::*;
use sp_keyring::AccountKeyring;
use substrate_api_client::{ac_compose_macros::compose_extrinsic, SubmitAndWatch, XtStatus};
#[derive(Parser)]
pub struct SetHeartbeatTimeoutCommand {
	/// Heartbeat timeout
	timeout: u64,
}

impl SetHeartbeatTimeoutCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let mut chain_api = get_chain_api(cli);

		// has to be //Alice as this is the genesis admin for teerex pallet,
		// otherwise `set_heartbeat_timeout` call won't work
		chain_api.set_signer(AccountKeyring::Alice.pair().into());

		// call set_heartbeat_timeout
		let xt = compose_extrinsic!(
			chain_api,
			TEEREX,
			"set_heartbeat_timeout",
			codec::Compact(self.timeout)
		);
		chain_api.submit_and_watch_extrinsic_until(xt, XtStatus::Finalized).unwrap();

		println!("[+] TrustedOperation got finalized: SetHeartbeatTimeoutCommand");

		Ok(CliResultOk::None)
	}
}
