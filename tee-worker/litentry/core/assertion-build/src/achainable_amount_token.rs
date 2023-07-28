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
use lc_data_providers::achainable::{AchainableClient, AchainableTagDeFi};

const VC_SUBJECT_DESCRIPTION: &str = "Uniswap User V2/V3";
const VC_SUBJECT_TYPE: &str = "Uniswap User V2/V3 Assertion";

pub fn build_amount_token(
	req: &AssertionBuildRequest,
	param: AchainableAmountToken,
) -> Result<Credential> {
	debug!("Assertion Achainable build_amount_token, who: {:?}", account_id_to_string(&req.who));

	let (name,chain, amount, token) = get_amount_token_params(&param)?;
	let p = ParamsBasicTypeWithAmountToken::new(name, chain, amount, token);

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut flag = false;
	for address in &addresses {
		if flag {
			break
		}

		let ret = client.query_system_label(address, Params::ParamsBasicTypeWithAmountHolding(p.clone()));
		match ret {
			Ok(r) => flag = r,
			Err(e) => error!("Request query_system_label failed {:?}", e),
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			// credential_unsigned.add_amount_token(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::AmountToken(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn get_amount_token_params(param: &AchainableAmountToken) -> Result<(String, String, String, Option<String>)> {
	let name = param.name.clone();
	let chain = param.chain.clone();
	let amount = param.amount.clone();
	let token = param.token.clone().map(|t| t).unwrap_or_else(|| ParameterString::new());

	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::AmountToken(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let amount = vec_to_string(amount.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::AmountToken(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let token = if param.token.is_some() {
		let token = param.token.clone().unwrap();
		let token = vec_to_string(token.to_vec()).map_err(|_| {
			Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::AmountHolding(param.clone())),
				ErrorDetail::ParseError,
			)
		})?;
		Some(token)
	} else { None };

	Ok((name, chain, amount, token))
}
