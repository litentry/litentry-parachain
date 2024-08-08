/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

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

use codec::{Decode, Encode};
use frame_support::storage::storage_prefix;
pub use ita_sgx_runtime::{Balance, Index, Runtime};
use ita_stf::{Getter, TrustedCall, TrustedCallSigned};
use itc_parentchain_indirect_calls_executor::error::Error;
use itp_api_client_types::StaticEvent;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_externalities::{SgxExternalities, SgxExternalitiesTrait};
use itp_stf_primitives::{traits::IndirectExecutor, types::TrustedOperation};
use itp_stf_state_handler::handle_state::HandleState;
use itp_storage::{key_to_account_id, storage_map_key, StorageHasher};
use itp_types::{
	parentchain::{
		events::ParentchainBlockProcessed, AccountId, FilterEvents, HandleParentchainEvents,
		ParentchainEventProcessingError, ParentchainId, ProcessedEventsArtifacts,
	},
	Delegator, RsaRequest, ScorePayment, H256,
};
use lc_dynamic_assertion::AssertionLogicRepository;
use lc_evm_dynamic_assertions::repository::EvmAssertionRepository;
// use lc_parachain_extrinsic_task_sender::{ParachainExtrinsicSender, SendParachainExtrinsic};
use litentry_hex_utils::decode_hex;
use litentry_primitives::{Assertion, Identity, ValidationData, Web3Network};
use log::*;
use pallet_identity_management_tee::IdentityContext;
use sp_core::{blake2_256, H160};
use sp_runtime::traits::Header;
use sp_std::vec::Vec;
use std::{collections::BTreeMap, format, println, string::String, sync::Arc};

pub struct ParentchainEventHandler<
	OCallApi: EnclaveOnChainOCallApi,
	HS: HandleState<StateT = SgxExternalities>,
> {
	pub assertion_repository: Arc<EvmAssertionRepository>,
	pub ocall_api: Arc<OCallApi>,
	pub state_handler: Arc<HS>,
}

