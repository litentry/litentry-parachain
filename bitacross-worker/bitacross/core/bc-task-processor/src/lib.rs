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
	get_current_timestamp, CeremonyCommand, CeremonyCommandTmp, CeremonyError, CeremonyErrorReason,
	CeremonyEvent, CeremonyId, CeremonyRegistry, MuSig2Ceremony, SignBitcoinPayload,
};
use bc_musig2_event::process_event;
use bc_relayer_registry::RelayerRegistryLookup;
use bc_signer_registry::SignerRegistryLookup;
use bc_task_sender::{
	init_bit_across_task_sender_storage, BitAcrossProcessingResult, BitAcrossRequest,
};
use codec::{Decode, Encode};
use core::{ops::Deref, time::Duration};
use frame_support::{ensure, sp_runtime::app_crypto::sp_core::blake2_256};
use futures_sgx::AsyncReadExt;
use ita_stf::TrustedCallSigned;
use itc_direct_rpc_client::{DirectRpcClient, DirectRpcClientFactory, RpcClientFactory};
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
use lc_direct_call::{
	handler::{kill_ceremony, nonce_share, partial_signature_share, sign_bitcoin, sign_ethereum},
	CeremonyRoundCall, CeremonyRoundCallSigned, DirectCall, DirectCallSigned,
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
	sync::Arc,
	vec,
	vec::Vec,
};
use threadpool::ThreadPool;
use itp_enclave_metrics::EnclaveMetric;

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
	ceremony_commands_thread_count: u8,
	ceremony_events_thread_count: u8,
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
	// timeout tick
	let ceremony_registry = context.ceremony_registry.clone();
	let ceremony_command_tmp = context.ceremony_command_tmp.clone();
	let time_to_live = 30u64;
	let cloned_ocall_api = context.ocall_api.clone();
	std::thread::spawn(move || loop {
		std::thread::sleep(Duration::from_secs(3));
		let now = get_current_timestamp();
		let mut timed_out_count: u8 = 0;
		{
			let mut ceremony_registry_write = ceremony_registry.write().unwrap();
			ceremony_registry_write
				.retain(|_, &mut (_, create_time)| {

					if now - create_time < time_to_live {
						true
					} else {
						timed_out_count += 1;
						false
					}
				});
		}
		{
			let mut command_tmp_write = ceremony_command_tmp.write().unwrap();
			command_tmp_write.retain(|_, &mut (_, create_time)| now - create_time < time_to_live);
		}

		if timed_out_count > 0 {
			let _ = cloned_ocall_api.update_metric(EnclaveMetric::Musig2CeremonyTimedout(timed_out_count));
		}

	});

	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let peers_map = Arc::new(Mutex::new(HashMap::<[u8; 32], DirectRpcClient>::new()));
	let command_threads_pool = ThreadPool::new(ceremony_commands_thread_count.into());
	let event_threads_pool = ThreadPool::new(ceremony_events_thread_count.into());

	while let Ok(req) = bit_across_task_receiver.recv() {
		let context = context.clone();
		let event_threads_pool = event_threads_pool.clone();
		let peers_map = peers_map.clone();
		command_threads_pool.execute(move || {
			if let Some((ceremony_id, command)) = handle_request(req, context.clone()) {
				handle_ceremony_command(
					context,
					ceremony_id,
					command,
					event_threads_pool,
					peers_map,
				);
			}
		});
	}

	command_threads_pool.join();
	event_threads_pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

