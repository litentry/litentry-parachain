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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::*;
use blake2_rfc::blake2b::Blake2b;
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	error::Error as RestClientError,
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestPath, RestPost,
};
use lc_data_providers::{build_client, GLOBAL_DATA_PROVIDER_CONFIG};
use rust_base58::ToBase58;
use serde::{Deserialize, Serialize};
use ss58_registry::Ss58AddressFormat;

const VC_A14_SUBJECT_DESCRIPTION: &str =
	"The user has participated in any Polkadot on-chain governance events";
const VC_A14_SUBJECT_TYPE: &str = "Polkadot Governance Participation Proof";

// mostly copied from https://github.com/hack-ink/substrate-minimal/blob/main/subcryptor/src/lib.rs
// no_std version is used here
pub fn ss58_address_of(
	public_key: &[u8],
	network: &str,
) -> core::result::Result<String, ErrorDetail> {
	let network = Ss58AddressFormat::try_from(network).map_err(|_| ErrorDetail::ParseError)?;
	let prefix = u16::from(network);
	let mut bytes = match prefix {
		0..=63 => vec![prefix as u8],
		64..=16_383 => {
			let first = ((prefix & 0b0000_0000_1111_1100) as u8) >> 2;
			let second = ((prefix >> 8) as u8) | ((prefix & 0b0000_0000_0000_0011) as u8) << 6;

			vec![first | 0b01000000, second]
		},
		_ => Err(ErrorDetail::ParseError)?,
	};

	bytes.extend(public_key);

	let blake2b = {
		let mut context = Blake2b::new(64);
		context.update(b"SS58PRE");
		context.update(&bytes);
		context.finalize()
	};

	bytes.extend(&blake2b.as_bytes()[0..2]);

	Ok(bytes.to_base58())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct A14Data {
	params: A14DataParams,
	include_metadata: bool,
	include_widgets: bool,
}

impl RestPath<String> for A14Data {
	fn get_path(path: String) -> core::result::Result<String, RestClientError> {
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

// TODO: merge it to new achainable API client once the migration is done
pub struct A14Client {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for A14Client {
	fn default() -> Self {
		Self::new()
	}
}

impl A14Client {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().achainable_auth_key.clone().as_str(),
		);
		let client =
			build_client("https://label-production.graph.tdf-labs.io/v1/run/label/a719e99c-1f9b-432e-8f1d-cb3de0f14dde", headers);
		A14Client { client }
	}

	pub fn send_request(&mut self, data: &A14Data) -> Result<A14Response> {
		self.client
			.post_capture::<String, A14Data, A14Response>(String::default(), data)
			.map_err(|e| {
				Error::RequestVCFailed(
					Assertion::A14,
					ErrorDetail::DataProviderError(ErrorString::truncate_from(
						format!("{e:?}").as_bytes().to_vec(),
					)),
				)
			})
	}
}

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	debug!("Assertion A14 build, who: {:?}", account_id_to_string(&req.who));

	// achainable expects polkadot addresses (those start with 1...)
	let mut polkadot_addresses = vec![];
	for identity in &req.identities {
		if let Identity::Substrate(address) = identity.0 {
			let address = ss58_address_of(address.as_ref(), "polkadot")
				.map_err(|_| Error::RequestVCFailed(Assertion::A14, ErrorDetail::ParseError))?;
			polkadot_addresses.push(address);
		}
	}

	let mut value = false;
	let mut client = A14Client::new();

	for address in polkadot_addresses {
		let data = A14Data {
			params: A14DataParams { address },
			include_metadata: false,
			include_widgets: false,
		};
		let response = client.send_request(&data)?;

		let result = response
			.data
			.get("result")
			.and_then(|r| r.as_bool())
			.ok_or(Error::RequestVCFailed(Assertion::A14, ErrorDetail::ParseError))?;
		if result {
			value = result;
			break
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			// add subject info
			credential_unsigned.add_subject_info(VC_A14_SUBJECT_DESCRIPTION, VC_A14_SUBJECT_TYPE);

			// add assertion
			credential_unsigned.add_assertion_a14(value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A14, e.into_error_detail()))
		},
	}
}
