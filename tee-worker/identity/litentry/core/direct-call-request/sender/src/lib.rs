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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use lazy_static::lazy_static;
use litentry_primitives::AesRequest;

#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

use std::{
	format,
	string::String,
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc,
	},
};

#[derive(Debug)]
pub struct DirectCallRequest {
	pub request: AesRequest,
}

// Global storage of the sender. Should not be accessed directly.
lazy_static! {
	static ref GLOBAL_DIRECT_CALL_REQUEST_SENDER: Arc<Mutex<Option<Sender<DirectCallRequest>>>> =
		Arc::new(Mutex::new(Default::default()));
}

pub struct DirectCallRequestSender {}

impl DirectCallRequestSender {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for DirectCallRequestSender {
	fn default() -> Self {
		Self::new()
	}
}

impl DirectCallRequestSender {
	pub fn send(&self, request: DirectCallRequest) -> Result<(), String> {
		log::debug!("send direct call request: {:?}", request);
		let mutex_guard =
			GLOBAL_DIRECT_CALL_REQUEST_SENDER.lock().map_err(|_| "Mutex lock failed")?;
		let request_sender: Sender<DirectCallRequest> =
			mutex_guard.clone().ok_or("direct call task sender was not initialized")?;
		// Release mutex lock, so we don't block the lock longer than necessary.
		drop(mutex_guard);

		request_sender
			.send(request)
			.map_err(|e| format!("Unable to send request: {:?}", e))?;

		Ok(())
	}
}

pub fn init_direct_call_request_sender() -> Receiver<DirectCallRequest> {
	let (sender, receiver) = channel();
	let mut task_sender = GLOBAL_DIRECT_CALL_REQUEST_SENDER.lock().expect("Mutex lock failed");
	*task_sender = Some(sender);

	receiver
}
