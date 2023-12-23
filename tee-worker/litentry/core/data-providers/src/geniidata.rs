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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{
	build_client_with_cert, DataProviderConfigReader, Error as DataProviderError,
	ReadDataProviderConfig,
};
use http::header::ACCEPT;
use http_req::response::Headers;
use itc_rest_client::{
	error::Error as RestClientError,
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestGet, RestPath,
};
use litentry_primitives::ErrorDetail;
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseItem {
	pub tick: String,
	pub address: String,
	pub overall_balance: String,
	pub transferable_balance: String,
	pub available_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReponseData {
	pub count: u64,
	pub limit: String,
	pub offset: String,
	pub list: Vec<ResponseItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeniidataResponse {
	pub code: u64,
	pub message: String,
	pub data: ReponseData,
}

impl RestPath<String> for GeniidataResponse {
	fn get_path(path: String) -> core::result::Result<String, RestClientError> {
		Ok(path)
	}
}

pub struct GeniidataClient {
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

impl GeniidataClient {
	pub fn new() -> core::result::Result<Self, ErrorDetail> {
		let data_provider_config = DataProviderConfigReader::read()?;

		let mut headers = Headers::new();
		headers.insert(ACCEPT.as_str(), "application/json");
		headers.insert("api-key", data_provider_config.geniidata_api_key.as_str());

		let client = build_client_with_cert(data_provider_config.geniidata_url.as_str(), headers);

		Ok(GeniidataClient { client })
	}

	pub fn create_brc20_amount_holder_sum(
		&mut self,
		addresses: Vec<String>,
	) -> Result<Vec<ResponseItem>, DataProviderError> {
		let mut all_items: Vec<ResponseItem> = Vec::new();
		for address in addresses {
			let query = vec![("limit", "20"), ("offset", "0"), ("address", &address)];
			let response = self
				.client
				.get_with::<String, GeniidataResponse>("".to_string(), query.as_slice())
				.map_err(|e| {
					DataProviderError::GeniiDataError(format!("GeniiData response error: {}", e))
				})?;

			all_items.extend(response.data.list);
		}

		Ok(all_items)
	}
}
