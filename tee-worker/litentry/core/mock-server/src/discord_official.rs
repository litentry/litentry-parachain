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

use lc_data_providers::discord_official::{DiscordMessage, DiscordMessageAuthor};
use warp::{http::Response, Filter};

pub(crate) fn query_message(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("api" / "channels" / String / "messages" / String))
		.map(|channel_id, message_id| {
			let expected_channel_id = "919848392035794945".to_string();
			let expected_message_id = "1".to_string();

			if expected_channel_id == channel_id && expected_message_id == message_id {
				let body = DiscordMessage {
					id: message_id,
					channel_id,
					content: "Hello, litentry.".into(),
					author: DiscordMessageAuthor {
						id: "001".to_string(),
						username: "elon".to_string(),
					},
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}
