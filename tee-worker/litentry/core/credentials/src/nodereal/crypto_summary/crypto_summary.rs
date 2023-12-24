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

use crate::{
	assertion_logic::{AssertionLogic, Op},
	format_assertion_to_date, Credential,
};
use lazy_static::lazy_static;
use lc_data_providers::allium::CryptoSummaryItem;
use std::{
	collections::HashMap,
	string::{String, ToString},
	vec::Vec,
};

const VC_CRYPTO_SUMMARY_DESCRIPTIONS: &str = "DESCRIPTION placeholder";
const VC_CRYPTO_SUMMARY_TYPE: &str = "TYPE placeholder";

lazy_static! {
	static ref CRYPTO_SUMMARY_TOKEN_ADDRESSES: HashMap<String, String> = {
		let mut map = HashMap::new();
		map.insert("0x6982508145454Ce325dDbE47a25d4ec3d2311933", "PEPE");
		map.insert("0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE", "SHIB");
		map.insert("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", "UNI");
		map.insert("0x0000000000000000000000000000000000000000", "ETH");
		map.insert("0xB8c77482e45F1F44dE1745F52C74426C631bDD52", "BNB");
		map.insert("0x514910771AF9Ca656af840dff83E8264EcF986CA", "LINK");
		map.insert("0x5283d291dbcf85356a21ba090e6db59121208b44", "BLUR");
		map.insert("0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1", "ARB");
		map.insert("0x0d8775f648430679a709e98d2b0cb6250d2887ef", "BAT");
		map.insert("0xe28b3B32B6c345A34Ff64674606124Dd5Aceca30", "INJ");
		map.insert("0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", "AAVE");
		map.insert("0x163f8c2467924be0ae7b5347228cabf260318753", "WLD");
		map.insert("0x50D1c9771902476076eCFc8B2A83Ad6b9355a4c9", "FTT");
		map.insert("0x152649eA73beAb28c5b49B26eb48f7EAD6d4c898", "CAKE");
		map.insert("0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723", "LIT");
		// Add more token addresses and names as needed
		map.iter()
			.map(|(address, name)| (address.to_lowercase(), name.to_string()))
			.collect()
	};
}

lazy_static! {
	static ref CRYPTO_SUMMARY_NFT_ADDRESSES: HashMap<String, String> = {
		let mut map = HashMap::new();
		map.insert("0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197", "The Weirdo Ghost Gang");
		map.insert("0x23581767a106ae21c074b2276D25e5C3e136a68b", "Moonbirds");
		map.insert("0x142e03367eDE17Cd851477A4287D1F35676E6dC2", "Yogapetz");
		map.insert("0x59325733eb952a92e069C87F0A6168b29E80627f", "Mocaverse");
		map.insert("0xeC19CAeF9C046f5f87A497154766742ab9C90820", "y00ts");
		map.insert("0x5CC5B05a8A13E3fBDB0BB9FcCd98D38e50F90c38", "The Sandbox");
		map.insert("0x596a5CD859AD53fEc23Cd3fCD77522f0B407920d", "MATR1X KUKU");
		map.insert("0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e", "Doodles");
		map.insert("0x49cF6f5d44E70224e2E23fDcdd2C053F30aDA28B", "CloneX");
		map.insert("0xd774557b647330C91Bf44cfEAB205095f7E6c367", "Nakamigos");
		map.insert("0x6339e5E072086621540D0362C4e3Cea0d643E114", "Opepen");
		map.insert("0xe1dC516B1486Aba548eecD2947A11273518434a4", "The Grapes");
		map.insert("0x769272677faB02575E84945F03Eca517ACc544C", "The Captainz");
		map.insert("0x39ee2c7b3cb80254225884ca001F57118C8f21B6", "The Potatoz");
		map.insert("0x7Bd29408f11D2bFC23c34f18275bBf23bB716Bc7", "Meebits");
		// Add more token addresses and names as needed
		map.iter()
			.map(|(address, name)| (address.to_lowercase(), name.to_string()))
			.collect()
	};
}

pub trait AlliumCryptoSummaryCredential {
	fn update_crypto_summary_credential(&mut self, items: &Vec<CryptoSummaryItem>);
}

impl AlliumCryptoSummaryCredential for Credential {
	fn update_crypto_summary_credential(&mut self, items: &Vec<CryptoSummaryItem>) {
		let found_tokens: Vec<String> = items
			.iter()
			.filter_map(|item| {
				let lowercase_token_address = item.token_address.to_lowercase();
				CRYPTO_SUMMARY_TOKEN_ADDRESSES
					.get(&lowercase_token_address)
					.map(|name| name.to_string())
			})
			.collect();

		// 	let mut and_logic = AssertionLogic::new_and();
		// 	AssertionLogic::new_item("$network", Op::Equal, &ETH);
		// 	AssertionLogic::new_item("$network", Op::Equal, &Polkadot);
		// 	AssertionLogic::new_item("$network", Op::Equal, &Polygon);

		// 	AssertionLogic::new_item("$token", Op::Equal, "");

		// // from_date's Op is ALWAYS Op::LessThan
		// let from_date_logic = AssertionLogic::new_item("$from_date", Op::LessThan, from_date);

		// // minimum_amount' Op is ALWAYS Op::Equal
		// let minimum_amount_logic =
		// 	AssertionLogic::new_item("$minimum_amount", Op::Equal, minimum_amount);

		// // to_date's Op is ALWAYS Op::GreaterEq
		// let to_date = format_assertion_to_date();
		// let to_date_logic = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);

		// let assertion = AssertionLogic::new_and()
		// 	.add_item(minimum_amount_logic)
		// 	.add_item(from_date_logic)
		// 	.add_item(to_date_logic);

		// self.credential_subject.assertions.push(assertion);
		// self.credential_subject.values.push(value);

		self.add_subject_info(VC_CRYPTO_SUMMARY_DESCRIPTIONS, VC_CRYPTO_SUMMARY_TYPE);
	}
}
