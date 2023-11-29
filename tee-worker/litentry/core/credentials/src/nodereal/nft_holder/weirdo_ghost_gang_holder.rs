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
	Credential,
};

// VC type / info
const WEIRDO_GHOST_GANG_HOLDER_INFOS: (&str, &str) =
	("WeirdoGhostGang Holder", "You are WeirdoGhostGang NFT holder");

pub trait WeirdoGhostGangHolderAssertionUpdate {
	fn update_weirdo_ghost_gang_holder_assertion(&mut self, value: bool);
}

impl WeirdoGhostGangHolderAssertionUpdate for Credential {
	fn update_weirdo_ghost_gang_holder_assertion(&mut self, value: bool) {
		let assertion = AssertionLogic::new_item("$is_weirdo_ghost_gang_holder", Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		self.add_subject_info(WEIRDO_GHOST_GANG_HOLDER_INFOS.1, WEIRDO_GHOST_GANG_HOLDER_INFOS.0);
	}
}
