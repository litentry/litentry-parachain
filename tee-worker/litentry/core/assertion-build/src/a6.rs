// Copyright 2020-2024 Trust Computing GmbH.
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
use lc_common::abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls};
use lc_credentials::IssuerRuntimeVersion;
use lc_data_providers::{
	twitter_official::TwitterOfficialClient, DataProviderConfig, Error as DataProviderError,
};

const VC_A6_SUBJECT_DESCRIPTION: &str = "The range of the user's Twitter follower count";
const VC_A6_SUBJECT_TYPE: &str = "Twitter Follower Amount";

/// Following ranges:
///
///    * 1+ follower
///    * 100+ followers
///    * 1,000+ followers
///    * 10,000+ followers
///    * 100,000+ followers
pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Assertion A6 build, who: {:?}", account_id_to_string(&req.who),);

	let mut client = TwitterOfficialClient::v2(
		&data_provider_config.twitter_official_url,
		&data_provider_config.twitter_auth_token_v2,
	);
	let mut sum: u32 = 0;

	let identities = req
		.identities
		.iter()
		.map(|(identity, _)| identity.clone())
		.collect::<Vec<Identity>>();

	loop_with_abort_strategy::<fn(&_) -> bool, Identity, DataProviderError>(
		identities,
		|identity| {
			if let Identity::Twitter(address) = identity {
				let twitter_handler = address.inner_ref().to_vec();
				let user = client.query_user_by_name(twitter_handler)?;

				if let Some(metrics) = user.public_metrics {
					sum += metrics.followers_count;
				}
			}
			Ok(LoopControls::Continue)
		},
		AbortStrategy::FailFast::<fn(&_) -> bool>,
	)
	.map_err(|errors| {
		Error::RequestVCFailed(
			Assertion::A6,
			ErrorDetail::StfError(ErrorString::truncate_from(format!("{:?}", errors[0]).into())),
		)
	})?;

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

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A6_SUBJECT_DESCRIPTION, VC_A6_SUBJECT_TYPE);
			credential_unsigned.add_assertion_a6(min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A6, e.into_error_detail()))
		},
	}
}
