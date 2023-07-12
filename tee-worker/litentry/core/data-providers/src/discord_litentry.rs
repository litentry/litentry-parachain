// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::{build_client, vec_to_string, Error, HttpError, GLOBAL_DATA_PROVIDER_CONFIG};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use log::*;
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
			GLOBAL_DATA_PROVIDER_CONFIG
				.write()
				.unwrap()
				.discord_litentry_url
				.clone()
				.as_str(),
			headers,
		);
		DiscordLitentryClient { client }
	}

	// user has joined Discord guild
	pub fn check_join(
		&mut self,
		guild_id: Vec<u8>,
		handler: Vec<u8>,
	) -> Result<DiscordResponse, Error> {
		let guild_id_s = vec_to_string(guild_id)?;
		let handler_s = vec_to_string(handler)?;
		debug!("discord check join, guild_id: {}, handler: {}", guild_id_s, handler_s);

		let path = "/discord/joined".to_string();
		let query = vec![("guildid", guild_id_s.as_str()), ("handler", handler_s.as_str())];
		self.client
			.get_with::<String, DiscordResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}

	// user has commented in channel with Role 'ID-Hubber'
	pub fn check_id_hubber(
		&mut self,
		guild_id: Vec<u8>,
		channel_id: Vec<u8>,
		role_id: Vec<u8>,
		handler: Vec<u8>,
	) -> Result<DiscordResponse, Error> {
		let guild_id_s = vec_to_string(guild_id)?;
		let channel_id_s = vec_to_string(channel_id)?;
		let role_id_s = vec_to_string(role_id)?;
		let handler_s = vec_to_string(handler)?;
		debug!(
			"discord check id_hubber, guild_id: {}, channel_id: {}, role_id: {}, handler: {}",
			guild_id_s, channel_id_s, role_id_s, handler_s
		);

		let path = "/discord/commented/idhubber".to_string();
		let query = vec![
			("guildid", guild_id_s.as_str()),
			("channelid", channel_id_s.as_str()),
			("roleid", role_id_s.as_str()),
			("handler", handler_s.as_str()),
		];

		let res = self
			.client
			.get_with::<String, DiscordResponse>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)));

		res
	}

	// assign ID-Hubber Role to User
	pub fn assign_id_hubber(
		&mut self,
		guild_id: Vec<u8>,
		handler: Vec<u8>,
	) -> Result<DiscordResponse, Error> {
		let guild_id_s = vec_to_string(guild_id)?;
		let handler_s = vec_to_string(handler)?;
		debug!("discord assign id_hubber, guild_id: {}, handler: {}", guild_id_s, handler_s);

		let path = "/discord/assgin/idhubber".to_string();
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
	use lc_mock_server::{default_getter, run};
	use std::sync::Arc;

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER_CONFIG.write().unwrap().set_discord_litentry_url(url);
	}

	#[test]
	fn check_join_work() {
		init();
		let guild_id = "919848390156767232".as_bytes().to_vec();
		let handler = "againstwar#4779".as_bytes().to_vec();
		let mut client = DiscordLitentryClient::new();
		let response = client.check_join(guild_id, handler);
		assert!(response.is_ok(), "check join discord error: {:?}", response);
	}

	#[test]
	fn check_id_hubber_work() {
		init();
		let guild_id = "919848390156767232".as_bytes().to_vec();
		let channel_id = "919848392035794945".as_bytes().to_vec();
		let role_id = "1034083718425493544".as_bytes().to_vec();
		let handler = "ericzhang.eth#0114".as_bytes().to_vec();
		let mut client = DiscordLitentryClient::new();
		let response = client.check_id_hubber(guild_id, channel_id, role_id, handler);
		assert!(response.is_ok(), "check discord id hubber error: {:?}", response);
	}
}
