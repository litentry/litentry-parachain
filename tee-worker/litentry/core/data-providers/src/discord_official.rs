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

use crate::{build_client, vec_to_string, Error, HttpError, UserInfo, G_DATA_PROVIDERS};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use serde::{Deserialize, Serialize};
use std::{default::Default, format, string::String, vec, vec::Vec};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordMessage {
	pub id: String, // message_id
	pub channel_id: String,
	pub content: String,
	pub author: DiscordMessageAuthor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordMessageAuthor {
	pub id: String, //user_id
	pub username: String,
}

impl Default for DiscordOfficialClient {
	fn default() -> Self {
		Self::new()
	}
}

impl RestPath<String> for DiscordMessage {
	fn get_path(path: String) -> Result<String, HttpError> {
		Ok(path)
	}
}

impl UserInfo for DiscordMessage {
	fn get_user_id(&self) -> Option<String> {
		Some(self.author.id.clone())
	}
}

pub struct DiscordOfficialClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl DiscordOfficialClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().discord_auth_token.clone().as_str(),
		);
		let client = build_client(
			G_DATA_PROVIDERS.read().unwrap().discord_official_url.clone().as_str(),
			headers,
		);
		DiscordOfficialClient { client }
	}

	pub fn query_message(
		&mut self,
		channel_id: Vec<u8>,
		message_id: Vec<u8>,
	) -> Result<DiscordMessage, Error> {
		let channel_id = vec_to_string(channel_id)?;
		let message_id = vec_to_string(message_id)?;
		let path = format!("/api/channels/{}/messages/{}", channel_id, message_id);
		let query = vec![];
		self.client
			.get_with::<String, DiscordMessage>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use httpmock::prelude::*;
	use lc_mock_server::standalone_server;

	#[test]
	fn query_message_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let channel_id = "919848392035794945";
		let message_id = "1";

		let user_id = "001";
		let username = "elon";
		let author = DiscordMessageAuthor { id: user_id.into(), username: username.into() };

		let body = DiscordMessage {
			id: message_id.into(),
			channel_id: channel_id.into(),
			content: "Hello, elon.".into(),
			author,
		};

		let path = format! {"/api/channels/{}/messages/{}", channel_id, message_id};
		server.mock(|when, then| {
			when.method(GET).path(path);
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});

		let mut client = DiscordOfficialClient::new();
		let channel_id = "919848392035794945".as_bytes().to_vec();
		let message_id = "1".as_bytes().to_vec();
		let result = client.query_message(channel_id, message_id);
		assert!(result.is_ok(), "query discord error: {:?}", result);
	}
}
