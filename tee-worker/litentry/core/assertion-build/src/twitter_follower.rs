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
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::{
	twitter_official::{TargetUser, TwitterOfficialClient},
	vec_to_string,
};
use log::*;
use std::{format, vec::Vec};

const VC_SUBJECT_DESCRIPTION: &str = "The user has followed {:?}";
const VC_SUBJECT_TYPE: &str = "A follower of the {:?}";
const VC_SUBJECT_TAG: [&str; 1] = ["Twitter"];

pub fn build(
	twitter_screen_name: ParameterString,
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	who: &AccountId,
) -> Result<Credential> {
	debug!(
		"Assertion TwitterFollower build, who: {:?}, identities: {:?}",
		account_id_to_string(&who),
		identities
	);

	let twitter_screen_name_s = vec_to_string(twitter_screen_name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::TwitterFollower(twitter_screen_name.clone()),
			ErrorDetail::ParseError,
		)
	})?;

	let mut result = false;

	let mut twitter_official_v1_1 = TwitterOfficialClient::v1_1();
	for identity in identities {
		if let Identity::Web2 { network, address } = identity {
			if matches!(network, Web2Network::Twitter) {
				let twitter_handler = address.to_vec();

				let relationship = twitter_official_v1_1
					.query_friendship(
						twitter_handler.clone(),
						TargetUser::Name(twitter_screen_name.to_vec()),
					)
					.map_err(|e| {
						// invalid permissions, rate limitation, etc
						log::warn!("TwitterFollower query_friendship error:{:?}", e);
						Error::RequestVCFailed(
							Assertion::TwitterFollower(twitter_screen_name.clone()),
							ErrorDetail::StfError(ErrorString::truncate_from(
								format!("{:?}", e).into(),
							)),
						)
					})?;

				if relationship.source.following {
					result = true;
					break
				}
			}
		}
	}

	match Credential::new_default(who, &shard.clone()) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_SUBJECT_DESCRIPTION,
				VC_SUBJECT_TYPE,
				VC_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_twitter_follower_assertion(twitter_screen_name_s, result);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential A5 failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::TwitterFollower(twitter_screen_name),
				e.into_error_detail(),
			))
		},
	}
}
