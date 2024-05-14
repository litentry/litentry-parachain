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

use bc_enclave_registry::EnclaveRegistryLookup;
use codec::Encode;
use core::time::Duration;
use itc_direct_rpc_client::{Response, RpcClient, RpcClientFactory};
use itc_direct_rpc_server::SendRpcResponse;
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_sgx_crypto::{
	key_repository::{AccessKey, AccessPubkey},
	ShieldingCryptoEncrypt,
};
use itp_utils::hex::ToHexPrefixed;
use log::{debug, error, info, trace, warn};
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::{blake2_256, ed25519, Pair as SpCorePair, H256};
use std::vec::Vec;

#[cfg(feature = "std")]
use std::sync::Mutex;

use bc_musig2_ceremony::{
	CeremonyCommandsRegistry, CeremonyEvent, CeremonyId, CeremonyRegistry, SignBitcoinError,
	SignerId,
};

use itp_rpc::{Id, RpcRequest};
use itp_sgx_crypto::schnorr::Pair as SchnorrPair;
use itp_types::{DirectRequestStatus, Hash};
use lc_direct_call::DirectCall;
use litentry_primitives::{aes_encrypt_default, Address32, AesRequest, Identity, ShardIdentifier};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	string::ToString,
	sync::{
		mpsc::{sync_channel, SyncSender},
		Arc,
	},
	vec,
};

