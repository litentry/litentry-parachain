/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

use crate::{
	enclave_account,
	ocall_bridge::bridge_api::{OCallBridgeError, OCallBridgeResult, WorkerOnChainBridge},
};
use codec::{Decode, Encode};
use itp_enclave_api::enclave_base::EnclaveBase;
use itp_node_api::{api_client::AccountApi, node_api_factory::CreateNodeApi};
use itp_types::{WorkerRequest, WorkerResponse};
use itp_utils::ToHexPrefixed;
use log::*;
use sp_core::storage::StorageKey;
use sp_runtime::OpaqueExtrinsic;
use std::{sync::Arc, thread, vec::Vec};
use substrate_api_client::XtStatus;

pub struct WorkerOnChainOCall<E, F> {
	enclave_api: Arc<E>,
	node_api_factory: Arc<F>,
}

impl<E, F> WorkerOnChainOCall<E, F> {
	pub fn new(enclave_api: Arc<E>, node_api_factory: Arc<F>) -> Self {
		WorkerOnChainOCall { enclave_api, node_api_factory }
	}
}

impl<E, F> WorkerOnChainBridge for WorkerOnChainOCall<E, F>
where
	E: EnclaveBase,
	F: CreateNodeApi,
{
	fn worker_request(&self, request: Vec<u8>) -> OCallBridgeResult<Vec<u8>> {
		let requests: Vec<WorkerRequest> = Decode::decode(&mut request.as_slice()).unwrap();
		if requests.is_empty() {
			debug!("requests is empty, returning empty vector");
			return Ok(Vec::<u8>::new().encode())
		}

		let api = self.node_api_factory.create_api()?;

		let resp: Vec<WorkerResponse<Vec<u8>>> = requests
			.into_iter()
			.map(|req| match req {
				WorkerRequest::ChainStorage(key, hash) => WorkerResponse::ChainStorage(
					key.clone(),
					api.get_opaque_storage_by_key_hash(StorageKey(key.clone()), hash).unwrap(),
					api.get_storage_proof_by_keys(vec![StorageKey(key)], hash).unwrap().map(
						|read_proof| read_proof.proof.into_iter().map(|bytes| bytes.0).collect(),
					),
				),
			})
			.collect();

		let encoded_response: Vec<u8> = resp.encode();

		Ok(encoded_response)
	}

	fn send_to_parentchain(&self, extrinsics_encoded: Vec<u8>) -> OCallBridgeResult<()> {
		// TODO: improve error handling, using a mut status is not good design?
		let mut status: OCallBridgeResult<()> = Ok(());

		let extrinsics: Vec<OpaqueExtrinsic> =
			match Decode::decode(&mut extrinsics_encoded.as_slice()) {
				Ok(calls) => calls,
				Err(_) => {
					status = Err(OCallBridgeError::SendExtrinsicsToParentchain(
						"Could not decode extrinsics".to_string(),
					));
					Default::default()
				},
			};

		if !extrinsics.is_empty() {
			let mut send_extrinsic_failed = false;
			debug!("Enclave wants to send {} extrinsics", extrinsics.len());
			let api = self.node_api_factory.create_api()?;
			for call in extrinsics.into_iter() {
				debug!("Send extrinsic, call length: {}", call.to_hex().len());
				if let Err(e) = api.send_extrinsic(call.to_hex(), XtStatus::Ready) {
					error!("Could not send extrsinic to node: {:?}", e);
					send_extrinsic_failed = true;
				}
			}

			// Try to reset nonce, see
			// - https://github.com/litentry/litentry-parachain/issues/1036
			// - https://github.com/integritee-network/worker/issues/970
			// It has to be done in a separate thread as nested ECALL/OCALL is disallowed
			//
			// This workaround is likely to cause duplicate nonce or "transaction outdated" error in the parentchain
			// tx pool, because the retrieved on-chain nonce doesn't count the pending tx, meanwhile the extrinsic factory
			// keeps composing new extrinsics. So the nonce used for composing the new extrinsics can collide with the nonce
			// in the already submitted tx. As a result, a few txs could be dropped during the parentchain tx pool processing.
			// Not to mention the thread dispatch delay and network delay (query on-chain nonce).
			//
			// However, we still consider it better than the current situation, where the nonce never gets rectified and
			// all following extrinsics will be blocked. Moreover, the txs sent to the parentchain are mostly
			// "notification extrinsics" and don't cause chain state change, therefore we deem it less harmful to drop them.
			// The worst case is some action is wrongly intepreted as "failed" (because F/E doesn't get the event in time)
			// while it actually succeeds. In that case, the user needs to re-do the extrinsic, which is suboptimal,
			// but still better than the chain stalling.
			//
			// To have a better synchronisation handling we probably need a sending queue in extrinsic factory that
			// can be paused on demand (or wait for the nonce synchronisation).
			//
			// Another small thing that can be improved is to use rpc.system.accountNextIndex instead of system.account.nonce
			// see https://polkadot.js.org/docs/api/cookbook/tx/#how-do-i-take-the-pending-tx-pool-into-account-in-my-nonce
			if send_extrinsic_failed {
				// drop &self lifetime
				let node_api_factory_cloned = self.node_api_factory.clone();
				let enclave_cloned = self.enclave_api.clone();
				thread::spawn(move || {
					let api = node_api_factory_cloned.create_api().unwrap();
					let enclave_account = enclave_account(enclave_cloned.as_ref());
					warn!("send_extrinsic failed, try to reset nonce ...");
					match api.get_nonce_of(&enclave_account) {
						Ok(nonce) => {
							warn!("query on-chain nonce OK, reset nonce to: {}", nonce);
							if let Err(e) = enclave_cloned.set_nonce(nonce) {
								warn!("failed to reset nonce due to: {:?}", e);
							}
						},
						Err(e) => warn!("query on-chain nonce failed: {:?}", e),
					}
				});
			}
		}

		status
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::tests::mocks::enclave_api_mock::EnclaveMock;
	use itp_node_api::{
		api_client::ParentchainApi,
		node_api_factory::{CreateNodeApi, Result as NodeApiResult},
	};
	use mockall::mock;

	#[test]
	fn given_empty_worker_request_when_submitting_then_return_empty_response() {
		mock! {
			NodeApiFactory {}
			impl CreateNodeApi for NodeApiFactory {
				fn create_api(&self) -> NodeApiResult<ParentchainApi>;
			}
		}

		let mock_enclave = Arc::new(EnclaveMock {});
		let mock_node_api_factory = Arc::new(MockNodeApiFactory::new());

		let on_chain_ocall = WorkerOnChainOCall::new(mock_enclave, mock_node_api_factory);

		let response = on_chain_ocall.worker_request(Vec::<u8>::new().encode()).unwrap();

		assert!(!response.is_empty()); // the encoded empty vector is not empty
		let decoded_response: Vec<u8> = Decode::decode(&mut response.as_slice()).unwrap();
		assert!(decoded_response.is_empty()); // decode the response, and we get an empty vector again
	}
}
