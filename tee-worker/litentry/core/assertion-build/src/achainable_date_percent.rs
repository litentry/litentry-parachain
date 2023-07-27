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

use crate::{std::string::ToString, *};
use lc_data_providers::{
	achainable::{AchainableClient, AchainableTagAccount, ParamsBasicTypeWithClassOfYear},
	vec_to_string,
};

const VC_SUBJECT_DESCRIPTION: &str = "Class of year";
const VC_SUBJECT_TYPE: &str = "ETH Class of year Assertion";

pub fn build_date_percent(
	req: &AssertionBuildRequest,
	param: AchainableDatePercent,
) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

	let chain = param.clone().chain;
	let token = param.clone().token;
	let date = param.clone().date;
	let percent = param.clone().percent;

	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::DatePercent(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let token = vec_to_string(token.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::DatePercent(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let date = vec_to_string(date.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::DatePercent(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let percent = vec_to_string(percent.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::DatePercent(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			// credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::DatePercent(param)),
				e.into_error_detail(),
			))
		},
	}
}
