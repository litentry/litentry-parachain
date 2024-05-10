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

use lc_credentials::IssuerRuntimeVersion;

use crate::*;

const VC_A1_SUBJECT_DESCRIPTION: &str =
	"You've identified at least one account/address in both Web2 and Web3.";
const VC_A1_SUBJECT_TYPE: &str = "Basic Identity Verification";

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	debug!("Assertion A1 build, who: {:?}", account_id_to_string(&req.who));

	let mut is_web2 = false;
	let mut is_web3 = false;
	for (identity, _) in &req.identities {
		is_web2 |= identity.is_web2();
		is_web3 |= identity.is_web3();
	}

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A1_SUBJECT_DESCRIPTION, VC_A1_SUBJECT_TYPE);
			credential_unsigned.add_assertion_a1(is_web2 && is_web3);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A1, e.into_error_detail()))
		},
	}
}
