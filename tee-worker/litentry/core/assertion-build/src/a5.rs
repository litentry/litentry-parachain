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

use crate::Error;
use lc_data_providers::{
	twitter_litentry::TwitterLitentryClient, twitter_official::TwitterOfficialClient,
};
use litentry_primitives::{
	Identity, IdentityHandle, IdentityWebType, ParameterString, Web2Network,
};
use std::{
	string::{String, ToString},
	vec::Vec,
};

pub fn build(
	identities: Vec<Identity>,
	twitter_account: ParameterString,
	original_tweet_id: ParameterString,
) -> Result<(), Error> {
	let mut twitter_litentry_client = TwitterLitentryClient::new();
	let mut twitter_official_client = TwitterOfficialClient::new();
	for identity in identities {
		if identity.web_type == IdentityWebType::Web2(Web2Network::Twitter) {
			if let IdentityHandle::String(twitter_id) = identity.handle {
				let twitter_id = twitter_id.to_vec();
				match twitter_litentry_client
					.check_follow(twitter_id.clone(), twitter_account.to_vec())
				{
					Ok(true) => {
						match twitter_official_client
							.query_retweet(twitter_id, original_tweet_id.to_vec())
						{
							Ok(_) => {
								// TODO generate vc;
								return Ok(())
							},
							Err(e) => {
								log::warn!("Assertion5 query_retweet error:{:?}", e)
							},
						}
					},
					Ok(false) => {
						log::debug!(
							"account:{:?} don't follow {:?}",
							twitter_id,
							String::from_utf8(twitter_account.to_vec())
						);
					},
					Err(e) => {
						log::warn!("Assertion5 request error:{:?}", e)
					},
				}
			}
		}
	}
	Err(Error::Assertion5Error("not match any identities".to_string()))
}
