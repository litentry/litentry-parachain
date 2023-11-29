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

use crate::{build_client, Error, HttpError, GLOBAL_DATA_PROVIDER_CONFIG};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
	vec,
};

pub trait VIP3QuerySet {
	fn sbt_info(&mut self, address: &str) -> Result<VIP3SBTInfoResponse, Error>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VIP3SBTReqBody {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelEntity {
	// level == 0: NO VIP3 SBT holding
	// level == 1: Silver
	// level == 2: Gold
	pub level: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VIP3SBTInfoResponse {
	pub code: usize,
	pub msg: String,
	pub data: LevelEntity,
}

impl RestPath<String> for VIP3SBTInfoResponse {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

pub struct VIP3Client {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl VIP3Client {
	pub fn new() -> Self {
		let api_url = GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().vip3_url.clone();

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client(api_url.as_str(), headers);

		VIP3Client { client }
	}
}

impl VIP3QuerySet for VIP3Client {
	fn sbt_info(&mut self, address: &str) -> Result<VIP3SBTInfoResponse, Error> {
		let path = "/api/v1/sbt/info".to_string();
		let query = vec![("addr", address)];

		self.client
			.get_with::<String, VIP3SBTInfoResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}
}
