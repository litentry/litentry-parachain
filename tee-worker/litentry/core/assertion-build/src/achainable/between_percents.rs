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

use crate::{achainable::request_achainable, *};
use lc_data_providers::{
	achainable::{Params, ParamsBasicTypeWithBetweenPercents},
	vec_to_string,
};

pub fn build_between_percents(
	req: &AssertionBuildRequest,
	param: AchainableBetweenPercents,
) -> Result<Credential> {
	debug!("Assertion Achainable build_between_percents, who: {:?}", account_id_to_string(&req.who));

	let (name, greater_than_or_equal_to, less_than_or_equal_to) =
		parse_between_percents_params(&param)?;

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let p = ParamsBasicTypeWithBetweenPercents::new(
		name,
		&param.chain,
		greater_than_or_equal_to,
		less_than_or_equal_to,
	);
	let _flag =
		request_achainable(addresses, Params::ParamsBasicTypeWithBetweenPercents(p.clone()))?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut _credential_unsigned) => {
			Ok(_credential_unsigned)
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

fn parse_between_percents_params(
	param: &AchainableBetweenPercents,
) -> Result<(String, String, String)> {
	let name = param.clone().name;
	let greater_than_or_equal_to = param.clone().greater_than_or_equal_to;
	let less_than_or_equal_to = param.clone().less_than_or_equal_to;

	let name = vec_to_string(name.to_vec()).map_err(|_| {
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

	Ok((name, greater_than_or_equal_to, less_than_or_equal_to))
}
