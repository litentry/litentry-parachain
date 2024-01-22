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
use litentry_primitives::AesRequest;
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

#[derive(Debug)]
pub struct VCRequest {
	pub sender: oneshot::Sender<Result<Vec<u8>, String>>,
	pub request: AesRequest,
}

#[derive(Encode, Decode, Clone)]
pub struct VCResponse {
	pub vc_payload: Vec<u8>,
}

pub type VcSender = MpscSender<VCRequest>;

// Global storage of the sender. Should not be accessed directly.
lazy_static! {
	static ref GLOBAL_VC_TASK_SENDER: Arc<Mutex<Option<VcTaskSender>>> =
		Arc::new(Mutex::new(Default::default()));
}

pub struct VcRequestSender {}
impl VcRequestSender {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for VcRequestSender {
	fn default() -> Self {
		Self::new()
	}
}

impl VcRequestSender {
	pub fn send(&self, request: VCRequest) -> Result<(), String> {
		debug!("send vc request: {:?}", request);

		// Acquire lock on extrinsic sender
		let mutex_guard = GLOBAL_VC_TASK_SENDER.lock().unwrap();
		let vc_task_sender = mutex_guard.clone().unwrap();
		// Release mutex lock, so we don't block the lock longer than necessary.
		drop(mutex_guard);

		// Send the request to the receiver loop.
		vc_task_sender.send(request)?;

		Ok(())
	}
}

/// Initialization of the extrinsic sender. Needs to be called before any sender access.
pub fn init_vc_task_sender_storage() -> Receiver<VCRequest> {
	let (sender, receiver) = channel();
	// It makes no sense to handle the unwrap, as this statement fails only if the lock has been poisoned
	// I believe at that point it is an unrecoverable error
	let mut vc_task_storage = GLOBAL_VC_TASK_SENDER.lock().unwrap();
	*vc_task_storage = Some(VcTaskSender::new(sender));
	receiver
}

/// Wrapping struct around the actual sender. Should not be accessed directly. (unnecessary)
#[derive(Clone, Debug)]
struct VcTaskSender {
	sender: VcSender,
}

impl VcTaskSender {
	pub fn new(sender: VcSender) -> Self {
		Self { sender }
	}

	fn send(&self, request: VCRequest) -> Result<(), String> {
		self.sender
			.send(request)
			.map_err(|e| format!("Failed to send message to VC Handler: {:?}", e))
	}
}
