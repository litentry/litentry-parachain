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

use litentry_primitives::{UserShieldingKeyNonceType, UserShieldingKeyType};
use sp_core::sr25519::Pair as Sr25519Pair;
use std::{sync::Arc, thread};
use tokio::{
	sync::oneshot::{channel, error::RecvError},
	task::LocalSet,
};
use warp::Filter;

pub mod achainable;
pub mod discord_litentry;
pub mod discord_official;
pub mod twitter_litentry;
pub mod twitter_official;

// the nonce that is used to generate the verification message for the mock server
// it should have come from the client/user side, but here a mock number is used
pub const MOCK_VERIFICATION_NONCE: UserShieldingKeyNonceType = [1u8; 12];

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

pub fn default_getter(_who: &Sr25519Pair) -> UserShieldingKeyType {
	UserShieldingKeyType::default()
}

pub fn run<F>(getter: Arc<F>, port: u16) -> Result<String, RecvError>
where
	F: Fn(&Sr25519Pair) -> UserShieldingKeyType + Send + Sync + 'static,
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
					.or(achainable::query())
					.or(achainable::fresh_account())
					.or(achainable::og_account())
					.or(achainable::class_of_2020())
					.or(achainable::class_of_2021())
					.or(achainable::class_of_2022())
					.or(achainable::found_on_bsc())
					.or(achainable::is_polkadot_validator())
					.or(achainable::is_kusama_validator())
					.or(achainable::polkadot_dolphin())
					.or(achainable::kusama_dolphin())
					.or(achainable::polkadot_whale())
					.or(achainable::kusama_whale())
					.or(achainable::less_than_10_eth_holder())
					.or(achainable::less_than_10_lit_holder())
					.or(achainable::not_less_than_100_eth_holder())
					.or(achainable::between_10_to_100_eth_holder())
					.or(achainable::eth_millionaire())
					.or(achainable::eth2_validator_eligible())
					.or(achainable::not_less_than_100_weth_holder())
					.or(achainable::not_less_than_100_lit_bep20_holder())
					.or(achainable::native_lit_holder())
					.or(achainable::erc20_lit_holder())
					.or(achainable::bep20_lit_holder()),
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
