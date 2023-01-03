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
use serde::{Deserialize, Serialize};
use std::{
	default::Default,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterAPIV2Response<T> {
	pub data: Option<T>,
	pub meta: Option<ResponseMeta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMeta {
	pub result_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
	pub author_id: String,
	pub id: String,
	pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterUser {
	pub id: String,
	pub name: String,
	pub username: String,
	pub public_metrics: TwitterUserPublicMetrics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterUserPublicMetrics {
	pub followers_count: u32,
	pub following_count: u32,
}

impl RestPath<String> for Tweet {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

impl<T> RestPath<String> for TwitterAPIV2Response<T> {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

impl UserInfo for Tweet {
	fn get_user_id(&self) -> Option<String> {
		Some(self.author_id.clone())
	}
}

pub struct TwitterOfficialClient {
	client: RestClient<HttpClient<DefaultSend>>,
}

impl Default for TwitterOfficialClient {
	fn default() -> Self {
		Self::new()
	}
}

/// rate limit: https://developer.twitter.com/en/docs/twitter-api/rate-limits
impl TwitterOfficialClient {
	pub fn new() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().twitter_auth_token.clone().as_str(),
		);
		let client = build_client(
			G_DATA_PROVIDERS.read().unwrap().twitter_official_url.clone().as_str(),
			headers,
		);
		TwitterOfficialClient { client }
	}

	/// rate limit: 300/15min(per App) 900/15min(per User)
	pub fn query_tweet(&mut self, tweet_id: Vec<u8>) -> Result<Tweet, Error> {
		let tweet_id = vec_to_string(tweet_id)?;
		let path = format!("/2/tweets/{}", tweet_id);
		let query: Vec<(&str, &str)> =
			vec![("ids", tweet_id.as_str()), ("expansions", "author_id")];
		self.client
			.get_with::<String, Tweet>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))
	}

	/// rate limit: 450/15min(per App) 180/15min(per User)
	///
	/// Building queries for Search Tweets: https://developer.twitter.com/en/docs/twitter-api/tweets/search/integrate/build-a-query
	pub fn query_retweet(
		&mut self,
		user: Vec<u8>,
		original_tweet_id: Vec<u8>,
	) -> Result<Tweet, Error> {
		let original_tweet_id = vec_to_string(original_tweet_id)?;
		let user = vec_to_string(user)?;
		let query_value = format!("from: {} retweets_of_tweet_id: {}", user, original_tweet_id);
		let query: Vec<(&str, &str)> =
			vec![("query", query_value.as_str()), ("expansions", "author_id")];
		let resp = self
			.client
			.get_with::<String, TwitterAPIV2Response<Vec<Tweet>>>(
				"/2/tweets/search/recent".to_string(),
				query.as_slice(),
			)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;
		let tweets = resp.data.ok_or_else(|| Error::RequestError("tweet not found".to_string()))?;
		if !tweets.is_empty() {
			Ok(tweets.get(0).unwrap().clone())
		} else {
			Err(Error::RequestError("tweet not found".to_string()))
		}
	}

	/// rate limit: 300/15min(per App) 900/15min(per User)
	pub fn query_user(&mut self, user: Vec<u8>) -> Result<TwitterUser, Error> {
		let user = vec_to_string(user)?;
		let query = vec![("user.fields", "public_metrics")];
		let resp = self
			.client
			.get_with::<String, TwitterAPIV2Response<TwitterUser>>(
				format!("/2/users/{}", user),
				query.as_slice(),
			)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;
		let user = resp.data.ok_or_else(|| Error::RequestError("user not found".to_string()))?;
		Ok(user)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use httpmock::prelude::*;
	use lc_mock_server::{mock_tweet_payload, standalone_server};
	use litentry_primitives::{
		ChallengeCode, Identity, IdentityHandle, IdentityString, IdentityWebType, Web2Network,
	};
	use sp_core::crypto::AccountId32 as AccountId;

	#[test]
	fn query_tweet_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let tweet_id = "100";

		let account_id = AccountId::new([0u8; 32]);
		let twitter_identity = Identity {
			web_type: IdentityWebType::Web2(Web2Network::Twitter),
			handle: IdentityHandle::String(
				IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
			),
		};
		let chanllenge_code: ChallengeCode =
			[8, 104, 90, 56, 35, 213, 18, 250, 213, 210, 119, 241, 2, 174, 24, 8];
		let payload = mock_tweet_payload(&account_id, &twitter_identity, &chanllenge_code);

		let tweet = Tweet {
			author_id: "ericzhangeth".into(),
			id: tweet_id.into(),
			text: serde_json::to_string(&payload).unwrap(),
		};

		let path = format! {"/2/tweets/{}", tweet_id};
		server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("ids", tweet_id)
				.query_param("expansions", "author_id");
			then.status(200).body(serde_json::to_string(&tweet).unwrap());
		});

		let mut client = TwitterOfficialClient::new();
		let result = client.query_tweet(tweet_id.as_bytes().to_vec());
		assert!(result.is_ok(), "error: {:?}", result);
	}

	#[test]
	fn query_retweet_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let author_id = "ericzhangeth";
		let id = "100";

		let account_id = AccountId::new([0u8; 32]);
		let twitter_identity = Identity {
			web_type: IdentityWebType::Web2(Web2Network::Twitter),
			handle: IdentityHandle::String(
				IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
			),
		};
		let chanllenge_code: ChallengeCode =
			[8, 104, 90, 56, 35, 213, 18, 250, 213, 210, 119, 241, 2, 174, 24, 8];
		let payload = mock_tweet_payload(&account_id, &twitter_identity, &chanllenge_code);

		let tweets = vec![Tweet {
			author_id: author_id.into(),
			id: id.into(),
			text: serde_json::to_string(&payload).unwrap(),
		}];
		let body = TwitterAPIV2Response { data: Some(tweets), meta: None };

		let path = "/2/tweets/search/recent";

		let user = "ericzhangeth";
		let original_tweet_id = "100";
		let query_value = format!("from: {} retweets_of_tweet_id: {}", user, original_tweet_id);

		server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("query", query_value)
				.query_param("expansions", "author_id");
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});

		let mut client = TwitterOfficialClient::new();

		let user = author_id.clone().as_bytes().to_vec();
		let original_tweet_id = id.as_bytes().to_vec();
		let response = client.query_retweet(user, original_tweet_id);

		assert!(response.is_ok(), "error: {:?}", response);
	}

	#[test]
	fn query_user_work() {
		standalone_server();
		let server = httpmock::MockServer::connect("localhost:9527");

		let user = "1256908613857226756";

		let twitter_user_data = TwitterUser {
			id: user.into(),
			name: "ericzhang".into(),
			username: "elon".into(),
			public_metrics: TwitterUserPublicMetrics {
				followers_count: 100_u32,
				following_count: 99_u32,
			},
		};

		let body = TwitterAPIV2Response { data: Some(twitter_user_data), meta: None };

		let path = format! {"/2/users/{}", user};

		server.mock(|when, then| {
			when.method(GET).path(path).query_param("user.fields", "public_metrics");
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});

		let mut client = TwitterOfficialClient::new();
		let result = client.query_user(user.as_bytes().to_vec());
		assert!(result.is_ok(), "error: {:?}", result);
	}
}
