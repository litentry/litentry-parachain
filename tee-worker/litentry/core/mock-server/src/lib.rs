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

// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use std::thread;
use tokio::{
	sync::oneshot::{channel, error::RecvError},
	task::LocalSet,
};
use warp::Filter;

pub mod achainable;
pub mod blockchain_info;
pub mod discord_litentry;
pub mod discord_official;
pub mod geniidata;
pub mod karat_dao;
pub mod litentry_archive;
pub mod magic_craft;
pub mod moralis;
pub mod nodereal;
pub mod nodereal_jsonrpc;
pub mod oneblock;
pub mod twitter_official;
pub mod vip3;
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

pub fn run(port: u16) -> Result<String, RecvError> {
	let (result_in, result_out) = channel();
	thread::spawn(move || {
		let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
		LocalSet::new().block_on(&runtime, async {
			let (addr, srv) = warp::serve(
				twitter_official::query_tweet()
					.or(twitter_official::query_retweeted_by())
					.or(twitter_official::query_user_by_name())
					.or(twitter_official::query_user_by_id())
					.or(twitter_official::request_user_access_token())
					.or(discord_official::query_message())
					.or(discord_official::get_user_info())
					.or(discord_official::request_user_access_token())
					.or(discord_litentry::check_id_hubber())
					.or(discord_litentry::check_join())
					.or(discord_litentry::has_role())
					.or(nodereal_jsonrpc::query())
					.or(karat_dao::query())
					.or(magic_craft::query())
					.or(moralis::query_nft())
					.or(moralis::query_erc20())
					.or(moralis::query_solana())
					.or(blockchain_info::query_rawaddr())
					.or(blockchain_info::query_multiaddr())
					.or(achainable::query())
					.or(achainable::query_labels())
					.or(litentry_archive::query_user_joined_evm_campaign())
					.or(vip3::query_user_sbt_level())
					.or(nodereal::query())
					.or(geniidata::query())
					.or(oneblock::query())
					.boxed(),
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
