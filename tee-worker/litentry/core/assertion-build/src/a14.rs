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
use frame_support::traits::Len;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::{
	graphql::{AchainableQuery, GraphQLClient, VerifiedCredentialsIsHodlerIn},
	vec_to_string,
};
use litentry_primitives::{Address32, SupportedNetwork};
use log::*;
use std::{
	format,
	string::{String, ToString},
	vec::Vec,
};

const VC_SUBJECT_DESCRIPTION: &str =
	"The user has been consistently holding at least {x} amount of {y} tokens at given years on native network";
const VC_SUBJECT_TYPE: &str = "Native Token Holding Assertion";
const VC_SUBJECT_TAG: [&str; 2] = ["Litentry", "Polkadot"];

pub fn build(
	min_balance: ParameterString,
	years: ParameterYears,
	token: NativeToken,
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	who: &AccountId,
) -> Result<Credential> {
	debug!(
		"Assertion A14 build, who: {:?}, identities: {:?}",
		account_id_to_string(&who),
		identities
	);

	let q_min_balance = vec_to_string(min_balance.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::A14(min_balance.clone(), token.clone(), years.clone()),
			ErrorDetail::ParseError,
		)
	})?;

	let mut client = GraphQLClient::new();
	let supported_network: SupportedNetwork = token.clone().into();
	let addresses: Vec<String> = collect_addresses_from_identities(identities, supported_network);
	let assertion_dates = prepare_assertion_dates(&years).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::A14(min_balance.clone(), token.clone(), years.clone()),
			ErrorDetail::ParseError,
		)
	})?;
	let mut results: Vec<(String, bool)> = Vec::with_capacity(assertion_dates.len());
	for from_date in assertion_dates.iter() {
		let vch = VerifiedCredentialsIsHodlerIn::new(
			addresses.clone(),
			from_date.to_string(),
			SupportedNetwork::Litentry,
			"".to_string(),
			q_min_balance.to_string(),
		);
		match client.verified_credentials_is_hodler(vch) {
			Ok(is_hodler_out) => results
				.push((from_date.to_string(), is_hodler_out.hodlers.iter().any(|h| h.is_hodler))),
			Err(e) =>
				error!("Assertion A14 request check_verified_credentials_is_hodler error: {:?}", e),
		};
	}

	match Credential::new_default(who, &shard.clone()) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(
				VC_SUBJECT_DESCRIPTION,
				VC_SUBJECT_TYPE,
				VC_SUBJECT_TAG.to_vec(),
			);
			credential_unsigned
				.update_holder_at_dates(&q_min_balance, &results)
				.map_err(|e| {
					Error::RequestVCFailed(
						Assertion::A14(min_balance, token, years),
						e.into_error_detail(),
					)
				})?;

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::A14(min_balance, token, years),
				e.into_error_detail(),
			))
		},
	}
}

fn collect_addresses_from_identities(
	identities: Vec<Identity>,
	supported_network: SupportedNetwork,
) -> Vec<String> {
	identities
		.iter()
		.map(|i| match supported_network {
			SupportedNetwork::Litentry => match i {
				Identity::Substrate { network: SubstrateNetwork::Litentry, address } =>
					Some(prepare_address(address)),
				_ => None,
			},
			SupportedNetwork::Polkadot => match i {
				Identity::Substrate { network: SubstrateNetwork::Polkadot, address } =>
					Some(prepare_address(address)),
				_ => None,
			},
			_ => None,
		})
		.filter(Option::is_some)
		.flatten()
		.collect()
}

fn prepare_address(account_id: &Address32) -> String {
	let mut address = account_id_to_string(account_id.as_ref());
	address.insert_str(0, "0x");
	address
}

fn prepare_assertion_dates(
	years: &ParameterYears,
) -> std::result::Result<Vec<String>, lc_data_providers::Error> {
	let mut formatted_years = Vec::with_capacity(years.len());
	for year in years {
		let year = vec_to_string(year.to_vec())?;
		formatted_years.push(format!("{}-01-01", year));
	}
	Ok(formatted_years)
}

#[cfg(test)]
mod tests {
	use crate::a14::prepare_assertion_dates;
	use litentry_primitives::ParameterYears;
	use sp_runtime::BoundedVec;

	#[test]
	pub fn test_prepare_assertion_dates() {
		let years: ParameterYears = BoundedVec::truncate_from(
			vec!["2020", "2021", "2022"]
				.iter()
				.map(|date| {
					let bytes_vec = date.as_bytes().to_vec();
					BoundedVec::truncate_from(bytes_vec)
				})
				.collect(),
		);

		assert_eq!(
			vec!["2020-01-01", "2021-01-01", "2022-01-01"],
			prepare_assertion_dates(&years).unwrap()
		)
	}
}
