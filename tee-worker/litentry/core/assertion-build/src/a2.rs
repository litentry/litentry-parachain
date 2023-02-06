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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::Result;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_data_providers::discord_litentry::DiscordLitentryClient;
use litentry_primitives::{
	Assertion, Identity, ParameterString, ParentchainBlockNumber, Web2Network,
};
use log::*;
use parachain_core_primitives::VCMPError;
use std::vec::Vec;

pub fn build(
	identities: Vec<Identity>,
	guild_id: ParameterString,
	handler: ParameterString,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	let mut discord_cnt: i32 = 0;
	let mut has_joined: bool = false;

	let mut client = DiscordLitentryClient::new();
	for identity in identities {
		if let Identity::Web2 { network, address: _addr } = identity {
			if matches!(network, Web2Network::Discord) {
				discord_cnt += 1;
				if let Ok(response) = client.check_join(guild_id.to_vec(), handler.to_vec()) {
					if response.data {
						has_joined = true;
						break
					}
				}
			}
		}
	}

	match Credential::generate_unsigned_credential(
		&Assertion::A2(guild_id.clone(), handler.clone()),
		who,
		&shard.clone(),
		bn,
	) {
		Ok(mut credential_unsigned) => {
			if discord_cnt > 0 && has_joined {
				credential_unsigned.credential_subject.set_value(true);

				//Assign Role "ID-Hubber" to User
				match client.assign_id_hubber(guild_id.to_vec(), handler.to_vec()) {
					Ok(response) => {
						//check response
						if !response.data {
							error!("assign_id_hubber {} {}", response.message, response.msg_code);
						}
					},
					Err(e) => {
						error!("assign_id_hubber failed {:?}", e);
						return Err(VCMPError::Assertion2Failed)
					},
				}
			} else {
				credential_unsigned.credential_subject.set_value(false);
			}
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential A2 failed {:?}", e);
		},
	}
	Err(VCMPError::Assertion2Failed)
}

#[cfg(test)]
mod tests {
	use crate::a2::build;
	use frame_support::BoundedVec;
	use itp_stf_primitives::types::ShardIdentifier;
	use itp_types::AccountId;
	use lc_data_providers::G_DATA_PROVIDERS;
	use litentry_primitives::{Identity, IdentityString, Web2Network};
	use log;
	use std::{format, vec, vec::Vec};

	#[test]
	fn assertion2_verification_works() {
		G_DATA_PROVIDERS
			.write()
			.unwrap()
			.set_discord_litentry_url("http://localhost:9527".to_string());
		let guildid: u64 = 919848390156767232;
		let guild_id_vec: Vec<u8> = format!("{}", guildid).as_bytes().to_vec();
		let handler_vec: Vec<u8> = "againstwar%234779".to_string().as_bytes().to_vec();
		let identities = vec![Identity::Web2 {
			network: Web2Network::Discord,
			address: IdentityString::truncate_from(handler_vec.clone()),
		}];
		let guild_id = BoundedVec::try_from(guild_id_vec).unwrap();
		let handler = BoundedVec::try_from(handler_vec).unwrap();
		let who = AccountId::from([0; 32]);
		let shard = ShardIdentifier::default();

		let _ = build(identities, guild_id, handler, &shard, &who, 1);
		log::info!("assertion2 test");
	}
}
