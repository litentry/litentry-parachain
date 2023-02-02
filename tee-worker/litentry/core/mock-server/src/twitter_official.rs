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
#![allow(opaque_hidden_inferred_bound)]

use crate::mock_tweet_payload;
use lc_data_providers::twitter_official::*;
use litentry_primitives::{ChallengeCode, Identity, IdentityString, Web2Network};
use sp_core::crypto::AccountId32 as AccountId;
use std::{collections::HashMap, sync::Arc};
use warp::{http::Response, Filter};

pub(crate) fn query_tweet<F>(
	func: Arc<F>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
	F: Fn(&Identity) -> ChallengeCode + Send + Sync + 'static,
{
	warp::get()
		.and(warp::path!("2" / "tweets" / u32))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |_tweet_id, p: HashMap<String, String>| {
			log::info!("query_tweet");
			let default = String::default();
			let ids = p.get("ids").unwrap_or(&default);
			let expansions = p.get("expansions").unwrap_or(&default);
			let expected_tweet_id = "100".to_string();

			if expansions.as_str() != "author_id" || ids != &expected_tweet_id {
				Response::builder().status(400).body(String::from("Error query"))
			} else {
				let account_id = AccountId::new([
					212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130,
					44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
				]); // Alice
				let twitter_identity = Identity::Web2 {
					network: Web2Network::Twitter,
					address: IdentityString::try_from("mock_user".as_bytes().to_vec()).unwrap(),
				};
				let chanllenge_code = func(&twitter_identity);
				log::info!(
					"query_tweet, challenge_code:{:?}",
					sp_core::hexdisplay::HexDisplay::from(&chanllenge_code)
				);
				let payload = mock_tweet_payload(&account_id, &twitter_identity, &chanllenge_code);
				let tweet =
					Tweet { author_id: "mock_user".into(), id: expected_tweet_id, text: payload };

				Response::builder().body(serde_json::to_string(&tweet).unwrap())
			}
		})
}

pub(crate) fn query_retweet<F>(
	func: Arc<F>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
	F: Fn(&Identity) -> ChallengeCode + Send + Sync + 'static,
{
	warp::get()
		.and(warp::path!("2" / "tweets" / "search" / "recent"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let query = p.get("query").unwrap_or(&default);
			let expansions = p.get("expansions").unwrap_or(&default);

			let user = "ericzhangeth";
			let original_tweet_id = "100";
			let expected_query =
				format!("from: {} retweets_of_tweet_id: {}", user, original_tweet_id);
			if expansions.as_str() != "author_id" || query != &expected_query {
				Response::builder().status(400).body(String::from("Error query"))
			} else {
				let author_id = "ericzhangeth";
				let id = "100";

				let account_id = AccountId::new([0u8; 32]);
				let twitter_identity = Identity::Web2 {
					network: Web2Network::Twitter,
					address: IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
				};
				let chanllenge_code = func(&twitter_identity);
				let payload = mock_tweet_payload(&account_id, &twitter_identity, &chanllenge_code);

				let tweets = vec![Tweet {
					author_id: author_id.into(),
					id: id.into(),
					text: serde_json::to_string(&payload).unwrap(),
				}];
				let body = TwitterAPIV2Response { data: Some(tweets), meta: None };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}

pub(crate) fn query_user(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("2" / "users" / String))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |user_id, p: HashMap<String, String>| {
			let expected_user_id = "1256908613857226756".to_string();

			let default = String::default();
			let user_fields = p.get("user.fields").unwrap_or(&default);

			if user_fields.as_str() != "public_metrics" || user_id != expected_user_id {
				Response::builder().status(400).body(String::from("Error query"))
			} else {
				let twitter_user_data = TwitterUser {
					id: expected_user_id,
					name: "ericzhang".to_string(),
					username: "elon".to_string(),
					public_metrics: TwitterUserPublicMetrics {
						followers_count: 100_u32,
						following_count: 99_u32,
					},
				};
				let body = TwitterAPIV2Response { data: Some(twitter_user_data), meta: None };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}
