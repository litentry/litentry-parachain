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
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

use codec::{Decode, Encode};
use itp_types::{ShardIdentifier, H256};
use lazy_static::lazy_static;
use lc_stf_task_sender::AssertionBuildRequest;
use log::*;
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc,
	},
	vec::Vec,
};

#[cfg(feature = "std")]
use futures::channel::oneshot;

#[cfg(feature = "sgx")]
use futures_sgx::channel::oneshot;

#[cfg(feature = "sgx")]
pub use jsonrpc_core_sgx::Error as RpcError;

#[cfg(feature = "std")]
pub use jsonrpc_core::Error as RpcError;

#[derive(Debug)]
pub struct VCRequest {
	pub encrypted_trusted_call: Vec<u8>,
	pub sender: oneshot::Sender<Result<Vec<u8>, RpcError>>,
	pub shard: ShardIdentifier,
}

#[derive(Encode, Decode, Clone)]
pub struct VCResponse {
	pub assertion_request: AssertionBuildRequest,
	pub vc_hash: H256,
	pub vc_payload: Vec<u8>,
	pub vc_index: H256,
}

pub type VcSender = Sender<VCRequest>;

// Global storage of the sender. Should not be accessed directly.
lazy_static! {
	static ref GLOBAL_VC_REQUEST_TASK: Arc<Mutex<Option<VcTaskSender>>> =
		Arc::new(Mutex::new(Default::default()));
}

/// Trait to send an stf request to the stf request thread.
pub trait SendVcRequest {
	fn send_vc_request(&self, request: VCRequest);
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

impl SendVcRequest for VcRequestSender {
	fn send_vc_request(&self, request: VCRequest) {
		debug!("send vc request: {:?}", request);

		// Acquire lock on extrinsic sender
		// TODO: Can we optimise using RwLock
		let mutex_guard = GLOBAL_VC_REQUEST_TASK.lock().unwrap();

		let vc_task_sender = mutex_guard.clone().unwrap();

		// Release mutex lock, so we don't block the lock longer than necessary.
		drop(mutex_guard);

		// Send the request to the receiver loop.
		vc_task_sender.send(request);
	}
}

/// Initialization of the extrinsic sender. Needs to be called before any sender access.
pub fn init_vc_task_sender_storage() -> Receiver<VCRequest> {
	let (sender, receiver) = channel();
	let mut vc_task_storage = GLOBAL_VC_REQUEST_TASK.lock().unwrap();
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

	fn send(&self, request: VCRequest) {
		self.sender.send(request).unwrap();
	}
}
