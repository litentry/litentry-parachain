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

use crate::{
	trusted_base_cli::commands::bitacross::utils::{random_aes_key, send_direct_request_and_watch},
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	Cli, CliResult, CliResultOk,
};
use bc_musig2_ceremony::SignBitcoinPayload;
use itp_stf_primitives::types::KeyPair;
use lc_direct_call::DirectCall;
use litentry_primitives::{aes_decrypt, AesOutput};
use sp_core::Pair;

#[derive(Parser)]
pub struct RequestDirectCallSignBitcoinCommand {
	payload: Vec<u8>,
	merkle_root: String,
}

impl RequestDirectCallSignBitcoinCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let key: [u8; 32] = random_aes_key();

		let merkle_root_bytes = hex::decode(self.merkle_root.clone()).unwrap();

		let dc = DirectCall::SignBitcoin(
			alice.public().into(),
			key,
			SignBitcoinPayload::TaprootSpendable(
				self.payload.clone(),
				merkle_root_bytes.try_into().unwrap(),
			),
		)
		.sign(&KeyPair::Sr25519(Box::new(alice)), &mrenclave, &shard);

		let mut aes_output: AesOutput =
			send_direct_request_and_watch(cli, trusted_cli, dc, key).unwrap();
		let signature = aes_decrypt(&key, &mut aes_output).unwrap();
		println!("Got signature: {:?}", signature);

		Ok(CliResultOk::None)
	}
}