#[allow(clippy::type_complexity)]
fn handle_ceremony_command<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
	ceremony_id: CeremonyId,
	command: CeremonyCommand,
	event_threads_pool: ThreadPool,
	peers_map: Arc<Mutex<HashMap<[u8; 32], DirectRpcClient>>>,
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
	// check whether to store command to tmp
	let is_first_round = {
		context
			.ceremony_registry
			.read()
			.unwrap()
			.get(&ceremony_id)
			.map(|(c, _)| c.read().unwrap().is_first_round())
	};
	match (is_first_round, &command) {
		(None, CeremonyCommand::InitCeremony(_, _, _, _))
		| (Some(true), CeremonyCommand::SaveNonce(_, _))
		| (Some(false), CeremonyCommand::SavePartialSignature(_, _))
		| (_, CeremonyCommand::KillCeremony) => {},
		(None, CeremonyCommand::SaveNonce(_, _))
		| (Some(true), CeremonyCommand::SavePartialSignature(_, _)) => {
			context
				.ceremony_command_tmp
				.write()
				.unwrap()
				.entry(ceremony_id)
				.and_modify(|(command_tmp, _)| command_tmp.write().unwrap().push(command.clone()))
				.or_insert((Arc::new(RwLock::new(vec![command])), get_current_timestamp()));
			return
		},
		(is_first_round, command) => {
			error!(
				"receive wrong command: is_first_round: {:?}, command: {:?}, drop it",
				is_first_round, command
			);
			return
		},
	}

	// try to udpate peers_map
	let my_identity: Address32 =
		context.signing_key_access.retrieve_key().unwrap().public().0.into();
	context
		.enclave_registry_lookup
		.get_all()
		.iter()
		.for_each(|(identity, address)| {
			if my_identity != *identity
				&& !peers_map.lock().unwrap().contains_key(identity.as_ref())
			{
				info!("creating new connection to peer: {:?}", address);
				match (DirectRpcClientFactory {}).create(address) {
					Ok(client) => {
						peers_map.lock().unwrap().insert(*identity.as_ref(), client);
					},
					Err(e) => error!("Could not connect to peer {}, reason: {:?}", address, e),
				}
			}
		});

	// process commands and events
	let mut commands_to_process = vec![command];
	while !commands_to_process.is_empty() {
		let command = commands_to_process.pop().unwrap();

		let event = process_command(context.clone(), ceremony_id.clone(), command);

		if let Some(event) = event {
			// update metrics
			match event {
				CeremonyEvent::FirstRoundStarted(_, _, _) => {
					let _ = context.ocall_api.update_metric(EnclaveMetric::Musig2CeremonyStarted);
				},
				CeremonyEvent::CeremonyError(_, _, _) => {
					let _ = context.ocall_api.update_metric(EnclaveMetric::Musig2CeremonyFailed);
				},
				CeremonyEvent::CeremonyEnded(_, _, _, _) => {
					let ceremony_start_time = context.ceremony_registry.read().unwrap().get(&ceremony_id).unwrap().1;
					let _ = context.ocall_api.update_metric(EnclaveMetric::Musig2CeremonyDuration(Duration::from_millis(get_current_timestamp() - ceremony_start_time)));
				}
				_ => {}
			}


			match event {
				CeremonyEvent::FirstRoundStarted(_, _, _)
				| CeremonyEvent::SecondRoundStarted(_, _, _) => {
					// get all ceremony_command_tmp
					let mut ceremony_command_tmp_write =
						context.ceremony_command_tmp.write().unwrap();
					if let Some((ceremony_command_tmp, _)) =
						ceremony_command_tmp_write.remove(&ceremony_id)
					{
						commands_to_process = ceremony_command_tmp.read().unwrap().clone();
					}
				},
				CeremonyEvent::CeremonyEnded(_, _, _, _)
				| CeremonyEvent::CeremonyError(_, _, _) => {
					// remove ceremony
					{
						let mut registry_write = context.ceremony_registry.write().unwrap();
						registry_write.remove(&ceremony_id);
					}
					{
						context.ceremony_command_tmp.write().unwrap().remove(&ceremony_id);
					}
				},
			}

			process_event(
				context.signing_key_access.clone(),
				context.shielding_key.clone(),
				context.ocall_api.clone(),
				context.responder.clone(),
				event,
				ceremony_id.clone(),
				event_threads_pool.clone(),
				peers_map.clone(),
			);
		}
	}
}

#[allow(clippy::type_complexity)]
fn process_command<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
	ceremony_id: CeremonyId,
	command: CeremonyCommand,
) -> Option<CeremonyEvent>
where
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
	match command {
		CeremonyCommand::InitCeremony(aes_key, signers, payload, check_run) => {
			// InitCeremony should create ceremony first
			let result = MuSig2Ceremony::new(
				context.signing_key_pub,
				aes_key,
				signers,
				payload,
				context.bitcoin_key_repository.clone(),
				check_run,
			);

			match result {
				Ok((ceremony, event)) => {
					{
						let mut registry_write = context.ceremony_registry.write().unwrap();
						if registry_write.contains_key(&ceremony_id) {
							let error =
								CeremonyError::CeremonyInitError(CeremonyErrorReason::AlreadyExist);
							return Some(CeremonyEvent::CeremonyError(vec![], error, aes_key))
						}
						registry_write.insert(
							ceremony_id,
							(Arc::new(RwLock::new(ceremony)), get_current_timestamp()),
						);
					}
					Some(event)
				},
				Err(e) => {
					error!("Could not start ceremony, error: {:?}", e);
					let error =
						CeremonyError::CeremonyInitError(CeremonyErrorReason::CreateCeremonyError);
					Some(CeremonyEvent::CeremonyError(vec![], error, aes_key))
				},
			}
		},
		CeremonyCommand::SaveNonce(signer, nonce) => {
			let ceremony_rwlock =
				context.ceremony_registry.read().unwrap().get(&ceremony_id).cloned();
			if let Some(ceremony_rwlock) = ceremony_rwlock {
				let mut ceremony_write_lock = ceremony_rwlock.0.write().unwrap();
				let event_ret = ceremony_write_lock.receive_nonce(signer, nonce);
				match event_ret {
					Ok(event) => event,
					Err(e) => Some(CeremonyEvent::CeremonyError(
						ceremony_write_lock.get_signers_except_self(),
						e,
						*ceremony_write_lock.get_aes_key(),
					)),
				}
			} else {
				None
			}
		},
		CeremonyCommand::SavePartialSignature(signer, partial_signature) => {
			let ceremony_rwlock =
				context.ceremony_registry.read().unwrap().get(&ceremony_id).cloned();
			if let Some(ceremony_rwlock) = ceremony_rwlock {
				let mut ceremony_write_lock = ceremony_rwlock.0.write().unwrap();
				let event_ret = ceremony_write_lock.receive_partial_sign(signer, partial_signature);
				match event_ret {
					Ok(event) => event,
					Err(e) => Some(CeremonyEvent::CeremonyError(
						ceremony_write_lock.get_signers_except_self(),
						e,
						*ceremony_write_lock.get_aes_key(),
					)),
				}
			} else {
				None
			}
		},
		CeremonyCommand::KillCeremony => {
			{
				context.ceremony_registry.write().unwrap().remove(&ceremony_id);
			}
			{
				context.ceremony_command_tmp.write().unwrap().remove(&ceremony_id);
			}
			None
		},
	}
}

