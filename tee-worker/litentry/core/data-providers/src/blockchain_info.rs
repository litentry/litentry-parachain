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
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

pub struct BlockChainInfoClient {
	retry_option: RetryOption,
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

#[derive(Debug)]
pub struct BlockChainInfoRequest {
	path: String,
	query: Option<Vec<(String, String)>>,
}

impl BlockChainInfoClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let api_retry_delay = data_provider_config.blockchain_info_api_retry_delay;
		let api_retry_times = data_provider_config.blockchain_info_api_retry_times;
		let api_url = data_provider_config.blockchain_info_api_url.clone();
		let retry_option =
			RetryOption { retry_delay: Some(api_retry_delay), retry_times: Some(api_retry_times) };

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client_with_cert(api_url.as_str(), headers);

		BlockChainInfoClient { retry_option, client }
	}

	fn get<T>(&mut self, params: BlockChainInfoRequest, fast_fail: bool) -> Result<T, Error>
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
pub struct GetSingleAddressResponse {
	pub final_balance: u128,
}

impl<'a> RestPath<ReqPath<'a>> for GetSingleAddressResponse {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMultiAddressesResponse {
	pub wallet: GetMultiAddressesResponseWallet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMultiAddressesResponseWallet {
	pub final_balance: u128,
}

impl<'a> RestPath<ReqPath<'a>> for GetMultiAddressesResponse {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

// https://www.blockchain.com/explorer/api/blockchain_api
pub trait BlockChainInfoDataApi {
	fn get_single_address(
		&mut self,
		address: String,
		fail_fast: bool,
	) -> Result<GetSingleAddressResponse, Error>;

	fn get_multi_addresses(
		&mut self,
		addresses: Vec<String>,
		fail_fast: bool,
	) -> Result<GetMultiAddressesResponse, Error>;
}

impl BlockChainInfoDataApi for BlockChainInfoClient {
	fn get_single_address(
		&mut self,
		address: String,
		fail_fast: bool,
	) -> Result<GetSingleAddressResponse, Error> {
		let query: Vec<(String, String)> = vec![("limit".to_string(), "0".into())];

		let params =
			BlockChainInfoRequest { path: format!("rawaddr/{}", address), query: Some(query) };

		debug!("get_single_address, params: {:?}", params);

		match self.get::<GetSingleAddressResponse>(params, fail_fast) {
			Ok(resp) => {
				debug!("get_single_address, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_single_address, error: {:?}", e);
				Err(e)
			},
		}
	}

	fn get_multi_addresses(
		&mut self,
		addresses: Vec<String>,
		fail_fast: bool,
	) -> Result<GetMultiAddressesResponse, Error> {
		let query: Vec<(String, String)> =
			vec![("active".to_string(), addresses.join("|")), ("n".to_string(), "0".into())];

		let params = BlockChainInfoRequest { path: "multiaddr".into(), query: Some(query) };

		debug!("get_multi_addresses, params: {:?}", params);

		match self.get::<GetMultiAddressesResponse>(params, fail_fast) {
			Ok(resp) => {
				debug!("get_multi_addresses, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_multi_addresses, error: {:?}", e);
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
		let url = run(0).unwrap() + "/blockchain_info/";

		let mut config = DataProviderConfig::new().unwrap();
		config.set_blockchain_info_api_url(url).unwrap();
		config
	}

	#[test]
	fn does_get_single_address_works() {
		let config = init();
		let mut client = BlockChainInfoClient::new(&config);
		let mut response = client
			.get_single_address(
				"bc1pgr5fw4p9gl9me0vzjklnlnap669caxc0gsk4j62gff2qktlw6naqm4m3d0".into(),
				true,
			)
			.unwrap();
		assert_eq!(response.final_balance, 185123167511);

		response = client
			.get_single_address("bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj".into(), false)
			.unwrap();
		assert_eq!(response.final_balance, 0);
	}

	#[test]
	fn does_get_multi_addresses_works() {
		let config = init();
		let mut client = BlockChainInfoClient::new(&config);
		let response = client
			.get_multi_addresses(
				vec![
					"bc1pgr5fw4p9gl9me0vzjklnlnap669caxc0gsk4j62gff2qktlw6naqm4m3d0".into(),
					"bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj".into(),
				],
				true,
			)
			.unwrap();
		assert_eq!(response.wallet.final_balance, 185123167511);
	}
}
