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
use lc_data_providers::twitter_official::TwitterOfficialClient;
use litentry_primitives::{Identity, IdentityHandle, IdentityWebType, Web2Network};
use std::vec::Vec;

/// Following ranges:
///
///    * 1+ follower
///    * 100+ followers
///    * 1,000+ followers
///    * 10,000+ followers
///    * 100,000+ followers
pub fn build(identities: Vec<Identity>) -> Result<(), Error> {
	let mut client = TwitterOfficialClient::new();
	let mut sum: u32 = 0;
	for identity in identities {
		if identity.web_type == IdentityWebType::Web2(Web2Network::Twitter) {
			if let IdentityHandle::String(twitter_id) = identity.handle {
				match client.query_user(twitter_id.to_vec()) {
					Ok(user) => {
						sum += user.public_metrics.followers_count;
					},
					Err(e) => {
						log::warn!("Assertion6 request error:{:?}", e)
					},
				}
			}
		}
	}
	match sum {
		0 | 1 => {
			log::warn!("level 0");
		},
		2..=100 => {
			log::warn!("level 1");
		},
		101..=1000 => {
			log::warn!("level 2");
		},
		1001..=10000 => {
			log::warn!("level 3");
		},
		10001..=100000 => {
			log::warn!("level 4");
		},
		100001..=u32::MAX => {
			log::warn!("level 5");
		},
	}
	Ok(())
}
