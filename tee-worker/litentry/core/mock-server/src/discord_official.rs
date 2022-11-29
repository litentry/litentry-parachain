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
use lc_data_providers::discord_official::{DiscordMessage, DiscordMessageAuthor};

use crate::Mock;

pub trait DiscordOfficialAPI {
	fn query_message(mock_server: &MockServer);
}

pub struct DiscordOfficial {}
impl DiscordOfficial {
	pub fn new() -> Self {
		DiscordOfficial {}
	}
}

impl Default for DiscordOfficial {
	fn default() -> Self {
		Self::new()
	}
}

impl DiscordOfficialAPI for DiscordOfficial {
	fn query_message(mock_server: &MockServer) {
		let channel_id = "919848392035794945";
		let message_id = "1";

		let user_id = "001";
		let username = "elon";
		let author = DiscordMessageAuthor { id: user_id.into(), username: username.into() };

		let body = DiscordMessage {
			id: message_id.into(),
			channel_id: channel_id.into(),
			content: "Hello, litentry.".into(),
			author,
		};

		let path = format! {"/api/channels/{}/messages/{}", channel_id, message_id};
		mock_server.mock(|when, then| {
			when.method(GET).path(path);
			then.status(200).body(serde_json::to_string(&body).unwrap());
		});
	}
}

impl Mock for DiscordOfficial {
	fn mock(&self, mock_server: &httpmock::MockServer) {
		DiscordOfficial::query_message(mock_server);
	}
}
