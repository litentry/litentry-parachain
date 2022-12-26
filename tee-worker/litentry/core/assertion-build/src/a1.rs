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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// #[cfg(all(not(feature = "std"), feature = "sgx"))]
// use crate::sgx_reexport_prelude::*;

use crate::{Error, Result};
use lc_stf_task_sender::MaxIdentityLength;
use std::string::ToString;

use litentry_primitives::Identity;
use sp_runtime::BoundedVec;

pub fn build(identities: BoundedVec<Identity, MaxIdentityLength>) -> Result<()> {
	let mut web2_cnt = 0;
	let mut web3_cnt = 0;

	for identity in &identities {
		if identity.is_web2() {
			web2_cnt += 1;
		} else if identity.is_web3() {
			web3_cnt += 1;
		}
	}

	if web2_cnt > 0 && web3_cnt > 0 {
		// TODO: generate_vc();
		Ok(())
	} else {
		Err(Error::Assertion1Error("Assertion1 fail.".to_string()))
	}
}
