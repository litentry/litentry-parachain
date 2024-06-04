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

use crate::{command_utils::get_shielding_key, Cli, CliResult, CliResultOk};
use codec::Encode;
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use std::format;

// Scale encodes string and ecrypts it with worker's shielding key
// usage example:
// ./litentry-cli shield-text test
#[derive(Parser)]
pub struct ShieldTextCommand {
	value: String,
}

impl ShieldTextCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let shielding_key = get_shielding_key(cli).unwrap();
		let encrypted = shielding_key.encrypt(&self.value.encode()).unwrap();
		std::println!("Shielded: {:?}", hex::encode(encrypted.clone()));
		Ok(CliResultOk::Bytes { bytes: encrypted })
	}
}
