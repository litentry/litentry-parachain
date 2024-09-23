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
#![allow(opaque_hidden_inferred_bound)]

use ita_stf::helpers::get_expected_raw_message;
use lc_data_providers::discord_official::{
	DiscordMessage, DiscordMessageAuthor, DiscordUser, DiscordUserAccessToken,
};
use litentry_primitives::{Identity, IdentityString};
use sp_core::{sr25519::Pair as Sr25519Pair, Pair};
use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn query_message(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("api" / "channels" / String / "messages" / String))
		.map(|channel_id, message_id| {
			let expected_channel_id = "919848392035794945".to_string();
			let expected_message_id = "1".to_string();

			if expected_channel_id == channel_id && expected_message_id == message_id {
				let alice = Sr25519Pair::from_string("//Alice", None).unwrap();
				let discord_identity = Identity::Discord(IdentityString::new(b"alice".to_vec()));
				let payload = hex::encode(get_expected_raw_message(
					&alice.public().into(),
					&discord_identity,
					0,
				));
				let body = DiscordMessage {
					id: message_id,
					channel_id,
					content: payload,
					author: DiscordMessageAuthor {
						id: "002".to_string(),
						username: "alice".to_string(),
					},
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn get_user_info(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get().and(warp::path!("api" / "users" / String)).map(|user_id| {
		let current_user = "@me".to_string();
		if current_user == user_id || user_id == "001" {
			let body = DiscordUser {
				id: "001".to_string(),
				username: "bob".to_string(),
				discriminator: "0".to_string(),
			};
			Response::builder().body(serde_json::to_string(&body).unwrap())
		} else if user_id == "002" {
			let body = DiscordUser {
				id: user_id,
				username: "alice".to_string(),
				discriminator: "0".to_string(),
			};
			Response::builder().body(serde_json::to_string(&body).unwrap())
		} else {
			Response::builder().status(400).body(String::from("Error query"))
		}
	})
}

pub(crate) fn request_user_access_token(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!("api" / "oauth2" / "token"))
		.and(warp::body::form())
		.map(|_: HashMap<String, String>| {
			let body = DiscordUserAccessToken {
				token_type: "bearer".to_string(),
				expires_in: 7200,
				access_token: "dGFxeU1MbWRlSVhxSUgxX3VUdUJrM1FTRUtaMmFPdFM0XzMzcVlFSi0xM1dyOjE3MTMzNDEwODQ5NTg6MToxOmF0OjE".to_string(),
				refresh_token: "dGFxeU1MbWRlSVhxSUgxX3VUdUJrM1FTRUtaMmFPdFM0XzMzcVlFSi0xM1dyOjE3MTMzNDEwODQ5NTg6MToxOmF0OjE".to_string(),
				scope: "identify".to_string(),
			};

			Response::builder()
				.header("Content-Type", "application/json")
				.body(serde_json::to_string(&body).unwrap())
		})
}
