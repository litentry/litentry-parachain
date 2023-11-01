// Copyright 2020-2023 Trust Computing GmbH.
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

use crate::{achainable::request_achainable_balance, *};
use lc_credentials::litentry_profile::token_balance::TokenBalanceInfo;
use lc_data_providers::{ETokenAddress, TokenFromString};

// Input params:
// {
//     "name": "ERC20 balance over {amount}",
//     "address": "0xb59490ab09a0f526cc7305822ac65f2ab12f9723",
//     "params": {
//         "chain": "ethereum",
//         "amount": "0",
//         "token": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
//     }
// }

/// LIT / USDC / USDT Holder
/// assertions:[
/// {
///    and:[
///        {
///            src:$lit_holding_amount,
///            op: >=,
///            dst:100
///        },
///        {
///            src:$lit_holding_amount,
///            op: <,
///            dst:200
///        },
///    ]
/// }
///
///
pub fn build_amount_token(
	req: &AssertionBuildRequest,
	param: AchainableAmountToken,
) -> Result<Credential> {
	debug!("Assertion Achainable build_amount_token, who: {:?}", account_id_to_string(&req.who));

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let token = ETokenAddress::from_vec(param.clone().token.unwrap_or_default());
	let achainable_param = AchainableParams::AmountToken(param);
	let balance = request_achainable_balance(addresses, achainable_param.clone())?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_token_balance(token, &balance);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(achainable_param),
				e.into_error_detail(),
			))
		},
	}
}
