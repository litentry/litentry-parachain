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

use lc_data_providers::achainable::web3_network_to_chain;
use litentry_primitives::{all_evm_web3networks, EVMTokenType, Web3Network};

use crate::{
	assertion_logic::{AssertionLogic, Op},
	litentry_profile::{BalanceRange, BalanceRangeIndex},
	Credential,
};
pub trait EVMTokenAddress {
	fn get_address(&self, network: Web3Network) -> Option<&'static str>;
}

impl EVMTokenAddress for EVMTokenType {
	fn get_address(&self, network: Web3Network) -> Option<&'static str> {
		match (self, network) {
			(EVMTokenType::Ton, Web3Network::Bsc) =>
				Some("0x76a797a59ba2c17726896976b7b3747bfd1d220f"),
			(EVMTokenType::Ton, Web3Network::Ethereum) =>
				Some("0x582d872a1b094fc48f5de31d3b73f2d9be47def1"),
			(EVMTokenType::Trx, Web3Network::Bsc) =>
				Some("0xCE7de646e7208a4Ef112cb6ed5038FA6cC6b12e3"),
			(EVMTokenType::Trx, Web3Network::Ethereum) =>
				Some("0x50327c6c5a14dcade707abad2e27eb517df87ab5"),
			(EVMTokenType::Pepe, Web3Network::Ethereum) =>
				Some("0x6982508145454Ce325dDbE47a25d4ec3d2311933"),
			(EVMTokenType::Shib, Web3Network::Ethereum) =>
				Some("0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE"),
			(EVMTokenType::Uni, Web3Network::Ethereum) =>
				Some("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"),
			(EVMTokenType::Bnb, Web3Network::Ethereum) =>
				Some("0xB8c77482e45F1F44dE1745F52C74426C631bDD52"),
			(EVMTokenType::Link, Web3Network::Ethereum) =>
				Some("0x514910771AF9Ca656af840dff83E8264EcF986CA"),
			(EVMTokenType::Blur, Web3Network::Ethereum) =>
				Some("0x5283D291DBCF85356A21bA090E6db59121208b44"),
			(EVMTokenType::Arb, Web3Network::Ethereum) =>
				Some("0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1"),
			(EVMTokenType::Bat, Web3Network::Ethereum) =>
				Some("0x0D8775F648430679A709E98d2b0Cb6250d2887EF"),
			(EVMTokenType::Inj, Web3Network::Ethereum) =>
				Some("0xe28b3B32B6c345A34Ff64674606124Dd5Aceca30"),
			(EVMTokenType::Aave, Web3Network::Ethereum) =>
				Some("0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9"),
			(EVMTokenType::Wld, Web3Network::Ethereum) =>
				Some("0x163f8C2467924be0ae7B5347228CABF260318753"),
			(EVMTokenType::Ftt, Web3Network::Ethereum) =>
				Some("0x50D1c9771902476076eCFc8B2A83Ad6b9355a4c9"),
			(EVMTokenType::Cake, Web3Network::Ethereum) =>
				Some("0x152649eA73beAb28c5b49B26eb48f7EAD6d4c898"),
			(EVMTokenType::Lit, Web3Network::Ethereum) =>
				Some("0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723"),
			(EVMTokenType::Eth, Web3Network::Ethereum) =>
				Some("0x0000000000000000000000000000000000000000"),
			_ => None,
		}
	}
}

const EVM_HOLDING_AMOUNT_RANGE: [f64; 8] =
	[0.0, 100.0, 200.0, 500.0, 800.0, 1200.0, 1600.0, 3000.0];

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

trait AssertionTokenName {
	fn get_name(&self) -> &'static str;
}

impl AssertionTokenName for EVMTokenType {
	fn get_name(&self) -> &'static str {
		match self {
			EVMTokenType::Ton => "TON",
			EVMTokenType::Trx => "TRX",
		}
	}
}

pub trait EVMAmountHoldingAssertionUpdate {
	fn update_evm_amount_holding_assertion(&mut self, token_type: EVMTokenType, amount: f64);
}

impl EVMAmountHoldingAssertionUpdate for Credential {
	fn update_evm_amount_holding_assertion(&mut self, token_type: EVMTokenType, amount: f64) {
		self.add_subject_info(DESCRIPTION, TYPE);

		update_assertion(token_type, amount, self);
	}
}

fn update_assertion(token_type: EVMTokenType, balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.token,
		Op::Equal,
		token_type.get_name(),
	));

	let mut network_assertion = AssertionLogic::new_or();
	for newtork in all_evm_web3networks() {
		match create_network_assertion_logic(newtork, token_type.clone()) {
			Some(network_assertion_item) => {
				network_assertion = network_assertion.add_item(network_assertion_item);
			},
			None => continue,
		}
	}

	assertion = assertion.add_item(network_assertion);

	let index = BalanceRange::index(&EVM_HOLDING_AMOUNT_RANGE, balance);
	match index {
		Some(index) => {
			let min = format!("{}", &EVM_HOLDING_AMOUNT_RANGE[index]);
			let max = format!("{}", &EVM_HOLDING_AMOUNT_RANGE[index + 1]);
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
				&format!("{}", &EVM_HOLDING_AMOUNT_RANGE.last().unwrap()),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

fn create_network_assertion_logic(
	network: Web3Network,
	token_type: EVMTokenType,
) -> Option<AssertionLogic> {
	let mut assertion = AssertionLogic::new_and();
	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.network,
		Op::Equal,
		web3_network_to_chain(&network).as_str(),
	));
	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.address,
		Op::Equal,
		token_type.get_address(network)?,
	));
	Some(assertion)
}
