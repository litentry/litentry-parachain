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

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use core::time::Duration;
#[cfg(feature = "std")]
use threadpool::ThreadPool;

#[cfg(feature = "sgx")]
use threadpool_sgx::ThreadPool;

#[cfg(feature = "std")]
use std::sync::Mutex;

#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

use bc_enclave_registry::EnclaveRegistryLookup;
use bc_musig2_ceremony::{CeremonyEvent, CeremonyId, CeremonyRegistry, SignerId};
use codec::Encode;
use itc_direct_rpc_client::{DirectRpcClient, DirectRpcClientFactory, RpcClient, RpcClientFactory};
use itc_direct_rpc_server::SendRpcResponse;
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_rpc::{Id, RpcRequest};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
pub use itp_types::{DirectRequestStatus, Hash};
use itp_utils::hex::ToHexPrefixed;
use lc_direct_call::CeremonyRoundCall;
use litentry_primitives::{Address32, Identity, PlainRequest, ShardIdentifier};
use log::*;
use sp_core::{blake2_256, ed25519, Pair as SpCorePair, H256};
use std::{collections::HashMap, string::ToString, sync::Arc, thread::sleep, vec};

#[allow(clippy::too_many_arguments)]
pub fn process_event<OCallApi, SIGNINGAK, Responder, ECL, BKR>(
	signing_key_access: Arc<SIGNINGAK>,
	ocall_api: Arc<OCallApi>,
	responder: Arc<Responder>,
	enclave_registry_lookup: Arc<ECL>,
	event: CeremonyEvent,
	ceremony_id: CeremonyId,
	event_threads_pool: ThreadPool,
	peers_map: Arc<Mutex<HashMap<[u8; 32], DirectRpcClient>>>,
	ceremony_registry: Arc<RwLock<CeremonyRegistry<BKR>>>,
) where
	OCallApi: EnclaveAttestationOCallApi + 'static,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	Responder: SendRpcResponse<Hash = H256> + 'static,
	ECL: EnclaveRegistryLookup + Send + Sync + 'static,
	BKR: AccessKey<KeyType = SchnorrPair> + Send + Sync + 'static,
{
	let my_identity: Address32 = signing_key_access.retrieve_key().unwrap().public().0.into();
	let identity = Identity::Substrate(my_identity);
	let mr_enclave = ocall_api.get_mrenclave_of_self().unwrap().m;

	match event {
		CeremonyEvent::FirstRoundStarted(signers, message, nonce) => {
			let direct_call = CeremonyRoundCall::NonceShare(identity, message, nonce.serialize());
			let request = prepare_request(signing_key_access.as_ref(), mr_enclave, direct_call);

			signers.iter().for_each(|signer_id| {
				debug!(
					"Sharing nonce with signer: {:?} for ceremony: {:?}",
					signer_id, ceremony_id
				);

				let signer_id = *signer_id;
				let peers_map_clone = peers_map.clone();
				let request = request.clone();
				let enclave_lookup_cloned = enclave_registry_lookup.clone();
				let ceremony_registry_cloned = ceremony_registry.clone();
				let ceremony_id_cloned = ceremony_id.clone();
				event_threads_pool.execute(move || {
					send_request(
						signer_id,
						&ceremony_id_cloned,
						request,
						peers_map_clone,
						enclave_lookup_cloned,
						ceremony_registry_cloned,
					);
				});
			});
		},
		CeremonyEvent::SecondRoundStarted(signers, message, signature) => {
			let direct_call =
				CeremonyRoundCall::PartialSignatureShare(identity, message, signature.serialize());
			let request = prepare_request(signing_key_access.as_ref(), mr_enclave, direct_call);

			signers.iter().for_each(|signer_id| {
				debug!(
					"Sharing partial signature with signer: {:?} for ceremony: {:?}",
					signer_id, ceremony_id
				);

				let signer_id = *signer_id;
				let peers_map_clone = peers_map.clone();
				let request = request.clone();
				let enclave_lookup_cloned = enclave_registry_lookup.clone();
				let ceremony_registry_cloned = ceremony_registry.clone();
				let ceremony_id_cloned = ceremony_id.clone();
				event_threads_pool.execute(move || {
					send_request(
						signer_id,
						&ceremony_id_cloned,
						request,
						peers_map_clone,
						enclave_lookup_cloned,
						ceremony_registry_cloned,
					);
				});
			});
		},
		CeremonyEvent::CeremonyEnded(signature, is_check_run, verification_result) => {
			debug!("Ceremony {:?} ended, signature {:?}", ceremony_id, signature);
			let hash = blake2_256(&ceremony_id.encode());
			let result = if is_check_run {
				verification_result.encode()
			} else {
				let result = signature;
				result.encode()
			};
			event_threads_pool.execute(move || {
				if let Err(e) = responder.send_state_with_status(
					Hash::from_slice(&hash),
					result,
					DirectRequestStatus::Ok,
				) {
					error!("Could not send response to {:?}, reason: {:?}", &hash, e);
				}
			});
		},
		CeremonyEvent::CeremonyError(signers, error) => {
			debug!("Ceremony {:?} error {:?}", ceremony_id, error);
			let hash = blake2_256(&ceremony_id.encode());
			let encoded_result = error.encode();
			event_threads_pool.execute(move || {
				if let Err(e) = responder.send_state_with_status(
					Hash::from_slice(&hash),
					encoded_result,
					DirectRequestStatus::Error,
				) {
					error!("Could not send response to {:?}, reason: {:?}", &hash, e);
				}
			});

			let direct_call = CeremonyRoundCall::KillCeremony(identity, ceremony_id.clone());
			let request = prepare_request(signing_key_access.as_ref(), mr_enclave, direct_call);

			//kill ceremonies on other workers
			signers.iter().for_each(|signer_id| {
				debug!(
					"Requesting ceremony kill on signer: {:?} for ceremony: {:?}",
					signer_id, ceremony_id
				);

				let signer_id = *signer_id;
				let peers_map_clone = peers_map.clone();
				let request = request.clone();
				let enclave_lookup_cloned = enclave_registry_lookup.clone();
				let ceremony_registry_cloned = ceremony_registry.clone();
				let ceremony_id_cloned = ceremony_id.clone();
				event_threads_pool.execute(move || {
					send_request(
						signer_id,
						&ceremony_id_cloned,
						request,
						peers_map_clone,
						enclave_lookup_cloned,
						ceremony_registry_cloned,
					);
				});
			});
		},
	}
}

