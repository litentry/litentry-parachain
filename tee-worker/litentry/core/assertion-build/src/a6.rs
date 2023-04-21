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
use lc_data_providers::twitter_official::TwitterOfficialClient;
use log::*;
use std::{format, vec::Vec};

const VC_A6_SUBJECT_DESCRIPTION: &str = "The range of the user's Twitter follower count";
const VC_A6_SUBJECT_TYPE: &str = "Twitter Follower Amount";
const VC_A6_SUBJECT_TAG: [&str; 1] = ["Twitter"];

/// Following ranges:
///
///    * 1+ follower
///    * 100+ followers
///    * 1,000+ followers
///    * 10,000+ followers
///    * 100,000+ followers
pub fn build(
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	debug!(
		"Assertion A6 build, who: {:?}, bn: {}, identities: {:?}",
		account_id_to_string(&who),
		bn,
		identities
	);

	let mut client = TwitterOfficialClient::v2();
	let mut sum: u32 = 0;

	for identity in identities {
		if let Identity::Web2 { network, address } = identity {
			if matches!(network, Web2Network::Twitter) {
				let twitter_handler = address.to_vec();
				match client.query_user(twitter_handler) {
					Ok(user) =>
						if let Some(metrics) = user.public_metrics {
							sum += metrics.followers_count;
						},
					Err(e) => {
						log::warn!("Assertion6 request error:{:?}", e);
						return Err(Error::RequestVCFailed(
							Assertion::A6,
							ErrorDetail::StfError(ErrorString::truncate_from(
								format!("{:?}", e).into(),
							)),
						))
					},
				}
			}
		}
	}

	info!("sum followers: {}", sum);
	let min: u32;
	let max: u32;

	match sum {
		0 | 1 => {
			min = 0;
			max = 1;
		},
		2..=100 => {
			min = 1;
			max = 100;
		},
		101..=1000 => {
			min = 100;
			max = 1000;
		},
		1001..=10000 => {
			min = 1000;
			max = 10000;
		},
		10001..=100000 => {
			min = 10000;
			max = 100000;
		},
		100001..=u32::MAX => {
			min = 100000;
			max = u32::MAX;
		},
	}

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A6_SUBJECT_DESCRIPTION,
				VC_A6_SUBJECT_TYPE,
				VC_A6_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_assertion_a6(min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A6, e.into_error_detail()))
		},
	}
}
