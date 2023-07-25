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
	achainable::{AchainableClient, AchainableHolder, ParamsBasicTypeWithAmountHoding},
	vec_to_string,
};
use std::string::ToString;

const VC_A7_SUBJECT_DESCRIPTION: &str =
	"The length of time a user continues to hold a particular token (with particular threshold of token amount)";
const VC_A7_SUBJECT_TYPE: &str = "DOT Holding Assertion";
const VC_A7_SUBJECT_TAG: [&str; 1] = ["Polkadot"];

pub fn build(req: &AssertionBuildRequest, min_balance: ParameterString) -> Result<Credential> {
	debug!("Assertion A7 build, who: {:?}", account_id_to_string(&req.who),);

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A7(min_balance.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = AchainableClient::new();
	let identities: Vec<(Web3Network, Vec<String>)> = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut is_hold = false;
	let mut optimal_hold_index = 0_usize;
	for (index, date) in ASSERTION_FROM_DATE.iter().enumerate() {
		if is_hold {
			break
		}

		for address in &addresses {
			let holding = ParamsBasicTypeWithAmountHoding::new(
				&Web3Network::Polkadot,
				q_min_balance.to_string(),
				date.to_string(),
				None,
			);

			match client.is_holder(address, holding) {
				Ok(is_polkadot_holder) =>
					if is_polkadot_holder {
						optimal_hold_index = index;
						is_hold = true;

						break
					},
				Err(e) => error!("Assertion A7 request is_holder error: {:?}", e),
			}
		}
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
