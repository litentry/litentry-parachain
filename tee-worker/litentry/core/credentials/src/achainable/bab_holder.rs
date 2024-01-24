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

const BAB_HOLDER_DESCRIPTIONS: &str = "You are a holder of a certain kind of NFT";
const BAB_HOLDER_TYPE: &str = "NFT Holder";
const BAB_HOLDER_BREAKDOWN: &str = "is_bab_holder";

pub trait UpdateBABHolder {
	fn update_bab_holder(&mut self, is_bab_holder: bool);
}

impl UpdateBABHolder for Credential {
	fn update_bab_holder(&mut self, is_bab_holder: bool) {
		let bab_holder = AssertionLogic::new_item(BAB_HOLDER_BREAKDOWN, Op::Equal, "true");

		let assertion = AssertionLogic::new_and().add_item(bab_holder);
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(is_bab_holder);

		self.add_subject_info(BAB_HOLDER_DESCRIPTIONS, BAB_HOLDER_TYPE);
	}
}
