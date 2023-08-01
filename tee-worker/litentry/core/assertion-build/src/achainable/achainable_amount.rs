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
	achainable::{Params, ParamsBasicTypeWithAmount},
	vec_to_string,
};

const CREATED_OVER_AMOUNT_CONTRACTS: &str = "Created over {amount} contracts";
const BALANCE_OVER_AMOUNT: &str = "Balance over {amount}";

/// NOTE:
/// Build Contract Creator Assertion Params
/// name: "Created over {amount} contracts"
/// chain: "ethereum",
/// amount: "0",
/// 
/// "assertions":[
/// {
///		"and":[
/// 		{
/// 			"src":"$is_contract_creator",
/// 			"op":"==",
/// 			"dst":"true"
/// 		}
/// 	]
/// }
/// 
/// 
/// Build ETH holder Assertion Params
/// name: Balance over {amount}
/// chain: "ethereum",
/// amount: "0",
/// 
/// assertions":[
/// {
///		"and":[
/// 		{
/// 			"src":"$is_eth_holder",
/// 			"op":"==",
/// 			"dst":"true"
/// 		}
/// 	]
/// }
/// 
pub fn build_amount(req: &AssertionBuildRequest, param: AchainableAmount) -> Result<Credential> {
	debug!("Assertion Achainable build_amount, who: {:?}", account_id_to_string(&req.who));

	let (name, chain, amount) = get_amount_params(&param)?;
	let p = ParamsBasicTypeWithAmount::new(name.clone(), chain, amount);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let flag = request_achainable(addresses, Params::ParamsBasicTypeWithAmount(p.clone()))?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			let (desc, subtype, content) = get_assertion_content(&name); 
			credential_unsigned.add_subject_info(desc, subtype);
			credential_unsigned.update_content(flag, content);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::Amount(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn get_amount_params(param: &AchainableAmount) -> Result<(String, String, String)> {
	let name = param.name.clone();
	let chain = param.chain.clone();
	let amount = param.amount.clone();

	let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amount(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amount(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let amount = vec_to_string(amount.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Amount(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	Ok((name, chain, amount))
}

/// TODO: info needs update
fn get_assertion_content(name: &String) -> (&str, &str, &str) {
	let assertion = if name == CREATED_OVER_AMOUNT_CONTRACTS {
		("", "", "$is_contract_creator")
	} else if name == BALANCE_OVER_AMOUNT {
		("", "", "$is_eth_holder")
	} else {
		("", "", "")
	};

	assertion
}