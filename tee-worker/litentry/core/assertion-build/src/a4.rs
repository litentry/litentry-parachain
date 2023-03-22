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
	Identity, ParentchainBalance, ParentchainBlockNumber, ASSERTION_FROM_DATE,
};
use log::*;
use std::{
	str::from_utf8,
	string::{String, ToString},
	vec,
	vec::Vec,
};

// ERC20 LIT token address
const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";
const VC_SUBJECT_DESCRIPTION: &str =
	"Check whether any of the linked accounts hold a minimum amount of LIT NOW";
const VC_SUBJECT_TYPE: &str = "LIT Holder";

pub fn build(
	identities: Vec<Identity>,
	min_balance: ParentchainBalance,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	debug!(
		"Assertion A4 build, who: {:?}, bn: {}, identities: {:?}",
		account_id_to_string(&who),
		bn,
		identities
	);

	let mut client = GraphQLClient::new();
	let mut found = false;
	let mut from_date_index = 0_usize;

	for identity in identities.iter() {
		if found {
			break
		}

		let mut verified_network = VerifiedCredentialsNetwork::Polkadot;
		if identity.is_web3() {
			match identity {
				Identity::Substrate { network, .. } => verified_network = (*network).into(),
				Identity::Evm { network, .. } => verified_network = (*network).into(),
				_ => {},
			}
		}
		if matches!(
			verified_network,
			VerifiedCredentialsNetwork::Litentry
				| VerifiedCredentialsNetwork::Litmus
				| VerifiedCredentialsNetwork::LitentryRococo
				| VerifiedCredentialsNetwork::Ethereum
		) {
			let q_min_balance: f64 = if verified_network == VerifiedCredentialsNetwork::Litentry
				|| verified_network == VerifiedCredentialsNetwork::Litmus
				|| verified_network == VerifiedCredentialsNetwork::LitentryRococo
			{
				(min_balance / (10 ^ 12)) as f64
			} else {
				(min_balance / (10 ^ 18)) as f64
			};

			let mut addresses: Vec<String> = vec![];
			match &identity {
				Identity::Evm { address, .. } => {
					let mut address = account_id_to_string(address.as_ref());
					address.insert_str(0, "0x");
					debug!("	[AssertionBuild] A4 EVM address : {}", address);

					addresses.push(address);
				},
				Identity::Substrate { address, .. } => {
					let mut address = account_id_to_string(address.as_ref());
					address.insert_str(0, "0x");
					debug!("	[AssertionBuild] A4 Substrate address : {}", address);

					addresses.push(address);
				},
				Identity::Web2 { address, .. } => match from_utf8(address.as_ref()) {
					Ok(addr) => addresses.push(addr.to_string()),
					Err(e) => error!(
						"	[AssertionBuild] A4 parse error Web2 address {:?}, {:?}",
						address, e
					),
				},
			}
			let mut tmp_token_addr = String::from("");
			if verified_network == VerifiedCredentialsNetwork::Ethereum {
				tmp_token_addr = LIT_TOKEN_ADDRESS.to_string();
			}

			for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
				// if found is true, no need to check it continually
				if found {
					from_date_index = index + 1;
					break
				}

				let vch = VerifiedCredentialsIsHodlerIn::new(
					addresses.clone(),
					from_date.to_string(),
					verified_network.clone(),
					tmp_token_addr.clone(),
					q_min_balance,
				);
				match client.check_verified_credentials_is_hodler(vch) {
					Ok(is_hodler_out) => {
						for holder in is_hodler_out.verified_credentials_is_hodler.iter() {
							found = found || holder.is_hodler;
						}
					},
					Err(e) => error!("	[BuildAssertion] A4, Request, {:?}", e),
				}
			}
		}
	}

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			credential_unsigned.update_holder(from_date_index, min_balance);

			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(Error::Assertion4Failed)
}
