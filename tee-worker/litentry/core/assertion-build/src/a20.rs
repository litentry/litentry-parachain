// Copyright 2020-2024 Trust Computing GmbH.
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

use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{error::Error as RestClientError, RestGet, RestPath};
use lc_credentials::IssuerRuntimeVersion;
use lc_data_providers::{build_client_with_cert, DataProviderConfig};
use serde::{Deserialize, Serialize};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::*;
use std::string::ToString;

const VC_A20_SUBJECT_DESCRIPTION: &str =
	"The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.";
const VC_A20_SUBJECT_TYPE: &str = "IDHub EVM Version Early Bird";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EarlyBirdResponse {
	has_joined: bool,
}
impl RestPath<String> for EarlyBirdResponse {
	fn get_path(_path: String) -> core::result::Result<String, RestClientError> {
		Ok("events/does-user-joined-evm-campaign".to_string())
	}
}

pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	// Note: it's not perfectly implemented here
	//       it only attests if the main address meets the criteria, but we should have implemented
	//       the supported web3networks and attested the linked identities.
	//       However, this VC is probably too old to change
	let who = match req.who {
		Identity::Substrate(account) => account_id_to_string(&account),
		Identity::Evm(account) => account_id_to_string(&account),
		Identity::Bitcoin(account) => account_id_to_string(&account),
		_ => unreachable!(),
	};
	debug!("Assertion A20 build, who: {:?}", who);

	let mut headers = Headers::new();
	headers.insert(CONNECTION.as_str(), "close");
	let mut client = build_client_with_cert(&data_provider_config.litentry_archive_url, headers);
	let query = vec![("account", who.as_str())];
	let value = client
		.get_with::<String, EarlyBirdResponse>("".to_string(), query.as_slice())
		.map(|data| data.has_joined)
		.map_err(|e| {
			Error::RequestVCFailed(
				Assertion::A20,
				ErrorDetail::DataProviderError(ErrorString::truncate_from(
					format!("{e:?}").as_bytes().to_vec(),
				)),
			)
		})?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_A20_SUBJECT_DESCRIPTION, VC_A20_SUBJECT_TYPE);
			credential_unsigned.add_assertion_a20(value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A20, e.into_error_detail()))
		},
	}
}
