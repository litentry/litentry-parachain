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
use clap::Parser;
use ita_stf::{Index, TrustedCall, TrustedCallSigning};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::Identity;
use sp_core::Pair;

// usage exmaple:
//
// # remove `my_twitter` identity from substrate //Bob account
// ./bin/litentry-cli trusted --mrenclave <mrenclave> \
//   --direct remove-identity \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 \
//   did:litentry:twitter:my_twitter
//
// # remove an evm identity from substrate //Bob account
// ./bin/litentry-cli trusted --mrenclave <mrenclave> \
//   --direct remove-identity \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 \
//   did:litentry:evm:0x0D9bFD1f18f5f4FD08247DC54aD3528909c4b3E9

#[derive(Parser)]
pub struct RemoveIdentityCommand {
	/// The prime identity in did format
	src_did: String,
	/// The Identities vec in did format, separated by `,` to-be-removed
	#[clap(num_args = 0.., value_delimiter = ',', default_value = "")]
	dst_dids: Vec<String>,
}

impl RemoveIdentityCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let src_id: Identity = Identity::from_did(self.src_did.as_str()).unwrap();

		let identities: Vec<Identity> = self
			.dst_dids
			.iter()
			.filter(|&dst_did| !dst_did.is_empty())
			.map(|dst_did| Identity::from_did(dst_did.as_str()).unwrap())
			.collect();

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);

		let top = TrustedCall::remove_identity(alice.public().into(), src_id, identities)
			.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
			.into_trusted_operation(trusted_cli.direct);
		Ok(perform_trusted_operation::<()>(cli, trusted_cli, &top).map(|_| CliResultOk::None)?)
	}
}
