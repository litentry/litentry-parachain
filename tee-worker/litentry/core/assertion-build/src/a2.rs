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

use crate::*;
use lc_data_providers::{discord_litentry::DiscordLitentryClient, vec_to_string};

const VC_A2_SUBJECT_DESCRIPTION: &str =
	"The user has obtained an ID-Hubber role in a Litentry Discord channel";
const VC_A2_SUBJECT_TYPE: &str = "Discord ID-Hubber Role Verification";
const VC_A2_SUBJECT_TAG: [&str; 1] = ["Discord"];

pub fn build(req: &AssertionBuildRequest, guild_id: ParameterString) -> Result<Credential> {
	debug!("Assertion A2 build, who: {:?}", account_id_to_string(&req.who));

	let mut discord_cnt: i32 = 0;
	let mut has_joined: bool = false;

	let guild_id_s = vec_to_string(guild_id.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A2(guild_id.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = DiscordLitentryClient::new();
	for identity in &req.identities {
		if let Identity::Discord(address) = &identity.0 {
			discord_cnt += 1;
			if let Ok(response) = client.check_join(guild_id.to_vec(), address.to_vec()) {
				if response.data {
					has_joined = true;

					//Assign role "ID-Hubber" to each discord account
					if let Ok(response) =
						client.assign_id_hubber(guild_id.to_vec(), address.to_vec())
					{
						if !response.data {
							error!("assign_id_hubber {} {}", response.message, response.msg_code);
						}
					}
				}
			}
		}
	}

	match Credential::new_default(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A2_SUBJECT_DESCRIPTION,
				VC_A2_SUBJECT_TYPE,
				VC_A2_SUBJECT_TAG.to_vec(),
			);

			let value = discord_cnt > 0 && has_joined;
			credential_unsigned.add_assertion_a2(value, guild_id_s);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential A2 failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A2(guild_id), e.into_error_detail()))
		},
	}
}

#[cfg(test)]
mod tests {
	use crate::{a2::build, AssertionBuildRequest};
	use frame_support::BoundedVec;
	use itp_stf_primitives::types::ShardIdentifier;
	use itp_types::AccountId;
	use lc_data_providers::G_DATA_PROVIDERS;
	use litentry_primitives::{Assertion, Identity, IdentityNetworkTuple, IdentityString};
	use log;
	use std::{format, vec, vec::Vec};

	#[test]
	fn build_a2_works() {
		G_DATA_PROVIDERS
			.write()
			.unwrap()
			.set_discord_litentry_url("http://localhost:19527".to_string());
		let guild_id_u: u64 = 919848390156767232;
		let guild_id_vec: Vec<u8> = format!("{}", guild_id_u).as_bytes().to_vec();

		let handler_vec: Vec<u8> = "againstwar%234779".to_string().as_bytes().to_vec();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Discord(IdentityString::truncate_from(handler_vec.clone())), vec![])];

		let guild_id = BoundedVec::try_from(guild_id_vec).unwrap();
		let req = AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			who: AccountId::from([0; 32]),
			assertion: Assertion::A2(guild_id.clone()),
			identities,
			hash: Default::default(),
		};

		let _ = build(&req, guild_id);
		log::info!("build A2 done");
	}
}
