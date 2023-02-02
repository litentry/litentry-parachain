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
use litentry_primitives::{Assertion, Identity, ParameterString, ParentchainBlockNumber};
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
	from_date: ParameterString,
	min_balance: u128,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	let mut client = GraphQLClient::new();
	let mut flag = false;

	let q_min_balance: f64 = (min_balance / (10 ^ 12)) as f64;
	let q_from_date = String::from_utf8(from_date.clone().into_inner()).unwrap();

	for identity in identities.iter() {
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
			let credentials = VerifiedCredentialsIsHodlerIn {
				addresses,
				from_date: q_from_date.clone(),
				network: verified_network,
				token_address: tmp_token_addr,
				min_balance: q_min_balance,
			};
			let is_holder_out = client
				.check_verified_credentials_is_hodler(credentials)
				.map_err(from_data_provider_error)?;

			for holder in is_holder_out.verified_credentials_is_hodler.iter() {
				flag = flag || holder.is_hodler;
			}
		}
	}

	let a4 = Assertion::A4(min_balance, from_date);
	match Credential::generate_unsigned_credential(&a4, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.credential_subject.set_value(flag);

			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(Error::Assertion4Failed)
}
