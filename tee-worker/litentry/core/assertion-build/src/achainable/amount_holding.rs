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

use crate::{achainable::request_achainable_balance, *};
use lc_credentials::{litentry_profile::token_balance::TokenBalanceInfo, IssuerRuntimeVersion};
use lc_data_providers::{DataProviderConfig, ETokenAddress, TokenFromString};

pub fn build_amount_holding(
	req: &AssertionBuildRequest,
	param: AchainableAmountHolding,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let token = ETokenAddress::from_vec(param.clone().token.unwrap_or_default());
	let achainable_param = AchainableParams::AmountHolding(param);
	let balance =
		request_achainable_balance(addresses, achainable_param.clone(), data_provider_config)?
			.parse::<f64>()
			.map_err(|_| {
				Error::RequestVCFailed(
					Assertion::Achainable(achainable_param.clone()),
					ErrorDetail::ParseError,
				)
			})?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
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
