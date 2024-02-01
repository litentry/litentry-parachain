#![feature(trait_alias)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use codec::{Decode, Encode};
use futures::channel::oneshot;
use lazy_static::lazy_static;
use log::*;
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	format,
	string::String,
	sync::{
		mpsc::{channel, Receiver, Sender as MpscSender},
		Arc,
	},
	vec::Vec,
};

// TODO: Request should be changed to AesRequest
#[derive(Debug)]
pub struct BitAcrossRequest {
	pub sender: oneshot::Sender<Result<Vec<u8>, String>>,
	pub request: Vec<u8>,
}

#[derive(Encode, Decode, Clone)]
pub struct BitAcrossResponse {
	pub payload: Vec<u8>,
}

pub type BitAcrossSender = MpscSender<BitAcrossRequest>;

// Global storage of the sender. Should not be accessed directly.
lazy_static! {
	static ref GLOBAL_BIT_ACROSS_TASK_SENDER: Arc<Mutex<Option<BitAcrossTaskSender>>> =
		Arc::new(Mutex::new(Default::default()));
}

pub struct BitAcrossRequestSender {}
impl BitAcrossRequestSender {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for BitAcrossRequestSender {
	fn default() -> Self {
		Self::new()
	}
}

impl BitAcrossRequestSender {
	pub fn send(&self, request: BitAcrossRequest) -> Result<(), String> {
		debug!("send vc request: {:?}", request);

		// Acquire lock on extrinsic sender
		let mutex_guard = GLOBAL_BIT_ACROSS_TASK_SENDER.lock().unwrap();
		let bit_across_task_sender = mutex_guard.clone().unwrap();
		// Release mutex lock, so we don't block the lock longer than necessary.
		drop(mutex_guard);

		// Send the request to the receiver loop.
		bit_across_task_sender.send(request)?;

		Ok(())
	}
}

/// Initialization of the extrinsic sender. Needs to be called before any sender access.
pub fn init_bit_across_task_sender_storage() -> Receiver<BitAcrossRequest> {
	let (sender, receiver) = channel();
	// It makes no sense to handle the unwrap, as this statement fails only if the lock has been poisoned
	// I believe at that point it is an unrecoverable error
	let mut bit_across_task_storage = GLOBAL_BIT_ACROSS_TASK_SENDER.lock().unwrap();
	*bit_across_task_storage = Some(BitAcrossTaskSender::new(sender));
	receiver
}

/// Wrapping struct around the actual sender. Should not be accessed directly. (unnecessary)
#[derive(Clone, Debug)]
pub struct BitAcrossTaskSender {
	sender: BitAcrossSender,
}

impl BitAcrossTaskSender {
	pub fn new(sender: BitAcrossSender) -> Self {
		Self { sender }
	}

	fn send(&self, request: BitAcrossRequest) -> Result<(), String> {
		self.sender
			.send(request)
			.map_err(|e| format!("Failed to send message to VC Handler: {:?}", e))
	}
}
