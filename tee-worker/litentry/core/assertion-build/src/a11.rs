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

use crate::{Error, Result};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};
use litentry_primitives::{
	Assertion, EvmNetwork, Identity, ParentchainBalance, ParentchainBlockNumber,
	ASSERTION_FROM_DATE,
};
use log::*;
use std::{
	string::{String, ToString},
	vec,
	vec::Vec,
};

const VC_SUBJECT_DESCRIPTION: &str = "The user held ETH before a specific date/year";
const VC_SUBJECT_TYPE: &str = "ETH Hodler";

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
	let mut found = false;
	let mut from_date_index = 0_usize;

	for id in identities {
		if found {
			break
		}

		if let Identity::Evm { network, address } = id {
			if matches!(network, EvmNetwork::Ethereum) {
				let mut address = account_id_to_string(address.as_ref());
				address.insert_str(0, "0x");
				debug!("	[AssertionBuild] A11 Ethereum address : {}", address);

				let addresses = vec![address.to_string()];
				for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
					// if found is true, no need to check it continually
					if found {
						from_date_index = index + 1;
						break
					}

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
								found = found || hodler.is_hodler;
							}
						},
						Err(e) => error!("	[BuildAssertion] A11, Request, {:?}", e),
					}
				}
			}
		}
	}

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.update_holder(from_date_index, min_balance);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVcFailed(Assertion::A11(min_balance), e.to_error_detail()))
		},
	}
}
