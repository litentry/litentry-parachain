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
use lc_data_providers::{self, DataProviderConfigReader, ReadDataProviderConfig};

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.filter(|(newtwork_type, _)| newtwork_type.is_evm())
		.flat_map(|(newtwork_type, addresses)| {
			addresses.into_iter().map(move |address| (newtwork_type, address))
		})
		.collect::<Vec<(Web3Network, String)>>();

	let mut eth_client = NoderealJsonrpcClient::new(NoderealChain::Eth);

	for address in addresses.iter() {
		let param = GetTokenBalance20Param {
			contract_address: token_type.get_address(address.0).unwrap_or_default().into(),
			address: address.1.clone(),
			block_number: "latest".into(),
		};
		match address.0 {
			Web3Network::Bsc => match bsc_client.get_token_balance_20(&param) {
				Ok(balance) => {
					total_balance += balance;
				},
				Err(err) => return Err(err),
			},
			Web3Network::Ethereum => match eth_client.get_token_balance_20(&param) {
				Ok(balance) => {
					total_balance += balance;
				},
				Err(err) => return Err(err),
			},
			_ => {},
		}
	}

	let data_provider_config = DataProviderConfigReader::read()
		.map_err(|e| Error::RequestVCFailed(Assertion::CryptoSummary, e))?;
	let mut client = AlliumClient::new(&data_provider_config);

	let response = client.create_crypto_summary(addresses).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::CryptoSummary,
			ErrorDetail::DataProviderError(ErrorString::truncate_from(
				format!("{e:?}").as_bytes().to_vec(),
			)),
		)
	})?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_crypto_summary_credential(&response);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::CryptoSummary, e.into_error_detail()))
		},
	}
}
