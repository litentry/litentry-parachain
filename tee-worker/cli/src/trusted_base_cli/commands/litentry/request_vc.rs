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
	get_layer_two_nonce,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{Index, TrustedCall, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::{Assertion, Identity, ParameterString};
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct RequestVcCommand {
	/// did account to whom the vc will be issued
	did: String,
	/// subcommand to define the vc type requested
	#[clap(subcommand)]
	command: Command,
}

// see `assertion.rs`
#[derive(Subcommand)]
pub enum Command {
	A1,
	A2(A2Arg),
}

#[derive(Args)]
pub struct A2Arg {
	pub guild_id: String,
}

impl RequestVcCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice");
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();

		let (mrenclave, shard) = get_identifiers(trusted_cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);

		let assertion = match &self.command {
			Command::A1 => Assertion::A1,
			Command::A2(s) =>
				Assertion::A2(ParameterString::truncate_from(s.guild_id.clone().into_bytes())),
		};

		// let top: TrustedOperation =
		// 	TrustedCall::set_user_shielding_key(alice.public().into(), id, key, Default::default())
		// 		.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
		// 		.into_trusted_operation(trusted_cli.direct);
		// Ok(perform_trusted_operation(cli, trusted_cli, &top).map(|_| CliResultOk::None)?)

		Ok(CliResultOk::None)
	}
}
