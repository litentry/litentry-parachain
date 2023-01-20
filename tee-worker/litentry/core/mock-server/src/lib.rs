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

use codec::Encode;
use litentry_primitives::{ChallengeCode, Identity};
use sp_core::{blake2_256, crypto::AccountId32 as AccountId};
use warp::Filter;

// pub mod discord_litentry;
// pub mod discord_official;
// pub mod twitter_litentry;
pub mod twitter_official;
//
// pub use discord_litentry::*;
// pub use discord_official::*;
// pub use twitter_litentry::*;
// pub use twitter_official::*;

pub fn mock_tweet_payload(who: &AccountId, identity: &Identity, code: &ChallengeCode) -> String {
	let mut payload = code.encode();
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	hex::encode(blake2_256(payload.as_slice()))
}

pub async fn run() {
	warp::serve(
		twitter_official::query_tweet()
			.or(twitter_official::query_retweet())
			.or(twitter_official::query_user()),
	)
	.run(([0, 0, 0, 0], 9527))
	.await;

	// let mut mock_server_manager = MockServerManager::new();
	//
	// let discord_litentry = Box::new(DiscordLitentry::new());
	// mock_server_manager.register(discord_litentry);
	//
	// let discord_official = Box::new(DiscordOfficial::new());
	// mock_server_manager.register(discord_official);
	//
	// let twitter_litentry = Box::new(TwitterLitentry::new());
	// mock_server_manager.register(twitter_litentry);
	//
	// let twitter_official = Box::new(TwitterOfficial::new());
	// mock_server_manager.register(twitter_official);
	//
	// mock_server_manager.run();
}
