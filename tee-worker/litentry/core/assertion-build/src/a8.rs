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

use crate::{Error, Result};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsNetwork, VerifiedCredentialsTotalTxs,
};
use litentry_primitives::{
	Assertion, Identity, IndexingNetwork, IndexingNetworks, ParentchainBlockNumber,
};
use log::*;
use std::{collections::HashSet, string::String, vec, vec::Vec};

const VC_SUBJECT_DESCRIPTION: &str = "User has over X number of transactions";
const VC_SUBJECT_TYPE: &str = "Total EVM and Substrate Transactions";

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
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.add_assertion_a8(target_networks, min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A8(networks), e.to_error_detail()))
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

fn get_total_tx_ranges(total_txs: u64) -> (u64, u64) {
	let min: u64;
	let max: u64;

	match total_txs {
		0 | 1 => {
			min = 0;
			max = 1;
		},
		2..=10 => {
			min = 1;
			max = 10;
		},
		11..=100 => {
			min = 10;
			max = 100;
		},
		101..=1000 => {
			min = 100;
			max = 1000
		},
		1001..=10000 => {
			min = 1000;
			max = 10000;
		},
		10001..=u64::MAX => {
			min = 10000;
			max = u64::MAX;
		},
	}

	(min, max)
}

#[cfg(test)]
mod tests {
	use super::*;
	use litentry_primitives::IndexingNetworks;
	#[test]
	fn assertion_networks_to_verifed_network_works() {
		let litentry = IndexingNetwork::Litentry;
		let mut networks = IndexingNetworks::with_bounded_capacity(1);

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
}
