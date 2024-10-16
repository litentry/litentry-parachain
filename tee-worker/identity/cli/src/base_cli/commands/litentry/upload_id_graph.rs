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

use crate::{command_utils::get_worker_api_direct, Cli, CliResult, CliResultOk};
use base58::ToBase58;
use codec::{Decode, Encode};
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{Id, RpcRequest};
use itp_types::ShardIdentifier;

// usage: ./bin/litentry-cli upload-id-graph

#[derive(Parser)]
pub struct UploadIdGraphCommand;
impl UploadIdGraphCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let direct_api = get_worker_api_direct(cli);
		let mrenclave = direct_api.get_state_mrenclave().unwrap();
		let shard = ShardIdentifier::decode(&mut &mrenclave[..]).unwrap();
		let jsonrpc_call: String = RpcRequest::compose_jsonrpc_call(
			Id::Text("1".to_string()),
			"omni_UploadIDGraph".to_owned(),
			vec![shard.encode().to_base58()],
		)
		.unwrap();

		let _ = direct_api.get(&jsonrpc_call).unwrap();
		Ok(CliResultOk::None)
	}
}