#[allow(clippy::type_complexity)]
fn handle_request<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	request: BitAcrossRequest,
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
) -> Option<(CeremonyId, CeremonyCommand)>
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
	match request {
		BitAcrossRequest::Request(mut aes_request, sender) => {
			match handle_direct_call(&mut aes_request, context) {
				Ok((processing_ret, to_process)) => {
					if let Some(processing_ret) = processing_ret {
						if let Err(e) = sender.send(Ok(processing_ret)) {
							warn!("Unable to submit response back to the handler: {:?}", e);
						}
					}
					to_process
				},
				Err(e) => {
					if let Err(e) = sender.send(Err(e)) {
						warn!("Unable to submit response back to the handler: {:?}", e);
					}
					None
				},
			}
		},
		BitAcrossRequest::ShareCeremonyData(mut aes_request) =>
			handle_ceremony_round_call(&mut aes_request, context).unwrap_or_default(),
	}
}

#[allow(clippy::type_complexity)]
fn handle_direct_call<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
) -> Result<(Option<BitAcrossProcessingResult>, Option<(CeremonyId, CeremonyCommand)>), Vec<u8>>
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
				context.signer_registry_lookup.clone(),
				context.enclave_registry_lookup.as_ref(),
				false,
			)
			.map_err(|e| {
				error!("SignBitcoin error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})?;
			let ret = BitAcrossProcessingResult::Submitted(hash);
			Ok((Some(ret), Some((payload, command))))
		},
		DirectCall::CheckSignBitcoin(signer) => {
			let payload = SignBitcoinPayload::Derived([0u8; 32].to_vec());
			let aes_key = [0u8; 32];
			let hash = blake2_256(&payload.encode());
			let command = sign_bitcoin::handle(
				signer,
				payload.clone(),
				aes_key,
				context.relayer_registry_lookup.deref(),
				context.signer_registry_lookup.clone(),
				context.enclave_registry_lookup.as_ref(),
				true,
			)
			.map_err(|e| {
				error!("SignBitcoinCheck error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})?;
			let ret = BitAcrossProcessingResult::Submitted(hash);
			Ok((Some(ret), Some((payload, command))))
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
		.map(|r| {
			(Some(BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r).encode())), None)
		}),
	}
}

#[allow(clippy::type_complexity)]
fn handle_ceremony_round_call<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<SKR, SIGNINGAK, EKR, BKR, S, H, O, RRL, ERL, SRL, Responder>>,
) -> Result<Option<(CeremonyId, CeremonyCommand)>, Vec<u8>>
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
	let crc = request
		.decrypt(Box::new(enclave_shielding_key))
		.ok()
		.and_then(|v| CeremonyRoundCallSigned::decode(&mut v.as_slice()).ok())
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
	debug!("Ceremony round call is: {:?}", crc);
	ensure!(crc.verify_signature(&mrenclave, &request.shard), "Failed to verify sig".to_string());
	match crc.call {
		CeremonyRoundCall::NonceShare(signer, aes_key, message, nonce) =>
			nonce_share::handle(signer, &message, nonce, context.enclave_registry_lookup.clone())
				.map_err(|e| {
					error!("NonceShare error: {:?}", e);
					aes_encrypt_default(&aes_key, &e.encode()).encode()
				})
				.map(|command| Some((message, command))),
		CeremonyRoundCall::PartialSignatureShare(signer, aes_key, message, signature) =>
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
			.map(|command| Some((message, command))),
		CeremonyRoundCall::KillCeremony(signer, aes_key, message) =>
			kill_ceremony::handle(signer, context.enclave_registry_lookup.as_ref())
				.map_err(|e| {
					error!("KillCeremony error: {:?}", e);
					aes_encrypt_default(&aes_key, &e.encode()).encode()
				})
				.map(|command| Some((message, command))),
	}
}
