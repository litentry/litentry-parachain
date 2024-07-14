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

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use thiserror_sgx as thiserror;
	pub use threadpool_sgx as threadpool;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;

#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use std::sync::Mutex;

#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

use bc_enclave_registry::EnclaveRegistryLookup;
use bc_musig2_ceremony::{
	get_current_timestamp, CeremonyCommand, CeremonyCommandTmp, CeremonyEvent, CeremonyId,
	CeremonyRegistry,
};
use bc_musig2_runner::process_event;
use bc_relayer_registry::RelayerRegistryLookup;
use bc_signer_registry::SignerRegistryLookup;
use bc_task_sender::{init_bit_across_task_sender_storage, BitAcrossProcessingResult};
use codec::{Decode, Encode};
use core::{ops::Deref, time::Duration};
use frame_support::{ensure, sp_runtime::app_crypto::sp_core::blake2_256};
use ita_stf::TrustedCallSigned;
use itc_direct_rpc_client::{DirectRpcClientFactory, Response, RpcClientFactory};
use itc_direct_rpc_server::SendRpcResponse;
use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{
	ecdsa::Pair as EcdsaPair,
	key_repository::{AccessKey, AccessPubkey},
	schnorr::Pair as SchnorrPair,
	ShieldingCryptoDecrypt, ShieldingCryptoEncrypt,
};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_types::DirectRequestStatus;
use lc_direct_call::{
	handler::{kill_ceremony, nonce_share, partial_signature_share, sign_bitcoin, sign_ethereum},
	DirectCall, DirectCallSigned,
};
use litentry_primitives::{aes_encrypt_default, Address32, AesRequest, DecryptableRequest};
use log::*;
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::{ed25519, Pair, H256};
use std::{
	boxed::Box,
	collections::HashMap,
	format,
	string::{String, ToString},
	sync::{mpsc::channel, Arc},
	vec,
	vec::Vec,
};
use threadpool::ThreadPool;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct BitAcrossTaskContext<
	SKR,
	SIGNINGAK,
	EKR,
	BKR,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	RRL: RelayerRegistryLookup,
	ERL: EnclaveRegistryLookup,
	SRL: SignerRegistryLookup,
	Responder,
> where
	SKR: AccessKey + AccessPubkey<KeyType = Rsa3072PubKey>,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair>,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	Responder: SendRpcResponse<Hash = H256>,
{
	pub shielding_key: Arc<SKR>,
	pub signing_key_access: Arc<SIGNINGAK>,
	pub ethereum_key_repository: Arc<EKR>,
	pub bitcoin_key_repository: Arc<BKR>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
	pub relayer_registry_lookup: Arc<RRL>,
	pub enclave_registry_lookup: Arc<ERL>,
	pub signer_registry_lookup: Arc<SRL>,
	pub signing_key_pub: [u8; 32],
	pub responder: Arc<Responder>,
	pub ceremony_registry: Arc<RwLock<CeremonyRegistry<BKR>>>,
	pub ceremony_command_tmp: Arc<RwLock<CeremonyCommandTmp>>,
}

impl<
		SKR,
		SIGNINGAK,
		EKR,
		BKR,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
		RRL: RelayerRegistryLookup,
		ERL: EnclaveRegistryLookup,
		SRL: SignerRegistryLookup,
		Responder,
	> BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>
