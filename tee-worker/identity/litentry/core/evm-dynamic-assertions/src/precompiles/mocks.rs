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

use http_req::{request::Method, response::Response};
use itc_rest_client::{
	error::Error,
	http_client::{EncodedBody, SendHttpRequest},
	rest_client::Url,
	Query, RestPath,
};

use serde_json::json;

#[derive(Default)]
pub struct MockedHttpClient {
	pub malformed_json: bool,
	pub http_err: bool,
}

impl MockedHttpClient {
	pub fn malformed_json() -> Self {
		Self { malformed_json: true, http_err: false }
	}

	pub fn http_error() -> Self {
		Self { malformed_json: false, http_err: true }
	}
}

impl SendHttpRequest for MockedHttpClient {
	fn send_request<U, T>(
		&self,
		_base_url: Url,
		_method: Method,
		_params: U,
		_query: Option<&Query<'_>>,
		_maybe_body: Option<String>,
	) -> Result<(Response, EncodedBody), Error>
	where
		T: RestPath<U>,
	{
		const HEAD: &[u8; 102] = b"HTTP/1.1 200 OK\r\n\
                         Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                         Content-Type: text/html\r\n\
                         Content-Length: 100\r\n\r\n";

		let response = Response::from_head(HEAD).unwrap();

		Ok((response, vec![]))
	}

	fn send_request_raw(
		&self,
		_url: Url,
		_method: Method,
		_maybe_body: Option<String>,
		_headers: Vec<(String, String)>,
	) -> Result<(Response, EncodedBody), Error> {
		if !self.http_err {
			const HEAD: &[u8; 102] = b"HTTP/1.1 200 OK\r\n\
                         Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                         Content-Type: text/html\r\n\
                         Content-Length: 100\r\n\r\n";
			Ok((
				Response::from_head(HEAD).unwrap(),
				if self.malformed_json {
					"{{".as_bytes().to_vec()
				} else {
					serde_json::to_string(&json!({
						"bool": true,
						"i64": 10,
						"string": "string",
						"not_bool": 10
					}))
					.unwrap()
					.as_bytes()
					.to_vec()
				},
			))
		} else {
			Err(Error::HttpError(404, "Not found".to_string()))
		}
	}
}
