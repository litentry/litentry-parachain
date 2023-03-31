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

use crate::{add_now_to_from_dates, Error, Result};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::{format_assertion_to_date, Credential};
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};
use litentry_primitives::{
	Assertion, EvmNetwork, Identity, ParentchainBalance, ParentchainBlockNumber,
};
use log::*;
use std::{
	string::{String, ToString},
	vec,
	vec::Vec,
};

const VC_SUBJECT_DESCRIPTION: &str = "The user held ETH before a specific date/year";
const VC_SUBJECT_TYPE: &str = "ETH Holder";

pub fn build(
	identities: Vec<Identity>,
	min_balance: ParentchainBalance,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	debug!(
		"Assertion A11 build, who: {:?}, bn: {}, identities: {:?}",
		account_id_to_string(&who),
		bn,
		identities,
	);

	// ETH decimals is 18.
	let q_min_balance: f64 = (min_balance / (10 ^ 18)) as f64;

	let mut client = GraphQLClient::new();
	let mut addresses = vec![];

	for id in identities {
		if let Identity::Evm { network, address } = id {
			if matches!(network, EvmNetwork::Ethereum) {
				let mut address = account_id_to_string(address.as_ref());
				address.insert_str(0, "0x");
				debug!("Assertion A11 Ethereum address : {}", address);

				addresses.push(address);
			}
		}
	}

	// is_hold default value is true
	// If there is no link, it will definitely not be "hold"
	let is_empty = addresses.is_empty();
	let mut is_hold = is_empty;

	let now = format_assertion_to_date();
	let mut hold_from_date = now.clone();

	if !is_empty {
		// Including 8 items from NOW -> 2017-01-01
		// Because it is necessary to determine the continuous holding, the reverse order query is started from NOW.
		// As long as there are terminals, this interval is taken as the result.
		// query from NOW -> 2023 -> ... -> 2017

		let dates = add_now_to_from_dates(&now);
		debug!("Assertion A11 dates: {:?}", dates);

		for from_date in dates.iter() {
			let vch = VerifiedCredentialsIsHodlerIn::new(
				addresses.clone(),
				from_date.to_string(),
				VerifiedCredentialsNetwork::Ethereum,
				String::from(""),
				q_min_balance,
			);
			match client.check_verified_credentials_is_hodler(vch) {
				Ok(is_hodler_out) => {
					for hodler in is_hodler_out.verified_credentials_is_hodler.iter() {
						is_hold = is_hold && hodler.is_hodler;
					}
				},
				Err(e) => error!(
					"Assertion A11 request check_verified_credentials_is_hodler error: {:?}",
					e
				),
			}

			// If Continuous holding， update hold_from_date, or just break
			if is_hold {
				hold_from_date = from_date.to_string();
			} else {
				break
			}
		}
	}

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.update_holder(is_hold, min_balance, &hold_from_date, &now);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A11(min_balance), e.to_error_detail()))
		},
	}
}
