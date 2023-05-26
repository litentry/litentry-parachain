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
	trusted_cli::TrustedCli, trusted_command_utils::get_pair_from_str,
	trusted_operation::perform_trusted_operation, Cli, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::UserShieldingKeyType;
use sp_core::Pair;

#[derive(Parser)]
pub struct UserShiledingKeyCommand {
	/// AccountId in ss58check format
	account: String,
}

impl UserShiledingKeyCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let who = get_pair_from_str(trusted_cli, &self.account);
		let top: TrustedOperation = TrustedGetter::user_shielding_key(who.public().into())
			.sign(&KeyPair::Sr25519(Box::new(who)))
			.into();
		let key = perform_trusted_operation(cli, trusted_cli, &top)
			.and_then(|v| Ok(UserShieldingKeyType::decode(&mut v.unwrap().as_slice()).ok()));
		println!("{}", hex::encode(key.unwrap().unwrap()));

		// Ok(perform_trusted_operation(cli, trusted_cli, &top).map(|_| CliResultOk::None)?)
		Ok(CliResultOk::None)
	}
}
