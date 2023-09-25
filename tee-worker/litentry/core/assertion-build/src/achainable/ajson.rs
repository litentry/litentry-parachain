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
use http::header::{CONNECTION, AUTHORIZATION};
use http_req::response::Headers;
use itc_rest_client::{RestPath, RestPost, rest_client::RestClient, http_client::{DefaultSend, HttpClient}};
use lc_credentials::assertion_logic::{AssertionLogic, Op};
use lc_data_providers::{vec_to_string, DataProviderConfigReader, ReadDataProviderConfig, DataProviderConfig, build_client,
};
use serde::{Serialize, Deserialize};
use serde_json::{Map, json};

////////////////////////////////////////////////////////////////////////////////
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
	pub name: String,
	pub address: String,
	pub params: serde_json::Value,

	/// TODO:
	/// Because some interfaces of the achainable API cannot meet the current assertion requirements well, this trade-off is being made.
	/// This field is added here to request once interface to obtain the specific account creation date, and then match it with the product's time interval.
	/// And according to TDF developers, this field is unstable and may be cancelled in the future. Even so, this is currently the most appropriate approach
	/// So, this is the current solution.
	pub include_metadata: bool,
}
impl RestPath<ASystemLabelReqPath> for AReqBody {
	fn get_path(req_params: ASystemLabelReqPath) -> std::result::Result<std::string::String, itc_rest_client::error::Error> {
		Ok(req_params.path)
	}
}

impl AReqBody {
	pub fn new(name: String, address: String, params: serde_json::Value) -> Self {
		AReqBody { name, address, params, include_metadata: true }
	}
}

pub trait JObjectRequest {
    fn request(&mut self, address: &str, param: serde_json::Value) -> core::result::Result<bool, Error>;
}

impl JObjectRequest for JsonClient {
    fn request(&mut self, address: &str, param: serde_json::Value) -> core::result::Result<bool, Error> {
        let body = AReqBody::new("name".into(), address.into(), param);

		let response = self.client
			.post_capture::<ASystemLabelReqPath, AReqBody, serde_json::Value>(ASystemLabelReqPath::default(), &body).unwrap();
		debug!(">>> response: {:?}", response);
        Ok(true)
    }
}


// TODO: merge it to new achainable API client once the migration is done
pub struct JsonClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl JsonClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			"26353d4c-b01c-4466-98a5-80d3fc53a9d8",
		);
		let client =
			build_client("https://label-production.graph.tdf-labs.io/v1/run/system-labels", headers);
        JsonClient { client }
	}
}

pub trait UpdateJsonObject {
    fn update(&mut self, ret: bool);
}
impl UpdateJsonObject for Credential {
    fn update(&mut self, value: bool) {
        let is_fresh_account = AssertionLogic::new_item("$is_fresh_account", Op::Equal, "true");
		let assertion = AssertionLogic::new_and().add_item(is_fresh_account);

		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

        self.add_subject_info("Json object description", "Json object type");
    }
}

pub fn build(req: &AssertionBuildRequest, json_object: ParameterString) -> Result<Credential> {
	debug!("Assertion AchainableJson build, who: {:?}", account_id_to_string(&req.who));

    let jobj = pre_build(&json_object)?;
    let res = transform_value(&jobj);

    let address = "0xA7EFAe728D2936e78BDA97dc267687568dD593f3".into();
    let data_provider_config = DataProviderConfigReader::read().unwrap();
    let mut client = JsonClient::new(&data_provider_config);
    let ret = client.request(address, res)?;

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update(ret);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::AchainableJsonObject(json_object), e.into_error_detail()))
		},
	}
}

fn pre_build(json_object: &ParameterString) -> Result<String> {
	vec_to_string(json_object.to_vec()).map_err(|_| {
		Error::RequestVCFailed(Assertion::AchainableJsonObject(json_object.clone()), ErrorDetail::ParseError)
	})

    // // Is valid json
    // serde_json::from_str(&data).map_err(|_| {
	// 	Error::RequestVCFailed(Assertion::A4(json_object.clone()), ErrorDetail::ParseError)
	// })
}

fn transform_value(value: &str) -> serde_json::Value {
    let mut map: Map<String, serde_json::Value> = serde_json::from_str(value).expect("failed to read file");
    map.remove("name");
    map.remove("address");

    json!(map)
}