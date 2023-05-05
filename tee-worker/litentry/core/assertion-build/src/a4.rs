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

/// Here's an example of different assertions in this VC:
///
/// Imagine:
/// ALICE holds 100 LITs since 2018-03-02
/// BOB   holds 100 LITs since 2023-03-07
/// CAROL holds 0.1 LITs since 2020-02-22
///
/// min_amount is 1 LIT
///
/// If they all request A4, these are the received assertions:
/// ALICE:
/// [
///    from_date: < 2019-01-01
///    to_date: >= 2023-03-30 (now)
///    value: true
/// ]
///
/// BOB:
/// [
///    from_date: < 2017-01-01
///    to_date: >= 2023-03-30 (now)
///    value: false
/// ]
///
/// CAROL:
/// [
///    from_date: < 2017-01-01
///    to_date: >= 2023-03-30 (now)
///    value: false
/// ]
///
/// So just from the assertion results you can't distinguish between:
/// BOB, who just started to hold recently,
/// and CAROL, who has been holding for 3 years, but with too little amount
///
/// This is because the data provider doesn't provide more information, it only
/// takes the query with from_date and min_ammount, and returns true or false.
///
/// Please note:
/// the operators are mainly for IDHub's parsing, we will **NEVER** have:
/// - `from_date` with >= op, nor
/// - `value` is false but the `from_date` is something other than 2017-01-01.
///  
use crate::*;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::{
	graphql::{GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork},
	vec_to_string,
};
use log::*;
use std::{
	collections::{HashMap, HashSet},
	string::{String, ToString},
	vec::Vec,
};

// ERC20 LIT token address
const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

const VC_A4_SUBJECT_DESCRIPTION: &str =
	"The user has been consistently holding at least {x} amount of tokens before 2023 Jan 1st 00:00:00 UTC on the supporting networks";
const VC_A4_SUBJECT_TYPE: &str = "LIT Holding Assertion";
const VC_A4_SUBJECT_TAG: [&str; 3] = ["Ethereum", "Litmus", "Litentry"];

pub fn build(
	identities: Vec<Identity>,
	min_balance: ParameterString,
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

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::A4(min_balance.clone()), ErrorDetail::ParseError)
	})?;

	let mut client = GraphQLClient::new();
	let mut networks: HashMap<VerifiedCredentialsNetwork, HashSet<String>> = HashMap::new();

	identities.iter().for_each(|identity| {
		match identity {
			Identity::Substrate { network, address } => {
				let mut address = account_id_to_string(address.as_ref());
				address.insert_str(0, "0x");

				if_match_networks_collect_address(&mut networks, (*network).into(), address);
			},
			Identity::Evm { network, address } => {
				let mut address = account_id_to_string(address.as_ref());
				address.insert_str(0, "0x");

				if_match_networks_collect_address(&mut networks, (*network).into(), address);
			},
			_ => {},
		};
	});

	let mut is_hold = false;
	let mut optimal_hold_index = usize::MAX;

	// If both Substrate and Evm networks meet the conditions, take the interval with the longest holding time.
	// Here's an example:
	//
	// ALICE holds 100 LITs since 2018-03-02 on substrate network
	// ALICE holds 100 LITs since 2020-03-02 on evm network
	//
	// min_amount is 1 LIT
	//
	// the result should be
	// Alice:
	// [
	//    from_date: < 2019-01-01
	//    to_date: >= 2023-03-30 (now)
	//    value: true
	// ]
	for (verified_network, addresses) in networks {
		// If found query result is the optimal solution, i.e optimal_hold_index = 0, (2017-01-01)
		// there is no need to query other networks.
		if optimal_hold_index == 0 {
			break
		}

		// Each query loop needs to reset is_hold to false
		is_hold = false;

		let addresses: Vec<String> = addresses.into_iter().collect();
		let token_address = if verified_network == VerifiedCredentialsNetwork::Ethereum {
			LIT_TOKEN_ADDRESS
		} else {
			""
		};

		// TODO:
		// There is a problem here, because TDF does not support mixed network types,
		// It is need to request TDF 2 (substrate+evm networks) * 7 (ASSERTION_FROM_DATE) = 14 http requests.
		// If TDF can handle mixed network type, and even supports from_date array,
		// so that ideally, up to one http request can yield results.
		for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
			let vch = VerifiedCredentialsIsHodlerIn::new(
				addresses.clone(),
				from_date.to_string(),
				verified_network.clone(),
				token_address.to_string(),
				q_min_balance.to_string(),
			);
			match client.check_verified_credentials_is_hodler(vch) {
				Ok(is_hodler_out) => {
					for hodler in is_hodler_out.verified_credentials_is_hodler.iter() {
						is_hold = is_hold || hodler.is_hodler;
					}
				},
				Err(e) => error!(
					"Assertion A4 request check_verified_credentials_is_hodler error: {:?}",
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

	match Credential::new_default(who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_A4_SUBJECT_DESCRIPTION,
				VC_A4_SUBJECT_TYPE,
				VC_A4_SUBJECT_TAG.to_vec(),
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
			Err(Error::RequestVCFailed(Assertion::A4(min_balance), e.into_error_detail()))
		},
	}
}

fn if_match_networks_collect_address(
	networks: &mut HashMap<VerifiedCredentialsNetwork, HashSet<String>>,
	verified_network: VerifiedCredentialsNetwork,
	address: String,
) {
	if matches!(
		verified_network,
		VerifiedCredentialsNetwork::Litentry
			| VerifiedCredentialsNetwork::Litmus
			| VerifiedCredentialsNetwork::LitentryRococo
			| VerifiedCredentialsNetwork::Ethereum
	) {
		match networks.get_mut(&verified_network) {
			Some(set) => {
				set.insert(address);
			},
			None => {
				let mut set = HashSet::new();
				set.insert(address);

				networks.insert(verified_network.clone(), set);
			},
		}
	}
}
