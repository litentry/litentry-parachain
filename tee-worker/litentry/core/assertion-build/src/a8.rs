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

use crate::*;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsNetwork, VerifiedCredentialsTotalTxs,
};
use log::*;
use std::{collections::HashSet, string::String, vec, vec::Vec};

const VC_A8_SUBJECT_DESCRIPTION: &str = "The total amount of transaction the user has ever made in each of the available networks (including invalid transactions)";
const VC_A8_SUBJECT_TYPE: &str = "EVM/Substrate Transaction Count on Networks";
const VC_A8_SUBJECT_TAG: [&str; 6] =
	["Litentry", "Litmus", "Polkadot", "Kusama", "Ethereum", "Khala"];

pub const INDEXING_NETWORKS: [IndexingNetwork; 6] = [
	IndexingNetwork::Litentry,
	IndexingNetwork::Litmus,
	IndexingNetwork::Polkadot,
	IndexingNetwork::Khala,
	IndexingNetwork::Ethereum,
	IndexingNetwork::Kusama,
];

pub fn build(
	identities: Vec<Identity>,
	networks: IndexingNetworks,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	debug!(
		"Assertion A8 build, who: {:?}, bn: {}, identities: {:?}, networks:{:?}",
		account_id_to_string(&who),
		bn,
		identities,
		networks
	);

	let mut client = GraphQLClient::new();
	let mut total_txs: u64 = 0;
	let target_networks = to_verifed_network(networks.clone());

	let mut verified_addresses = HashSet::<String>::new();
	let mut verified_networks = HashSet::<VerifiedCredentialsNetwork>::new();

	identities.iter().for_each(|identity| match identity {
		Identity::Substrate { network, address } => {
			let mut address = account_id_to_string(address.as_ref());
			address.insert_str(0, "0x");

			if_match_network_collect_address(
				&target_networks,
				(*network).into(),
				address,
				&mut verified_networks,
				&mut verified_addresses,
			);
		},
		Identity::Evm { network, address } => {
			let mut address = account_id_to_string(address.as_ref());
			address.insert_str(0, "0x");

			if_match_network_collect_address(
				&target_networks,
				(*network).into(),
				address,
				&mut verified_networks,
				&mut verified_addresses,
			);
		},
		_ => {},
	});

	if !verified_addresses.is_empty() && !verified_networks.is_empty() {
		let addresses = verified_addresses.into_iter().collect();
		let networks = verified_networks.into_iter().collect();
		let query = VerifiedCredentialsTotalTxs::new(addresses, networks);

		if let Ok(result) = client.query_total_transactions(query) {
			total_txs += result.iter().map(|v| v.total_transactions).sum::<u64>();
		}
	}

	debug!("Assertion A8 total_transactions: {}", total_txs);

	let (min, max) = get_total_tx_ranges(total_txs);
	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A8_SUBJECT_DESCRIPTION,
				VC_A8_SUBJECT_TYPE,
				VC_A8_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_assertion_a8(target_networks, min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A8(networks), e.into_error_detail()))
		},
	}
}

fn to_verifed_network(networks: IndexingNetworks) -> Vec<VerifiedCredentialsNetwork> {
	let mut target_networks = vec![];

	if networks.is_empty() {
		// return all networks
		INDEXING_NETWORKS.iter().for_each(|network| {
			let vnetwork = VerifiedCredentialsNetwork::from(network.clone());
			target_networks.push(vnetwork);
		})
	} else {
		networks.iter().for_each(|network| {
			let vnetwork = VerifiedCredentialsNetwork::from(network.clone());
			target_networks.push(vnetwork);
		});
	}

	target_networks
}

fn if_match_network_collect_address(
	target_networks: &[VerifiedCredentialsNetwork],
	network: VerifiedCredentialsNetwork,
	address: String,
	verified_networks: &mut HashSet<VerifiedCredentialsNetwork>,
	verified_addresses: &mut HashSet<String>,
) {
	if target_networks.contains(&network) {
		verified_networks.insert(network);
		verified_addresses.insert(address);
	}
}

