// Copyright 2020-2022 Litentry Technologies GmbH.
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
	trusted_command_utils::{get_accountid_from_str, get_identifiers, get_pair_from_str},
	trusted_commands::TrustedArgs,
	trusted_operation::perform_trusted_operation,
	Cli,
};
use codec::Decode;
use ita_stf::{Index, TrustedCall, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct SetChallengeCodeCommand {
	/// AccountId in ss58check format
	account: String,
	identity: String,
	/// challenge code in hex string
	code_hex: String,
}

impl SetChallengeCodeCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedArgs) {
		let who = get_accountid_from_str(&self.account);
		let root = get_pair_from_str(trusted_args, "//Alice");

		let (mrenclave, shard) = get_identifiers(trusted_args);
		let nonce = get_layer_two_nonce!(root, cli, trusted_args);
		// compose the extrinsic
		let identity = serde_json::from_str(self.identity.as_str()).unwrap();

		let mut code = [0u8; 16];
		hex::decode_to_slice(&self.code_hex, &mut code).expect("decoding code failed");

		let top: TrustedOperation =
			TrustedCall::set_challenge_code_runtime(root.public().into(), who, identity, code)
				.sign(&KeyPair::Sr25519(Box::new(root)), nonce, &mrenclave, &shard)
				.into_trusted_operation(trusted_args.direct);
		perform_trusted_operation(cli, trusted_args, &top);
	}
}
