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
	achainable::{Params, ParamsBasicTypeWithAmounts},
	vec_to_string,
};

const VC_SUBJECT_DESCRIPTION: &str = "Class of year";
const VC_SUBJECT_TYPE: &str = "ETH Class of year Assertion";

pub fn build_amounts(req: &AssertionBuildRequest, param: AchainableAmounts) -> Result<Credential> {
	debug!("Assertion Achainable build_amounts, who: {:?}", account_id_to_string(&req.who));

	let (name, chain, amount1, amount2) = get_amounts_params(&param)?;
	let p = ParamsBasicTypeWithAmounts::new(name, chain, amount1, amount2);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let _flag = request_achainable(addresses, Params::ParamsBasicTypeWithAmounts(p.clone()))?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			// credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::Amounts(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn get_amounts_params(param: &AchainableAmounts) -> Result<(String, String, String, String)> {
	let name = param.name.clone();
	let chain = param.chain.clone();
	let amount1 = param.amount1.clone();
	let amount2 = param.amount2.clone();

	let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amounts(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amounts(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let amount1 = vec_to_string(amount1.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amounts(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let amount2 = vec_to_string(amount2.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amounts(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	Ok((name, chain, amount1, amount2))
}
