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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use hex_sgx as hex;
	pub use http_req_sgx as http_req;
	pub use http_sgx as http;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use codec::{Decode, Encode};
use core::time::Duration;
use http_req::response::Headers;
use itc_rest_client::{
	error::Error as HttpError,
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
};
use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

use litentry_primitives::{ErrorDetail, ErrorString, IntoErrorDetail};
use std::{
	format,
	string::{String, ToString},
	vec::Vec,
};
use url::Url;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

pub mod discord_litentry;
pub mod discord_official;
pub mod twitter_litentry;
pub mod twitter_official;

pub mod graphql;

const TIMEOUT: Duration = Duration::from_secs(3u64);

#[derive(PartialEq, Eq, Clone, Encode, Decode, Serialize, Deserialize)]
pub struct DataProvidersStatic {
	pub twitter_official_url: String,
	pub twitter_litentry_url: String,
	pub twitter_auth_token_v2: String,
	pub twitter_auth_token_v1_1: String,
	pub discord_official_url: String,
	pub discord_litentry_url: String,
	pub discord_auth_token: String,
	pub graphql_url: String,
	pub graphql_auth_key: String,
	pub credential_endpoint: String,
}
impl Default for DataProvidersStatic {
	fn default() -> Self {
		Self::new()
	}
}
impl DataProvidersStatic {
	pub fn new() -> Self {
		DataProvidersStatic {
			twitter_official_url: "https://api.twitter.com".to_string(),
			twitter_litentry_url: "".to_string(),
			twitter_auth_token_v2: "Bearer ".to_string(),
			twitter_auth_token_v1_1: "Bearer ".to_string(),
			discord_official_url: "https://discordapp.com".to_string(),
			discord_litentry_url: "".to_string(),
			discord_auth_token: "".to_string(),
			graphql_url: "https://graph.tdf-labs.io/".to_string(),
			graphql_auth_key: "".to_string(),
			credential_endpoint: "".to_string(),
		}
	}
	pub fn set_twitter_official_url(&mut self, v: String) {
		debug!("set_twitter_official_url: {:?}", v);
		self.twitter_official_url = v;
	}
	pub fn set_twitter_litentry_url(&mut self, v: String) {
		debug!("set_twitter_litentry_url: {:?}", v);
		self.twitter_litentry_url = v;
	}
	pub fn set_twitter_auth_token_v2(&mut self, v: String) {
		debug!("set_twitter_auth_token_v2: {:?}", v);
		self.twitter_auth_token_v2 = v;
	}
	pub fn set_twitter_auth_token_v1_1(&mut self, v: String) {
		debug!("set_twitter_auth_token_v1_1: {:?}", v);
		self.twitter_auth_token_v1_1 = v;
	}
	pub fn set_discord_official_url(&mut self, v: String) {
		debug!("set_discord_official_url: {:?}", v);
		self.discord_official_url = v;
	}
	pub fn set_discord_litentry_url(&mut self, v: String) {
		debug!("set_discord_litentry_url: {:?}", v);
		self.discord_litentry_url = v;
	}
	pub fn set_discord_auth_token(&mut self, v: String) {
		debug!("set_discord_auth_token: {:?}", v);
		self.discord_auth_token = v;
	}
	pub fn set_graphql_url(&mut self, v: String) {
		debug!("set_graphql_url: {:?}", v);
		self.graphql_url = v;
	}
	pub fn set_graphql_auth_key(&mut self, v: String) {
		debug!("set_graphql_auth_key: {:?}", v);
		self.graphql_auth_key = v;
	}
	pub fn set_credential_endpoint(&mut self, v: String) {
		debug!("set_credential_endpoint: {:?}", v);
		self.credential_endpoint = v;
	}
}

lazy_static! {
	pub static ref G_DATA_PROVIDERS: RwLock<DataProvidersStatic> =
		RwLock::new(DataProvidersStatic::new());
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("UTF8 error: {0}")]
	Utf8Error(String),

	#[error("GraphQL error: {0}")]
	GraphQLError(String),
}

impl IntoErrorDetail for Error {
	fn into_error_detail(self) -> ErrorDetail {
		ErrorDetail::DataProviderError(ErrorString::truncate_from(
			format!("{self:?}").as_bytes().to_vec(),
		))
	}
}

pub trait UserInfo {
	fn get_user_id(&self) -> Option<String>;
}

pub fn vec_to_string(vec: Vec<u8>) -> Result<String, Error> {
	let tmp = String::from_utf8(vec.to_vec()).map_err(|e| Error::Utf8Error(e.to_string()))?;
	let tmp = tmp.trim();
	if tmp.is_empty() {
		return Err(Error::Utf8Error("empty string".to_string()))
	}
	Ok(tmp.to_string())
}

pub fn build_client(base_url: &str, headers: Headers) -> RestClient<HttpClient<DefaultSend>> {
	debug!("base_url: {}", base_url);
	let base_url = Url::parse(base_url).unwrap();
	let http_client = HttpClient::new(DefaultSend {}, true, Some(TIMEOUT), Some(headers), None);
	RestClient::new(http_client, base_url)
}
