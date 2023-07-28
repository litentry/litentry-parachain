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
use lc_data_providers::{
	achainable::{ParamsBasicTypeWithBetweenPercents, AchainableClient, Params},
	vec_to_string,
};

const VC_SUBJECT_DESCRIPTION: &str = "Balance between percents";
const VC_SUBJECT_TYPE: &str = "Balance between percents";

pub fn build_between_percents(
	req: &AssertionBuildRequest,
	param: AchainableBetweenPercents,
) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

	let chain = param.clone().chain;
	let greater_than_or_equal_to = param.clone().greater_than_or_equal_to;
	let less_than_or_equal_to = param.clone().less_than_or_equal_to;

	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::BetweenPercents(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let greater_than_or_equal_to =
		vec_to_string(greater_than_or_equal_to.to_vec()).map_err(|_| {
			Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::BetweenPercents(param.clone())),
				ErrorDetail::ParseError,
			)
		})?;
	let less_than_or_equal_to = vec_to_string(less_than_or_equal_to.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::BetweenPercents(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	let mut client: AchainableClient = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let p = ParamsBasicTypeWithBetweenPercents::new("Balance between percents".into(), chain, greater_than_or_equal_to, less_than_or_equal_to);
	let mut flag = false;
	for address in &addresses {
		if flag {
			break
		}

		let ret = client.query_system_label(address, Params::ParamsBasicTypeWithBetweenPercents(p.clone()));
		match ret {
			Ok(r) => flag = r,
			Err(e) => error!("Request Balance between percents failed {:?}", e),
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			// credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::BetweenPercents(param)),
				e.into_error_detail(),
			))
		},
	}
}
