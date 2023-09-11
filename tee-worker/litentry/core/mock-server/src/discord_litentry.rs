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
#![allow(opaque_hidden_inferred_bound)]

use lc_data_providers::discord_litentry::DiscordResponse;
use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn check_join(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("discord" / "joined"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let guild_id = p.get("guildid").unwrap_or(&default);
			let handler = p.get("handler").unwrap_or(&default);
			let expected_guild_id = "919848390156767232";
			let expected_handler = "againstwar";

			if expected_guild_id == guild_id.as_str() && expected_handler == handler.as_str() {
				let body = DiscordResponse {
					data: true,
					message: "success".into(),
					has_errors: false,
					msg_code: 200,
					success: true,
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}

pub(crate) fn check_id_hubber(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("discord" / "commented" / "idhubber"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let guild_id = p.get("guildid").unwrap_or(&default);
			let channel_id = p.get("channelid").unwrap_or(&default);
			let role_id = p.get("roleid").unwrap_or(&default);
			let handler = p.get("handler").unwrap_or(&default);
			let expected_guild_id = "919848390156767232";
			let expected_channel_id = "919848392035794945";
			let expected_role_id = "1034083718425493544";
			let expected_handler = "ericzhang.eth";

			if expected_guild_id == guild_id.as_str()
				&& expected_handler == handler.as_str()
				&& expected_channel_id == channel_id.as_str()
				&& expected_role_id == role_id.as_str()
			{
				let body = DiscordResponse {
					data: true,
					message: "success".into(),
					has_errors: false,
					msg_code: 200,
					success: true,
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}
