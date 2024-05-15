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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{
	build_client_with_cert, DataProviderConfig, Error, HttpError, ReqPath, RetryOption,
	RetryableRestGet,
};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestPath,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
	str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

pub struct MagicCraftClient {
	retry_option: RetryOption,
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

#[derive(Debug)]
pub struct MagicCraftRequest {
	path: String,
	query: Option<Vec<(String, String)>>,
}

impl MagicCraftClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let api_retry_delay = data_provider_config.magic_craft_api_retry_delay;
		let api_retry_times = data_provider_config.magic_craft_api_retry_times;
		let api_url = data_provider_config.magic_craft_api_url.clone();
		let api_key = data_provider_config.magic_craft_api_key.clone();
		let retry_option =
			RetryOption { retry_delay: Some(api_retry_delay), retry_times: Some(api_retry_times) };

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert("X-API-Key", api_key.as_str());
		let client = build_client_with_cert(api_url.as_str(), headers);

		MagicCraftClient { retry_option, client }
	}

	fn get<T>(&mut self, params: MagicCraftRequest, fast_fail: bool) -> Result<T, Error>
	where
		T: serde::de::DeserializeOwned + for<'a> RestPath<ReqPath<'a>>,
	{
		let retry_option: Option<RetryOption> =
			if fast_fail { None } else { Some(self.retry_option.clone()) };
		if let Some(query) = params.query {
			let transformed_query: Vec<(&str, &str)> =
				query.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
			self.client.get_with_retry::<ReqPath, T>(
				ReqPath::new(params.path.as_str()),
				&transformed_query,
				retry_option,
			)
		} else {
			self.client
				.get_retry::<ReqPath, T>(ReqPath::new(params.path.as_str()), retry_option)
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserVerificationResponse {
	pub user: bool,
}

impl<'a> RestPath<ReqPath<'a>> for UserVerificationResponse {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

pub trait MagicCraftApi {
	fn user_verification(
		&mut self,
		address: String,
		fail_fast: bool,
	) -> Result<UserVerificationResponse, Error>;
}

impl MagicCraftApi for MagicCraftClient {
	fn user_verification(
		&mut self,
		address: String,
		fail_fast: bool,
	) -> Result<UserVerificationResponse, Error> {
		let query: Vec<(String, String)> = vec![("wallet_address".to_string(), address)];

		let params = MagicCraftRequest { path: "litentry/user".into(), query: Some(query) };

		debug!("user_verification, params: {:?}", params);

		match self.get::<UserVerificationResponse>(params, fail_fast) {
			Ok(resp) => {
				debug!("user_verification, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("user_verification, error: {:?}", e);
				Err(e)
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use lc_mock_server::run;

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap() + "/magic_craft/";

		let mut config = DataProviderConfig::new().unwrap();
		config.set_magic_craft_api_url(url).unwrap();
		config
	}

	#[test]
	fn does_user_verification_works() {
		let config = init();
		let mut client = MagicCraftClient::new(&config);
		let mut response = client
			.user_verification("0x49ad262c49c7aa708cc2df262ed53b64a17dd5ee".into(), true)
			.unwrap();
		assert_eq!(response.user, true);

		response = client
			.user_verification("0x9401518f4ebba857baa879d9f76e1cc8b31ed197".into(), false)
			.unwrap();
		assert_eq!(response.user, false);
	}
}
