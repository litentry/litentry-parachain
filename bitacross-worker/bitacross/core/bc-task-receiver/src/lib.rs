#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use hex_sgx as hex;
	pub use threadpool_sgx as threadpool;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use bc_task_sender::init_bit_across_task_sender_storage;
use litentry_primitives::AesRequest;
use log::*;
use std::{
	string::{String, ToString},
	vec::Vec,
};
use threadpool::ThreadPool;

pub fn run_bit_across_handler_runner() {
	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let n_workers = 2;
	let pool = ThreadPool::new(n_workers);

	while let Ok(mut req) = bit_across_task_receiver.recv() {
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(&mut req.request)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

pub fn handle_request(_request: &mut AesRequest) -> Result<Vec<u8>, String>
where
{
	Err("Not Implemented".to_string())
}
