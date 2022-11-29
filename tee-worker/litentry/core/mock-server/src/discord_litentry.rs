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

use httpmock::{Method::GET, MockServer};
use lc_data_providers::discord_litentry::DiscordResponse;

use crate::Mock;

pub trait DiscordLitentryAPI {
	fn check_join(mock_server: &MockServer);
	fn check_id_hubber(mock_server: &MockServer);
}

pub struct DiscordLitentry {}
impl DiscordLitentry {
	pub fn new() -> Self {
		DiscordLitentry {}
	}
}

impl Default for DiscordLitentry {
	fn default() -> Self {
		Self::new()
	}
}

impl DiscordLitentryAPI for DiscordLitentry {
	fn check_join(mock_server: &MockServer) {
		let body = DiscordResponse {
			data: true,
			message: "success".into(),
			has_errors: false,
			msg_code: 200,
			success: true,
		};

		let path = "/discord/joined";
		mock_server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("guildid", "919848390156767232")
				.query_param("handler", "againstwar#4779");
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});
	}

	fn check_id_hubber(mock_server: &MockServer) {
		let body = DiscordResponse {
			data: true,
			message: "success".into(),
			has_errors: false,
			msg_code: 200,
			success: true,
		};

		mock_server.mock(|when, then| {
			when.method(GET)
				.path("/discord/commented/idhubber")
				.query_param("guildid", "919848390156767232")
				.query_param("handler", "ericzhang.eth#0114");

			then.status(200).body(serde_json::to_string(&body).unwrap());
		});
	}
}

impl Mock for DiscordLitentry {
	fn mock(&self, mock_server: &httpmock::MockServer) {
		DiscordLitentry::check_join(mock_server);
		DiscordLitentry::check_id_hubber(mock_server);
	}
}
