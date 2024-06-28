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
use std::sync::Mutex;

#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

use core::ops::Deref;

use bc_musig2_ceremony::{CeremonyCommandsRegistry, CeremonyRegistry};
use bc_task_sender::{init_bit_across_task_sender_storage, BitAcrossProcessingResult};
use codec::{Decode, Encode};
use frame_support::{ensure, sp_runtime::app_crypto::sp_core::blake2_256};
use lc_direct_call::{DirectCall, DirectCallSigned};
use litentry_primitives::{aes_encrypt_default, AesRequest};
use log::*;
use std::{
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use threadpool::ThreadPool;

use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;

use bc_enclave_registry::EnclaveRegistryLookup;
use bc_relayer_registry::RelayerRegistryLookup;
use bc_signer_registry::SignerRegistryLookup;
use ita_stf::TrustedCallSigned;
use itp_sgx_crypto::{ecdsa::Pair as EcdsaPair, schnorr::Pair as SchnorrPair};
use lc_direct_call::handler::{
	kill_ceremony, nonce_share, partial_signature_share, sign_bitcoin, sign_ethereum,
};
use litentry_primitives::DecryptableRequest;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct BitAcrossTaskContext<
	SKR,
	EKR,
	BKR,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	RRL: RelayerRegistryLookup,
	ERL: EnclaveRegistryLookup,
	SRL: SignerRegistryLookup,
> where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub shielding_key: Arc<SKR>,
	pub ethereum_key_repository: Arc<EKR>,
	pub bitcoin_key_repository: Arc<BKR>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
	pub relayer_registry_lookup: Arc<RRL>,
	pub musig2_ceremony_registry: Arc<Mutex<CeremonyRegistry<BKR>>>,
	pub enclave_registry_lookup: Arc<ERL>,
	pub signer_registry_lookup: Arc<SRL>,
	pub musig2_ceremony_pending_commands: Arc<Mutex<CeremonyCommandsRegistry>>,
	pub signing_key_pub: [u8; 32],
}

impl<
		SKR,
		EKR,
		BKR,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
		RRL: RelayerRegistryLookup,
		ERL: EnclaveRegistryLookup,
		SRL: SignerRegistryLookup,
	> BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL, ERL, SRL>
where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
{
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		shielding_key: Arc<SKR>,
		ethereum_key_repository: Arc<EKR>,
		bitcoin_key_repository: Arc<BKR>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
		relayer_registry_lookup: Arc<RRL>,
		musig2_ceremony_registry: Arc<Mutex<CeremonyRegistry<BKR>>>,
		enclave_registry_lookup: Arc<ERL>,
		signer_registry_lookup: Arc<SRL>,
		musig2_ceremony_pending_commands: Arc<Mutex<CeremonyCommandsRegistry>>,
		signing_key_pub: [u8; 32],
	) -> Self {
		Self {
			shielding_key,
			ethereum_key_repository,
			bitcoin_key_repository,
			enclave_signer,
			state_handler,
			ocall_api,
			relayer_registry_lookup,
			musig2_ceremony_registry,
			enclave_registry_lookup,
			signer_registry_lookup,
			musig2_ceremony_pending_commands,
			signing_key_pub,
		}
	}
}

#[allow(clippy::type_complexity)]
pub fn run_bit_across_handler_runner<SKR, EKR, BKR, S, H, O, RRL, ERL, SRL>(
	context: Arc<BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL, ERL, SRL>>,
) where
	SKR: AccessKey + Send + Sync + 'static,
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
{
	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let n_workers = 2;
	let pool = ThreadPool::new(n_workers);

	while let Ok(mut req) = bit_across_task_receiver.recv() {
		let context_pool = context.clone();
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(&mut req.request, context_pool)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

#[allow(clippy::type_complexity)]
pub fn handle_request<SKR, EKR, BKR, S, H, O, RRL, ERL, SRL>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL, ERL, SRL>>,
) -> Result<BitAcrossProcessingResult, Vec<u8>>
where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	RRL: RelayerRegistryLookup + 'static,
	ERL: EnclaveRegistryLookup + 'static,
	SRL: SignerRegistryLookup + 'static,
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
			sign_bitcoin::handle(
				signer,
				payload,
				aes_key,
				context.relayer_registry_lookup.deref(),
				context.musig2_ceremony_registry.clone(),
				context.musig2_ceremony_pending_commands.clone(),
				context.signer_registry_lookup.clone(),
				&context.signing_key_pub,
				context.bitcoin_key_repository.clone(),
			)
			.map_err(|e| {
				error!("SignBitcoin error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})?;
			Ok(BitAcrossProcessingResult::Submitted(hash))
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
		.map(|r| BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r).encode())),
		DirectCall::NonceShare(signer, aes_key, message, nonce) => nonce_share::handle(
			signer,
			message,
			nonce,
			context.musig2_ceremony_registry.clone(),
			context.musig2_ceremony_pending_commands.clone(),
			context.enclave_registry_lookup.clone(),
		)
		.map_err(|e| {
			error!("NonceShare error: {:?}", e);
			aes_encrypt_default(&aes_key, &e.encode()).encode()
		})
		.map(|r| {
			BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r.encode()).encode())
		}),
		DirectCall::PartialSignatureShare(signer, aes_key, message, signature) =>
			partial_signature_share::handle(
				signer,
				message,
				signature,
				context.musig2_ceremony_registry.clone(),
				context.enclave_registry_lookup.clone(),
			)
			.map_err(|e| {
				error!("PartialSignatureShare error: {:?}", e);
				aes_encrypt_default(&aes_key, &e.encode()).encode()
			})
			.map(|r| {
				BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r.encode()).encode())
			}),
		DirectCall::KillCeremony(signer, aes_key, message) => kill_ceremony::handle(
			signer,
			message,
			context.musig2_ceremony_registry.clone(),
			context.musig2_ceremony_pending_commands.clone(),
			context.enclave_registry_lookup.clone(),
		)
		.map_err(|e| {
			error!("KillCeremony error: {:?}", e);
			aes_encrypt_default(&aes_key, &e.encode()).encode()
		})
		.map(|r| {
			BitAcrossProcessingResult::Ok(aes_encrypt_default(&aes_key, &r.encode()).encode())
		}),
	}
}
