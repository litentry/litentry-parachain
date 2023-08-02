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

use crate::{achainable::request_achainable_classofyear, *};
use lc_credentials::{format_assertion_to_date, Credential};
use lc_data_providers::{
	achainable::{Params, ParamsBasicTypeWithClassOfYear},
	vec_to_string,
};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::AchainableClassOfYear;
use log::debug;
use std::string::ToString;

const VC_SUBJECT_DESCRIPTION: &str =
	"The class of year that the user account was created on a particular network (must have on-chain records)";
const VC_SUBJECT_TYPE: &str = "Account Class Of Year";

/// NOTE:
///
/// Build class of year
/// name: Account created between {dates}
/// chain: ethereum
///
/// True:
/// assertions":[
/// {
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2015-07-30"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "2017-01-01"
/// 			}
/// 		],
/// }
///
/// False:
/// assertions":[
/// {
/// 		and: [
/// 			{
/// 				"src":"$from_date",
/// 				"op":"==",
/// 				"dst":"2015-01-01"
/// 			},
/// 			{
/// 				"src": "to_date",
/// 				"op": "==",
/// 				"dst": "NOW"
/// 			}
/// 		],
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

	let p = ParamsBasicTypeWithClassOfYear::one(
		name.clone(),
		chain.clone(),
		"2015-07-30".to_string(),
		"2017-01-01".to_string(),
	);
	let longest_created_date =
		request_achainable_classofyear(addresses, Params::ParamsBasicTypeWithClassOfYear(p));
	let found = !longest_created_date.is_empty();
	let (from, to) = get_class_of_year_interval(longest_created_date);

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.update_class_of_year(found, from, to);

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

const INTERVAL: usize = 4;
const FROM_DATE: [&str; INTERVAL] = ["2015-07-30", "2017-01-02", "2019-01-02", "2022-01-02"];
const TO_DATE: [&str; 3] = ["2017-01-01", "2019-01-01", "2022-01-01"];

fn get_class_of_year_interval(date: String) -> (String, String) {
	let now = format_assertion_to_date();
	let mut to_date = TO_DATE.to_vec();
	to_date.push(&now);

	for indx in 0..INTERVAL {
		if date >= FROM_DATE[indx].to_string() && date <= to_date[indx].to_string() {
			return (FROM_DATE[indx].to_string(), to_date[indx].to_string())
		}
	}

	// If not in range
	(FROM_DATE[0].to_string(), to_date[INTERVAL - 1].to_string())
}