// it will try to send request until it succeeds, the peer is removed from registry or ceremony is removed
fn send_request<ECL, BKR>(
	signer_id: SignerId,
	ceremony_id: &CeremonyId,
	request: RpcRequest,
	peers_map: Arc<Mutex<HashMap<[u8; 32], DirectRpcClient>>>,
	enclave_registry_lookup: Arc<ECL>,
	ceremony_registry: Arc<RwLock<CeremonyRegistry<BKR>>>,
) where
	ECL: EnclaveRegistryLookup,
	BKR: AccessKey<KeyType = SchnorrPair>,
{
	loop {
		let client = peers_map.lock().unwrap().get(&signer_id).cloned();
		if let Some(mut client) = client {
			if let Err(e) = client.send(&request) {
				error!("Could not send request to signer: {:?}, reason: {:?}", signer_id, e);
				sleep(Duration::from_secs(1));
				let mut peers_lock = peers_map.lock().unwrap();
				peers_lock.remove(&signer_id);
			} else {
				// finish if request was sent
				break
			}
		} else {
			// check if ceremony still exists, if not stop
			if !ceremony_registry.read().unwrap().contains_key(ceremony_id) {
				break
			}

			if let Some(url) = enclave_registry_lookup.get_worker_url(&Address32::from(signer_id)) {
				match (DirectRpcClientFactory {}).create(&url) {
					Ok(new_client) => {
						peers_map.lock().unwrap().insert(signer_id, new_client.clone());
					},
					Err(e) => {
						error!("Could not connect to peer {}, reason: {:?}", url, e);
						sleep(Duration::from_secs(1));
					},
				}
			} else {
				error!("Could not find {:?} in registry", signer_id.to_hex());
				// stop if peer is not found in registry
				break
			}
		}
	}
}

fn prepare_request<SIGNINGAK>(
	signing_key_access: &SIGNINGAK,
	mr_enclave: [u8; 32],
	ceremony_round_call: CeremonyRoundCall,
) -> RpcRequest
where
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
{
	let shard = ShardIdentifier::from_slice(&mr_enclave);
	// same as above
	let dc_signed_encoded = ceremony_round_call
		.sign(&signing_key_access.retrieve_key().unwrap().into(), &mr_enclave, &shard)
		.encode();
	let request = PlainRequest { shard, payload: dc_signed_encoded };
	RpcRequest {
		jsonrpc: "2.0".to_string(),
		method: "bitacross_btcDataShare".to_string(),
		params: vec![request.to_hex()],
		id: Id::Number(1),
	}
}
