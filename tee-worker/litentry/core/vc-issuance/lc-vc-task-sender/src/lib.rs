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
	pub use jsonrpc_core_sgx as jsonrpc_core;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

use codec::{Decode, Encode};
use futures::channel::oneshot;
use itp_types::{ShardIdentifier, H256};
pub use jsonrpc_core::{types::error::ErrorCode, Error as RpcError};
use lazy_static::lazy_static;
use lc_stf_task_sender::AssertionBuildRequest;
use log::*;
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	format,
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc,
	},
	vec::Vec,
};

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
	fn send_vc_request(&self, request: VCRequest) -> Result<(), RpcError>;
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
	fn send_vc_request(&self, request: VCRequest) -> Result<(), RpcError> {
		debug!("send vc request: {:?}", request);

		// Acquire lock on extrinsic sender
		// TODO: Can we optimise using RwLock
		let mutex_guard = GLOBAL_VC_REQUEST_TASK.lock().unwrap();

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

	fn send(&self, request: VCRequest) -> Result<(), RpcError> {
		if let Err(e) = self.sender.send(request) {
			let mut error = RpcError::new(ErrorCode::InternalError);
			error.message = format!("Failed to send message to VC Task receive: {:?}", e);
			return Err(error)
		}
		Ok(())
	}
}
