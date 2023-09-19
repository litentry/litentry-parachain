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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

// a dummy comment
// another one
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

use litentry_primitives::{
	AchainableParams, Assertion, ErrorDetail, ErrorString, IntoErrorDetail, ParameterString,
	VCMPError,
};
use std::{
	format,
	string::{String, ToString},
	vec::Vec,
};
use url::Url;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

pub mod achainable;
pub mod discord_litentry;
pub mod discord_official;
pub mod twitter_official;

const TIMEOUT: Duration = Duration::from_secs(3u64);

pub const LIT_TOKEN_ADDRESS: &str = "0xb59490ab09a0f526cc7305822ac65f2ab12f9723";
pub const WBTC_TOKEN_ADDRESS: &str = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599";
pub const WETH_TOKEN_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const UNISWAP_TOKEN_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
pub const USDT_TOKEN_ADDRESS: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";

#[derive(PartialEq, Eq, Clone, Encode, Decode, Serialize, Deserialize)]
pub struct DataProviderConfig {
	pub twitter_official_url: String,
	pub twitter_litentry_url: String,
	pub twitter_auth_token_v2: String,
	pub discord_official_url: String,
	pub discord_litentry_url: String,
	pub discord_auth_token: String,
	pub achainable_url: String,
	pub achainable_auth_key: String,
	pub credential_endpoint: String,
	pub oneblock_notion_key: String,
	pub oneblock_notion_url: String,
}

impl Default for DataProviderConfig {
	fn default() -> Self {
		Self::new()
	}
}

impl DataProviderConfig {
	pub fn new() -> Self {
		DataProviderConfig {
			twitter_official_url: "https://api.twitter.com".to_string(),
			twitter_litentry_url: "".to_string(),
			twitter_auth_token_v2: "Bearer ".to_string(),
			discord_official_url: "https://discordapp.com".to_string(),
			discord_litentry_url: "".to_string(),
			discord_auth_token: "".to_string(),
			achainable_url: "https://graph.tdf-labs.io/".to_string(),
			achainable_auth_key: "".to_string(),
			credential_endpoint: "".to_string(),
			oneblock_notion_key: "".to_string(),
			oneblock_notion_url: "".to_string(),
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
	pub fn set_achainable_url(&mut self, v: String) {
		debug!("set_achainable_url: {:?}", v);
		self.achainable_url = v;
	}
	pub fn set_achainable_auth_key(&mut self, v: String) {
		debug!("set_achainable_auth_key: {:?}", v);
		self.achainable_auth_key = v;
	}
	pub fn set_credential_endpoint(&mut self, v: String) {
		debug!("set_credential_endpoint: {:?}", v);
		self.credential_endpoint = v;
	}
	pub fn set_oneblock_notion_key(&mut self, v: String) {
		debug!("set_oneblock_notion_key: {:?}", v);
		self.oneblock_notion_key = v;
	}
	pub fn set_oneblock_notion_url(&mut self, v: String) {
		debug!("set_oneblock_notion_url: {:?}", v);
		self.oneblock_notion_url = v;
	}
}

lazy_static! {
	pub static ref GLOBAL_DATA_PROVIDER_CONFIG: RwLock<DataProviderConfig> =
		RwLock::new(DataProviderConfig::new());
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("UTF8 error: {0}")]
	Utf8Error(String),

	#[error("Achainable error: {0}")]
	AchainableError(String),
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

pub trait ConvertParameterString {
	fn to_string(&self, field: &ParameterString) -> Result<String, VCMPError>;
}

impl ConvertParameterString for AchainableParams {
	fn to_string(&self, field: &ParameterString) -> Result<String, VCMPError> {
		vec_to_string(field.to_vec()).map_err(|_| {
			VCMPError::RequestVCFailed(Assertion::Achainable(self.clone()), ErrorDetail::ParseError)
		})
	}
}
