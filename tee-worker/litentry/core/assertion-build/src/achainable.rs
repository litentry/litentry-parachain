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
	achainable::{AchainableClient, AchainableHolder, ParamsBasicTypeWithAmountHolding, ParamsBasicTypeWithClassOfYear, AchainableTagAccount},
	vec_to_string,
};
use std::string::ToString;

const VC_A20_SUBJECT_DESCRIPTION: &str =
	"The length of time a user continues to hold a particular token (with particular threshold of token amount)";
const VC_A20_SUBJECT_TYPE: &str = "ETH Holding Assertion";
const VC_A20_SUBJECT_TAG: [&str; 1] = ["Ethereum"];

pub fn build(req: &AssertionBuildRequest, param: AchainableParams) -> Result<Credential> {
    match param {
        AchainableParams::ClassOfYear(c) => build_class_of_year(req, c),
    }
}

fn build_class_of_year(req: &AssertionBuildRequest, param: ParamsBasicTypeWithClassOfYearN) -> Result<Credential> {
	debug!("Assertion A20 build, who: {:?}", account_id_to_string(&req.who));

    let name = param.clone().name;
    let chain = param.clone().chain;
    let date1 = param.clone().date1;
    let date2 = param.clone().date2;

    let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A1, ErrorDetail::ParseError)
	})?;
    let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A1, ErrorDetail::ParseError)
	})?;
    let date1 = vec_to_string(date1.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A1, ErrorDetail::ParseError)
	})?;
    let date2 = vec_to_string(date2.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A1, ErrorDetail::ParseError)
	})?;

    let p = ParamsBasicTypeWithClassOfYear {
        name,
        chain,
        date1: date1.clone(),
        date2: date2.clone(),
    };

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

    let mut flag = false;
    for address in &addresses {
        if flag {
            break;
        }

        match client.class_of_year(address, p.clone()) {
            Ok(b) => flag = b,
            Err(e) => error!("Request class of year failed {:?}", e),
        }
    }

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A20_SUBJECT_DESCRIPTION,
				VC_A20_SUBJECT_TYPE,
			);
			credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A20(AchainableParams::ClassOfYear(param)), e.into_error_detail()))
		},
	}
}