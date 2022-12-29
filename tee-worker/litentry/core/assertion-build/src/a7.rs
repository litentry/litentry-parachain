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

use crate::{from_data_provider_error, Error, Result};
use std::{
	str::from_utf8,
	string::{String, ToString},
	vec,
	vec::Vec,
};

use lc_stf_task_sender::MaxIdentityLength;
use litentry_primitives::{
	Identity, IdentityHandle, IdentityWebType, SubstrateNetwork, Web3Network,
};
use sp_runtime::BoundedVec;

use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};

pub fn build(
	identities: BoundedVec<Identity, MaxIdentityLength>,
	from_date: String,
	min_balance: f64,
) -> Result<()> {
	let mut client = GraphQLClient::new();

	for identity in identities {
		if let IdentityWebType::Web3(Web3Network::Substrate(SubstrateNetwork::Polkadot)) =
			identity.web_type
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
			let credentials = VerifiedCredentialsIsHodlerIn {
				addresses,
				from_date: from_date.clone(),
				network: VerifiedCredentialsNetwork::Polkadot,
				token_address: String::from(""),
				min_balance,
			};
			let _is_holder_out = client
				.check_verified_credentials_is_holder(credentials)
				.map_err(from_data_provider_error)?;
			return Ok(())
		}
	}
	// no valid response
	Err(Error::Assertion7Failed)
}
