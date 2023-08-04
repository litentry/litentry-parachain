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
	achainable::{Params, ParamsBasicTypeWithToken},
	vec_to_string,
};

pub fn build_token(req: &AssertionBuildRequest, param: AchainableToken) -> Result<Credential> {
	debug!("Assertion Achainable build_token, who: {:?}", account_id_to_string(&req.who));

	let (name, token) = parse_token_params(&param)?;
	let p = ParamsBasicTypeWithToken::new(name, &param.chain, token);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let _flag = request_achainable(addresses, Params::ParamsBasicTypeWithToken(p.clone()))?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut _credential_unsigned) => Ok(_credential_unsigned),
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::Token(param)),
				e.into_error_detail(),
			))
		},
	}
}

fn parse_token_params(param: &AchainableToken) -> Result<(String, String)> {
	let name = param.clone().name;
	let token = param.clone().token;
	let name = vec_to_string(name.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Token(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;
	let token = vec_to_string(token.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Token(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	Ok((name, token))
}
