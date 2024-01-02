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

use super::{Item, SummaryHoldings};
use crate::*;

const VC_CRYPTO_SUMMARY_DESCRIPTIONS: &str = "Generate a summary of your on-chain identity";
const VC_CRYPTO_SUMMARY_TYPE: &str = "IDHub Crypto Summary ";

/// Doesn't exists on BSC including:
/// PEPE, BLUR, WLD
pub const CRYPTO_SUMMARY_TOKEN_ADDRESSES_BSC: [(&str, &str); 12] = [
	("0xb1547683DA678f2e1F003A780143EC10Af8a832B", "SHIB"),
	("0xBf5140A22578168FD562DCcF235E5D43A02ce9B1", "UNI"),
	("0x2170Ed0880ac9A755fd29B2688956BD959F933F8", "ETH"),
	("0xF8A0BF9cF54Bb92F17374d9e9A321E6a111a51bD", "LINK"),
	("0xa050FFb3eEb8200eEB7F61ce34FF644420FD3522", "ARB"),
	("0x101d82428437127bF1608F699CD651e6Abf9766E", "BAT"),
	("0xa2B726B1145A4773F68593CF171187d8EBe4d495", "INJ"),
	("0xfb6115445Bff7b52FeB98650C87f44907E58f802", "AAVE"),
	("0x49BA054B9664e99ac335667a917c63bB94332E84", "FTT"),
	("0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82", "CAKE"),
	("0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723", "LIT"),
	("", "BNB"), // Keep address empty
];

pub const CRYPTO_SUMMARY_TOKEN_ADDRESSES_ETH: [(&str, &str); 15] = [
	("0x6982508145454Ce325dDbE47a25d4ec3d2311933", "PEPE"),
	("0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE", "SHIB"),
	("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", "UNI"),
	("0xB8c77482e45F1F44dE1745F52C74426C631bDD52", "BNB"),
	("0x514910771AF9Ca656af840dff83E8264EcF986CA", "LINK"),
	("0x5283d291dbcf85356a21ba090e6db59121208b44", "BLUR"),
	("0xB50721BCf8d664c30412Cfbc6cf7a15145234ad1", "ARB"),
	("0x0d8775f648430679a709e98d2b0cb6250d2887ef", "BAT"),
	("0xe28b3B32B6c345A34Ff64674606124Dd5Aceca30", "INJ"),
	("0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", "AAVE"),
	("0x163f8c2467924be0ae7b5347228cabf260318753", "WLD"),
	("0x50D1c9771902476076eCFc8B2A83Ad6b9355a4c9", "FTT"),
	("0x152649eA73beAb28c5b49B26eb48f7EAD6d4c898", "CAKE"),
	("0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723", "LIT"),
	("", "ETH"), // Keep address empty
];

pub const CRYPTO_SUMMARY_NFT_ADDRESSES_ETH: [(&str, &str); 15] = [
	("0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197", "The Weirdo Ghost Gang"),
	("0x23581767a106ae21c074b2276D25e5C3e136a68b", "Moonbirds"),
	("0x142e03367eDE17Cd851477A4287D1F35676E6dC2", "Yogapetz"),
	("0x59325733eb952a92e069C87F0A6168b29E80627f", "Mocaverse"),
	("0xeC19CAeF9C046f5f87A497154766742ab9C90820", "y00ts"),
	("0x5CC5B05a8A13E3fBDB0BB9FcCd98D38e50F90c38", "The Sandbox"),
	("0x596a5CD859AD53fEc23Cd3fCD77522f0B407920d", "MATR1X KUKU"),
	("0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e", "Doodles"),
	("0x49cF6f5d44E70224e2E23fDcdd2C053F30aDA28B", "CloneX"),
	("0xd774557b647330C91Bf44cfEAB205095f7E6c367", "Nakamigos"),
	("0x6339e5E072086621540D0362C4e3Cea0d643E114", "Opepen"),
	("0xe1dC516B1486Aba548eecD2947A11273518434a4", "The Grapes"),
	("0x769272677faB02575E84945F03Eca517ACc544C", "The Captainz"),
	("0x39ee2c7b3cb80254225884ca001F57118C8f21B6", "The Potatoz"),
	("0x7Bd29408f11D2bFC23c34f18275bBf23bB716Bc7", "Meebits"),
];

pub trait CryptoSummaryCredentialUpdate {
	fn update_crypto_summary_credential(&mut self, summary: SummaryHoldings);
}

impl CryptoSummaryCredentialUpdate for Credential {
	fn update_crypto_summary_credential(&mut self, summary: SummaryHoldings) {
		let (value, and_logic) = build_assertions(summary);

		self.credential_subject.assertions.push(and_logic);
		self.credential_subject.values.push(value);

		self.add_subject_info(VC_CRYPTO_SUMMARY_DESCRIPTIONS, VC_CRYPTO_SUMMARY_TYPE);
	}
}

fn build_assertions(summary: SummaryHoldings) -> (bool, AssertionLogic) {
	let is_empty = summary.is_empty();

	let mut and_logic = AssertionLogic::new_and();

	// TOKENs
	let token_assertion = token_items(summary.summary.token);
	and_logic = and_logic.add_item(token_assertion);

	// NFTs
	let nft_assertion = token_items(summary.summary.nft);
	and_logic = and_logic.add_item(nft_assertion);

	(!is_empty, and_logic)
}

fn token_items(items: Vec<Item>) -> AssertionLogic {
	let mut and_logic = AssertionLogic::new_and();
	for item in items {
		let mut item_logic = AssertionLogic::new_and();

		let network_item = AssertionLogic::new_item("$network", Op::Equal, &item.network);
		item_logic = item_logic.add_item(network_item);

		for token in item.list {
			let mut inner_logic = AssertionLogic::new_and();

			let name_item = AssertionLogic::new_item("$token_name", Op::Equal, &token.name);
			inner_logic = inner_logic.add_item(name_item);

			let address_item =
				AssertionLogic::new_item("$token_address", Op::Equal, &token.address);
			inner_logic = inner_logic.add_item(address_item);

			item_logic = item_logic.add_item(inner_logic)
		}

		and_logic = and_logic.add_item(item_logic);
	}

	and_logic
}

#[cfg(test)]
mod tests {
	use crate::nodereal::crypto_summary::SummaryHoldings;

	use super::build_assertions;

	#[test]
	fn build_assertions_works() {
		let flag_bsc_token = [
			false, true, true, true, true, true, true, true, true, true, true, true, true, true,
			true,
		];
		let flag_eth_token = [
			false, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false,
		];
		let flag_eth_nft = [
			true, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false,
		];

		let summary = SummaryHoldings::construct(&flag_bsc_token, &flag_eth_token, &flag_eth_nft);
		let (value, logic) = build_assertions(summary);
		println!("assertions: {}", serde_json::to_string(&logic).unwrap());
		assert!(value);
	}
}
