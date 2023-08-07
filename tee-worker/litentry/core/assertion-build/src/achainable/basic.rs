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

/// NOTE:
/// Build is uniswap v2/v3 user
/// name: Because it is necessary to request the interface four times to determine whether the uniswapv2/v3 user requires it, we can agree on the name here as IsUniswapV23User.
/// chain: ethereum
///
/// assertions":[
/// {
///        {
///            src:$is_uniswap_v2_user,
///            op:==,
///            dst:true
///        },
///        {
///            src: is_uniswap_v3_user,
///            op: ==,
///            dst: true
///         }
/// }
///
pub fn build_basic(req: &AssertionBuildRequest, param: AchainableBasic) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

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

fn get_uniswap_v23_info() -> (&'static str, &'static str) {
	(
		"You are a trader or liquidity provider of Uniswap V2 or V3
	Uniswap V2 Factory Contract: 0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f
	Uniswap V3 Factory Contract: 0x1f98431c8ad98523631ae4a59f267346ea31f984",
		"Uniswap V2/V3 User",
	)
}
