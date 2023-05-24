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
	command_utils::{get_accountid_from_str, get_chain_api},
	Cli,
};

#[derive(Parser)]
pub struct GetAccountNonceCommand {
	/// AccountId in ss58check format
	account: String,
}

impl GetAccountNonceCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		let api = get_chain_api(cli);
		let accountid = get_accountid_from_str(&self.account);
		let nonce = api.get_account_info(&accountid).unwrap().map_or_else(|| 0, |info| info.nonce);

		println!("Account {:?} nonce : {nonce}", accountid);
	}
}
