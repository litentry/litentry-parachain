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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

use crate::achainable;
use litentry_primitives::{AchainableParams, Assertion};
use std::string::String;

const BASE_URL: &str = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas";
const NOT_IMPLEMENTED: &str =
	"https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/0-base.json";

/// Returns the respective JSON Schema for the given assertion and its credential.
/// JSON Schemas can be found on https://github.com/litentry/vc-jsonschema
pub fn get_json_schema_url(assertion: Assertion) -> String {
	match assertion {
		Assertion::A1 => format!("{BASE_URL}/1-basic-identity-verification/1-0-0.json"),

		Assertion::A2(_) => format!("{BASE_URL}/2-litentry-discord-member/1-0-0.json"),

		Assertion::A3(_, _, _) => format!("{BASE_URL}/3-active-discord-id-hubber/1-0-0.json"),

		Assertion::A4(_) => format!("{BASE_URL}/4-token-holding-time/1-0-0.json"),

		Assertion::A6 => format!("{BASE_URL}/6-twitter-follower-amount/1-0-0.json"),

		Assertion::A7(_) => format!("{BASE_URL}/4-token-holding-time/1-0-0.json"),

		Assertion::A8(_) => format!("{BASE_URL}/7-evm-substrate-transaction-count/1-0-0.json"),

		Assertion::A10(_) => format!("{BASE_URL}/4-token-holding-time/1-0-0.json"),

		Assertion::A11(_) => format!("{BASE_URL}/4-token-holding-time/1-0-0.json"),

		Assertion::A13(_) => format!("{BASE_URL}/8-decoded-2023-basic-special-badge/1-0-0.json"),

		Assertion::A14 =>
			format!("{BASE_URL}/9-polkadot-governance-participation-proof/1-0-0.json"),

		Assertion::Achainable(params) => match params {
			AchainableParams::AmountHolding(_) =>
				format!("{BASE_URL}/18-token-holding-amount/1-0-0.json"),

			AchainableParams::AmountToken(_) =>
				format!("{BASE_URL}/18-token-holding-amount/1-0-0.json"),

			AchainableParams::Amount(_) => format!("{BASE_URL}/11-token-holder/1-0-0.json"),

			AchainableParams::Basic(_) => format!("{BASE_URL}/11-token-holder/1-0-0.json"),

			AchainableParams::ClassOfYear(_) =>
				format!("{BASE_URL}/10-account-class-of-year/1-0-0.json"),

			AchainableParams::Mirror(_) => format!("{BASE_URL}/23-mirror-contributor/1-0-0.json"),

			// The following assertions are Unused and produce no specific claims. They Generates
			// generic JSON Credentials
			AchainableParams::Amounts(_) => format!("{}", NOT_IMPLEMENTED),
			AchainableParams::BetweenPercents(_) => format!("{}", NOT_IMPLEMENTED),
			AchainableParams::Date(_) => format!("{}", NOT_IMPLEMENTED),
			AchainableParams::DateInterval(_) => format!("{}", NOT_IMPLEMENTED),
			AchainableParams::DatePercent(_) => format!("{}", NOT_IMPLEMENTED),
			AchainableParams::Token(_) => format!("{}", NOT_IMPLEMENTED),
		},

		Assertion::A20 => format!("{BASE_URL}/12-idhub-evm-version-early-bird/1-0-0.json"),

		Assertion::Oneblock(_) => format!("{BASE_URL}/13-oneblock-student-phase-12/1-0-0.json"),

		Assertion::GenericDiscordRole(_) =>
			format!("{BASE_URL}/15-generic-discord-role/1-0-0.json"),

		Assertion::BnbDomainHolding =>
			format!("{BASE_URL}/16-bnb-domain-holding-amount/1-0-0.json"),

		Assertion::BnbDigitDomainClub(_) =>
			format!("{BASE_URL}/17-bnb-3d-4d-club-domain-holding-amount/1-0-0.json"),

		Assertion::VIP3MembershipCard(_) => format!("{BASE_URL}/20-vip3-card-holder/1-0-0.json"),

		Assertion::WeirdoGhostGangHolder =>
			format!("{BASE_URL}/19-weirdoghostgang-holder/1-0-0.json"),

		Assertion::LITStaking => format!("{BASE_URL}/18-token-holding-amount/1-0-0.json"),

		Assertion::TokenHoldingAmount(_) | Assertion::EVMAmountHolding(_) =>
			format!("{BASE_URL}/22-evm-holding-amount/1-0-0.json"),

		Assertion::BRC20AmountHolder =>
			format!("{BASE_URL}/21-token-holding-amount-list/1-0-0.json"),

		Assertion::CryptoSummary => format!("{BASE_URL}/24-crypto-summary/1-0-0.json"),

		Assertion::PlatformUser(_) => format!("{BASE_URL}/25-platform-user/1-0-0.json"),
	}
}
