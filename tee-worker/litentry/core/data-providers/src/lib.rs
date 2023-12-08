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

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use chrono_sgx as chrono;
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
use itp_utils::if_not_production;
use log::debug;
use serde::{Deserialize, Serialize};
use std::vec;

use itc_rest_client::http_client::SendWithCertificateVerification;
use litentry_primitives::{
	AchainableParams, Assertion, ErrorDetail, ErrorString, IntoErrorDetail, ParameterString,
	VCMPError,
};
use std::{
	env, format,
	string::{String, ToString},
	vec::Vec,
};
use url::Url;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

pub mod achainable;
pub mod achainable_names;
pub mod discord_litentry;
pub mod discord_official;
pub mod nodereal;
pub mod nodereal_jsonrpc;
pub mod twitter_official;
pub mod vip3;

const TIMEOUT: Duration = Duration::from_secs(3u64);

pub const WBTC_TOKEN_ADDRESS: &str = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599";
pub const WETH_TOKEN_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const USDT_TOKEN_ADDRESS: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
pub const USDC_TOKEN_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
pub const LIT_TOKEN_ADDRESS: &str = "0xb59490ab09a0f526cc7305822ac65f2ab12f9723";

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ETokenAddress {
	Wbtc,
	Lit,
	Usdc,
	Usdt,
	Unknown,
}

pub trait TokenFromString {
	fn from_vec(vec: ParameterString) -> ETokenAddress;
}
impl TokenFromString for ETokenAddress {
	fn from_vec(vec: ParameterString) -> ETokenAddress {
		let address = vec_to_string(vec.to_vec()).unwrap_or_default();
		if address == WBTC_TOKEN_ADDRESS {
			ETokenAddress::Wbtc
		} else if address == LIT_TOKEN_ADDRESS {
			ETokenAddress::Lit
		} else if address == USDT_TOKEN_ADDRESS {
			ETokenAddress::Usdt
		} else if address == USDC_TOKEN_ADDRESS {
			ETokenAddress::Usdc
		} else {
			ETokenAddress::Unknown
		}
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Serialize, Deserialize, Debug)]
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
	pub sora_quiz_master_id: String,
	pub sora_quiz_attendee_id: String,
	pub nodereal_api_key: String,
	pub nodereal_api_retry_delay: u64,
	pub nodereal_api_retry_times: u16,
	pub nodereal_api_url: String,
	pub nodereal_api_chain_network_url: String,
	pub contest_legend_discord_role_id: String,
	pub contest_popularity_discord_role_id: String,
	pub contest_participant_discord_role_id: String,
	pub vip3_url: String,
}

impl Default for DataProviderConfig {
	fn default() -> Self {
		Self::new()
	}
}

