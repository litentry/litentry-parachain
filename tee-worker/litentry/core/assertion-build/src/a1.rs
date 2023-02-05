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

use crate::Result;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_stf_task_sender::MaxIdentityLength;
use litentry_primitives::{Assertion, Identity, ParentchainBlockNumber};
use log::*;
use parachain_core_primitives::VCMPError;
use sp_runtime::BoundedVec;

pub fn build(
	identities: BoundedVec<Identity, MaxIdentityLength>,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	let mut web2_cnt = 0;
	let mut web3_cnt = 0;

	for identity in &identities {
		if identity.is_web2() {
			web2_cnt += 1;
		} else if identity.is_web3() {
			web3_cnt += 1;
		}
	}

	match Credential::generate_unsigned_credential(&Assertion::A1, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_assertion_a1(web2_cnt, web3_cnt);
			credential_unsigned.credential_subject.set_value(true);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(VCMPError::Assertion1Failed)
		},
	}
}
