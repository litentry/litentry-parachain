// Copyright 2020-2023 Trust Computing GmbH.
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

// use crate::error::{Error, Result};
use ita_sgx_runtime::Hash;
use ita_stf::{aes_encrypt_default, IdentityManagement, OpaqueCall, VCMPCallIndexes, H256};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_teerex::TeerexCallIndexes, provider::AccessNodeMetadata, NodeMetadataTrait,
};
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lazy_static::lazy_static;
use lc_stf_task_receiver::StfTaskContext;
use litentry_primitives::{Assertion, Identity};
use log::*;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::sync::{
	mpsc::{channel, Receiver, Sender},
	Arc,
};

// Note: Should I use RequestType or decouple it completely?
use lc_stf_task_sender::RequestType;
#[cfg(feature = "std")]
use std::sync::Mutex;

pub type VcSender = Sender<RequestType>;

// Global storage of the sender. Should not be accessed directly.
lazy_static! {
	static ref GLOBAL_VC_REQUEST_TASK: Arc<Mutex<Option<VcTaskSender>>> =
		Arc::new(Mutex::new(Default::default()));
}

/// Trait to send an stf request to the stf request thread.
pub trait SendVcRequest {
	fn send_vc_request(&self, request: RequestType);
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
	fn send_vc_request(&self, request: RequestType) {
		debug!("send vc request: {:?}", request);

		// Acquire lock on extrinsic sender
		let mutex_guard = GLOBAL_VC_REQUEST_TASK.lock().unwrap();

		let vc_task_sender = mutex_guard.clone().unwrap();

		// Release mutex lock, so we don't block the lock longer than necessary.
		drop(mutex_guard);

		// Send the request to the receiver loop.
		vc_task_sender.send(request);
	}
}

/// Initialization of the extrinsic sender. Needs to be called before any sender access.
pub fn init_vc_task_sender_storage() -> Receiver<RequestType> {
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

	fn send(&self, request: RequestType) {
		self.sender.send(request).unwrap();
	}
}
