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
	achainable::{Params, ParamsBasicTypeWithAmountHolding},
	ConvertParameterString,
};

const VC_SUBJECT_DESCRIPTION: &str = "Achainable amount holding";
const VC_SUBJECT_TYPE: &str = "Amount holding";

pub fn build_amount_holding(
	req: &AssertionBuildRequest,
	param: AchainableAmountHolding,
) -> Result<Credential> {
	debug!("Assertion Achainable build_amount_holding, who: {:?}", account_id_to_string(&req.who));

	let (name, amount, date, token) = parse_amount_holding_params(&param)?;
	let p = ParamsBasicTypeWithAmountHolding::one(
		name,
		&param.chain,
		amount.clone(),
		date.clone(),
		token,
	);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let flag = request_achainable(addresses, Params::ParamsBasicTypeWithAmountHolding(p.clone()))?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.update_holder(flag, &amount, &date);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::AmountHolding(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn parse_amount_holding_params(
	param: &AchainableAmountHolding,
) -> Result<(String, String, String, Option<String>)> {
	let name = param.name.to_string();
	let amount = param.amount.to_string();
	let date = param.date.to_string();
	let token = param.token.as_ref().map(|v| v.to_string());

	Ok((name, amount, date, token))
}
