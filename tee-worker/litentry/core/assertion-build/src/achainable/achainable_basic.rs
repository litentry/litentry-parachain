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

use crate::{achainable::is_uniswap_v2_or_v3_user, *};
use lc_data_providers::vec_to_string;

/// NOTE:
/// Build is uniswap v2/v3 user
/// name: Because it is necessary to request the interface four times to determine whether the uniswapv2/v3 user requires it, we can agree on the name here as IsUniswapV23User.
/// chain: ethereum
/// 
/// assertions":[
/// {
///		"or":[
/// 		{
/// 			"src":"$is_uniswap_v2_user",
/// 			"op":"==",
/// 			"dst":"true"
/// 		},
/// 		{
/// 			"src": "is_uniswap_v3_user",
/// 			"op": "==",
/// 			"dst": "true"
/// 		}
/// 	]
/// }
/// 
pub fn build_basic(req: &AssertionBuildRequest, param: AchainableBasic) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

	let name = param.name.clone();
	let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Basic(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let chain = param.chain.clone();
	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Basic(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let flag = is_uniswap_v2_or_v3_user(addresses)?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {

			let (desc, subtype) = get_uniswap_v23_info();
			credential_unsigned.add_subject_info(desc, subtype);
			credential_unsigned.update_uniswap_v23_info(flag);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::Basic(param)),
				e.into_error_detail(),
			))
		},
	}
}

/// TODO: Info needs update
fn get_uniswap_v23_info() -> (&'static str, &'static str) {
	("", "")
}