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

use crate::{build_client, DataProviderConfig, Error, HttpError};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
	thread, time, vec,
	vec::Vec,
};

pub struct KaratDaoClient {
	api_retry_delay: u64,
	api_retry_times: u16,
	client: RestClient<HttpClient<DefaultSend>>,
}

#[derive(Debug)]
pub struct KaraDaoRequest {
	path: String,
	query: Option<Vec<(String, String)>>,
}

impl KaratDaoClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let api_retry_delay = data_provider_config.karat_dao_api_retry_delay;
		let api_retry_times = data_provider_config.karat_dao_api_retry_times;
		let api_url = data_provider_config.karat_dao_api_url.clone();

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client(api_url.as_str(), headers);

		KaratDaoClient { api_retry_delay, api_retry_times, client }
	}

	fn retry<A, T>(&mut self, action: A) -> Result<T, Error>
	where
		A: Fn(&mut KaratDaoClient) -> Result<T, HttpError>,
	{
		let mut retries = 0;
		// retry delay 1 second
		let base_delay = time::Duration::from_millis(self.api_retry_delay);
		// maximum 5 retry times
		let maximum_retries = self.api_retry_times;

		loop {
			if retries > 0 {
				debug!("Fail to call karat dao api, begin retry: {}", retries);
			}

			if retries > maximum_retries {
				return Err(Error::RequestError(format!(
					"Fail to call call karat dao api within {} retries",
					maximum_retries
				)))
			}

			match action(self) {
				Ok(response) => return Ok(response),
				Err(err) => {
					let req_err: Error =
						Error::RequestError(format!("karat dao api error: {}", err));
					match err {
						HttpError::HttpError(code, _) =>
							if code == 429 {
								// Too Many Requests
								// exponential back off
								thread::sleep(base_delay * 2u32.pow(retries as u32));
								retries += 1;
							} else {
								return Err(req_err)
							},
						_ => return Err(req_err),
					}
				},
			}
		}
	}

	fn get<T>(&mut self, params: KaraDaoRequest) -> Result<T, Error>
	where
		T: serde::de::DeserializeOwned + RestPath<String>,
	{
		if let Some(query) = params.query {
			let transformed_query: Vec<(&str, &str)> =
				query.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
			self.retry(|c| c.client.get_with::<String, T>(params.path.clone(), &transformed_query))
		} else {
			self.retry(|c| c.client.get::<String, T>(params.path.clone()))
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserVerificationResponse {
	pub result: UserVerificationResult,
}

impl RestPath<String> for UserVerificationResponse {
	fn get_path(path: String) -> Result<String, HttpError> {
		Ok(path)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserVerificationResult {
	pub is_valid: bool,
}

pub trait KaraDaoApi {
	fn user_verification(&mut self, address: String) -> Result<UserVerificationResponse, Error>;
}

impl KaraDaoApi for KaratDaoClient {
	fn user_verification(&mut self, address: String) -> Result<UserVerificationResponse, Error> {
		let query: Vec<(String, String)> = vec![("address".to_string(), address)];

		let params = KaraDaoRequest { path: "user/verification".into(), query: Some(query) };

		debug!("user_verification, params: {:?}", params);

		match self.get::<UserVerificationResponse>(params) {
			Ok(resp) => {
				debug!("user_verification, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("user_verification, error: {:?}", e);
				Err(Error::RequestError(format!("{:?}", e)))
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
		let url = run(0).unwrap() + "/karat_dao/";

		let mut config = DataProviderConfig::new();
		config.set_karat_dao_api_url(url);
		config
	}

	#[test]
	fn does_user_verification_works() {
		let config = init();
		let mut client = KaratDaoClient::new(&config);
		let mut response = client
			.user_verification("0x49ad262c49c7aa708cc2df262ed53b64a17dd5ee".into())
			.unwrap();
		assert_eq!(response.result.is_valid, true);

		response = client
			.user_verification("0x9401518f4ebba857baa879d9f76e1cc8b31ed197".into())
			.unwrap();
		assert_eq!(response.result.is_valid, false);
	}
}
