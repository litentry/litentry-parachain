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
use codec::Encode;

use litentry_primitives::{ChallengeCode, Identity};
use sp_core::{blake2_256, crypto::AccountId32 as AccountId};
use std::{sync::Arc, thread};
use tokio::{
	sync::oneshot::{channel, error::RecvError},
	task::LocalSet,
};
use warp::Filter;

pub mod discord_litentry;
pub mod discord_official;
pub mod graphql;
pub mod twitter_litentry;
pub mod twitter_official;

// It should only works on UNIX.
async fn shutdown_signal() {
	let mut hangup_stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
		.expect("Cannot install SIGINT signal handler");
	let mut sigint_stream =
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
			.expect("Cannot install SIGINT signal handler");
	let mut sigterm_stream =
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("Cannot install SIGINT signal handler");

	tokio::select! {
		_val = hangup_stream.recv() => log::warn!("Received SIGINT"),
		_val = sigint_stream.recv() => log::warn!("Received SIGINT"),
		_val = sigterm_stream.recv() => log::warn!("Received SIGTERM"),
	}
}

pub fn mock_tweet_payload(who: &AccountId, identity: &Identity, code: &ChallengeCode) -> String {
	let mut payload = code.encode();
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	hex::encode(blake2_256(payload.as_slice()))
}

pub fn run<F>(getter: Arc<F>, port: u16) -> Result<String, RecvError>
where
	F: Fn(&AccountId, &Identity) -> ChallengeCode + Send + Sync + 'static,
{
	let (result_in, result_out) = channel();
	thread::spawn(move || {
		let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
		LocalSet::new().block_on(&runtime, async {
			let (addr, srv) = warp::serve(
				twitter_official::query_tweet(getter.clone())
					.or(twitter_official::query_retweeted_by())
					.or(twitter_official::query_user())
					.or(twitter_official::query_friendship())
					.or(twitter_litentry::check_follow())
					.or(discord_official::query_message())
					.or(discord_litentry::check_id_hubber())
					.or(discord_litentry::check_join())
					.or(graphql::query()),
			)
			.bind_with_graceful_shutdown(([127, 0, 0, 1], port), shutdown_signal());
			log::info!("mock-server listen on addr:{:?}", addr);
			let _ = result_in.send(format!("http://{:?}", addr));
			let join = tokio::task::spawn_local(srv);
			let _ = join.await;
		});
	});
	result_out.blocking_recv()
}
