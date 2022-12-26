// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate::{build_client, vec_to_string, Error, HttpError, G_DATA_PROVIDERS};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use serde::{Deserialize, Serialize};
use std::{
	default::Default,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

pub struct TwitterLitentryClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for TwitterLitentryClient {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckFollow {
	#[serde(rename(deserialize = "data"))]
	result: bool,
}

impl RestPath<String> for CheckFollow {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

impl TwitterLitentryClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		let client = build_client(
			G_DATA_PROVIDERS.read().unwrap().twitter_litentry_url.clone().as_str(),
			headers,
		);
		TwitterLitentryClient { client }
	}

	/// check if the source account follow the target account.
	pub fn check_follow(&mut self, source: Vec<u8>, target: Vec<u8>) -> Result<bool, Error> {
		let source = vec_to_string(source)?;
		let target = vec_to_string(target)?;
		let query = vec![("handler1", target.as_str()), ("handler2", source.as_str())];
		let response = self
			.client
			.get_with::<String, CheckFollow>(
				"twitter/followers/verification".to_string(),
				query.as_slice(),
			)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		Ok(response.result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use httpmock::prelude::*;
	use lc_mock_server::standalone_server;

	#[test]
	fn check_follow_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let body = r#"{ "data": false }"#;
		let path = "/twitter/followers/verification";
		server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("handler1", "litentry")
				.query_param("handler2", "ericzhangeth");
			then.status(200).body(body);
		});

		let mut client = TwitterLitentryClient::new();
		let source = "ericzhangeth".as_bytes().to_vec();
		let target = "litentry".as_bytes().to_vec();

		let result = client.check_follow(source, target);
		assert!(result.is_ok(), "error: {:?}", result);
	}
}