#[allow(clippy::too_many_arguments)]
pub fn init_ceremonies_thread<ClientFactory, AK, ER, OCallApi, SIGNINGAK, SHIELDAK, Responder>(
	signing_key_access: Arc<SIGNINGAK>,
	shielding_key_access: Arc<SHIELDAK>,
	client_factory: Arc<ClientFactory>,
	enclave_registry: Arc<ER>,
	ceremony_registry: Arc<Mutex<CeremonyRegistry<AK>>>,
	ceremony_commands: Arc<Mutex<CeremonyCommandsRegistry>>,
	ocall_api: Arc<OCallApi>,
	responder: Arc<Responder>,
) where
	ClientFactory: RpcClientFactory + Send + Sync + 'static,
	AK: AccessKey<KeyType = SchnorrPair> + Send + Sync + 'static,
	ER: EnclaveRegistryLookup + Send + Sync + 'static,
	OCallApi: EnclaveAttestationOCallApi + 'static,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	SHIELDAK: AccessPubkey<KeyType = Rsa3072PubKey> + Send + Sync + 'static,
	Responder: SendRpcResponse<Hash = H256> + 'static,
{
	let (responses_sender, responses_receiver) = sync_channel(1000);
	std::thread::spawn(move || {
		let my_identity: Address32 = signing_key_access.retrieve_key().unwrap().public().0.into();
		let identity = Identity::Substrate(my_identity);
		loop {
			{
				let mut ceremonies_to_remove = vec![];

				// do not hold lock for too long
				let ceremonies_to_process: Vec<CeremonyId> =
					ceremony_registry.lock().unwrap().keys().cloned().collect();

				for ceremony_id_to_process in ceremonies_to_process {
					// do not hold lock for too long as logic below includes I/O
					let events = if let Some(ceremony) =
						ceremony_registry.lock().unwrap().get_mut(&ceremony_id_to_process)
					{
						ceremony.tick()
					} else {
						warn!("Could not find ceremony with id: {:?}", ceremony_id_to_process);
						continue
					};

					trace!("Got ceremony {:?} events {:?}", ceremony_id_to_process, events);
					// should be retrieved once, but cannot be at startup because it's not yet initialized so it panics ...
					let mr_enclave = ocall_api.get_mrenclave_of_self().unwrap().m;
					for event in events {
						debug!(
							"Processing ceremony event: {:?} for ceremony: {:?}",
							event, ceremony_id_to_process
						);
						match event {
							CeremonyEvent::FirstRoundStarted(signers, message, nonce) => {
								signers.iter().for_each(|signer_id| {
									let aes_key = random_aes_key();
									let direct_call = DirectCall::NonceShare(
										identity.clone(),
										aes_key,
										message.clone(),
										nonce.serialize(),
									);

									debug!(
										"Sharing nonce with signer: {:?} for ceremony: {:?}",
										signer_id, ceremony_id_to_process
									);

									let request = prepare_request(
										aes_key,
										shielding_key_access.as_ref(),
										signing_key_access.as_ref(),
										mr_enclave,
										direct_call,
									);
									send_to_all_signers(
										&client_factory,
										&enclave_registry,
										&responses_sender,
										signer_id,
										&request,
									);
								});
							},
							CeremonyEvent::SecondRoundStarted(signers, message, signature) => {
								signers.iter().for_each(|signer_id| {
									let aes_key = random_aes_key();
									let direct_call = DirectCall::PartialSignatureShare(
										identity.clone(),
										aes_key,
										message.clone(),
										signature.serialize(),
									);

									debug!(
										"Sharing partial signature with signer: {:?} for ceremony: {:?}",
										signer_id,
										ceremony_id_to_process
									);

									let request = prepare_request(
										aes_key,
										shielding_key_access.as_ref(),
										signing_key_access.as_ref(),
										mr_enclave,
										direct_call,
									);
									send_to_all_signers(
										&client_factory,
										&enclave_registry,
										&responses_sender,
										signer_id,
										&request,
									);
								});
							},
							CeremonyEvent::CeremonyEnded(signature, request_aes_key) => {
								debug!(
									"Ceremony {:?} ended, signature {:?}",
									ceremony_id_to_process, signature
								);
								let hash = blake2_256(&ceremony_id_to_process.encode());
								let result = signature;
								let encrypted_result =
									aes_encrypt_default(&request_aes_key, &result.encode())
										.encode();
								if let Err(e) = responder.send_state_with_status(
									Hash::from_slice(&hash),
									encrypted_result,
									DirectRequestStatus::Ok,
								) {
									error!(
										"Could not send response to {:?}, reason: {:?}",
										&hash, e
									);
								}
								ceremonies_to_remove.push(ceremony_id_to_process.clone());
							},
							CeremonyEvent::CeremonyError(signers, error, request_aes_key) => {
								debug!("Ceremony {:?} error {:?}", ceremony_id_to_process, error);
								let hash = blake2_256(&ceremony_id_to_process.encode());
								let result = SignBitcoinError::CeremonyError;
								let encrypted_result =
									aes_encrypt_default(&request_aes_key, &result.encode())
										.encode();
								if let Err(e) = responder.send_state_with_status(
									Hash::from_slice(&hash),
									encrypted_result,
									DirectRequestStatus::Error,
								) {
									error!(
										"Could not send response to {:?}, reason: {:?}",
										&hash, e
									);
								}
								ceremonies_to_remove.push(ceremony_id_to_process.clone());

								//kill ceremonies on other workers
								signers.iter().for_each(|signer_id| {
									let aes_key = random_aes_key();
									let direct_call = DirectCall::KillCeremony(
										identity.clone(),
										aes_key,
										ceremony_id_to_process.clone(),
									);

									debug!(
										"Requesting ceremony kill on signer: {:?} for ceremony: {:?}",
										signer_id,
										ceremony_id_to_process
									);

									let request = prepare_request(
										aes_key,
										shielding_key_access.as_ref(),
										signing_key_access.as_ref(),
										mr_enclave,
										direct_call,
									);
									send_to_all_signers(
										&client_factory,
										&enclave_registry,
										&responses_sender,
										signer_id,
										&request,
									);
								});
							},
							CeremonyEvent::CeremonyTimedOut(signers, request_aes_key) => {
								debug!("Ceremony {:?} timed out", ceremony_id_to_process);
								let hash = blake2_256(&ceremony_id_to_process.encode());
								let result = SignBitcoinError::CeremonyError;
								let encrypted_result =
									aes_encrypt_default(&request_aes_key, &result.encode())
										.encode();
								if let Err(e) = responder.send_state_with_status(
									Hash::from_slice(&hash),
									encrypted_result,
									DirectRequestStatus::Error,
								) {
									error!(
										"Could not send response to {:?}, reason: {:?}",
										&hash, e
									);
								}
								ceremonies_to_remove.push(ceremony_id_to_process.clone());

								//kill ceremonies on other workers
								signers.iter().for_each(|signer_id| {
									let aes_key = random_aes_key();
									let direct_call = DirectCall::KillCeremony(
										identity.clone(),
										aes_key,
										ceremony_id_to_process.clone(),
									);

									debug!(
										"Requesting ceremony kill on signer: {:?} for ceremony: {:?}",
										signer_id,
										ceremony_id_to_process
									);

									let request = prepare_request(
										aes_key,
										shielding_key_access.as_ref(),
										signing_key_access.as_ref(),
										mr_enclave,
										direct_call,
									);
									send_to_all_signers(
										&client_factory,
										&enclave_registry,
										&responses_sender,
										signer_id,
										&request,
									);
								});
							},
						}
					}
				}

				let mut ceremony_commands = ceremony_commands.lock().unwrap();
				let mut ceremony_registry = ceremony_registry.lock().unwrap();
				ceremony_commands.retain(|_, ceremony_pending_commands| {
					ceremony_pending_commands.retain_mut(|c| {
						c.tick();
						c.ticks_left > 0
					});
					!ceremony_pending_commands.is_empty()
				});

				ceremonies_to_remove.iter().for_each(|ceremony_id| {
					debug!("Removing ceremony {:?}", ceremony_id);
					let _ = ceremony_registry.remove_entry(ceremony_id);
					let _ = ceremony_commands.remove_entry(ceremony_id);
				});
			}

			std::thread::sleep(Duration::from_millis(1000))
		}
	});

	// here we will process all responses
	std::thread::spawn(move || {
		while let Ok((_id, rpc_return_value)) = responses_receiver.recv() {
			info!("Got RPC return value: {:?}", rpc_return_value);
			if rpc_return_value.status == DirectRequestStatus::Error {
				error!("Got unexpected direct request status: {:?}", rpc_return_value.status);
			}
		}
	});
}

