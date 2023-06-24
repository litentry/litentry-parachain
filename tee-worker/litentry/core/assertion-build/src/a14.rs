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
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath, RestPost,
};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::Credential;
use lc_data_providers::{build_client, Error, HttpError};
use log::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

const VC_A14_SUBJECT_DESCRIPTION: &str = "The user has participated in any polkadot governance";
const VC_A14_SUBJECT_TYPE: &str = "Governance participation proof";
const VC_A14_SUBJECT_TAG: [&str; 2] = ["Polkadot", "Governance"];

// TODO: merge it to new achainable API client once the migration is done
pub struct A14Client {
	client: RestClient<HttpClient<DefaultSend>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A14Data {
	params: A14DataParams,
	includeMetadata: bool,
	includeWidgets: bool,
}

impl RestPath<String> for A14Data {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct A14DataParams {
	address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct A14Response {
	#[serde(flatten)]
	data: serde_json::Value,
}

impl A14Client {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().graphql_auth_key.clone().as_str(),
		);
		let client =
			build_client("https://label-production.graph.tdf-labs.io/v1/run/label/a719e99c-1f9b-432e-8f1d-cb3de0f14dde", headers);
		A14Client { client }
	}

	pub fn send_request(&self, data: &A14Data) -> Result<A14Response, Error> {
		self.client
			.post_capture::<String, A14Data, A14Response>(String::default(), data)
			.map_err(|e| Error::RequestError(e.into()))
	}
}

pub fn build(
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	who: &AccountId,
) -> Result<Credential> {
	debug!("Assertion A14 build, who: {:?}", account_id_to_string(&who));

	// achainable expects polkadot addresses (those start with 1...)
	let mut polkadot_addresses = vec![];
	for id in identities {
		if let Identity::Substrate { network: SubstrateNetwork::Polkadot, address } = id {
			let (_, address) = subcryptor::ss58_address_of(address.as_ref(), "polkadot")
				.map_err(|_| Error::RequestVCFailed(Assertion::A14, ErrorDetail::ParseError))?;
			polkadot_addresses.push(address);
		}
	}

	let mut value = false;
	let client = A14Client::new();

	for address in polkadot_addresses {
		let data = A14Data {
			params: A14DataParams { address },
			includeMetadata: false,
			includeWidgets: false,
		};
		let res = client
			.send_request(&data)
			.map_err(|e| Error::RequestVCFailed(Assertion::A14, ErrorDetail::ParseError))?;
	}

	match Credential::new_default(who, &shard.clone()) {
		Ok(mut credential_unsigned) => {
			// add subject info
			credential_unsigned.add_subject_info(
				VC_A14_SUBJECT_DESCRIPTION,
				VC_A14_SUBJECT_TYPE,
				VC_A14_SUBJECT_TAG.to_vec(),
			);

			// add assertion
			// credential_unsigned.add_assertion_a14(flag);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A1, e.into_error_detail()))
		},
	}
}
