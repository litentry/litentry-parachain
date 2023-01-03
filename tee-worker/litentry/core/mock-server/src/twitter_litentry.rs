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

use crate::Mock;

pub trait TwitterLitentryAPI {
	fn check_follow(mock_server: &MockServer);
}

pub struct TwitterLitentry {}
impl TwitterLitentry {
	pub fn new() -> Self {
		TwitterLitentry {}
	}
}

impl Default for TwitterLitentry {
	fn default() -> Self {
		Self::new()
	}
}

impl TwitterLitentryAPI for TwitterLitentry {
	fn check_follow(mock_server: &MockServer) {
		let body = r#"{ "data": false }"#;
		let path = "/twitter/followers/verification";
		mock_server.mock(|when, then| {
			when.method(GET)
				.path(path)
				.query_param("handler1", "litentry")
				.query_param("handler2", "ericzhangeth");
			then.status(200).body(body);
		});
	}
}

impl Mock for TwitterLitentry {
	fn mock(&self, mock_server: &httpmock::MockServer) {
		TwitterLitentry::check_follow(mock_server);
	}
}
