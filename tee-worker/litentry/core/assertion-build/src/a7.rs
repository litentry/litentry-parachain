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
use lc_data_providers::{
	achainable::{AchainableClient, AchainableQuery, VerifiedCredentialsIsHodlerIn},
	vec_to_string,
};

const VC_A7_SUBJECT_DESCRIPTION: &str =
	"The user has been consistently holding at least {x} amount of tokens before 2023 Jan 1st 00:00:00 UTC on the supporting networks";
const VC_A7_SUBJECT_TYPE: &str = "DOT Holding Assertion";
const VC_A7_SUBJECT_TAG: [&str; 1] = ["Polkadot"];

pub fn build(req: &AssertionBuildRequest, min_balance: ParameterString) -> Result<Credential> {
	debug!("Assertion A7 build, who: {:?}", account_id_to_string(&req.who),);

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A7(min_balance.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.vec_identity);

	let mut is_hold = false;
	let mut optimal_hold_index = usize::MAX;

	for (network, addresses) in identities {
		// If found query result is the optimal solution, i.e optimal_hold_index = 0, (2017-01-01)
		// there is no need to query other networks.
		if optimal_hold_index == 0 {
			break
		}

		// Each query loop needs to reset is_hold to false
		is_hold = false;

		let addresses: Vec<String> = addresses.into_iter().collect();
		let token_address = "";

		for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
			let vch = VerifiedCredentialsIsHodlerIn::new(
				addresses.clone(),
				from_date.to_string(),
				network,
				token_address.to_string(),
				q_min_balance.to_string(),
			);
			match client.verified_credentials_is_hodler(vch) {
				Ok(is_hodler_out) =>
					for hodler in is_hodler_out.hodlers.iter() {
						is_hold = is_hold || hodler.is_hodler;
					},
				Err(e) => error!(
					"Assertion A7 request check_verified_credentials_is_hodler error: {:?}",
					e
				),
			}

			if is_hold {
				if index < optimal_hold_index {
					optimal_hold_index = index;
				}

				break
			}
		}
	}

	// Found the optimal hold index, set the is_hold to true, otherwise
	// the optimal_hold_index is always 0 (2017-01-01)
	if optimal_hold_index != usize::MAX {
		is_hold = true;
	} else {
		optimal_hold_index = 0;
	}

	match Credential::new_default(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A7_SUBJECT_DESCRIPTION,
				VC_A7_SUBJECT_TYPE,
				VC_A7_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned.update_holder(
				is_hold,
				&q_min_balance,
				&ASSERTION_FROM_DATE[optimal_hold_index].into(),
			);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A7(min_balance), e.into_error_detail()))
		},
	}
}
