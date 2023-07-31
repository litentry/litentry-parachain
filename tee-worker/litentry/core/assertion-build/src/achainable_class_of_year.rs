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

use crate::{*, achainable::request_achainable};
use lc_credentials::Credential;
use lc_data_providers::{
	achainable::{Params, ParamsBasicTypeWithClassOfYear},
	vec_to_string,
};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::AchainableClassOfYear;
use log::debug;

const VC_ACHAINABLE_SUBJECT_DESCRIPTION: &str = "Class of year";
const VC_ACHAINABLE_SUBJECT_TYPE: &str = "ETH Class of year Assertion";

pub fn build_class_of_year(
	req: &AssertionBuildRequest,
	param: AchainableClassOfYear,
) -> Result<Credential> {
	debug!("Assertion Achainable build_class_of_year, who: {:?}", account_id_to_string(&req.who));

	let (name, chain, date1, date2) = get_class_of_year_params(&param)?;
	let p = ParamsBasicTypeWithClassOfYear::one(name, chain, date1.clone(), date2.clone());

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let flag = request_achainable(addresses, Params::ParamsBasicTypeWithClassOfYear(p.clone()))?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned
				.add_subject_info(VC_ACHAINABLE_SUBJECT_DESCRIPTION, VC_ACHAINABLE_SUBJECT_TYPE);
			credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::ClassOfYear(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn get_class_of_year_params(
	param: &AchainableClassOfYear,
) -> Result<(String, String, String, String)> {
	let name = param.clone().name;
	let chain = param.clone().chain;
	let date1 = param.clone().date1;
	let date2 = param.clone().date2;
	let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::ClassOfYear(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::ClassOfYear(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let date1 = vec_to_string(date1.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::ClassOfYear(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let date2 = vec_to_string(date2.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::ClassOfYear(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	Ok((name, chain, date1, date2))
}
