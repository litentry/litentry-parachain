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
	web3_token::{TokenAddress, TokenName},
};
use litentry_primitives::{Web3Network, Web3TokenType};

// TODO migration to v2 in the future
use lc_credentials::{
	assertion_logic::{AssertionLogic, Op},
	litentry_profile::{BalanceRange, BalanceRangeIndex},
	Credential,
};

const TOKEN_HOLDING_AMOUNT_RANGE: [f64; 10] =
	[0.0, 1.0, 50.0, 100.0, 200.0, 500.0, 800.0, 1200.0, 1600.0, 3000.0];

const TYPE: &str = "Token Holding Amount";
const DESCRIPTION: &str = "The amount of a particular token you are holding";

struct AssertionKeys {
	token: &'static str,
	network: &'static str,
	address: &'static str,
	holding_amount: &'static str,
}

const ASSERTION_KEYS: AssertionKeys = AssertionKeys {
	token: "$token",
	network: "$network",
	address: "$address",
	holding_amount: "$holding_amount",
};

pub trait TokenHoldingAmountAssertionUpdate {
	fn update_token_holding_amount_assertion(&mut self, token_type: Web3TokenType, amount: f64);
}

impl TokenHoldingAmountAssertionUpdate for Credential {
	fn update_token_holding_amount_assertion(&mut self, token_type: Web3TokenType, amount: f64) {
		self.add_subject_info(DESCRIPTION, TYPE);

		update_assertion(token_type, amount, self);
	}
}

fn update_assertion(token_type: Web3TokenType, balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.token,
		Op::Equal,
		token_type.get_token_name(),
	));

	let mut network_assertion: AssertionLogic = AssertionLogic::new_or();
	for network in token_type.get_supported_networks() {
		network_assertion =
			network_assertion.add_item(create_network_assertion_logic(network, token_type.clone()));
	}

	assertion = assertion.add_item(network_assertion);

	let index = BalanceRange::index(&TOKEN_HOLDING_AMOUNT_RANGE, balance);
	match index {
		Some(index) => {
			let min = format!("{}", &TOKEN_HOLDING_AMOUNT_RANGE[index]);
			let max = format!("{}", &TOKEN_HOLDING_AMOUNT_RANGE[index + 1]);
			let min_item =
				AssertionLogic::new_item(ASSERTION_KEYS.holding_amount, Op::GreaterEq, &min);
			let max_item =
				AssertionLogic::new_item(ASSERTION_KEYS.holding_amount, Op::LessThan, &max);

			assertion = assertion.add_item(min_item);
			assertion = assertion.add_item(max_item);

			credential.credential_subject.values.push(index != 0);
		},
		None => {
			let min_item = AssertionLogic::new_item(
				ASSERTION_KEYS.holding_amount,
				Op::GreaterEq,
				&format!("{}", &TOKEN_HOLDING_AMOUNT_RANGE.last().unwrap()),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

fn create_network_assertion_logic(
	network: Web3Network,
	token_type: Web3TokenType,
) -> AssertionLogic {
	let mut assertion = AssertionLogic::new_and();
	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.network,
		Op::Equal,
		web3_network_to_chain(&network),
	));
	if let Some(address) = token_type.get_token_address(network) {
		assertion = assertion.add_item(AssertionLogic::new_item(
			ASSERTION_KEYS.address,
			Op::Equal,
			address,
		));
	}
	assertion
}
