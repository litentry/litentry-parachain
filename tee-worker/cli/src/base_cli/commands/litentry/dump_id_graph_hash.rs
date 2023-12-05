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

use crate::{command_utils::get_worker_api_direct, Cli, CliResult, CliResultOk};
use codec::Decode;
use itc_rpc_client::direct_client::DirectApi;
use itp_types::ShardIdentifier;
use itp_utils::ToHexPrefixed;

// usage: ./bin/litentry-cli dump-id-graph-hash
//
// this command prints a list of (AccountId32, H256) collections which represent the IDGraph owner and hash, respectively
// example result:
// ```
// 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, 0xd47cd39a19bc0094a4ec6ebf147cd1bde820e8d07574df02505607ec53c39ceb
// 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, 0x6fd466064ab4aa8eeae19e0ddc25a99fefd3bf326caef98cf0c24b6467805f9d
// ```
// please note the evm address is converted to substrate account to faciliate the population of on-chain storage,
// as the parachain extrinsic `update_id_graph_hash` expects a T::AccountId, same as other extrinsic callbacks

#[derive(Parser)]
pub struct DumpIDGraphHashCommand {}

impl DumpIDGraphHashCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let direct_api = get_worker_api_direct(cli);
		let mrenclave = direct_api.get_state_mrenclave().unwrap();
		let shard = ShardIdentifier::decode(&mut &mrenclave[..]).unwrap();
		let id_graph_hash = direct_api.get_all_id_graph_hash(&shard).unwrap();

		id_graph_hash.iter().for_each(|(identity, hash)| {
			println!("{}, {:?}", identity.to_account_id().unwrap().to_hex(), hash)
		});

		Ok(CliResultOk::None)
	}
}
