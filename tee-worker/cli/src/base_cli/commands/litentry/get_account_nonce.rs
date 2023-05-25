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

use base58::FromBase58;
use codec::Decode;
use ita_stf::{Getter, PublicGetter};
use itp_stf_primitives::types::ShardIdentifier;

use crate::{
	command_utils::{decode_nonce, get_accountid_from_str, get_worker_api_direct},
	trusted_operation::get_state,
	Cli,
};

#[derive(Parser)]
pub struct GetAccountNonceCommand {
	/// AccountId in ss58check format
	account: String,

	/// Shard identifier
	shard: String,
}

impl GetAccountNonceCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		let shard_opt = match self.shard.from_base58() {
			Ok(s) => ShardIdentifier::decode(&mut &s[..]),
			_ => panic!("shard argument must be base58 encoded"),
		};

		let shard = match shard_opt {
			Ok(shard) => shard,
			Err(e) => panic!("{}", e),
		};

		let account = get_accountid_from_str(&self.account);

		let getter = Getter::public(PublicGetter::nonce(account));

		let direct_client = get_worker_api_direct(cli);

		let maybe_encoded_nonce = get_state(&direct_client, shard, &getter);
		let nonce = decode_nonce(maybe_encoded_nonce).map_or_else(|| 0, |nonce| nonce);
		print!("Get nonce: {nonce}");
	}
}
