/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Service to determine if the integritee services is initialized and registered on the node,
//! hosted on a http server.

use crate::error::ServiceResult;
use itp_settings::worker_mode::{ProvideWorkerMode, WorkerMode};
use log::*;
use parking_lot::RwLock;
use std::{default::Default, marker::PhantomData, net::SocketAddr, sync::Arc};
use warp::Filter;

pub async fn start_is_initialized_server<Handler>(
	initialization_handler: Arc<Handler>,
	port: u16,
) -> ServiceResult<()>
where
	Handler: IsInitialized + Send + Sync + 'static,
{
	let is_initialized_route = warp::path!("is_initialized").and_then(move || {
		let handler_clone = initialization_handler.clone();
		async move {
			if handler_clone.is_initialized() {
				Ok("I am initialized.")
			} else {
				Err(warp::reject::not_found())
			}
		}
	});

	let socket_addr: SocketAddr = ([0, 0, 0, 0], port).into();

	info!("Running initialized server on: {:?}", socket_addr);
	warp::serve(is_initialized_route).run(socket_addr).await;

	info!("Initialized server shut down");
	Ok(())
}

/// Trait to query of a worker is considered fully initialized.
pub trait IsInitialized {
	fn is_initialized(&self) -> bool;
}

/// Tracker for initialization. Used by components that ensure these steps were taken.
pub trait TrackInitialization {
	fn registered_on_parentchain(&self);

	fn sidechain_block_produced(&self);

	fn worker_for_shard_registered(&self);
}

pub struct InitializationHandler {
	registered_on_parentchain: RwLock<bool>,
	sidechain_block_produced: RwLock<bool>,
	worker_for_shard_registered: RwLock<bool>,
}

// Cannot use #[derive(Default)], because the compiler complains that WorkerModeProvider then
// also needs to implement Default. Which does not make sense, since it's only used in PhantomData.
// Explicitly implementing Default solves the problem
// (see https://stackoverflow.com/questions/59538071/the-trait-bound-t-stddefaultdefault-is-not-satisfied-when-using-phantomda).
impl Default for InitializationHandler {
	fn default() -> Self {
		Self {
			registered_on_parentchain: Default::default(),
			sidechain_block_produced: Default::default(),
			worker_for_shard_registered: Default::default(),
		}
	}
}

impl TrackInitialization for InitializationHandler {
	fn registered_on_parentchain(&self) {
		let mut registered_lock = self.registered_on_parentchain.write();
		*registered_lock = true;
	}

	fn sidechain_block_produced(&self) {
		let mut block_produced_lock = self.sidechain_block_produced.write();
		*block_produced_lock = true;
	}

	fn worker_for_shard_registered(&self) {
		let mut registered_lock = self.worker_for_shard_registered.write();
		*registered_lock = true;
	}
}

impl IsInitialized for InitializationHandler
where
{
	fn is_initialized(&self) -> bool {
		*self.registered_on_parentchain.read()
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	struct OffchainWorkerMode;
	impl ProvideWorkerMode for OffchainWorkerMode {
		fn worker_mode() -> WorkerMode {
			WorkerMode::OffChainWorker
		}
	}

	struct SidechainWorkerMode;
	impl ProvideWorkerMode for SidechainWorkerMode {
		fn worker_mode() -> WorkerMode {
			WorkerMode::Sidechain
		}
	}

	#[test]
	fn default_handler_is_initialized_returns_false() {
		let offchain_worker_handler = InitializationHandler::default();
		let sidechain_handler = InitializationHandler::default();

		assert!(!offchain_worker_handler.is_initialized());
		assert!(!sidechain_handler.is_initialized());
	}

	#[test]
	fn parentchain_registration_is_enough_for_initialized() {
		let initialization_handler = InitializationHandler::default();
		initialization_handler.registered_on_parentchain();

		assert!(initialization_handler.is_initialized());
	}

}
