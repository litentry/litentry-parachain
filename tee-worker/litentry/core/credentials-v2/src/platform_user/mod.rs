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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use lc_common::platform_user::PlatformName;
use litentry_primitives::PlatformUserType;

// TODO migration to v2 in the future
use lc_credentials::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};

const TYPE: &str = "Platform user";
const DESCRIPTION: &str = "You are a user of a certain platform";

pub trait PlatformUserAssertionUpdate {
	fn update_platform_user_assertion(
		&mut self,
		platform_user_type: PlatformUserType,
		is_user: bool,
	);
}

impl PlatformUserAssertionUpdate for Credential {
	fn update_platform_user_assertion(
		&mut self,
		platform_user_type: PlatformUserType,
		is_user: bool,
	) {
		self.add_subject_info(DESCRIPTION, TYPE);

		let mut assertion = AssertionLogic::new_and();
		assertion = assertion.add_item(AssertionLogic::new_item(
			"$platform",
			Op::Equal,
			platform_user_type.get_platform_name(),
		));

		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(is_user);
	}
}
