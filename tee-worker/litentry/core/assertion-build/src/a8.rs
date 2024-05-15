// Copyright 2020-2024 Trust Computing GmbH.
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
use lc_common::abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls};
use lc_credentials::IssuerRuntimeVersion;
use lc_data_providers::{
	achainable::{AchainableAccountTotalTransactions, AchainableClient},
	DataProviderConfig, Error as DataProviderError,
};
use litentry_primitives::BoundedWeb3Network;

const VC_A8_SUBJECT_DESCRIPTION: &str = "Gets the range of number of transactions a user has made for a specific token on all supported networks (invalid transactions are also counted)";
const VC_A8_SUBJECT_TYPE: &str = "EVM/Substrate Transaction Count";

pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Assertion A8 build, who: {:?}", account_id_to_string(&req.who),);
	// It should never fail because `req.assertion.get_supported_web3networks()`
	// returns the vector which is converted from a BoundedVec
	let bounded_web3networks: BoundedWeb3Network =
		req.assertion.get_supported_web3networks().try_into().unwrap();
	let mut client = AchainableClient::new(data_provider_config);
	let mut total_txs: u64 = 0;

	let identities: Vec<(Web3Network, Vec<String>)> = transpose_identity(&req.identities);
	let mut networks_set: HashSet<Web3Network> = HashSet::new();

	loop_with_abort_strategy::<fn(&_) -> bool, (Web3Network, Vec<String>), DataProviderError>(
		identities,
		|identity| {
			let network = identity.0;
			let addresses = identity.1.to_vec();

			networks_set.insert(network);

			let txs = client.total_transactions(&network, addresses)?;

			total_txs += txs;

			Ok(LoopControls::Continue)
		},
		AbortStrategy::FailFast::<fn(&_) -> bool>,
	)
	.map_err(|errors| {
		let e = errors[0].clone();
		error!("Assertion A8 query total_transactions error: {:?}", e);
		Error::RequestVCFailed(Assertion::A8(bounded_web3networks.clone()), e.into_error_detail())
	})?;

	debug!("Assertion A8 total_transactions: {}", total_txs);

	let networks = if networks_set.is_empty() {
		req.assertion.get_supported_web3networks()
	} else {
		networks_set.into_iter().collect::<Vec<Web3Network>>()
	};

	let (min, max) = get_total_tx_ranges(total_txs);

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A8_SUBJECT_DESCRIPTION, VC_A8_SUBJECT_TYPE);
			credential_unsigned.add_assertion_a8(networks, min, max);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A8(bounded_web3networks), e.into_error_detail()))
		},
	}
}

/*
Total transactions count range of assertion results:

0 		<= X < 1			=> false
1 		<= X < 10			=> true
10 		<= X < 20			=> true
20 		<= X < 50			=> true
50 		<= X < 100			=> true
100		<= X < 200			=> true
200		<= X < 300			=> true
300		<= X < 500			=> true
500 	<= X 				=> true
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
		10..=19 => {
			min = 10;
			max = 20;
		},
		20..=49 => {
			min = 20;
			max = 50;
		},
		50..=99 => {
			min = 50;
			max = 100;
		},
		100..=199 => {
			min = 100;
			max = 200;
		},
		200..=299 => {
			min = 200;
			max = 300;
		},
		300..=499 => {
			min = 300;
			max = 500;
		},
		500..=u64::MAX => {
			min = 500;
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
		assert_eq!(max, 20);
	}
}
