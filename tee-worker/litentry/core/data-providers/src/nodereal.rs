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
use crate::sgx_reexport_prelude::chrono::{offset::Utc as TzUtc, DateTime, NaiveDateTime};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "std")]
use chrono::{offset::Utc as TzUtc, DateTime, NaiveDateTime};

use crate::{build_client_with_cert, DataProviderConfig, Error, HttpError};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestPath, RestPost,
};
use litentry_primitives::ErrorDetail;
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
	vec::Vec,
};

fn now() -> DateTime<TzUtc> {
	#[cfg(feature = "std")]
	{
		TzUtc::now()
	}

	#[cfg(all(not(feature = "std"), feature = "sgx"))]
	{
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("system time before Unix epoch");
		let naive =
			NaiveDateTime::from_timestamp_opt(now.as_secs() as i64, now.subsec_nanos() as u32)
				.unwrap();

		DateTime::from_utc(naive, TzUtc)
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoderealServiceReqPath {
	path: String,
}

impl Default for NoderealServiceReqPath {
	fn default() -> Self {
		Self { path: "".to_string() + "/spaceid/domain/names/" }
	}
}

impl NoderealServiceReqPath {
	pub fn new(api_key: &str, tag: &str) -> Self {
		Self { path: api_key.to_string() + "/spaceid/domain/names/" + tag }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpaceIDReqBody(Vec<String>);
impl RestPath<NoderealServiceReqPath> for SpaceIDReqBody {
	fn get_path(req: NoderealServiceReqPath) -> Result<String, HttpError> {
		Ok(req.path)
	}
}

pub struct NoderealClient {
	pub api_key: String,
	pub api_url: String,
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

impl NoderealClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let api_key = data_provider_config.nodereal_api_key.to_string();
		let api_url = data_provider_config.nodereal_api_url.to_string();

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client_with_cert(api_url.as_str(), headers);

		NoderealClient { api_key, api_url, client }
	}
}

pub trait NoderealHttpMethods {
	fn post(
		&mut self,
		path: NoderealServiceReqPath,
		body: &SpaceIDReqBody,
	) -> Result<serde_json::Value, Error>;
}

impl NoderealHttpMethods for NoderealClient {
	fn post(
		&mut self,
		path: NoderealServiceReqPath,
		body: &SpaceIDReqBody,
	) -> Result<serde_json::Value, Error> {
		let response = self
			.client
			.post_capture::<NoderealServiceReqPath, SpaceIDReqBody, serde_json::Value>(path, body);
		response.map_err(|e| Error::NoderealError(format!("Nodereal response error: {}", e)))
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DomainInfo {
	pub node_hash: String,
	pub bind: String,
	pub name: String,
	pub expires: String,
}

impl DomainInfo {
	pub fn from_value(v: &serde_json::Value) -> Result<Self, ErrorDetail> {
		serde_json::from_value(v.clone()).map_err(|_| ErrorDetail::ParseError)
	}

	#[allow(deprecated)]
	pub fn is_expired(&self) -> Result<bool, Error> {
		// e.g. "2014-11-28T12:45:59.324310806Z"
		let now = now();

		// "expires": "2032-08-24T00:15:56Z"
		let expired =
			NaiveDateTime::parse_from_str(&self.expires, "%Y-%m-%dT%H:%M:%SZ").map_err(|e| {
				Error::Utf8Error(format!("Nodereal parse domain expires date error: {:?}", e))
			})?;
		#[allow(deprecated)]
		let expired: DateTime<TzUtc> = DateTime::from_utc(expired, TzUtc);

		Ok(expired <= now)
	}
}

pub trait BnbDomainApiList {
	fn by_owners(&mut self, owners: &[String]) -> Result<serde_json::Value, Error>;
	fn by_binds(&mut self, owners: &[String]) -> Result<serde_json::Value, Error>;
	fn by_names(&mut self, names: &[String]) -> Result<serde_json::Value, Error>;
}

impl BnbDomainApiList for NoderealClient {
	fn by_owners(&mut self, owners: &[String]) -> Result<serde_json::Value, Error> {
		let req_body = SpaceIDReqBody(owners.to_vec());
		let path = NoderealServiceReqPath::new(&self.api_key, "byOwners");
		self.post(path, &req_body)
	}

	fn by_binds(&mut self, owners: &[String]) -> Result<serde_json::Value, Error> {
		let req_body = SpaceIDReqBody(owners.to_vec());
		let path = NoderealServiceReqPath::new(&self.api_key, "byBinds");
		self.post(path, &req_body)
	}

	fn by_names(&mut self, names: &[String]) -> Result<serde_json::Value, Error> {
		let req_body = SpaceIDReqBody(names.to_vec());
		let path = NoderealServiceReqPath::new(&self.api_key, "byNames");
		self.post(path, &req_body)
	}
}

#[cfg(test)]
mod tests {
	use super::DomainInfo;

	#[test]
	fn is_expired_works() {
		let s = r#"
		{
			"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c111",
			"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c111",
			"name": "xxx",
			"expires": "2000-03-19T18:16:59Z"
		}
		"#;
		let value = serde_json::from_str(s).unwrap();
		let info = DomainInfo::from_value(&value);
		assert!(info.unwrap().is_expired().unwrap())
	}
}
