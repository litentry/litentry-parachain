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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use http_req_sgx as http_req;
	pub use http_sgx as http;
	pub use rust_base58_sgx as rust_base58;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

pub mod a1;
pub mod a10;
pub mod a11;
pub mod a12;
pub mod a13;
pub mod a14;
pub mod a2;
pub mod a3;
pub mod a4;
pub mod a5;
pub mod a6;
pub mod a7;
pub mod a8;

use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{
	Assertion, ErrorDetail, ErrorString, Identity, IdentityNetworkTuple, IntoErrorDetail,
	ParameterString, VCMPError as Error, Web3Network, ASSERTION_FROM_DATE,
};
use log::*;
use std::{collections::HashSet, format, string::String, vec, vec::Vec};

pub type Result<T> = core::result::Result<T, Error>;

/// Transpose a vector of identities with web3network information, which is Vec<IdentityNetworkTuple>,
/// to the vector of hex addresses which share the same network type, which is Vec<(Web3Network, Vec<String>)>.
///
/// TODO: improve the logic
pub fn transpose_identity(
	identities: &Vec<IdentityNetworkTuple>,
) -> Vec<(Web3Network, Vec<String>)> {
	let mut addresses: Vec<(String, Web3Network)> = vec![];
	let mut networks_set: HashSet<Web3Network> = HashSet::new();
	for (id, networks) in identities {
		networks.clone().into_iter().for_each(|n| {
			match id {
				Identity::Substrate(address) => {
					let mut address = account_id_to_string(address.as_ref());
					address.insert_str(0, "0x");
					addresses.push((address, n));
					networks_set.insert(n);
				},
				Identity::Evm(address) => {
					let mut address = account_id_to_string(address.as_ref());
					address.insert_str(0, "0x");
					addresses.push((address, n));
					networks_set.insert(n);
				},
				_ => {},
			};
		});
	}

	networks_set
		.iter()
		.map(|n| {
			let address: Vec<String> = addresses
				.iter()
				.filter(|(_, network)| n == network)
				.map(|(addr, _)| addr.clone())
				.collect();
			(*n, address)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_utils::ToHexPrefixed;

	#[test]
	fn transpose_identity_works() {
		let mut identities: Vec<IdentityNetworkTuple> = vec![];
		let id1 = Identity::Twitter("alice1".as_bytes().to_vec().try_into().unwrap());
		let id2 = Identity::Substrate([2u8; 32].into());
		let id3 = Identity::Substrate([3u8; 32].into());
		let id4 = Identity::Evm([4u8; 20].into());

		let network1: Vec<Web3Network> = vec![];
		let network2 = vec![Web3Network::Polkadot, Web3Network::Litentry];
		let network3 = vec![Web3Network::Litentry, Web3Network::Khala, Web3Network::Kusama];
		let network4 = vec![Web3Network::BSC];

		identities.push((id1, network1));
		identities.push((id2, network2));
		identities.push((id3, network3));
		identities.push((id4, network4));

		let mut result = transpose_identity(&identities);
		result.sort();
		assert_eq!(result.len(), 5);
		assert_eq!(result.get(0).unwrap(), &(Web3Network::Polkadot, vec![[2u8; 32].to_hex()]));
		assert_eq!(result.get(1).unwrap(), &(Web3Network::Kusama, vec![[3u8; 32].to_hex()]));
		assert_eq!(
			result.get(2).unwrap(),
			&(Web3Network::Litentry, vec![[2u8; 32].to_hex(), [3u8; 32].to_hex()])
		);
		assert_eq!(result.get(3).unwrap(), &(Web3Network::Khala, vec![[3u8; 32].to_hex()]));
		assert_eq!(result.get(4).unwrap(), &(Web3Network::BSC, vec![[4u8; 20].to_hex()]));
	}
}
