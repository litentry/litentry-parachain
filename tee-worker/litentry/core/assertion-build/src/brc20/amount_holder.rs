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

use crate::*;
use lc_credentials::{brc20::amount_holder::BRC20AmountHolderCredential, IssuerRuntimeVersion};
use lc_data_providers::{geniidata::GeniidataClient, DataProviderConfig};

pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut client = GeniidataClient::new(data_provider_config)
		.map_err(|e| Error::RequestVCFailed(Assertion::BRC20AmountHolder, e))?;
	let response = client.create_brc20_amount_holder_sum(addresses).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::BRC20AmountHolder,
			ErrorDetail::DataProviderError(ErrorString::truncate_from(
				format!("{e:?}").as_bytes().to_vec(),
			)),
		)
	})?;

	if response.is_empty() {
		Err(Error::RequestVCFailed(Assertion::BRC20AmountHolder, ErrorDetail::NoEligibleIdentity))
	} else {
		let runtime_version = IssuerRuntimeVersion {
			parachain: req.parachain_runtime_version,
			sidechain: req.sidechain_runtime_version,
		};

		let mut credential_unsigned = Credential::new(&req.who, &req.shard, &runtime_version)
			.map_err(|e| {
				error!("Generate unsigned credential failed {:?}", e);
				Error::RequestVCFailed(Assertion::BRC20AmountHolder, e.into_error_detail())
			})?;
		credential_unsigned.update_brc20_amount_holder_credential(&response);

		Ok(credential_unsigned)
	}
}
