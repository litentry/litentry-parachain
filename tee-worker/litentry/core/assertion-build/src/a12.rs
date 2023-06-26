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
use lc_credentials::Credential;
use lc_data_providers::{
	twitter_official::{TargetUser, TwitterOfficialClient},
	vec_to_string,
};
use litentry_primitives::IdGraphIdentifier;
use log::*;
use std::{format, vec::Vec};

const VC_SUBJECT_DESCRIPTION: &str = "The user has followed a specific user";
const VC_SUBJECT_TYPE: &str = "A follower of the twitter user";
const VC_SUBJECT_TAG: [&str; 1] = ["Twitter"];

pub fn build(
	twitter_screen_name: ParameterString,
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	id_graph_identifier: &IdGraphIdentifier,
) -> Result<Credential> {
	debug!(
		"Assertion 12 build, id_graph_identifier: {:?}, identities: {:?}",
		&id_graph_identifier, identities
	);

	let twitter_screen_name_s = vec_to_string(twitter_screen_name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A12(twitter_screen_name.clone()), ErrorDetail::ParseError)
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
						log::warn!("A12 query_friendship error:{:?}", e);
						Error::RequestVCFailed(
							Assertion::A12(twitter_screen_name.clone()),
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

	match Credential::new_default(id_graph_identifier, shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_SUBJECT_DESCRIPTION,
				VC_SUBJECT_TYPE,
				VC_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_assertion_a12(twitter_screen_name_s, result);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential A12 failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A12(twitter_screen_name), e.into_error_detail()))
		},
	}
}
