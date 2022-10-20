// Copyright 2020-2022 Litentry Technologies GmbH.
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
//
// See: https://www.notion.so/litentry/Expected-parameters-in-predefined-rulesets-14f74928aa2b43509167da12a3e75507

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

type Balance = u128;
type MaxStringLength = ConstU32<64>;
pub type ParameterString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	Assert1,
	Assert2(ParameterString, ParameterString), // (guild_id, user_id)
	Assert3(ParameterString, ParameterString), // (guild_id, user_id)
	Assert4,
	Assert5(ParameterString, ParameterString), // (twitter_account, tweet_id)
	Assert6,
	Assert7(Balance, u32), // (DOT_amount, year)
	Assert8(u64),          // (tx_amount)
	Assert9,
	Assert10(Balance, u32), // (WBTC_amount, year)
	Assert11(Balance, u32), // (ETH_amount, year)
	Assert12(Balance, u32), // (LIT_amount, year)
	Assert13(u32),          // (Karma_amount) - TODO: unsupported
}
