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

use crate::{
	build_client_with_cert, vec_to_string, DataProviderConfig, Error, HttpError, UserInfo,
};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestGet, RestPath,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUserAccessTokenData {
	pub client_id: String,
	pub client_secret: String,
	pub code: String,
	pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordUserAccessToken {
	pub access_token: String,
	pub refresh_token: String,
	pub token_type: String,
	pub expires_in: u32,
	pub scope: String,
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

impl RestPath<String> for DiscordUserAccessToken {
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
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

impl DiscordOfficialClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			data_provider_config.discord_auth_token.clone().as_str(),
		);
		let client = build_client_with_cert(
			data_provider_config.discord_official_url.clone().as_str(),
			headers,
		);
		DiscordOfficialClient { client }
	}

	pub fn with_access_token(url: &str, token_type: &str, access_token: &str) -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(AUTHORIZATION.as_str(), format!("{} {}", token_type, access_token).as_str());
		let client = build_client_with_cert(url, headers);
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

	pub fn request_user_access_token(
		&mut self,
		data: DiscordUserAccessTokenData,
	) -> Result<DiscordUserAccessToken, Error> {
		debug!("Discord create access token");

		let path = "/api/oauth2/token".to_string();

		let mut body = HashMap::new();
		body.insert("client_id".to_string(), data.client_id);
		body.insert("client_secret".to_string(), data.client_secret);
		body.insert("grant_type".to_string(), "authorization_code".to_string());
		body.insert("code".to_string(), data.code);
		body.insert("redirect_uri".to_string(), data.redirect_uri);

		let user_token = self
			.client
			.post_form_urlencoded_capture::<String, DiscordUserAccessToken>(path, body)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		Ok(user_token)
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use lc_mock_server::run;

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap();
		let mut data_provider_config = DataProviderConfig::new().unwrap();
		data_provider_config.set_discord_official_url(url).unwrap();
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
		assert_eq!(message.author.id, "002");
		assert_eq!(message.author.username, "alice");
		assert_eq!(
			message.content,
			"63e9a4e993c5dad5f8a19a22057d6c6a86172cf5380711467675061c7ed11bf8"
		);
		assert_eq!(message.channel_id, channel_id)
	}

	#[test]
	fn get_user_info_work() {
		let data_provider_config = init();

		let user_id = "@me";

		let mut client = DiscordOfficialClient::new(&data_provider_config);
		let result = client.get_user_info(user_id.to_string());
		assert!(result.is_ok(), "query discord error: {:?}", result);

		let user = result.unwrap();
		assert_eq!(user.id, "001".to_string());
		assert_eq!(user.username, "bob");
		assert_eq!(user.discriminator, "0");
	}

	#[test]
	fn request_user_access_token_work() {
		let data_provider_config = init();

		let data = DiscordUserAccessTokenData {
			client_id: "test-client-id".to_string(),
			client_secret: "test-client-secret".to_string(),
			code: "test-code".to_string(),
			redirect_uri: "http://localhost:3000/redirect".to_string(),
		};
		let mut client = DiscordOfficialClient::new(&data_provider_config);

		let result = client.request_user_access_token(data);
		assert!(result.is_ok(), "error: {:?}", result);
	}
}