impl DataProviderConfig {
	pub fn new() -> Self {
		std::println!("Initializing data providers config");

		// default prod config
		let mut config = DataProviderConfig {
			twitter_official_url: "https://api.twitter.com".to_string(),
			twitter_litentry_url: "http://127.0.0.1:9527”".to_string(),
			twitter_auth_token_v2: "".to_string(),
			discord_official_url: "https://discordapp.com".to_string(),
			discord_litentry_url: "http://127.0.0.1:9527”".to_string(),
			discord_auth_token: "".to_string(),
			achainable_url: "https://label-production.graph.tdf-labs.io/".to_string(),
			achainable_auth_key: "".to_string(),
			credential_endpoint: "wss://tee-staging.litentry.io".to_string(),
			oneblock_notion_key: "".to_string(),
			oneblock_notion_url:
				"https://api.notion.com/v1/blocks/e4068e6a326243468f35dcdc0c43f686/children"
					.to_string(),
			sora_quiz_master_id: "1164463721989554218".to_string(),
			sora_quiz_attendee_id: "1166941149219532800".to_string(),
			nodereal_api_key: "".to_string(),
			nodereal_api_retry_delay: 5000,
			nodereal_api_retry_times: 2,
			nodereal_api_url: "https://open-platform.nodereal.io/".to_string(),
			nodereal_api_chain_network_url: "".to_string(),
			contest_legend_discord_role_id: "1172576273063739462".to_string(),
			contest_popularity_discord_role_id: "1172576681119195208".to_string(),
			contest_participant_discord_role_id: "1172576734135210104".to_string(),
			vip3_url: "".to_string(),
		};

		// we allow to override following config properties for non prod dev
		if_not_production!({
			if let Ok(v) = env::var("TWITTER_OFFICIAL_URL") {
				config.set_twitter_official_url(v);
			}
			if let Ok(v) = env::var("TWITTER_LITENTRY_URL") {
				config.set_twitter_litentry_url(v);
			}
			if let Ok(v) = env::var("DISCORD_OFFICIAL_URL") {
				config.set_discord_official_url(v);
			}
			if let Ok(v) = env::var("DISCORD_LITENTRY_URL") {
				config.set_discord_litentry_url(v);
			}
			if let Ok(v) = env::var("ACHAINABLE_URL") {
				config.set_achainable_url(v);
			}
			if let Ok(v) = env::var("CREDENTIAL_ENDPOINT") {
				config.set_credential_endpoint(v);
			}
			if let Ok(v) = env::var("ONEBLOCK_NOTION_URL") {
				config.set_oneblock_notion_url(v);
			}
			if let Ok(v) = env::var("SORA_QUIZ_MASTER_ID") {
				config.set_sora_quiz_master_id(v);
			}
			if let Ok(v) = env::var("SORA_QUIZ_ATTENDEE_ID") {
				config.set_sora_quiz_attendee_id(v);
			}
			if let Ok(v) = env::var("NODEREAL_API_URL") {
				config.set_nodereal_api_url(v);
			}
			if let Ok(v) = env::var("CONTEST_LEGEND_DISCORD_ROLE_ID") {
				config.set_contest_legend_discord_role_id(v);
			}
			if let Ok(v) = env::var("CONTEST_POPULARITY_DISCORD_ROLE_ID") {
				config.set_contest_popularity_discord_role_id(v);
			}
			if let Ok(v) = env::var("CONTEST_PARTICIPANT_DISCORD_ROLE_ID") {
				config.set_contest_participant_discord_role_id(v);
			}
		});
		// set secrets from env variables
		if let Ok(v) = env::var("TWITTER_AUTH_TOKEN_V2") {
			config.set_twitter_auth_token_v2(v);
		}
		if let Ok(v) = env::var("DISCORD_AUTH_TOKEN") {
			config.set_discord_auth_token(v);
		}
		if let Ok(v) = env::var("ACHAINABLE_AUTH_KEY") {
			config.set_achainable_auth_key(v);
		}
		if let Ok(v) = env::var("ONEBLOCK_NOTION_KEY") {
			config.set_oneblock_notion_key(v);
		}
		if let Ok(v) = env::var("NODEREAL_API_KEY") {
			config.set_nodereal_api_key(v);
		}
		config
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
	pub fn set_sora_quiz_master_id(&mut self, v: String) {
		debug!("set_sora_quiz_master_id: {:?}", v);
		self.sora_quiz_master_id = v;
	}
	pub fn set_sora_quiz_attendee_id(&mut self, v: String) {
		debug!("set_sora_quiz_attendee_id: {:?}", v);
		self.sora_quiz_attendee_id = v;
	}
	pub fn set_nodereal_api_key(&mut self, v: String) {
		debug!("set_nodereal_api_key: {:?}", v);
		self.nodereal_api_key = v;
	}
	pub fn set_nodereal_api_retry_delay(&mut self, v: u64) {
		debug!("set_nodereal_api_retry_delay: {:?}", v);
		self.nodereal_api_retry_delay = v;
	}
	pub fn set_nodereal_api_retry_times(&mut self, v: u16) {
		debug!("set_nodereal_api_retry_times: {:?}", v);
		self.nodereal_api_retry_times = v;
	}
	pub fn set_nodereal_api_url(&mut self, v: String) {
		debug!("set_nodereal_api_url: {:?}", v);
		self.nodereal_api_url = v;
	}
	pub fn set_nodereal_api_chain_network_url(&mut self, v: String) {
		debug!("set_nodereal_api_chain_network_url: {:?}", v);
		self.nodereal_api_chain_network_url = v;
	}
	pub fn set_contest_legend_discord_role_id(&mut self, v: String) {
		debug!("set_contest_legend_discord_role_id: {:?}", v);
		self.contest_legend_discord_role_id = v;
	}
	pub fn set_contest_popularity_discord_role_id(&mut self, v: String) {
		debug!("set_contest_popularity_discord_role_id: {:?}", v);
		self.contest_popularity_discord_role_id = v;
	}
	pub fn set_contest_participant_discord_role_id(&mut self, v: String) {
		debug!("set_contest_participant_discord_role_id: {:?}", v);
		self.contest_participant_discord_role_id = v;
	}
	pub fn set_vip3_url(&mut self, v: String) {
		debug!("set_vip3_url: {:?}", v);
		self.vip3_url = v;
	}
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("UTF8 error: {0}")]
	Utf8Error(String),

	#[error("Achainable error: {0}")]
	AchainableError(String),

	#[error("Nodereal error: {0}")]
	NoderealError(String),
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

pub fn build_client_with_cert(
	base_url: &str,
	headers: Headers,
) -> RestClient<HttpClient<SendWithCertificateVerification>> {
	debug!("base_url: {}", base_url);
	let base_url = Url::parse(base_url).unwrap();
	let http_client = HttpClient::new(
		SendWithCertificateVerification::new(vec![]),
		true,
		Some(TIMEOUT),
		Some(headers),
		None,
	);
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
