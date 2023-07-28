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
	achainable::{AchainableClient, ParamsBasicTypeWithDate, Params},
	vec_to_string,
};

const VC_SUBJECT_DESCRIPTION: &str = "Account created after {date}";
const VC_SUBJECT_TYPE: &str = "Account created after {date}";

pub fn build_date(req: &AssertionBuildRequest, param: AchainableDate) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

	let chain = param.clone().chain;
	let date = param.clone().date;

	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Date(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let date = vec_to_string(date.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Date(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let p = ParamsBasicTypeWithDate::new("Account created after {date}".into(), chain, date);
	let mut flag = false;
	for address in &addresses {
		if flag {
			break
		}

		let ret = client.query_system_label(address, Params::ParamsBasicTypeWithDate(p.clone()));
		match ret {
			Ok(r) => flag = r,
			Err(e) => error!("Request class of year failed {:?}", e),
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
				Assertion::Achainable(AchainableParams::Date(param)),
				e.into_error_detail(),
			))
		},
	}
}
