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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{Error, Result};
use lc_data_providers::discord_litentry::DiscordLitentryClient;
use litentry_primitives::{Identity, ParameterString, Web2Network};
use std::vec::Vec;

pub fn build(
	identities: Vec<Identity>,
	guild_id: ParameterString,
	handler: ParameterString,
) -> Result<()> {
	let mut client = DiscordLitentryClient::new();
	for identity in identities {
		if let Identity::Web2 { network, address: _addr } = identity {
			if matches!(network, Web2Network::Discord) {
				// TODO not sure if addr = handler ?
				if let Ok(response) = client.check_join(guild_id.to_vec(), handler.to_vec()) {
					if response.data {
						// TODO:
						// generate_vc(who, identity, ...)

						// After receiving VC, F/E is expected to assign 'IDHubber' role and align with bot
						// https://github.com/litentry/tee-worker/issues/35
						// https://github.com/litentry/tee-worker/issues/36
						return Ok(())
					}
				}
			}
		}
	}
	Err(Error::Assertion2Failed)
}

#[cfg(test)]
mod tests {
	use crate::a2::build;
	use frame_support::BoundedVec;
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

		let _ = build(identities, guild_id, handler);
		log::info!("assertion2 test");
	}
}