/*
total transactions count range:

≥ 10000
≥ 1000
≥ 100
≥ 10
≥ 1

0 		<= X < 1
1 		<= X < 10
10 		<= X < 100
100 	<= X < 1000
1000 	<= X < 10000
10000 	<= X < u64::Max

*/
fn get_total_tx_ranges(total_txs: u64) -> (u64, u64) {
	let min: u64;
	let max: u64;

	match total_txs {
		0 => {
			min = 0;
			max = 1;
		},
		1..=9 => {
			min = 1;
			max = 10;
		},
		10..=99 => {
			min = 10;
			max = 100;
		},
		100..=999 => {
			min = 100;
			max = 1000
		},
		1000..=9999 => {
			min = 1000;
			max = 10000;
		},
		10000..=u64::MAX => {
			min = 10000;
			max = u64::MAX;
		},
	}

	(min, max)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assertion_networks_to_verifed_network_works() {
		let litentry = IndexingNetwork::Litentry;
		let mut networks = IndexingNetworks::with_bounded_capacity(1);
		networks.try_push(litentry.clone()).unwrap();

		let v_networks = to_verifed_network(networks.clone());
		assert_eq!(v_networks.len(), 1);

		let target_network = VerifiedCredentialsNetwork::from(litentry);
		assert_eq!(v_networks[0], target_network);
	}

	#[test]
	fn assertion_networks_to_verifed_network_non_works() {
		let networks = IndexingNetworks::with_bounded_capacity(1);
		let left = to_verifed_network(networks.clone());

		let mut right = vec![];
		INDEXING_NETWORKS.iter().for_each(|network| {
			let vnetwork = VerifiedCredentialsNetwork::from(network.clone());
			right.push(vnetwork);
		});

		assert_eq!(left, right);
	}

	#[test]
	fn assertion_networks_if_match_network_collect_address_works() {
		let mut verified_addresses = HashSet::<String>::new();
		let mut verified_networks = HashSet::<VerifiedCredentialsNetwork>::new();

		let mut address_litentry =
			"44f0633d7273a1e5bee1e54937dbb1cdfc0b210582b913c0fb3c7c7b9cdca9b9".to_string();
		address_litentry.insert_str(0, "0x");

		let mut address_polkadot =
			"44f0633d7273a1e5bee1e54937dbb1cdfc0b210582b913c0fb3c7c7b9cdca9b1".to_string();
		address_polkadot.insert_str(0, "0x");

		let mut target_networks = vec![];
		INDEXING_NETWORKS.iter().for_each(|network| {
			let vnetwork = VerifiedCredentialsNetwork::from(network.clone());
			target_networks.push(vnetwork);
		});

		let networks = [VerifiedCredentialsNetwork::Litentry, VerifiedCredentialsNetwork::Polkadot];
		let addresses = [
			"0x44f0633d7273a1e5bee1e54937dbb1cdfc0b210582b913c0fb3c7c7b9cdca9b9".to_string(),
			"0x44f0633d7273a1e5bee1e54937dbb1cdfc0b210582b913c0fb3c7c7b9cdca9b1".to_string(),
		];

		if_match_network_collect_address(
			&target_networks,
			VerifiedCredentialsNetwork::Litentry,
			address_litentry,
			&mut verified_networks,
			&mut verified_addresses,
		);
		if_match_network_collect_address(
			&target_networks,
			VerifiedCredentialsNetwork::Polkadot,
			address_polkadot,
			&mut verified_networks,
			&mut verified_addresses,
		);

		verified_networks
			.iter()
			.for_each(|network| assert!(networks.contains(&network)));

		verified_addresses
			.iter()
			.for_each(|address| assert!(addresses.contains(&address)));
	}

	#[test]
	fn get_total_tx_ranges_works() {
		let (min, max) = get_total_tx_ranges(0);
		assert_eq!(min, 0);
		assert_eq!(max, 1);

		let (min, max) = get_total_tx_ranges(5);
		assert_eq!(min, 1);
		assert_eq!(max, 10);

		let (min, max) = get_total_tx_ranges(10);
		assert_eq!(min, 10);
		assert_eq!(max, 100);
	}
}
