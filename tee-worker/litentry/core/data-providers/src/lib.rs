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

#![cfg_attr(not(feature = "std"), no_std)]

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

use core::time::Duration;
use http_req::response::Headers;
use itc_rest_client::{
	error::Error as HttpError,
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
};
use std::{
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

#[cfg(all(not(test), not(feature = "mockserver")))]
pub mod base_url {
	pub(crate) const TWITTER_OFFICIAL: &str = "https://api.twitter.com";
	pub(crate) const TWITTER_LITENTRY: &str = "http://47.57.13.126:8080";

	pub(crate) const DISCORD_OFFICIAL: &str = "https://discordapp.com";
	pub(crate) const DISCORD_LITENTRY: &str = "http://47.57.13.126:8080";

	pub(crate) const GRAPHQL_URL: &str = "https://graph.tdf-labs.io/";
	pub(crate) const GRAPHQL_AUTH_KEY: &str = "ac2115ec-e327-4862-84c5-f25b6b7d4533";
}

// #[cfg(test)]
#[cfg(any(test, feature = "mockserver"))]
pub mod base_url {
	pub(crate) const TWITTER_OFFICIAL: &str = "http://localhost:9527";
	pub(crate) const TWITTER_LITENTRY: &str = "http://localhost:9527";
	pub(crate) const DISCORD_OFFICIAL: &str = "http://localhost:9527";
	pub(crate) const DISCORD_LITENTRY: &str = "http://localhost:9527";
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
	// println!("base_url: {}", base_url);
	let base_url = Url::parse(base_url).unwrap();
	let http_client = HttpClient::new(DefaultSend {}, true, Some(TIMEOUT), Some(headers), None);
	RestClient::new(http_client, base_url)
}
