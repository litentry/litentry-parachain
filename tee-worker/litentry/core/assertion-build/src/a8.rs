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

use crate::Result;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_data_providers::graphql::{GraphQLClient, VerifiedCredentialsTotalTxs, VerifiedCredentialsNetwork};
use litentry_primitives::{Assertion, Identity, ParentchainBlockNumber, AssertionNetworks, Network};
use log::*;
use parachain_core_primitives::VCMPError;
use std::{str::from_utf8, string::ToString, vec, vec::Vec, collections::{HashMap, HashSet}};

pub fn build(
	identities: Vec<Identity>,
	networks: AssertionNetworks,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	log::debug!("	[AssertionBuild] A8 networks: {:?}", networks);

	let mut client = GraphQLClient::new();
	let mut total_txs: u64 = 0;

	let mut sub_maps = HashMap::<[u8; 32], HashSet<VerifiedCredentialsNetwork>>::new();
	let mut eth_maps = HashMap::<[u8; 20], HashSet<VerifiedCredentialsNetwork>>::new();

	for identity in identities {
		match identity {
			Identity::Substrate { network, address } => {
				let key = address.as_ref();
				if sub_maps.contains_key(key) {
					let values = sub_maps.get_mut(key).unwrap();
					values.insert(network.into());
				} else {
					let mut values = HashSet::<VerifiedCredentialsNetwork>::new();
					values.insert(network.into());
					sub_maps.insert(key.clone(), values);
				}
			},
			Identity::Evm { network, address } => {
				let key = address.as_ref();
				if eth_maps.contains_key(key) {
					let values = eth_maps.get_mut(key).unwrap();
					values.insert(network.into());
				} else {
					let mut values = HashSet::<VerifiedCredentialsNetwork>::new();
					values.insert(network.into());
					eth_maps.insert(key.clone(), values);
				}
			},
			_ => {
				debug!("ignore identity: {:?}", identity);
			},
		};
	}

	let available_networks = if !networks.is_empty() {
		let mut set = HashSet::new();
		for n in networks {
			set.insert(n);
		}

		set
	 } else {
		let litentry = Network::try_from("litentry".as_bytes().to_vec()).unwrap();
		let litmus = Network::try_from("litmus".as_bytes().to_vec()).unwrap();
		let polkadot = Network::try_from("polkadot".as_bytes().to_vec()).unwrap();
		let kusama = Network::try_from("kusama".as_bytes().to_vec()).unwrap();
		let khala = Network::try_from("khala".as_bytes().to_vec()).unwrap();
		let ethereum = Network::try_from("ethereum".as_bytes().to_vec()).unwrap();
		VerifiedCredentialsNetwork::from()
		HashSet::from([litentry, litmus, kusama, polkadot, khala, ethereum])

	};

	// substrate networks
	for (&key, mut v) in sub_maps.iter_mut() {
		let intersection: HashSet<_> = *v.intersection(&available_networks).collect();
		*v = intersection;
	}

	// eth networks
	for (&key, mut v) in eth_maps.iter_mut() {
		let intersection: HashSet<_> = *v.intersection(&available_networks).collect();
		*v = intersection;
	}

	for (addr, v) in sub_maps {
		let query = from_utf8(addr.as_ref()).map_or(None, |addr| {
			Some(VerifiedCredentialsTotalTxs::new(
				vec![addr.to_string()],
				Vec::from_iter(v),
			))
		});

		if let Some(query) = query {
			if let Ok(result) = client.query_total_transactions(query) {
				total_txs += result.iter().map(|v| v.total_transactions).sum::<u64>();
			}
		}
	}

	for (addr, v) in eth_maps {
		let query = from_utf8(addr.as_ref()).map_or(None, |addr| {
			Some(VerifiedCredentialsTotalTxs::new(
				vec![addr.to_string()],
				Vec::from_iter(v),
			))
		});

		if let Some(query) = query {
			if let Ok(result) = client.query_total_transactions(query) {
				total_txs += result.iter().map(|v| v.total_transactions).sum::<u64>();
			}
		}
	}
	
	debug!("total_transactions: {}", total_txs);

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

	let a8 = Assertion::A8(networks);
	match Credential::generate_unsigned_credential(&a8, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_assertion_a8(min, max);
			credential_unsigned.credential_subject.values.push(true);
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(VCMPError::Assertion8Failed)
}
