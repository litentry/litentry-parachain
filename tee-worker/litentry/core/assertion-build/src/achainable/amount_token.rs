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

use crate::{
	achainable::{query_lit_holding_amount, request_achainable_balance},
	*,
};
use lc_credentials::{achainable::lit_holding_amount::AchainableLitHoldingAmountUpdate, litentry_profile::token_balance::TokenBalanceInfo};
use lc_data_providers::{
	achainable_names::{AchainableNameAmountToken, GetAchainableName},
	ETokenAddress, TokenFromString,
};

const LIT_HOLDING_AMOUNT_NAME: &str = "LIT Holding Amount";

/// USDC / USDT Holder
/// assertions:[
/// {
///    and:[
///        {
///            src:$usdc_holding_amount,
///            op: >=,
///            dst:100
///        },
///        {
///            src:$usdc_holding_amount,
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
	debug!("Assertion Building AchainableAmountToken");

	let identities = transpose_identity(&req.identities);
	let achainable_param = AchainableParams::AmountToken(param.clone());

	// LIT Holding Amount
	// Since "LIT Holding Amount" is a custom name in this context, we need to differentiate it by identifying which VC it refers to.
	if is_lit_holding_amount(&achainable_param)? {
		let lit_holding_amount = query_lit_holding_amount(&achainable_param, &identities)?;

		return match Credential::new(&req.who, &req.shard) {
			Ok(mut credential_unsigned) => {
				credential_unsigned.update_lit_holding_amount(lit_holding_amount);
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

	// USDC / USDT Holder
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();
	let token = ETokenAddress::from_vec(param.token.unwrap_or_default());
	let balance = request_achainable_balance(addresses, achainable_param.clone())?
		.parse::<f64>()
		.map_err(|_| {
			Error::RequestVCFailed(
				Assertion::Achainable(achainable_param.clone()),
				ErrorDetail::ParseError,
			)
		})?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_token_balance(token, balance);
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

fn is_lit_holding_amount(param: &AchainableParams) -> Result<bool> {
	let name_amount_token = AchainableNameAmountToken::from(param.name()).map_err(|e| {
		Error::RequestVCFailed(Assertion::Achainable(param.clone()), e.into_error_detail())
	})?;

	Ok(name_amount_token.name() == LIT_HOLDING_AMOUNT_NAME)
}
