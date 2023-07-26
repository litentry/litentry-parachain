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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::*;

const VC_A1_SUBJECT_DESCRIPTION: &str =
	"The user has verified one identity in Web 2 and one identity in Web 3";
const VC_A1_SUBJECT_TYPE: &str = "Basic Identity Verification";
const VC_A1_SUBJECT_TAG: [&str; 1] = ["Litentry Network"];

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	debug!("Assertion A1 build, who: {:?}", account_id_to_string(&req.who));

	let mut web2_cnt = 0;
	let mut web3_cnt = 0;

	for identity in &req.identities {
		if identity.0.is_web2() {
			web2_cnt += 1;
		} else if identity.0.is_web3() {
			web3_cnt += 1;
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			// add subject info
			credential_unsigned.add_subject_info(
				VC_A1_SUBJECT_DESCRIPTION,
				VC_A1_SUBJECT_TYPE,
				VC_A1_SUBJECT_TAG.to_vec(),
			);

			// add assertion
			let flag = web2_cnt != 0 && web3_cnt != 0;
			credential_unsigned.add_assertion_a1(flag);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A1, e.into_error_detail()))
		},
	}
}
