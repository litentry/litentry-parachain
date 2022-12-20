// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate::{Error, Result};
use std::{
	str::from_utf8,
	string::{String, ToString},
	vec,
	vec::Vec,
};

use lc_stf_task_sender::MaxIdentityLength;
use litentry_primitives::{
	EvmNetwork, Identity, IdentityHandle, IdentityWebType, SubstrateNetwork, Web3Network,
};
use sp_runtime::BoundedVec;

use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};

const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

pub fn build(
	identities: BoundedVec<Identity, MaxIdentityLength>,
	from_date: String,
	mini_balance: f64,
) -> Result<()> {
	let mut client = GraphQLClient::new();

	for identity in identities {
		let mut network = VerifiedCredentialsNetwork::Polkadot;
		if let IdentityWebType::Web3(web3_type) = identity.web_type {
			match web3_type {
				Web3Network::Substrate(SubstrateNetwork::Litentry) =>
					network = VerifiedCredentialsNetwork::Litentry,
				Web3Network::Substrate(SubstrateNetwork::Litmus) =>
					network = VerifiedCredentialsNetwork::Litmus,
				Web3Network::Evm(EvmNetwork::Ethereum) =>
					network = VerifiedCredentialsNetwork::Ethereum,
				_ => (),
			}
		};

		if network == VerifiedCredentialsNetwork::Litentry
			|| network == VerifiedCredentialsNetwork::Litmus
			|| network == VerifiedCredentialsNetwork::Ethereum
		{
			let mut addresses: Vec<String> = vec![];
			match identity.handle {
				IdentityHandle::Address20(addr) =>
					addresses.push(from_utf8(&addr).unwrap().to_string()),
				IdentityHandle::Address32(addr) =>
					addresses.push(from_utf8(&addr).unwrap().to_string()),
				IdentityHandle::String(addr) =>
					addresses.push(from_utf8(&addr).unwrap().to_string()),
			}
			let mut tmp_token_addr = String::from("");
			if network == VerifiedCredentialsNetwork::Ethereum {
				tmp_token_addr = LIT_TOKEN_ADDRESS.to_string();
			}
			let credentials = VerifiedCredentialsIsHodlerIn {
				addresses,
				from_date: from_date.clone(),
				network,
				token_address: tmp_token_addr,
				mini_balance,
			};
			let is_hodler_out = client.verified_credentials_is_hodler(credentials);
			if let Ok(_hodler_out) = is_hodler_out {
				// TODO: generate VC

				return Ok(())
			}
		}
	}

	Err(Error::Assertion4Error("no valid response".to_string()))
}
