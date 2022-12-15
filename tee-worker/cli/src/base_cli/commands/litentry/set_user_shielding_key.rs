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

use super::IMP;
use crate::{
	command_utils::{get_chain_api, *},
	Cli,
};
use base58::FromBase58;
use codec::{Decode, Encode};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::types::ShardIdentifier;
use log::*;

use sp_core::sr25519 as sr25519_core;
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus};

#[derive(Parser)]
pub struct SetUserShieldingKeyCommand {
	/// AccountId in ss58check format
	account: String,

	/// Shielding key in hex string
	key_hex: String,

	/// Shard identifier
	shard: String,
}

impl SetUserShieldingKeyCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		let chain_api = get_chain_api(cli);

		let shard_opt = match self.shard.from_base58() {
			Ok(s) => ShardIdentifier::decode(&mut &s[..]),
			_ => panic!("shard argument must be base58 encoded"),
		};

		let shard = match shard_opt {
			Ok(shard) => shard,
			Err(e) => panic!("{}", e),
		};

		let who = get_pair_from_str(&self.account);
		let chain_api = chain_api.set_signer(sr25519_core::Pair::from(who));

		let mut key = [0u8; 32];
		hex::decode_to_slice(&self.key_hex, &mut key).expect("decoding key failed");

		let tee_shielding_key = get_shielding_key(cli).unwrap();
		let encrypted_key = tee_shielding_key.encrypt(&key.encode()).unwrap();

		let xt: UncheckedExtrinsicV4<_, _> =
			compose_extrinsic!(chain_api, IMP, "set_user_shielding_key", shard, encrypted_key);

		let tx_hash = chain_api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();
		println!("[+] TrustedOperation got finalized. Hash: {:?}\n", tx_hash);
	}
}
