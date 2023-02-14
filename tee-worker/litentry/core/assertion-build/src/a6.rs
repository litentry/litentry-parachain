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
use lc_data_providers::twitter_official::TwitterOfficialClient;
use litentry_primitives::{Assertion, Identity, ParentchainBlockNumber, Web2Network};
use log::*;
use parachain_core_primitives::VCMPError;
use std::vec::Vec;

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
	let mut client = TwitterOfficialClient::new();
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
						return Err(VCMPError::Assertion6Failed)
					},
				}
			}
		}
	}

	info!("sum followers: {}", sum);
	let min: u64;
	let max: u64;

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
			max = u64::MAX;
		},
	}

	match Credential::generate_unsigned_credential(&Assertion::A6, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_assertion_a6(min, max);
			credential_unsigned.credential_subject.values.push(true);
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}
	Err(VCMPError::Assertion6Failed)
}
