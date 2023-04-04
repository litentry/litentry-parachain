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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

type MaxStringLength = ConstU32<64>;
pub type ParameterString = BoundedVec<u8, MaxStringLength>;
pub type IndexingNetworks = BoundedVec<IndexingNetwork, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IndexingNetwork {
	Litentry,
	Litmus,
	Polkadot,
	Kusama,
	Khala,
	Ethereum,
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	A1,
	A2(ParameterString),                                    // (guild_id)
	A3(ParameterString, ParameterString, ParameterString),  // (guild_id, channel_id, role_id)
	A4(ParameterString),                                    // (minimum_amount)
	A5(ParameterString),                   					// (original_tweet_id)
	A6,
	A7(ParameterString),                                    // (minimum_amount)
	A8(IndexingNetworks),                                   // litentry, litmus, polkadot, kusama, khala, ethereum
	A9,
	A10(ParameterString),                                   // (minimum_amount)
	A11(ParameterString),                                   // (minimum_amount)
	A13(u32),                                               // (Karma_amount) - TODO: unsupported
}

pub const ASSERTION_FROM_DATE: [&str; 7] = [
	"2017-01-01",
	"2018-01-01",
	"2019-01-01",
	"2020-01-01",
	"2021-01-01",
	"2022-01-01",
	"2023-01-01",
];
