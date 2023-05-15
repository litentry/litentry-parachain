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

use crate::{build_client, vec_to_string, Error, HttpError, UserInfo, G_DATA_PROVIDERS};
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::RestClient,
	RestGet, RestPath,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterAPIV2Response<T> {
	pub data: Option<T>,
	pub meta: Option<ResponseMeta>,
	pub includes: Option<TwitterUsers>,
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
pub struct Retweeted {
	pub data: Vec<TwitterUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterUser {
	pub id: String,
	pub name: String,
	pub username: String,
	pub public_metrics: Option<TwitterUserPublicMetrics>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterUserPublicMetrics {
	pub followers_count: u32,
	pub following_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterUsers {
	pub users: Vec<TwitterUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
	pub source: SourceTwitterUser,
	pub target: TargetTwitterUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTwitterUser {
	pub id_str: String,
	pub screen_name: String,
	pub following: bool,
	pub followed_by: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetTwitterUser {
	pub id_str: String,
	pub screen_name: String,
	pub following: bool,
	pub followed_by: bool,
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

impl RestPath<String> for Relationship {
	fn get_path(path: String) -> core::result::Result<String, HttpError> {
		Ok(path)
	}
}

impl RestPath<String> for Retweeted {
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

/// rate limit: https://developer.twitter.com/en/docs/twitter-api/rate-limits
impl TwitterOfficialClient {
	pub fn v1_1() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().twitter_auth_token_v1_1.clone().as_str(),
		);
		let client = build_client(
			G_DATA_PROVIDERS.read().unwrap().twitter_official_url.clone().as_str(),
			headers.clone(),
		);

		TwitterOfficialClient { client }
	}

	pub fn v2() -> Self {
		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert(
			AUTHORIZATION.as_str(),
			G_DATA_PROVIDERS.read().unwrap().twitter_auth_token_v2.clone().as_str(),
		);
		let client = build_client(
			G_DATA_PROVIDERS.read().unwrap().twitter_official_url.clone().as_str(),
			headers.clone(),
		);

		TwitterOfficialClient { client }
	}

	/// V2, rate limit: 300/15min(per App) 900/15min(per User)
	pub fn query_tweet(&mut self, tweet_id: Vec<u8>) -> Result<Tweet, Error> {
		let tweet_id = vec_to_string(tweet_id)?;
		debug!("Twitter query tweet, id: {}", tweet_id);

		let path = format!("/2/tweets/{}", tweet_id);
		let query: Vec<(&str, &str)> = vec![("expansions", "author_id")];

		let resp = self
			.client
			.get_with::<String, TwitterAPIV2Response<Tweet>>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		let mut tweet = resp.data.ok_or(Error::RequestError("tweet not found".into()))?;

		// have to replace user_id with includes -> users -> username, otherwise the handle verification would fail
		if let Some(tweet_users) = resp.includes {
			if tweet_users.users.is_empty() {
				return Err(Error::RequestError("user not found from tweet".to_string()))
			}
			tweet.author_id = tweet_users.users[0].username.clone();
		}

		Ok(tweet)
	}

	/// V2, https://developer.twitter.com/en/docs/twitter-api/tweets/retweets/api-reference/get-tweets-id-retweeted_by
	/// rate limit: 75/15min(per App) 75/15min(per User)
	/// Note: The maximum result is 100 latest, when a user requests too late (after 100 retweets by others),
	/// the verification will fail.
	pub fn query_retweeted_by(&mut self, original_tweet_id: Vec<u8>) -> Result<Retweeted, Error> {
		let original_tweet_id = vec_to_string(original_tweet_id)?;
		debug!("Twitter original tweet id: {}", original_tweet_id);

		let path = format!("/2/tweets/{}/retweeted_by", original_tweet_id);
		let query: Vec<(&str, &str)> = vec![("max_results", "100")];

		let resp = self
			.client
			.get_with::<String, Retweeted>(path, query.as_slice())
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		Ok(resp)
	}

	/// V2, rate limit: 300/15min(per App) 900/15min(per User)
	pub fn query_user(&mut self, user_name: Vec<u8>) -> Result<TwitterUser, Error> {
		let user = vec_to_string(user_name)?;
		debug!("Twitter query user, user: {}", user);

		let query = vec![("user.fields", "public_metrics")];
		let resp = self
			.client
			.get_with::<String, TwitterAPIV2Response<TwitterUser>>(
				format!("/2/users/by/username/{}", user),
				query.as_slice(),
			)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		let user = resp.data.ok_or_else(|| Error::RequestError("user not found".to_string()))?;
		Ok(user)
	}

	/// V1.1, https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/follow-search-get-users/api-reference/get-friendships-show
	/// rate limit: 15/15min(per App) 180/15min(per User)
	pub fn query_friendship(
		&mut self,
		source_user_name: Vec<u8>,
		target_user_id: Vec<u8>,
	) -> Result<Relationship, Error> {
		let source = vec_to_string(source_user_name)?;
		let target_id = vec_to_string(target_user_id)?;
		debug!("Twitter query_friendship, source user: {}, target user: {}", source, target_id);

		let query: Vec<(&str, &str)> =
			vec![("source_screen_name", source.as_str()), ("target_id", target_id.as_str())];

		let resp = self
			.client
			.get_with::<String, Relationship>(
				"/1.1/friendships/show.json".to_string(),
				query.as_slice(),
			)
			.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

		Ok(resp)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::AccountId;
	use lc_mock_server::run;
	use litentry_primitives::{ChallengeCode, Identity};
	use std::sync::Arc;

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(|_: &AccountId, _: &Identity| ChallengeCode::default()), 0).unwrap();
		G_DATA_PROVIDERS.write().unwrap().set_twitter_official_url(url.clone());
	}

	#[test]
	fn query_tweet_work() {
		init();

		let mut client = TwitterOfficialClient::v2();
		let result = client.query_tweet("100".as_bytes().to_vec());
		assert!(result.is_ok(), "error: {:?}", result);
	}

	#[test]
	fn query_retweeted_work() {
		init();

		let mut client = TwitterOfficialClient::v2();
		let original_tweet_id = "100".as_bytes().to_vec();
		let response = client.query_retweeted_by(original_tweet_id);

		assert!(response.is_ok(), "error: {:?}", response);
	}

	#[test]
	fn query_user_by_name_work() {
		init();

		let user = "twitterdev";
		let mut client = TwitterOfficialClient::v2();
		let result = client.query_user(user.as_bytes().to_vec());
		assert!(result.is_ok(), "error: {:?}", result);
		// task.abort();
	}

	#[test]
	fn query_friendship_work() {
		init();

		let source = "twitterdev";
		let target_id = "783214"; //user: twitter
		let mut client = TwitterOfficialClient::v1_1();
		let result =
			client.query_friendship(source.as_bytes().to_vec(), target_id.as_bytes().to_vec());
		assert!(result.is_ok(), "error: {:?}", result);
	}
}
