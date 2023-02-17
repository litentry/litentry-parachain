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

use crate::{from_data_provider_error, Error, Result};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};
use litentry_primitives::{
	Assertion, Balance, Identity, ParentchainBlockNumber, ASSERTION_FROM_DATE,
};
use log::*;
use std::{
	str::from_utf8,
	string::{String, ToString},
	vec,
	vec::Vec,
};

// ERC20 LIT token address
const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

pub fn build(
	identities: Vec<Identity>,
	min_balance: Balance,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	let mut client = GraphQLClient::new();
	let mut found = false;
	let mut from_date_index = 0_usize;

	for identity in identities.iter() {
		if found {
			break
		}

		let mut verified_network = VerifiedCredentialsNetwork::Polkadot;
		if identity.is_web3() {
			match identity {
				Identity::Substrate { network, .. } => verified_network = (*network).into(),
				Identity::Evm { network, .. } => verified_network = (*network).into(),
				_ => {},
			}
		}
		if matches!(
			verified_network,
			VerifiedCredentialsNetwork::Litentry
				| VerifiedCredentialsNetwork::Litmus
				| VerifiedCredentialsNetwork::Ethereum
		) {
			let q_min_balance: f64 = if verified_network == VerifiedCredentialsNetwork::Litentry
				|| verified_network == VerifiedCredentialsNetwork::Litmus
			{
				(min_balance / (10 ^ 12)) as f64
			} else {
				(min_balance / (10 ^ 18)) as f64
			};

			let mut addresses: Vec<String> = vec![];
			match &identity {
				Identity::Evm { address, .. } =>
					addresses.push(from_utf8(address.as_ref()).unwrap().to_string()),
				Identity::Substrate { address, .. } =>
					addresses.push(from_utf8(address.as_ref()).unwrap().to_string()),
				Identity::Web2 { address, .. } =>
					addresses.push(from_utf8(address).unwrap().to_string()),
			}
			let mut tmp_token_addr = String::from("");
			if verified_network == VerifiedCredentialsNetwork::Ethereum {
				tmp_token_addr = LIT_TOKEN_ADDRESS.to_string();
			}

			for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
				// if found is true, no need to check it continually
				if found {
					from_date_index = index + 1;
					break
				}

				let credentials = VerifiedCredentialsIsHodlerIn {
					addresses: addresses.clone(),
					from_date: from_date.to_string(),
					network: verified_network.clone(),
					token_address: tmp_token_addr.clone(),
					min_balance: q_min_balance,
				};
				let is_hodler_out = client
					.check_verified_credentials_is_hodler(credentials)
					.map_err(from_data_provider_error)?;

				for holder in is_hodler_out.verified_credentials_is_hodler.iter() {
					found = found || holder.is_hodler;
				}
			}
		}
	}

	let a4 = Assertion::A4(min_balance);
	match Credential::generate_unsigned_credential(&a4, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_holder(from_date_index, min_balance);
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(Error::Assertion4Failed)
}