impl<OCallApi: EnclaveOnChainOCallApi, HS: HandleState<StateT = SgxExternalities>>
	ParentchainEventHandler<OCallApi, HS>
{
	fn link_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
		encrypted_validation_data: Vec<u8>,
		encrypted_web3networks: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");

		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;
		let validation_data =
			ValidationData::decode(&mut executor.decrypt(&encrypted_validation_data)?.as_slice())?;
		let web3networks: Vec<Web3Network> =
			Decode::decode(&mut executor.decrypt(&encrypted_web3networks)?.as_slice())?;

		let trusted_call = TrustedCall::link_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			validation_data,
			web3networks,
			None,
			Default::default(),
		);
		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn deactivate_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;

		let trusted_call = TrustedCall::deactivate_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);
		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;

		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn activate_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;

		let trusted_call = TrustedCall::activate_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);
		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;

		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn request_vc<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		assertion: Assertion,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");

		let trusted_call = TrustedCall::request_vc(
			enclave_account_id.into(),
			account.clone().into(),
			assertion,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn post_opaque_task<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		request: &RsaRequest,
	) -> Result<(), Error> {
		debug!("post opaque task: {:?}", request);
		executor.submit_trusted_call(request.shard, request.payload.to_vec());

		Ok(())
	}

	fn store_assertion<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		id: H160,
		byte_code: Vec<u8>,
		secrets: Vec<Vec<u8>>,
	) -> Result<(), Error> {
		debug!("store assertion byte_code: {:?}, secrets: {:?}", byte_code, secrets);
		let mut decrypted_secrets = Vec::with_capacity(secrets.len());

		for secret in secrets.iter() {
			let secret = String::decode(
				&mut executor
					.decrypt(secret)
					.map_err(|e| {
						Error::AssertionCreatedHandling(format!(
							"Could not decrypt secret, reason: {:?}",
							e
						))
					})?
					.as_slice(),
			)
			.map_err(|e| {
				Error::AssertionCreatedHandling(format!("Could not decode secret, reason: {:?}", e))
			})?;
			decrypted_secrets.push(secret);
		}
		self.assertion_repository
			.save(id, (byte_code, decrypted_secrets))
			.map_err(Error::AssertionCreatedHandling)?;
		Ok(())
	}

	fn update_staking_scores<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		block_header: impl Header<Hash = H256>,
	) -> Result<(), Error> {
		let scores_key_prefix = storage_prefix(b"ScoreStaking", b"Scores");
		let scores_storage_keys_response = self
			.ocall_api
			.get_storage_keys(scores_key_prefix.into())
			.map_err(|_| Error::Other("Failed to get storage keys".into()))?;
		let scores_storage_keys: Vec<Vec<u8>> = scores_storage_keys_response
			.into_iter()
			.filter_map(decode_storage_key)
			.collect();
		let account_ids: Vec<AccountId> =
			scores_storage_keys.iter().filter_map(key_to_account_id).collect();
		let scores: BTreeMap<AccountId, ScorePayment<Balance>> = self
			.ocall_api
			.get_multiple_storages_verified(
				scores_storage_keys,
				&block_header,
				&ParentchainId::Litentry,
			)
			.map_err(|_| Error::Other("Failed to get multiple storages".into()))?
			.into_iter()
			.filter_map(|entry| {
				// TODO: check of the key needs to be decoded here
				let storage_key = decode_storage_key(entry.key)?;
				let account_id = key_to_account_id(&storage_key)?;
				let score_payment = entry.value?;
				Some((account_id, score_payment))
			})
			.collect();

		let delegator_state_storage_keys: Vec<Vec<u8>> = account_ids
			.iter()
			.map(|account_id| {
				storage_map_key(
					"ParachainStaking",
					"DelegatorState",
					account_id,
					&StorageHasher::Blake2_128Concat,
				)
			})
			.collect();
		let delegator_states: BTreeMap<AccountId, Delegator<AccountId, Balance>> = self
			.ocall_api
			.get_multiple_storages_verified(
				delegator_state_storage_keys,
				&block_header,
				&ParentchainId::Litentry,
			)
			.map_err(|_| Error::Other("Failed to get multiple storages".into()))?
			.into_iter()
			.filter_map(|entry| {
				// TODO: check of the key needs to be decoded here
				let storage_key = decode_storage_key(entry.key)?;
				let account_id = key_to_account_id(&storage_key)?;
				let delegator = entry.value?;
				Some((account_id, delegator))
			})
			.collect();

		let id_graphs_storage_keys: Vec<Vec<u8>> = account_ids
			.iter()
			.map(|account_id| {
				storage_map_key(
					"IdentityManagement",
					"IDGraphs",
					&Identity::from(account_id.clone()),
					&StorageHasher::Blake2_128Concat,
				)
			})
			.collect();

		let shard = executor.get_default_shard();

		let accounts_graphs = self
			.state_handler
			.execute_on_current(&shard, |state, _| {
				let mut id_graphs_accounts: BTreeMap<AccountId, Vec<AccountId>> = BTreeMap::new();
				for id_graph_storage_key in id_graphs_storage_keys.iter() {
					let id_graph: Vec<(Identity, IdentityContext<Runtime>)> = state
						.iter_prefix::<Identity, IdentityContext<Runtime>>(id_graph_storage_key)
						.unwrap_or_default();
					let graph_accounts: Vec<AccountId> = id_graph
						.iter()
						.filter_map(|(identity, _)| identity.to_account_id())
						.collect();
					if let Some(account_id) = key_to_account_id(id_graph_storage_key) {
						id_graphs_accounts.insert(account_id, graph_accounts);
					}
				}

				id_graphs_accounts
			})
			.map_err(|_| Error::Other("Failed to get id graphs".into()))?;

		let mut new_scores_rewards: BTreeMap<AccountId, Balance> = BTreeMap::new();

		for account_id in account_ids.iter() {
			let default_id_graph = Vec::new();
			let id_graph = accounts_graphs.get(account_id).unwrap_or(&default_id_graph);
			for identity in id_graph.iter() {
				if let Some(delegator) = delegator_states.get(identity) {
					if let Some(score_payment) = scores.get(account_id) {
						new_scores_rewards.insert(
							account_id.clone(),
							score_payment.total_reward + delegator.total,
						);
					}
				}
			}
		}

		// let extrinsic_sender = ParachainExtrinsicSender::new();

		// TODO: after the new calls are created in ScoreStaking pallet
		// - create a call to update the scores for each account with the new rewards
		// - submit the call to the extrinsic sender
		// - send another extrinsic to indicate that the reward distribution has finished

		Ok(())
	}
}