where
	SKR: AccessKey + AccessPubkey<KeyType = Rsa3072PubKey>,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair>,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
	Responder: SendRpcResponse<Hash = H256>,
{
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		shielding_key: Arc<SKR>,
		signing_key_access: Arc<SIGNINGAK>,
		ethereum_key_repository: Arc<EKR>,
		bitcoin_key_repository: Arc<BKR>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
		relayer_registry_lookup: Arc<RRL>,
		enclave_registry_lookup: Arc<ERL>,
		signer_registry_lookup: Arc<SRL>,
		signing_key_pub: [u8; 32],
		ceremony_registry: Arc<RwLock<CeremonyRegistry<BKR>>>,
		ceremony_command_tmp: Arc<RwLock<CeremonyCommandTmp>>,
		responder: Arc<Responder>,
	) -> Self {
		Self {
			shielding_key,
			signing_key_access,
			ethereum_key_repository,
			bitcoin_key_repository,
			enclave_signer,
			state_handler,
			ocall_api,
			relayer_registry_lookup,
			enclave_registry_lookup,
			signer_registry_lookup,
			signing_key_pub,
			ceremony_registry,
			ceremony_command_tmp,
			responder,
		}
	}
}

#[allow(clippy::type_complexity)]
pub fn run_bit_across_handler_runner<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
) where
	SKR: AccessKey + AccessPubkey<KeyType = Rsa3072PubKey> + Send + Sync + 'static,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	EKR: AccessKey<KeyType = EcdsaPair> + Send + Sync + 'static,
	BKR: AccessKey<KeyType = SchnorrPair> + Send + Sync + 'static,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	RRL: RelayerRegistryLookup + Send + Sync + 'static,
	ERL: EnclaveRegistryLookup + Send + Sync + 'static,
	SRL: SignerRegistryLookup + Send + Sync + 'static,
	Responder: SendRpcResponse<Hash = H256> + Send + Sync + 'static,
{
	let (responses_sender, responses_receiver) = channel::<Response>();
	// here we will process all responses
	std::thread::spawn(move || {
		while let Ok((_id, rpc_return_value)) = responses_receiver.recv() {
			info!("Got RPC return value: {:?}", rpc_return_value);
			if rpc_return_value.status == DirectRequestStatus::Error {
				error!("Got unexpected direct request status: {:?}", rpc_return_value.status);
			}
		}
	});

	// timeout tick
	let ceremony_registry = context.ceremony_registry.clone();
	let ceremony_command_tmp = context.ceremony_command_tmp.clone();
	let time_to_live = 30u64;
	std::thread::spawn(move || loop {
		std::thread::sleep(Duration::from_secs(3));
		let now = get_current_timestamp();
		{
			let mut ceremony_registry_write = ceremony_registry.write().unwrap();
			ceremony_registry_write
				.retain(|_, &mut (_, create_time)| now - create_time < time_to_live);
		}
		{
			let mut command_tmp_write = ceremony_command_tmp.write().unwrap();
			command_tmp_write.retain(|_, &mut (_, create_time)| now - create_time < time_to_live);
		}
	});

	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let command_threads_count = 2;
	let event_threads_count = 2;
	let command_threads_pool = ThreadPool::new(command_threads_count);
	let event_threads_pool = ThreadPool::new(event_threads_count);
	info!("start {} command threads, {} event threads", command_threads_count, event_threads_count);
	let peers_map = Arc::new(Mutex::new(HashMap::new()));
	while let Ok(mut req) = bit_across_task_receiver.recv() {
		let context_pool = context.clone();
		let responses_sender = responses_sender.clone();
		let event_threads_pool = event_threads_pool.clone();
		let peers_map = peers_map.clone();
		command_threads_pool.execute(move || {
			let (ceremony_id, command) =
				match handle_request(&mut req.request, context_pool.clone()) {
					Ok((processing_ret, to_process)) => {
						if let Err(e) = req.sender.send(Ok(processing_ret)) {
							warn!("Unable to submit response back to the handler: {:?}", e);
						}
						let Some((ceremony_id, command)) = to_process else { return };
						(ceremony_id, command)
					},
					Err(e) => {
						if let Err(e) = req.sender.send(Err(e)) {
							warn!("Unable to submit response back to the handler: {:?}", e);
						}
						return
					},
				};

			// check if store command to tmp
			let is_first_round = {
				context_pool
					.ceremony_registry
					.read()
					.unwrap()
					.get(&ceremony_id)
					.map(|(c, _)| c.read().unwrap().is_first_round())
			};
			match (is_first_round, &command) {
				(None, CeremonyCommand::SaveNonce(_, _))
				| (Some(true), CeremonyCommand::SavePartialSignature(_, _)) => {
					context_pool
						.ceremony_command_tmp
						.write()
						.unwrap()
						.entry(ceremony_id.clone())
						.and_modify(|(command_tmp, _)| {
							command_tmp.write().unwrap().push(command.clone())
						})
						.or_insert((Arc::new(RwLock::new(vec![command])), get_current_timestamp()));
					return
				},
				(Some(true), CeremonyCommand::SaveNonce(_, _))
				| (Some(true), CeremonyCommand::Init)
				| (Some(false), CeremonyCommand::SavePartialSignature(_, _)) => {},
				(is_first_round, command) => {
					error!(
						"receive wrong command: is_first_round: {:?}, command: {}, drop it",
						is_first_round, command
					);
					return
				},
			}

			// try to udpate peers_map
			let my_identity: Address32 =
				context_pool.signing_key_access.retrieve_key().unwrap().public().0.into();
			context_pool.enclave_registry_lookup.get_all().iter().for_each(
				|(identity, address)| {
					if my_identity != *identity
						&& !peers_map.lock().unwrap().contains_key(identity.as_ref())
					{
						info!("creating new connection to peer: {:?}", address);
						match (DirectRpcClientFactory {}).create(address, responses_sender.clone())
						{
							Ok(client) => {
								peers_map.lock().unwrap().insert(*identity.as_ref(), client);
							},
							Err(e) =>
								error!("Could not connect to peer {}, reason: {:?}", address, e),
						}
					}
				},
			);

			// process commands and events
			let mut commands_to_process = vec![command];
			while !commands_to_process.is_empty() {
				let command = commands_to_process.pop().unwrap();
				let event = {
					let ceremony_rwlock = {
						context_pool
							.ceremony_registry
							.read()
							.unwrap()
							.get(&ceremony_id)
							.unwrap()
							.0
							.clone()
					};

					let mut ceremony = ceremony_rwlock.write().unwrap();
					ceremony.process_command(command)
				};

				if let Some(event) = event {
					match event {
						CeremonyEvent::FirstRoundStarted(_, _, _)
						| CeremonyEvent::SecondRoundStarted(_, _, _) => {
							let has_command_tmp = {
								let command_tmp_read =
									context_pool.ceremony_command_tmp.read().unwrap();
								command_tmp_read.contains_key(&ceremony_id)
							};
							if has_command_tmp {
								let ceremony_command_tmp = context_pool
									.ceremony_command_tmp
									.write()
									.unwrap()
									.remove(&ceremony_id)
									.unwrap()
									.0;
								commands_to_process = ceremony_command_tmp.read().unwrap().clone();
							}
						},
						CeremonyEvent::CeremonyEnded(_, _)
						| CeremonyEvent::CeremonyError(_, _, _) => {
							// remove ceremony
							{
								let mut registry_write =
									context_pool.ceremony_registry.write().unwrap();
								registry_write.remove(&ceremony_id);
							}
							{
								context_pool
									.ceremony_command_tmp
									.write()
									.unwrap()
									.remove(&ceremony_id);
							}
						},
					}

					process_event(
						context_pool.signing_key_access.clone(),
						context_pool.shielding_key.clone(),
						context_pool.ocall_api.clone(),
						context_pool.responder.clone(),
						event,
						ceremony_id.clone(),
						event_threads_pool.clone(),
						peers_map.clone(),
					);
				}
			}
		});
		warn!("command_threads_pool: {}", command_threads_pool.queued_count());
	}

	command_threads_pool.join();
	event_threads_pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

#[allow(clippy::type_complexity)]
pub fn handle_request<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
) -> Result<(BitAcrossProcessingResult, Option<(CeremonyId, CeremonyCommand)>), Vec<u8>>
where
	SKR: AccessKey + AccessPubkey<KeyType = Rsa3072PubKey>,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair>,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	RRL: RelayerRegistryLookup + 'static,
	ERL: EnclaveRegistryLookup + 'static,
	SRL: SignerRegistryLookup + 'static,
	Responder: SendRpcResponse<Hash = H256> + Send + Sync + 'static,
{
	let enclave_shielding_key = context.shielding_key.retrieve_key().map_err(|e| {
		let err = format!("Failed to retrieve shielding key: {:?}", e);
		error!("{}", err);
		err
	})?;
	let dc = request
		.decrypt(Box::new(enclave_shielding_key))
		.ok()
		.and_then(|v| DirectCallSigned::decode(&mut v.as_slice()).ok())
		.ok_or_else(|| {
			let err = "Failed to decode payload".to_string();
			error!("{}", err);
			err
		})?;

	let mrenclave = match context.ocall_api.get_mrenclave_of_self() {
		Ok(m) => m.m,
		Err(_) => {
			let err = "Failed to get mrenclave";
			error!("{}", err);
			return Err(err.encode())
		},
	};
	debug!("Direct call is: {:?}", dc);
	ensure!(dc.verify_signature(&mrenclave, &request.shard), "Failed to verify sig".to_string());
	match dc.call {
		DirectCall::SignBitcoin(signer, aes_key, payload) => {
			let hash = blake2_256(&payload.encode());
			let command = sign_bitcoin::handle(
				signer,
				payload.clone(),
				aes_key,
				context.relayer_registry_lookup.deref(),
				context.ceremony_registry.clone(),
				context.signer_registry_lookup.clone(),
				&context.signing_key_pub,
				context.bitcoin_key_repository.clone(),
			)
			.map_err(|e| {
				error!("SignBitcoin error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})?;
			let ret = BitAcrossProcessingResult::Submitted(hash);
			Ok((ret, Some((payload, command))))
		},
		DirectCall::SignEthereum(signer, aes_key, msg) => sign_ethereum::handle(
			signer,
			msg,
			context.relayer_registry_lookup.deref(),
			context.ethereum_key_repository.deref(),
		)
		.map_err(|e| {
			error!("SignEthereum error: {:?}", e);
			aes_encrypt_default(&aes_key, &e.encode()).encode()
		})
		.map(|r| (BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r).encode()), None)),
		DirectCall::NonceShare(signer, aes_key, message, nonce) =>
			nonce_share::handle(signer, &message, nonce, context.enclave_registry_lookup.clone())
				.map_err(|e| {
					error!("NonceShare error: {:?}", e);
					aes_encrypt_default(&aes_key, &e.encode()).encode()
				})
				.map(|command| {
					(
						BitAcrossProcessingResult::Ok(
							aes_encrypt_default(&aes_key, &().encode()).encode(),
						),
						Some((message, command)),
					)
				}),
		DirectCall::PartialSignatureShare(signer, aes_key, message, signature) =>
			partial_signature_share::handle(
				signer,
				&message,
				signature,
				context.enclave_registry_lookup.clone(),
			)
			.map_err(|e| {
				error!("PartialSignatureShare error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})
			.map(|command| {
				(
					BitAcrossProcessingResult::Ok(
						aes_encrypt_default(&aes_key, &().encode()).encode(),
					),
					Some((message, command)),
				)
			}),
		DirectCall::KillCeremony(signer, aes_key, message) => kill_ceremony::handle(
			signer,
			message,
			context.ceremony_registry.clone(),
			context.ceremony_command_tmp.clone(),
			context.enclave_registry_lookup.clone(),
		)
		.map_err(|e| {
			error!("KillCeremony error: {:?}", e);
			aes_encrypt_default(&aes_key, &e.encode()).encode()
		})
		.map(|r| {
			(
				BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r.encode()).encode()),
				None,
			)
		}),
	}
}
