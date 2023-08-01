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
use lc_credentials::Credential;
use lc_data_providers::{
	achainable::{Params, ParamsBasicTypeWithClassOfYear},
	vec_to_string,
};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::AchainableClassOfYear;
use log::debug;
use std::string::ToString;

const VC_ACHAINABLE_SUBJECT_DESCRIPTION: &str = "Class of year";
const VC_ACHAINABLE_SUBJECT_TYPE: &str = "ETH Class of year Assertion";

/// TODO: date
const CLASS_OF_YEAR_INTERVAL: usize = 4;
const FROM_CLASS_OF_YEAR: [&str; CLASS_OF_YEAR_INTERVAL] =
	["2009-01-01", "2017-01-01", "2020-01-01", "2022-01-01"];
const TO_CLASS_OF_YEAR: [&str; CLASS_OF_YEAR_INTERVAL] =
	["2016-12-31", "2019-12-31", "2021-12-31", "2022-12-31"];

/// NOTE:
///
/// Build class of year
/// name: Account created between {dates}
/// chain: ethereum
///
/// assertions":[
/// {
///		"or":[
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2009-01-01"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "2016-12-31"
/// 			}
/// 		],
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2017-01-01"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "2019-12-31"
/// 			}
/// 		],
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2020-01-01"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "2021-12-31"
/// 			}
/// 		],
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2022-01-01"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "2022-12-31"
/// 			}
/// 		],
/// 	]
/// }
pub fn build_class_of_year(
	req: &AssertionBuildRequest,
	param: AchainableClassOfYear,
) -> Result<Credential> {
	debug!("Assertion Achainable build_class_of_year, who: {:?}", account_id_to_string(&req.who));

	let (name, chain, _date1, _date2) = get_class_of_year_params(&param)?;

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut flag = false;
	for indx in 0..CLASS_OF_YEAR_INTERVAL {
		if flag {
			break
		}

		let date1 = FROM_CLASS_OF_YEAR[indx];
		let date2 = TO_CLASS_OF_YEAR[indx];
		let p = ParamsBasicTypeWithClassOfYear::one(
			name.clone(),
			chain.clone(),
			date1.to_string(),
			date2.to_string(),
		);

		flag = request_achainable(
			addresses.clone(),
			Params::ParamsBasicTypeWithClassOfYear(p.clone()),
		)?;
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned
				.add_subject_info(VC_ACHAINABLE_SUBJECT_DESCRIPTION, VC_ACHAINABLE_SUBJECT_TYPE);
			credential_unsigned.update_class_of_year(
				flag,
				FROM_CLASS_OF_YEAR.to_vec(),
				TO_CLASS_OF_YEAR.to_vec(),
			);

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