fn decode_storage_key(raw_key: Vec<u8>) -> Option<Vec<u8>> {
	let hex_key = String::decode(&mut raw_key.as_slice()).unwrap_or_default();
	decode_hex(hex_key).ok()
}

impl<Executor, OCallApi, HS> HandleParentchainEvents<Executor, TrustedCallSigned, Error>
	for ParentchainEventHandler<OCallApi, HS>
where
	Executor: IndirectExecutor<TrustedCallSigned, Error>,
	OCallApi: EnclaveOnChainOCallApi,
	HS: HandleState<StateT = SgxExternalities>,
{
	fn handle_events(
		&self,
		executor: &Executor,
		events: impl FilterEvents,
		block_header: impl Header<Hash = H256>,
	) -> Result<ProcessedEventsArtifacts, Error> {
		let mut handled_events: Vec<H256> = Vec::new();
		let mut successful_assertion_ids: Vec<H160> = Vec::new();
		let mut failed_assertion_ids: Vec<H160> = Vec::new();
		if let Ok(events) = events.get_link_identity_events() {
			debug!("Handling link_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found link_identity_event: {}", event);
					let result = Self::link_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
						event.encrypted_validation_data.clone(),
						event.encrypted_web3networks.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::LinkIdentityFailure)?;
		}

		if let Ok(events) = events.get_deactivate_identity_events() {
			debug!("Handling deactivate_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found deactivate_identity_event: {}", event);
					let result = Self::deactivate_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::DeactivateIdentityFailure)?;
		}

		if let Ok(events) = events.get_activate_identity_events() {
			debug!("Handling activate_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found activate_identity_event: {}", event);
					let result = Self::activate_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::ActivateIdentityFailure)?;
		}

		if let Ok(events) = events.get_vc_requested_events() {
			debug!("Handling VCRequested events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found VCRequested event: {}", event);
					let result =
						Self::request_vc(executor, &event.account, event.assertion.clone());
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::VCRequestedFailure)?;
		}

		if let Ok(events) = events.get_opaque_task_posted_events() {
			debug!("Handling OpaqueTaskPosted events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found OpaqueTaskPosted event: {:?}", event);
					let result = Self::post_opaque_task(executor, &event.request);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::OpaqueTaskPostedFailure)?;
		}

		if let Ok(events) = events.get_assertion_created_events() {
			debug!("Handling AssertionCreated events");
			events
				.into_iter()
				.try_for_each(|event| {
					debug!("found AssertionCreated event: {:?}", event);
					let event_hash = hash_of(&event);
					let result =
						self.store_assertion(executor, event.id, event.byte_code, event.secrets);
					handled_events.push(event_hash);
					if result.is_ok() {
						successful_assertion_ids.push(event.id);
					} else {
						failed_assertion_ids.push(event.id)
					}
					result
				})
				.map_err(|_| ParentchainEventProcessingError::AssertionCreatedFailure)?;
		}

		if let Ok(events) = events.get_parentchain_block_proccessed_events() {
			debug!("Handling ParentchainBlockProcessed events");
			events.iter().for_each(|event| {
				debug!("found ParentchainBlockProcessed event: {:?}", event);
				// This is for monitoring purposes
				handled_events.push(hash_of(ParentchainBlockProcessed::EVENT));
			});
		}

		if let Ok(events) = events.get_reward_distribution_started_events() {
			println!("Handling RewardDistributionStarted events");
			events
				.iter()
				.try_for_each(|event| {
					let event_hash = hash_of(&event);
					let result = self.update_staking_scores(executor, block_header.clone());
					handled_events.push(event_hash);

					result
				})
				.map_err(|_| ParentchainEventProcessingError::RewardDistributionStartedFailure)?;
		}

		Ok((handled_events, successful_assertion_ids, failed_assertion_ids))
	}
}

fn hash_of<T: Encode + ?Sized>(ev: &T) -> H256 {
	blake2_256(&ev.encode()).into()
}
