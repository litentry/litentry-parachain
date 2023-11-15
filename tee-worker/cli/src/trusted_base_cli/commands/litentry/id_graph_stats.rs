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

use crate::{
	trusted_cli::TrustedCli, trusted_command_utils::get_pair_from_str,
	trusted_operation::perform_trusted_operation, Cli, CliError, CliResult, CliResultOk,
};
use ita_stf::{TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::ParentchainAccountId;
use sp_core::Pair;

type IDGraphStatsVec = Vec<(ParentchainAccountId, u32)>;

#[derive(Parser)]
pub struct IDGraphStats {
	/// AccountId in ss58check format
	account: String,
}

impl IDGraphStats {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let who = get_pair_from_str(trusted_cli, &self.account, cli);
		let top: TrustedOperation = TrustedGetter::id_graph_stats(who.public().into())
			.sign(&KeyPair::Sr25519(Box::new(who)))
			.into();
		let id_graph_stats = perform_trusted_operation::<IDGraphStatsVec>(cli, trusted_cli, &top);
		println!("IDGraph stats:");
		match id_graph_stats {
			Ok(id_graph_stats) => {
				let mut total_number = 0_u32;

				id_graph_stats.iter().for_each(|item| {
					total_number += item.1;

					println!("{:?} -> {}", item.0, item.1);
				});

				println!("Total number: {}", total_number);
				Ok(CliResultOk::None)
			},
			_ => Err(CliError::Extrinsic { msg: "invalid id graph stats".to_string() }),
		}
	}
}
