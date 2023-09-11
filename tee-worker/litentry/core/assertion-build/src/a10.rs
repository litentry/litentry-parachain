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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::*;
use itp_utils::debug as lit_debug;
use lc_data_providers::{
	achainable::{AchainableClient, AchainableHolder, ParamsBasicTypeWithAmountHolding},
	vec_to_string, WBTC_TOKEN_ADDRESS,
};
use std::string::ToString;

const VC_A10_SUBJECT_DESCRIPTION: &str =
	"The length of time a user continues to hold a particular token (with particular threshold of token amount)";
const VC_A10_SUBJECT_TYPE: &str = "WBTC Holding Time";

// WBTC Holder
pub fn build(req: &AssertionBuildRequest, min_balance: ParameterString) -> Result<Credential> {
	lit_debug!("Assertion A10 build, who: {:?}", account_id_to_string(&req.who));

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A10(min_balance.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut is_hold = false;
	let mut optimal_hold_index = 0;
	for (index, date) in ASSERTION_FROM_DATE.iter().enumerate() {
		if is_hold {
			break
		}

		for address in &addresses {
			let holding = ParamsBasicTypeWithAmountHolding::new(
				&Web3Network::Ethereum,
				q_min_balance.to_string(),
				date.to_string(),
				Some(WBTC_TOKEN_ADDRESS.into()),
			);

			let is_wbtc_holder = client.is_holder(address, holding).map_err(|e| {
				error!("Assertion A10 request is_holder error: {:?}", e);
				Error::RequestVCFailed(Assertion::A10(min_balance.clone()), e.into_error_detail())
			})?;
			if is_wbtc_holder {
				optimal_hold_index = index;
				is_hold = true;

				break
			}
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A10_SUBJECT_DESCRIPTION, VC_A10_SUBJECT_TYPE);
			credential_unsigned.update_holder(
				is_hold,
				&q_min_balance,
				&ASSERTION_FROM_DATE[optimal_hold_index].into(),
			);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A10(min_balance), e.into_error_detail()))
		},
	}
}
