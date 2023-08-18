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
	achainable::{AchainableClient, AchainableHolder, ParamsBasicTypeWithAmountHolding},
	vec_to_string,
};
use std::string::ToString;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::sync::SgxMutex as Mutex;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::thread;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::time;

const VC_A11_SUBJECT_DESCRIPTION: &str =
	"The length of time a user continues to hold a particular token (with particular threshold of token amount)";
const VC_A11_SUBJECT_TYPE: &str = "ETH Holding Time";

pub fn build(req: &AssertionBuildRequest, min_balance: ParameterString) -> Result<Credential> {
	debug!("Assertion A11 build, who: {:?}", account_id_to_string(&req.who),);

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A11(min_balance.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let addresses: Vec<String> = vec!["0x6E97a66f5D57476582f6C130d2e00A25EE52e0B8".into(),
		// "0x46d316399616466d57a9da8c8e73154df58e56f8".into(),
		// "0x2de0f34004a4ae7dba78394b5b97471e4cbe2c8c".into(),
		// "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266".into(),
		// "0x6725e40066e36dbedf6bbdce7e2fc4ed556d8980".into(),
		// "0xec19dd802446f2fb6cae296eebe4aef94e6d67eb".into()
		];

	let mut is_hold = false;
	let mut optimal_hold_index = 0_usize;
	for (index, date) in ASSERTION_FROM_DATE.iter().enumerate() {
		if is_hold {
			break
		}

		for address in &addresses {
			error!(">>> index: {index} , date: {date}");
			error!(">>> address: {address}");

			let holding = ParamsBasicTypeWithAmountHolding::new(
				&Web3Network::Ethereum,
				q_min_balance.to_string(),
				date.to_string(),
				None,
			);

			let is_eth_holder = client.is_holder(address, holding).map_err(|e| {
				error!("Assertion A11 request is_holder error: {:?}", e);
				Error::RequestVCFailed(Assertion::A11(min_balance.clone()), e.into_error_detail())
			})?;

			if is_eth_holder {
				optimal_hold_index = index;
				is_hold = true;

				break
			}
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A11_SUBJECT_DESCRIPTION, VC_A11_SUBJECT_TYPE);
			credential_unsigned.update_holder(
				is_hold,
				&q_min_balance,
				&ASSERTION_FROM_DATE[optimal_hold_index].into(),
			);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A11(min_balance), e.into_error_detail()))
		},
	}
}
