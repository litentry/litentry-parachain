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
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use litentry_primitives::{Address32, IdGraphIdentifier};
use log::*;

const VC_A13_SUBJECT_DESCRIPTION: &str =
	"The user has a Polkadot Decoded 2023 Litentry Booth Special Badge";
const VC_A13_SUBJECT_TYPE: &str = "Decoded 2023 Basic Special Badge";
const VC_A13_SUBJECT_TAG: [&str; 2] = ["Polkadot decoded 2023", "Litentry"];

pub fn build(shard: &ShardIdentifier, who: &AccountId) -> Result<Credential> {
	debug!("Assertion A13 build, who: {:?}", account_id_to_string(&who));

	let address: Address32 = who.clone().into();
	let id_graph_identifier = IdGraphIdentifier::Substrate { address };

	match Credential::new_default(&id_graph_identifier, shard) {
		Ok(mut credential_unsigned) => {
			// add subject info
			credential_unsigned.add_subject_info(
				VC_A13_SUBJECT_DESCRIPTION,
				VC_A13_SUBJECT_TYPE,
				VC_A13_SUBJECT_TAG.to_vec(),
			);

			// add assertion
			credential_unsigned.add_assertion_a13();
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A13(who.clone()), e.into_error_detail()))
		},
	}
}
