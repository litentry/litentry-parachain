// Copyright 2020-2024 Trust Computing GmbH.
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
use lc_credentials::{
	achainable::lit_holding_amount::AchainableLitHoldingAmountUpdate,
	litentry_profile::token_balance::TokenBalanceInfo, IssuerRuntimeVersion,
};
use lc_data_providers::{
	achainable_names::AchainableNameAmountToken, DataProviderConfig, ETokenAddress, TokenFromString,
};

/// ERC20 Holder: USDC and others
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
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Assertion Building AchainableAmountToken");

	let identities = transpose_identity(&req.identities);
	let achainable_param = AchainableParams::AmountToken(param.clone());

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	let mut credential = Credential::new(&req.who, &req.shard, &runtime_version).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::Achainable(achainable_param.clone()),
			e.into_error_detail(),
		)
	})?;

	let amount_token_name =
		AchainableNameAmountToken::from(achainable_param.name()).map_err(|e| {
			Error::RequestVCFailed(
				Assertion::Achainable(achainable_param.clone()),
				e.into_error_detail(),
			)
		})?;
	match amount_token_name {
		AchainableNameAmountToken::LITHoldingAmount => {
			let lit_holding_amount =
				query_lit_holding_amount(&achainable_param, identities, data_provider_config)?;
			credential.update_lit_holding_amount(lit_holding_amount);
		},
		_ => {
			// Token Holder
			let addresses = identities
				.into_iter()
				.flat_map(|(_, addresses)| addresses)
				.collect::<Vec<String>>();
			let token = ETokenAddress::from_vec(param.token.unwrap_or_default());
			let balance = request_achainable_balance(
				addresses,
				achainable_param.clone(),
				data_provider_config,
			)?
			.parse::<f64>()
			.map_err(|_| {
				Error::RequestVCFailed(
					Assertion::Achainable(achainable_param.clone()),
					ErrorDetail::ParseError,
				)
			})?;

			credential.update_token_balance(token, balance);
		},
	}

	Ok(credential)
}
