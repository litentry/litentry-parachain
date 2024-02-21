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

use lc_common::{
	web3_network_to_chain,
	web3_nft::{NftAddress, NftName},
};
use litentry_primitives::{Web3Network, Web3NftType};

// TODO migration to v2 in the future
use lc_credentials::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};

const TYPE: &str = "NFT Holder";
const DESCRIPTION: &str = "You are a holder of a certain kind of NFT";

struct AssertionKeys {
	nft: &'static str,
	network: &'static str,
	address: &'static str,
}

const ASSERTION_KEYS: AssertionKeys =
	AssertionKeys { nft: "$nft", network: "$network", address: "$address" };

pub trait NFTHolderAssertionUpdate {
	fn update_nft_holder_assertion(&mut self, nft_type: Web3NftType, has_nft: bool);
}

impl NFTHolderAssertionUpdate for Credential {
	fn update_nft_holder_assertion(&mut self, nft_type: Web3NftType, has_nft: bool) {
		self.add_subject_info(DESCRIPTION, TYPE);

		let mut assertion = AssertionLogic::new_and();

		assertion = assertion.add_item(AssertionLogic::new_item(
			ASSERTION_KEYS.nft,
			Op::Equal,
			nft_type.get_nft_name(),
		));

		let mut network_assertion: AssertionLogic = AssertionLogic::new_or();
		for network in nft_type.get_supported_networks() {
			network_assertion = network_assertion
				.add_item(create_network_assertion_logic(network, nft_type.clone()));
		}

		assertion = assertion.add_item(network_assertion);
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(has_nft);
	}
}

fn create_network_assertion_logic(network: Web3Network, nft_type: Web3NftType) -> AssertionLogic {
	let mut assertion = AssertionLogic::new_and();
	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.network,
		Op::Equal,
		web3_network_to_chain(&network),
	));
	if let Some(address) = nft_type.get_nft_address(network) {
		assertion = assertion.add_item(AssertionLogic::new_item(
			ASSERTION_KEYS.address,
			Op::Equal,
			address,
		));
	}
	assertion
}
