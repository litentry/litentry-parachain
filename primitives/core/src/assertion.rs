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

// This file includes the predefined rulesets and the corresponding parameters
// when requesting VCs.

use crate::{AccountId, BoundedWeb3Network, Web3Network};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use sp_std::{str, vec, vec::Vec};

pub type ParameterString = BoundedVec<u8, ConstU32<64>>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountHolding {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount: ParameterString,
	pub date: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountToken {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmount {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmounts {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount1: ParameterString,
	pub amount2: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBasic {
	pub name: ParameterString,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBetweenPercents {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub greater_than_or_equal_to: ParameterString,
	pub less_than_or_equal_to: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableClassOfYear {
	pub name: ParameterString,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDateInterval {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub start_date: ParameterString,
	pub end_date: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDatePercent {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub token: ParameterString,
	pub date: ParameterString,
	pub percent: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDate {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub date: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableToken {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub token: ParameterString,
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum AchainableParams {
	AmountHolding(AchainableAmountHolding),
	AmountToken(AchainableAmountToken),
	Amount(AchainableAmount),
	Amounts(AchainableAmounts),
	Basic(AchainableBasic),
	BetweenPercents(AchainableBetweenPercents),
	ClassOfYear(AchainableClassOfYear),
	DateInterval(AchainableDateInterval),
	DatePercent(AchainableDatePercent),
	Date(AchainableDate),
	Token(AchainableToken),
}

impl AchainableParams {
	pub fn name(&self) -> ParameterString {
		match self {
			AchainableParams::AmountHolding(p) => p.name.clone(),
			AchainableParams::AmountToken(p) => p.name.clone(),
			AchainableParams::Amount(p) => p.name.clone(),
			AchainableParams::Amounts(p) => p.name.clone(),
			AchainableParams::Basic(p) => p.name.clone(),
			AchainableParams::BetweenPercents(p) => p.name.clone(),
			AchainableParams::ClassOfYear(p) => p.name.clone(),
			AchainableParams::DateInterval(p) => p.name.clone(),
			AchainableParams::DatePercent(p) => p.name.clone(),
			AchainableParams::Date(p) => p.name.clone(),
			AchainableParams::Token(p) => p.name.clone(),
		}
	}

	pub fn chain(&self) -> Web3Network {
		match self {
			AchainableParams::AmountHolding(p) => p.chain,
			AchainableParams::AmountToken(p) => p.chain,
			AchainableParams::Amount(p) => p.chain,
			AchainableParams::Amounts(p) => p.chain,
			AchainableParams::Basic(p) => p.chain,
			AchainableParams::BetweenPercents(p) => p.chain,
			AchainableParams::ClassOfYear(p) => p.chain,
			AchainableParams::DateInterval(p) => p.chain,
			AchainableParams::DatePercent(p) => p.chain,
			AchainableParams::Date(p) => p.chain,
			AchainableParams::Token(p) => p.chain,
		}
	}
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum Assertion {
	A1,
	A2(ParameterString),                                    // (guild_id)
	A3(ParameterString, ParameterString, ParameterString),  // (guild_id, channel_id, role_id)
	A4(ParameterString),                                    // (minimum_amount)
	A6,
	A7(ParameterString),                                    // (minimum_amount)
	A8(BoundedWeb3Network),                                 // litentry, litmus, polkadot, kusama, khala, ethereum
	A9,
	A10(ParameterString),                                   // (minimum_amount)
	A11(ParameterString),                                   // (minimum_amount)

	// ----- begin polkadot decoded 2023 -----
	A13(AccountId),                                         // (participant_account), can only be requested by delegatee
	A14,
	// for Holder assertions we'll reuse A4/A7
	// ----- end polkadot decoded 2023 -----

	Achainable(AchainableParams),

	// For EVM Version Early Bird
	A20,
	Oneblock,
}

impl Assertion {
	// Given an assertion enum type, retrieve the supported web3 networks.
	// So we limit the network types on the assertion definition level.
	//
	// The final networks used for assertion building are the common set of:
	// - "assertion supported networks" which are defined here, and
	// - "identity networks" which are defined by the user and stored in `IdentityContext`
	//
	// returns a vector of `Web3Network` guarantees it's a subnet of
	// the broader `Web3Network` (see network.rs)
	pub fn get_supported_web3networks(&self) -> Vec<Web3Network> {
		match self {
			// LIT holder, not including `LitentryRococo` as it's not supported by any data provider
			Self::A4(..) => vec![Web3Network::Litentry, Web3Network::Litmus, Web3Network::Ethereum],
			// DOT holder
			Self::A7(..) => vec![Web3Network::Polkadot],
			// WBTC/ETH holder
			Self::A10(..) | Self::A11(..) => vec![Web3Network::Ethereum],
			// total tx over `networks`
			Self::A8(network) => network.to_vec(),
			// polkadot paticipation
			Self::A14 => vec![Web3Network::Polkadot],
			// Achainable Assertions
			Self::Achainable(a) => vec![a.chain()],
			// we don't care about any specific web3 network
			_ => vec![],
		}
	}
}

pub const ASSERTION_FROM_DATE: [&str; 14] = [
	"2017-01-01",
	"2017-07-01",
	"2018-01-01",
	"2018-07-01",
	"2019-01-01",
	"2019-07-01",
	"2020-01-01",
	"2020-07-01",
	"2021-01-01",
	"2021-07-01",
	"2022-01-01",
	"2022-07-01",
	"2023-01-01",
	"2023-07-01",
];
