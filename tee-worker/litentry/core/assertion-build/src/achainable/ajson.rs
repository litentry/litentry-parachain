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
	RestPath, RestPost,
};
use lc_credentials::assertion_logic::{AssertionLogic, Op};
use lc_data_providers::{
	build_client, vec_to_string, DataProviderConfig, DataProviderConfigReader,
	ReadDataProviderConfig,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ASystemLabelReqPath {
	path: String,
}

impl Default for ASystemLabelReqPath {
	fn default() -> Self {
		Self { path: "/v1/run/system-labels".into() }
	}
}

impl ASystemLabelReqPath {
	pub fn new(path: &str) -> Self {
		Self { path: path.into() }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AReqBody {
	pub address: String,

	#[serde(flatten)]
	pub params: serde_json::Value,

	/// TODO:
	/// Because some interfaces of the achainable API cannot meet the current assertion requirements well, this trade-off is being made.
	/// This field is added here to request once interface to obtain the specific account creation date, and then match it with the product's time interval.
	/// And according to TDF developers, this field is unstable and may be cancelled in the future. Even so, this is currently the most appropriate approach
	/// So, this is the current solution.
	pub include_metadata: bool,
}
impl RestPath<ASystemLabelReqPath> for AReqBody {
	fn get_path(
		req_params: ASystemLabelReqPath,
	) -> std::result::Result<std::string::String, itc_rest_client::error::Error> {
		Ok(req_params.path)
	}
}

impl AReqBody {
	pub fn new(address: String, params: serde_json::Value) -> Self {
		AReqBody { address, params, include_metadata: true }
	}
}

pub trait JObjectRequest {
	fn request(&mut self, body: &AReqBody) -> core::result::Result<bool, Error>;
}

impl JObjectRequest for JsonClient {
	fn request(&mut self, body: &AReqBody) -> core::result::Result<bool, Error> {
		let response = self
			.client
			.post_capture::<ASystemLabelReqPath, AReqBody, serde_json::Value>(
				ASystemLabelReqPath::default(),
				body,
			)
			.unwrap();

		let ret = self.get(&response);
		Ok(ret)
	}
}

pub struct JsonClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl JsonClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let key = data_provider_config.achainable_auth_key.as_str();
		let url = data_provider_config.achainable_url.as_str();

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(AUTHORIZATION.as_str(), key);

		let client = build_client(url, headers);
		JsonClient { client }
	}
}

pub trait GetResult {
	fn get(&self, value: &serde_json::Value) -> bool;
}
impl GetResult for JsonClient {
	fn get(&self, value: &serde_json::Value) -> bool {
		value.get("result").and_then(|res| res.as_bool()).unwrap_or_default()
	}
}

pub trait UpdateJsonObject {
	fn update(&mut self, value: bool, types: &str, description: &str);
}
impl UpdateJsonObject for Credential {
	fn update(&mut self, value: bool, types: &str, description: &str) {
		let is_fresh_account = AssertionLogic::new_item("$found_on_bsc", Op::Equal, "true");
		let assertion = AssertionLogic::new_and().add_item(is_fresh_account);

		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		self.add_subject_info(description, types);
	}
}

fn pre_build(json_object: &ParameterString) -> Result<String> {
	// 1. ToString
	vec_to_string(json_object.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::AchainableJsonObject(json_object.clone()),
			ErrorDetail::ParseError,
		)
	})

	// 2. Check Json
}

pub fn build(req: &AssertionBuildRequest, json_object: ParameterString) -> Result<Credential> {
	let jobj = pre_build(&json_object)?;

	let inner = JsonObjectInner::from_str(&jobj).map_err(|_e| {
		Error::RequestVCFailed(
			Assertion::AchainableJsonObject(json_object.clone()),
			ErrorDetail::ParseError,
		)
	})?;
	let params = inner.get_params();

	let data_provider_config = DataProviderConfigReader::read().map_err(|e| {
		Error::RequestVCFailed(Assertion::AchainableJsonObject(json_object.clone()), e)
	})?;
	let mut client = JsonClient::new(&data_provider_config);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut ret = false;
	for address in addresses {
		if ret {
			break
		}

		let body = AReqBody::new(address, params.clone());
		ret |= client.request(&body)?;
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update(ret, &inner.types, &inner.description);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::AchainableJsonObject(json_object),
				e.into_error_detail(),
			))
		},
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonObjectInner {
	pub types: String,
	pub description: String,

	#[serde(flatten)]
	pub params: serde_json::Value,
}

impl JsonObjectInner {
	pub fn get_params(&self) -> serde_json::Value {
		let params = self.params.get("params").unwrap();
		params.clone()
	}
}

impl FromStr for JsonObjectInner {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use lc_data_providers::GLOBAL_DATA_PROVIDER_CONFIG;
	use lc_mock_server::{default_getter, run};

	fn new_achainable_client() -> JsonClient {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER_CONFIG.write().unwrap().set_achainable_url(url);

		let data_provider_config = DataProviderConfigReader::read().unwrap();
		JsonClient::new(&data_provider_config)
	}

	#[test]
	fn request_works() {
		let address = "0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1".into();
		let jobj = r#"
			{
				"types": "Im types", 
				"description": "Im description", 
				"params": {
					"name": "Account found on {chain}", 
					"params": {"chain": "bsc"}
				} 
			}
		"#;

		let inner = JsonObjectInner::from_str(&jobj).unwrap();
		let params = inner.get_params();
		let body = AReqBody::new(address, params);

		let mut client = new_achainable_client();
		let ret = client.request(&body).unwrap();

		assert!(ret);
	}
}