fn send_to_all_signers<ClientFactory, ER>(
	client_factory: &Arc<ClientFactory>,
	enclave_registry: &Arc<ER>,
	responses_sender: &SyncSender<Response>,
	signer_id: &SignerId,
	request: &RpcRequest,
) where
	ClientFactory: RpcClientFactory + Send + Sync + 'static,
	ER: EnclaveRegistryLookup + Send + Sync + 'static,
{
	enclave_registry.get_all().iter().for_each(|(identity, address)| {
		if signer_id == identity.as_ref() {
			trace!("creating new connection to peer: {:?}", address);
			match client_factory.create(address, responses_sender.clone()) {
				Ok(mut client) =>
					if let Err(e) = client.send(address, request) {
						error!("Could not send request to signer: {:?}, reason: {:?}", signer_id, e)
					},
				Err(e) => error!("Could not connect to peer {}, reason: {:?}", address, e),
			}
		}
	});
}

fn prepare_request<SHIELDAK, SIGNINGAK>(
	aes_key: [u8; 32],
	shielding_key_access: &SHIELDAK,
	signing_key_access: &SIGNINGAK,
	mr_enclave: [u8; 32],
	direct_call: DirectCall,
) -> RpcRequest
where
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	SHIELDAK: AccessPubkey<KeyType = Rsa3072PubKey> + Send + Sync + 'static,
{
	// this should never panic, if pub key is poisoned the state is corrupted
	let aes_key_encrypted =
		shielding_key_access.retrieve_pubkey().unwrap().encrypt(&aes_key).unwrap();

	let shard = ShardIdentifier::from_slice(&mr_enclave);
	// same as above
	let dc_signed =
		direct_call.sign(&signing_key_access.retrieve_key().unwrap().into(), &mr_enclave, &shard);
	let encrypted_dc = aes_encrypt_default(&aes_key, &dc_signed.encode());
	let request = AesRequest { shard, key: aes_key_encrypted, payload: encrypted_dc };
	RpcRequest {
		jsonrpc: "2.0".to_string(),
		method: "bitacross_submitRequest".to_string(),
		params: vec![request.to_hex()],
		id: Id::Number(1),
	}
}

#[cfg(feature = "std")]
fn random_aes_key() -> [u8; 32] {
	use rand::{thread_rng, RngCore};

	let mut seed = [0u8; 32];
	let mut rand = thread_rng();
	rand.fill_bytes(&mut seed);
	seed
}

#[cfg(feature = "sgx")]
fn random_aes_key() -> [u8; 32] {
	use sgx_rand::{Rng, StdRng};
	let mut seed = [0u8; 32];
	let mut rand = StdRng::new().unwrap();
	rand.fill_bytes(&mut seed);
	seed
}
