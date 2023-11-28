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

use crate::{build_client, vec_to_string, DataProviderConfig, Error, HttpError, UserInfo};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::{format, string::String, vec, vec::Vec};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUser {
	pub id: String,
	pub username: String,
	pub discriminator: String,
}

impl RestPath<String> for DiscordUser {
	fn get_path(path: String) -> Result<String, HttpError> {
		Ok(path)
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
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			data_provider_config.discord_auth_token.clone().as_str(),
		);
		let client =
			build_client(data_provider_config.discord_official_url.clone().as_str(), headers);
		DiscordOfficialClient { client }
	}

	pub fn query_message(
		&mut self,
		channel_id: Vec<u8>,
		message_id: Vec<u8>,
	) -> Result<DiscordMessage, Error> {
		let channel_id = vec_to_string(channel_id)?;
		let message_id = vec_to_string(message_id)?;
		debug!("discord query msg, channel_id: {}, message_id: {}", channel_id, message_id);

		let path = format!("/api/channels/{}/messages/{}", channel_id, message_id);
		let query = vec![];
		self.client
			.get_with::<String, DiscordMessage>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}

	pub fn get_user_info(&mut self, user_id: String) -> Result<DiscordUser, Error> {
		debug!("discord query user, id: {}", user_id);

		let path = format!("/api/users/{}", user_id);
		let query = vec![];
		self.client
			.get_with::<String, DiscordUser>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use lc_mock_server::run;

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap();
		let mut data_provider_config = DataProviderConfig::new();
		data_provider_config.set_discord_official_url(url);
		data_provider_config
	}

	#[test]
	fn query_message_work() {
		let data_provider_config = init();

		let channel_id = "919848392035794945";
		let message_id = "1";

		let mut client = DiscordOfficialClient::new(&data_provider_config);
		let result =
			client.query_message(channel_id.as_bytes().to_vec(), message_id.as_bytes().to_vec());
		assert!(result.is_ok(), "query discord error: {:?}", result);

		let message = result.unwrap();
		assert_eq!(message.id, message_id);
		assert_eq!(message.author.id, "001");
		assert_eq!(message.author.username, "elon");
		assert_eq!(message.content, "Hello, litentry.");
		assert_eq!(message.channel_id, channel_id)
	}
}
