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
use lc_data_providers::achainable::{AchainableClient, AchainableTotalTransactions};

const VC_A8_SUBJECT_DESCRIPTION: &str = "The total amount of transaction the user has ever made in each of the available networks (including invalid transactions)";
const VC_A8_SUBJECT_TYPE: &str = "EVM/Substrate Transaction Count on Networks";
const VC_A8_SUBJECT_TAG: [&str; 6] =
	["Litentry", "Litmus", "Polkadot", "Kusama", "Ethereum", "Khala"];

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	debug!("Assertion A8 build, who: {:?}", account_id_to_string(&req.who),);

	let mut client = AchainableClient::new();
	let mut total_txs: u64 = 0;

	let identities: Vec<(Web3Network, Vec<String>)> = transpose_identity(&req.vec_identity);
	let mut networks_set: HashSet<Web3Network> = HashSet::new();
	identities.iter().for_each(|(network, addresses)| {
		networks_set.insert(*network);

		match client.total_transactions(network, addresses) {
			Ok(txs) => total_txs += txs,
			Err(e) => error!("Assertion A8 query total_transactions error: {:?}", e),
		};
	});
	debug!("Assertion A8 total_transactions: {}", total_txs);

	let networks = networks_set.into_iter().collect::<Vec<Web3Network>>();

	let (min, max) = get_total_tx_ranges(total_txs);
	match Credential::new_default(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A8_SUBJECT_DESCRIPTION,
				VC_A8_SUBJECT_TYPE,
				VC_A8_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.add_assertion_a8(networks, min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			// In fact, it should never fail
			let bounded_web3networks =
				req.assertion.get_supported_web3networks().try_into().unwrap();
			Err(Error::RequestVCFailed(Assertion::A8(bounded_web3networks), e.into_error_detail()))
		},
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
	use core::assert_eq;

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
