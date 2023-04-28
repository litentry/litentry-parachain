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

use super::IMP;
use crate::{
	command_utils::{get_chain_api, *},
	Cli,
};
use base58::FromBase58;
use codec::{Decode, Encode};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::H256;
use litentry_primitives::{Address32, Identity};
use log::*;
use sp_application_crypto::Pair;
use sp_core::sr25519 as sr25519_core;
use substrate_api_client::{compose_extrinsic, CallIndex, UncheckedExtrinsicV4, XtStatus};

#[derive(Parser)]
pub struct CreateIdentityCommand {
	/// AccountId in ss58check format
	account: String,
	/// Identity to be created
	identity: String,
	/// Shard identifier
	shard: String,
}

impl CreateIdentityCommand {
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

		let who = sr25519_core::Pair::from_string(&self.account, None).unwrap();
		let chain_api = chain_api.set_signer(who.clone());

		let identity: Result<Identity, _> = serde_json::from_str(self.identity.as_str());
		if let Err(e) = identity {
			warn!("Deserialize Identity error: {:?}", e.to_string());
			return
		}

		let tee_shielding_key = get_shielding_key(cli).unwrap();
		let encrypted_identity = tee_shielding_key.encrypt(&identity.unwrap().encode()).unwrap();

		let vdata: Option<Vec<u8>> = None;
		pub type CreateIdentityFn = (CallIndex, H256, Address32, Vec<u8>, Option<Vec<u8>>);
		let xt: UncheckedExtrinsicV4<CreateIdentityFn, _> = compose_extrinsic!(
			chain_api,
			IMP,
			"create_identity",
			shard,
			who.public().0.into(),
			encrypted_identity.to_vec(),
			vdata
		);

		let tx_hash = chain_api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();
		println!("[+] TrustedOperation got finalized. Hash: {:?}\n", tx_hash);
	}
}
