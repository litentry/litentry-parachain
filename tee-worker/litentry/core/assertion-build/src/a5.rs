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
use lc_data_providers::{twitter_official::TwitterOfficialClient, vec_to_string};
use log::*;
use std::{format, vec::Vec};

const VC_A5_SUBJECT_DESCRIPTION: &str =
	"The user has followed a specific and retweet a specific tweet";
const VC_A5_SUBJECT_TYPE: &str = "Retweet a tweet as a follower of the tweet author";
const VC_A5_SUBJECT_TAG: [&str; 1] = ["Twitter"];

pub fn build(
	identities: Vec<Identity>,
	original_tweet_id: ParameterString,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	debug!(
		"Assertion A5 build, who: {:?}, bn: {}, identities: {:?}",
		account_id_to_string(&who),
		bn,
		identities
	);

	//ToDo: Check this string is a pure number or not, to avoid wasting API calls.
	let original_tweet_id_s = vec_to_string(original_tweet_id.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A5(original_tweet_id.clone()), ErrorDetail::ParseError)
	})?;

	let mut value = false;

	let mut twitter_official_v1_1 = TwitterOfficialClient::v1_1();
	let mut twitter_official_v2 = TwitterOfficialClient::v2();
	for identity in identities {
		if let Identity::Web2 { network, address } = identity {
			if matches!(network, Web2Network::Twitter) {
				let twitter_handler = address.to_vec();

				let tweet = twitter_official_v2
					.query_tweet(original_tweet_id.clone().to_vec())
					.map_err(|e| {
						// invalid permissions, bad original_tweet_id, rate limitation, etc
						log::warn!("Assertion5 query_tweet error:{:?}", e);
						Error::RequestVCFailed(
							Assertion::A5(original_tweet_id.clone()),
							ErrorDetail::StfError(ErrorString::truncate_from(
								format!("{:?}", e).into(),
							)),
						)
					})?;

				let relationship = twitter_official_v1_1
					.query_friendship(twitter_handler.clone(), tweet.author_id.as_bytes().to_vec())
					.map_err(|e| {
						// invalid permissions, rate limitation, etc
						log::warn!("Assertion5 query_friendship error:{:?}", e);
						Error::RequestVCFailed(
							Assertion::A5(original_tweet_id.clone()),
							ErrorDetail::StfError(ErrorString::truncate_from(
								format!("{:?}", e).into(),
							)),
						)
					})?;

				let is_following = relationship.source.following;

				let mut has_retweeted: bool = false;
				let retweets = twitter_official_v2
					.query_retweeted_by(original_tweet_id.clone().to_vec())
					.map_err(|e| {
						// invalid permissions, rate limitation, bad original_tweet_id, etc
						log::warn!("Assertion5 query_retweeted_by error:{:?}", e);
						Error::RequestVCFailed(
							Assertion::A5(original_tweet_id.clone()),
							ErrorDetail::StfError(ErrorString::truncate_from(
								format!("{:?}", e).into(),
							)),
						)
					})?;

				for retweeter in retweets.data {
					if retweeter.username.as_bytes().to_vec() == twitter_handler {
						has_retweeted = true;
						break
					}
				}

				//break the outer loop
				if is_following && has_retweeted {
					value = true;
					break
				}
			}
		}
	}

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A5_SUBJECT_DESCRIPTION,
				VC_A5_SUBJECT_TYPE,
				VC_A5_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_assertion_a5(original_tweet_id_s, value);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential A5 failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A5(original_tweet_id), e.into_error_detail()))
		},
	}
}
