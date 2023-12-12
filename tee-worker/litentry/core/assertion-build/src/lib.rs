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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_large_err)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use hex_sgx as hex;
	pub use http_req_sgx as http_req;
	pub use http_sgx as http;
	pub use rust_base58_sgx as rust_base58;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

pub mod a1;
pub mod a13;
pub mod a14;
pub mod a2;
pub mod a20;
pub mod a3;
pub mod a6;
pub mod a8;
pub mod achainable;
pub mod generic_discord_role;
pub mod holding_time;
pub mod lit_staking;
pub mod nodereal;
pub mod oneblock;
pub mod vip3;

use blake2_rfc::blake2b::Blake2b;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::achainable::web3_network_to_chain;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{
	AchainableAmount, AchainableAmountHolding, AchainableAmountToken, AchainableAmounts,
	AchainableBasic, AchainableBetweenPercents, AchainableDate, AchainableDateInterval,
	AchainableDatePercent, AchainableParams, AchainableToken, Assertion, ErrorDetail, ErrorString,
	Identity, IdentityNetworkTuple, IntoErrorDetail, OneBlockCourseType, ParameterString,
	VCMPError as Error, Web3Network, ASSERTION_FROM_DATE,
};
use log::*;
use rust_base58::ToBase58;
use ss58_registry::Ss58AddressFormat;
use std::{collections::HashSet, format, string::String, sync::Arc, vec, vec::Vec};

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
					let network = web3_network_to_chain(&n);
					let ss58_address = ss58_address_of(address.as_ref(), &network);
					if let Ok(address) = ss58_address {
						addresses.push((address, n));
						networks_set.insert(n);
					}
				},
				Identity::Evm(address) => {
					let address = account_id_to_string(address.as_ref());
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

// mostly copied from https://github.com/hack-ink/substrate-minimal/blob/main/subcryptor/src/lib.rs
// no_std version is used here
pub fn ss58_address_of(
	public_key: &[u8],
	network: &str,
) -> core::result::Result<String, ErrorDetail> {
	let network = Ss58AddressFormat::try_from(network).map_err(|_| ErrorDetail::ParseError)?;
	let prefix = u16::from(network);
	let mut bytes = match prefix {
		0..=63 => vec![prefix as u8],
		64..=16_383 => {
			let first = ((prefix & 0b0000_0000_1111_1100) as u8) >> 2;
			let second = ((prefix >> 8) as u8) | ((prefix & 0b0000_0000_0000_0011) as u8) << 6;

			vec![first | 0b01000000, second]
		},
		_ => Err(ErrorDetail::ParseError)?,
	};

	bytes.extend(public_key);

	let blake2b = {
		let mut context = Blake2b::new(64);
		context.update(b"SS58PRE");
		context.update(&bytes);
		context.finalize()
	};

	bytes.extend(&blake2b.as_bytes()[0..2]);

	Ok(bytes.to_base58())
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_utils::ToHexPrefixed;
	use litentry_primitives::IdentityString;

	#[test]
	fn transpose_identity_works() {
		let mut identities: Vec<IdentityNetworkTuple> = vec![];
		let id1 = Identity::Twitter(IdentityString::new("alice1".as_bytes().to_vec()));
		let id2 = [
			122, 14, 95, 161, 63, 226, 172, 179, 35, 141, 125, 220, 137, 243, 163, 0, 157, 186,
			194, 62, 45, 146, 65, 73, 222, 151, 78, 242, 131, 85, 243, 21,
		]
		.into();
		let id3 = [
			80, 35, 128, 158, 237, 244, 169, 75, 45, 16, 131, 141, 244, 63, 115, 159, 160, 91, 181,
			12, 104, 74, 172, 100, 15, 193, 96, 5, 27, 206, 248, 12,
		]
		.into();
		let id4 = [4_u8; 20].into();

		let network1: Vec<Web3Network> = vec![];
		let network2 = vec![Web3Network::Polkadot, Web3Network::Litentry];
		let network3 = vec![Web3Network::Litentry, Web3Network::Litmus, Web3Network::Kusama];
		let network4 = vec![Web3Network::Bsc];

		identities.push((id1, network1));
		identities.push((id2, network2));
		identities.push((id3, network3));
		identities.push((id4, network4));

		let mut result = transpose_identity(&identities);
		result.sort();
		debug!(">> {result:?}");
		assert_eq!(result.len(), 5);
		assert_eq!(
			result.get(0).unwrap(),
			&(
				Web3Network::Polkadot,
				vec!["13m37Uzx2PHjfGPt15TWXjqKBHTGVKFXKpZdxsQXDG3fL7F5".into()]
			)
		);
		assert_eq!(
			result.get(1).unwrap(),
			&(Web3Network::Kusama, vec!["EPPtryxh22cfPGFQ6KY6Gb2No46hbhTcUuH5myMJNfbPkEf".into()])
		);
		assert_eq!(
			result.get(2).unwrap(),
			&(
				Web3Network::Litentry,
				vec![
					"49AV8EnSwQQGW5wG5ajFEg5VDKtPBzmfAyji34LrZmUn46Rt".into(),
					"48DXPdgeqTPhC5zhfXqE3QJM7sCdHuxZ5ky5vbd5jAujYwyR".into()
				]
			)
		);
		assert_eq!(
			result.get(3).unwrap(),
			&(
				Web3Network::Litmus,
				vec!["jcP3mX494vUPJj8LzjjTmsq2zZrcXxTx1F2xywPY4RGsmMWQx".into()]
			)
		);
		assert_eq!(result.get(4).unwrap(), &(Web3Network::Bsc, vec![[4u8; 20].to_hex()]));
	}
}
