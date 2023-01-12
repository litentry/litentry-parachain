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
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};
use litentry_primitives::{EvmNetwork, Identity};
use std::{
	str::from_utf8,
	string::{String, ToString},
	vec,
	vec::Vec,
};

const WBTC_TOKEN_ADDRESS: &str = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599";

// WBTC holder
pub fn build(identities: Vec<Identity>, from_date: String, min_balance: f64) -> Result<()> {
	let mut client = GraphQLClient::new();

	for id in identities {
		if let Identity::Evm { network, address } = id {
			if matches!(network, EvmNetwork::Ethereum) {
				if let Ok(addr) = from_utf8(address.as_ref()) {
					if let Ok(response) = client.check_verified_credentials_is_hodler(
						VerifiedCredentialsIsHodlerIn::new(
							vec![addr.to_string()],
							from_date.clone(),
							VerifiedCredentialsNetwork::Ethereum,
							WBTC_TOKEN_ADDRESS.to_string(),
							min_balance,
						),
					) {
						for item in response.verified_credentials_is_hodler {
							if item.is_hodler {
								// TODO: generate VC
								return Ok(())
							}
						}
					}
				};
			}
		}
	}
	// no valid response
	Err(Error::Assertion7Failed)
}
