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

use crate::*;
use lc_credentials::brc20::amount_holder::BRC20AmountHolderCredential;
use lc_data_providers::{
	geniidata::GeniidataClient, DataProviderConfigReader, ReadDataProviderConfig,
};

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);

	// TODO: here still missing one step: convert 'address', which is a pubkey, into real BTC/BRC20 address,
	// which the data provider (GeniiData.com) can recognize.
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::BRC20AmountHolder, e))?;
	let mut client = GeniidataClient::new(&data_provider_config);

	let response = client.create_brc20_amount_holder_sum(addresses).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::BRC20AmountHolder,
			ErrorDetail::DataProviderError(ErrorString::truncate_from(
				format!("{e:?}").as_bytes().to_vec(),
			)),
		)
	})?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_brc20_amount_holder_credential(&response);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::BRC20AmountHolder, e.into_error_detail()))
		},
	}
}
