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

use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};
use std::string::ToString;

const UNISWAP_USER_DESCRIPTIONS: &str =
	"You are a trader or liquidity provider of Uniswap V2 or V3.
Uniswap V2 Factory Contract: 0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f.
Uniswap V3 Factory Contract: 0x1f98431c8ad98523631ae4a59f267346ea31f984.";
const UNISWAP_USER_TYPE: &str = "Uniswap V2/V3 User";

pub trait UpdateUniswapUser {
	fn update_uniswap_user(&mut self, is_v2_holder: bool, is_v3_holder: bool);
}

impl UpdateUniswapUser for Credential {
	fn update_uniswap_user(&mut self, is_v2_holder: bool, is_v3_holder: bool) {
		let uniswap_v2 =
			AssertionLogic::new_item("$is_uniswap_v2_user", Op::Equal, &is_v2_holder.to_string());
		let uniswap_v3 =
			AssertionLogic::new_item("$is_uniswap_v3_user", Op::Equal, &is_v3_holder.to_string());

		let assertion = AssertionLogic::new_and().add_item(uniswap_v2).add_item(uniswap_v3);
		self.credential_subject.assertions.push(assertion);

		// Always true
		self.credential_subject.values.push(true);

		self.add_subject_info(UNISWAP_USER_DESCRIPTIONS, UNISWAP_USER_TYPE);
	}
}
