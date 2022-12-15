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

#[macro_use]
extern crate lazy_static;

use std::{
	sync::Mutex,
	thread::{spawn, JoinHandle},
};

use codec::Encode;
use httpmock::{standalone::start_standalone_server, MockServer};
use litentry_primitives::{ChallengeCode, Identity};
use sp_core::{blake2_256, crypto::AccountId32 as AccountId};
use tokio::task::LocalSet;

pub mod discord_litentry;
pub mod discord_official;
pub mod twitter_litentry;
pub mod twitter_official;

pub use discord_litentry::*;
pub use discord_official::*;
pub use twitter_litentry::*;
pub use twitter_official::*;
pub fn standalone_server() {
	let _server = STANDALONE_SERVER.lock().unwrap_or_else(|e| e.into_inner());
}

lazy_static! {
	static ref STANDALONE_SERVER: Mutex<JoinHandle<Result<(), String>>> = Mutex::new(spawn(|| {
		let srv = start_standalone_server(9527, false, None, false, usize::MAX);
		let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
		LocalSet::new().block_on(&runtime, srv)
	}));
}

pub fn mock_tweet_payload(who: &AccountId, identity: &Identity, code: &ChallengeCode) -> String {
	let mut payload = code.encode();
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	hex::encode(blake2_256(payload.as_slice()))
}

pub trait Mock {
	fn mock(&self, mock_server: &MockServer);
}

struct MockServerManager {
	servers: Vec<Box<dyn Mock>>,
	mock_server: MockServer,
}
impl MockServerManager {
	pub fn new() -> Self {
		let mock_server = MockServer::connect("localhost:9527");
		MockServerManager { servers: vec![], mock_server }
	}

	pub fn register(&mut self, server: Box<dyn Mock>) {
		self.servers.push(server);
	}

	pub fn run(&self) {
		for server in &self.servers {
			server.mock(&self.mock_server);
		}
	}
}
pub fn run() {
	standalone_server();

	let mut mock_server_manager = MockServerManager::new();

	let discord_litentry = Box::new(DiscordLitentry::new());
	mock_server_manager.register(discord_litentry);

	let discord_official = Box::new(DiscordOfficial::new());
	mock_server_manager.register(discord_official);

	let twitter_litentry = Box::new(TwitterLitentry::new());
	mock_server_manager.register(twitter_litentry);

	let twitter_official = Box::new(TwitterOfficial::new());
	mock_server_manager.register(twitter_official);

	mock_server_manager.run();
}
