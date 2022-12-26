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
use std::format;

use lc_data_providers::discord_litentry::DiscordLitentryClient;
use litentry_primitives::ParameterString;

pub fn build(guild_id: ParameterString, handler: ParameterString) -> Result<()> {
	let mut client = DiscordLitentryClient::new();
	match client.check_join(guild_id.into_inner(), handler.into_inner()) {
		Err(e) => {
			log::error!("error build assertion2: {:?}", e);
			Err(Error::Assertion2Error(format!("{:?}", e)))
		},
		Ok(_response) => {
			// TODO:
			// generate_vc(who, identity, ...)

			// After receiving VC, F/E is expected to assign 'IDHubber' role and align with bot
			// https://github.com/litentry/tee-worker/issues/35
			// https://github.com/litentry/tee-worker/issues/36

			Ok(())
		},
	}
}

#[cfg(test)]
mod tests {
	use crate::a2::build;
	use frame_support::BoundedVec;
	use lc_data_providers::G_DATA_PROVIDERS;
	use log;

	#[test]
	fn assertion2_verification_works() {
		G_DATA_PROVIDERS
			.write()
			.unwrap()
			.set_discord_litentry_url("http://localhost:9527".to_string());
		let guildid: u64 = 919848390156767232;
		let guild_id_vec: Vec<u8> = format!("{}", guildid).as_bytes().to_vec();
		let handler_vec: Vec<u8> = "againstwar%234779".to_string().as_bytes().to_vec();

		let guild_id = BoundedVec::try_from(guild_id_vec).unwrap();
		let handler = BoundedVec::try_from(handler_vec).unwrap();

		let _ = build(guild_id, handler);
		log::info!("assertion2 test");
	}
}
