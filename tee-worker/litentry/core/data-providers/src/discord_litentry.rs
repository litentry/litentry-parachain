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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordResponse {
	pub data: bool,
	pub message: String,
	pub has_errors: bool,
	pub msg_code: u32,
	pub success: bool,
}

impl RestPath<String> for DiscordResponse {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

pub struct DiscordLitentryClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for DiscordLitentryClient {
	fn default() -> Self {
		Self::new()
	}
}

impl DiscordLitentryClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");

		let client = build_client(
			G_DATA_PROVIDERS.write().unwrap().discord_litentry_url.clone().as_str(),
			headers,
		);
		DiscordLitentryClient { client }
	}

	pub fn check_join(
		&mut self,
		guild_id: Vec<u8>,
		handler: Vec<u8>,
	) -> Result<DiscordResponse, Error> {
		let guild_id_s = vec_to_string(guild_id)?;
		let handler_s = vec_to_string(handler)?;

		let path = "/discord/joined".to_string();
		let query = vec![("guildid", guild_id_s.as_str()), ("handler", handler_s.as_str())];
		self.client
			.get_with::<String, DiscordResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}

	pub fn check_id_hubber(
		&mut self,
		guild_id: Vec<u8>,
		handler: Vec<u8>,
	) -> Result<DiscordResponse, Error> {
		let guild_id_s = vec_to_string(guild_id)?;
		let handler_s = vec_to_string(handler)?;
		let path = "/discord/commented/idhubber".to_string();
		let query = vec![("guildid", guild_id_s.as_str()), ("handler", handler_s.as_str())];

		let res = self
			.client
			.get_with::<String, DiscordResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)));

		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use httpmock::prelude::*;
	use lc_mock_server::standalone_server;
	use std::vec::Vec;

	#[test]
	fn check_join_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let body = DiscordResponse {
			data: true,
			message: "success".into(),
			has_errors: false,
			msg_code: 200,
			success: true,
		};

		let path = "/discord/joined";
		server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("guildid", "919848390156767232")
				.query_param("handler", "againstwar#4779");
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});

		let guildid = "919848390156767232";
		let handler = "againstwar#4779";
		let guild_id_vec = guildid.as_bytes().to_vec();
		let handler_vec = handler.as_bytes().to_vec();

		let mut client = DiscordLitentryClient::new();
		let response = client.check_join(guild_id_vec, handler_vec);

		assert!(response.is_ok(), "check join discord error: {:?}", response);
	}

	#[test]
	fn check_id_hubber_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let body = DiscordResponse {
			data: true,
			message: "success".into(),
			has_errors: false,
			msg_code: 200,
			success: true,
		};

		server.mock(|when, then| {
			when.method(GET)
				.path("/discord/commented/idhubber")
				.query_param("guildid", "919848390156767232")
				.query_param("handler", "ericzhang.eth#0114");

			then.status(200).body(serde_json::to_string(&body).unwrap());
		});

		let guildid: u64 = 919848390156767232;
		let guild_id_vec: Vec<u8> = format!("{}", guildid).as_bytes().to_vec();
		let handler_vec: Vec<u8> = "ericzhang.eth#0114".as_bytes().to_vec();

		let mut client = DiscordLitentryClient::new();
		let response = client.check_id_hubber(guild_id_vec, handler_vec);

		assert!(response.is_ok(), "check discord id hubber error: {:?}", response);
	}
}
