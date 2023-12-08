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

use crate::{
	error::Result,
	initialization::global_components::{
		GLOBAL_OCALL_API_COMPONENT, GLOBAL_SIDECHAIN_BLOCK_COMPOSER_COMPONENT,
		GLOBAL_SIDECHAIN_FAIL_SLOT_ON_DEMAND_COMPONENT,
		GLOBAL_SIDECHAIN_IMPORT_QUEUE_WORKER_COMPONENT, GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT,
		GLOBAL_STATE_HANDLER_COMPONENT, GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
	},
	sync::{EnclaveLock, EnclaveStateRWLock},
	utils::{
		get_extrinsic_factory_from_litentry_solo_or_parachain,
		get_stf_executor_from_solo_or_parachain, get_triggered_dispatcher_from_solo_or_parachain,
		get_validator_accessor_from_solo_or_parachain,
	},
};
use codec::Encode;
use itc_parentchain::{
	block_import_dispatcher::triggered_dispatcher::TriggerParentchainBlockImport,
	light_client::{
		concurrent_access::ValidatorAccess, BlockNumberOps, ExtrinsicSender, LightClientState,
		NumberFor,
	},
};
use itp_component_container::ComponentGetter;
use itp_enclave_metrics::EnclaveMetric;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi, EnclaveSidechainOCallApi};
use itp_settings::sidechain::SLOT_DURATION;
use itp_sgx_crypto::key_repository::AccessKey;
use itp_sgx_externalities::SgxExternalities;
use itp_stf_state_handler::{handle_state::HandleState, query_shard_state::QueryShardState};
use itp_time_utils::duration_now;
use itp_types::{Block, OpaqueCall, H256};
use itp_utils::if_not_production;
use its_primitives::{
	traits::{
		Block as SidechainBlockTrait, Header as HeaderTrait, ShardIdentifierFor, SignedBlock,
	},
	types::block::SignedBlock as SignedSidechainBlock,
};
use its_sidechain::{
	aura::{proposer_factory::ProposerFactory, Aura, SlotClaimStrategy},
	consensus_common::{Environment, Error as ConsensusError, ProcessBlockImportQueue},
	slots::{yield_next_slot, LastSlot, PerShardSlotWorkerScheduler, SlotInfo},
	validateer_fetch::ValidateerFetch,
};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use log::*;
use sgx_types::sgx_status_t;
use sp_core::{crypto::UncheckedFrom, Pair};
use sp_runtime::{
	generic::SignedBlock as SignedParentchainBlock, traits::Block as BlockTrait, MultiSignature,
};
use std::{sync::Arc, time::Instant, vec::Vec};

#[no_mangle]
pub unsafe extern "C" fn execute_trusted_calls() -> sgx_status_t {
	if let Err(e) = execute_top_pool_trusted_calls_internal() {
		return e.into()
	}

	sgx_status_t::SGX_SUCCESS
}

/// Internal [`execute_trusted_calls`] function to be able to use the `?` operator.
///
/// Executes `Aura::on_slot() for `slot` if it is this enclave's `Slot`.
///
/// This function makes an ocall that does the following:
///
/// *   Import all pending parentchain blocks.
/// *   Sends sidechain `confirm_block` xt's with the produced sidechain blocks.
/// *   Broadcast produced sidechain blocks to peer validateers.
fn execute_top_pool_trusted_calls_internal() -> Result<()> {
	let start_time = Instant::now();

	debug!("----------------------------------------");
	debug!("Start sidechain block production cycle");

	// We acquire lock explicitly (variable binding), since '_' will drop the lock after the statement.
	// See https://medium.com/codechain/rust-underscore-does-not-bind-fec6a18115a8
	let _enclave_write_lock = EnclaveLock::write_all()?;

	let slot_beginning_timestamp = duration_now();

	let parentchain_import_dispatcher = get_triggered_dispatcher_from_solo_or_parachain()?;

	let validator_access = get_validator_accessor_from_solo_or_parachain()?;

	// This gets the latest imported block. We accept that all of AURA, up until the block production
	// itself, will  operate on a parentchain block that is potentially outdated by one block
	// (in case we have a block in the queue, but not imported yet).
	let current_parentchain_header = validator_access.execute_on_validator(|v| {
		let latest_parentchain_header = v.latest_finalized_header()?;
		Ok(latest_parentchain_header)
	})?;

	// Import any sidechain blocks that are in the import queue. In case we are missing blocks,
	// a peer sync will happen. If that happens, the slot time might already be used up just by this import.
	let sidechain_block_import_queue_worker =
		GLOBAL_SIDECHAIN_IMPORT_QUEUE_WORKER_COMPONENT.get()?;

	let latest_parentchain_header =
		sidechain_block_import_queue_worker.process_queue(&current_parentchain_header)?;

	trace!(
		"Elapsed time to process sidechain block import queue: {} ms",
		start_time.elapsed().as_millis()
	);

	let stf_executor = get_stf_executor_from_solo_or_parachain()?;

	let top_pool_author = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;

	let block_composer = GLOBAL_SIDECHAIN_BLOCK_COMPOSER_COMPONENT.get()?;

	let extrinsics_factory = get_extrinsic_factory_from_litentry_solo_or_parachain()?;

	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;

	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;

	let authority = GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get()?.retrieve_key()?;

	let fail_on_demand = GLOBAL_SIDECHAIN_FAIL_SLOT_ON_DEMAND_COMPONENT.get()?;

	match yield_next_slot(
		slot_beginning_timestamp,
		SLOT_DURATION,
		latest_parentchain_header,
		&mut LastSlot,
	)? {
		Some(slot) => {
			if slot.duration_remaining().is_none() {
				warn!("No time remaining in slot, skipping AURA execution");
				return Ok(())
			}
			log_remaining_slot_duration(&slot, "Before AURA");

			let shards = state_handler.list_shards()?;
			let env = ProposerFactory::<Block, _, _, _, _>::new(
				top_pool_author,
				stf_executor,
				block_composer,
				ocall_api.clone(),
			);

			if_not_production!({
				if let Some(ref fail_on_demand) = *fail_on_demand {
					fail_on_demand.next_slot();
					if fail_on_demand.check_before_on_slot() {
						Result::Err(crate::error::Error::Sgx(sgx_status_t::SGX_ERROR_UNEXPECTED))?;
					}
				}
			});

			let (blocks, opaque_calls) =
				exec_aura_on_slot::<_, _, SignedSidechainBlock, _, _, _, _, _>(
					slot.clone(),
					authority,
					ocall_api.clone(),
					parentchain_import_dispatcher,
					env,
					shards,
					GLOBAL_SCHEDULED_ENCLAVE.clone(),
					state_handler,
				)?;

			if_not_production!({
				if let Some(ref fail_on_demand) = *fail_on_demand {
					if fail_on_demand.check_after_on_slot() {
						Result::Err(crate::error::Error::Sgx(sgx_status_t::SGX_ERROR_UNEXPECTED))?;
					}
				}
			});

			debug!("Aura executed successfully");

			// Drop lock as soon as we don't need it anymore.
			drop(_enclave_write_lock);

			log_remaining_slot_duration(&slot, "After AURA");

			send_blocks_and_extrinsics::<Block, _, _, _, _>(
				blocks,
				opaque_calls,
				ocall_api,
				validator_access.as_ref(),
				extrinsics_factory.as_ref(),
			)?;

			log_remaining_slot_duration(&slot, "After broadcasting and sending extrinsic");
		},
		None => {
			debug!("No slot yielded. Skipping block production.");
			return Ok(())
		},
	};

	debug!("End sidechain block production cycle");
	Ok(())
}

/// Executes aura for the given `slot`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn exec_aura_on_slot<
	Authority,
	ParentchainBlock,
	SignedSidechainBlock,
	OCallApi,
	PEnvironment,
	BlockImportTrigger,
	ScheduledEnclave,
	StateHandler,
>(
	slot: SlotInfo<ParentchainBlock>,
	authority: Authority,
	ocall_api: Arc<OCallApi>,
	block_import_trigger: Arc<BlockImportTrigger>,
	proposer_environment: PEnvironment,
	shards: Vec<ShardIdentifierFor<SignedSidechainBlock>>,
	scheduled_enclave: Arc<ScheduledEnclave>,
	state_handler: Arc<StateHandler>,
) -> Result<(Vec<SignedSidechainBlock>, Vec<OpaqueCall>)>
where
	ParentchainBlock: BlockTrait<Hash = H256>,
	SignedSidechainBlock:
		SignedBlock<Public = Authority::Public, Signature = MultiSignature> + 'static, // Setting the public type is necessary due to some non-generic downstream code.
	SignedSidechainBlock::Block: SidechainBlockTrait<Public = Authority::Public>,
	<<SignedSidechainBlock as SignedBlock>::Block as SidechainBlockTrait>::HeaderType:
		HeaderTrait<ShardIdentifier = H256>,
	SignedSidechainBlock::Signature: From<Authority::Signature>,
	Authority: Pair<Public = sp_core::ed25519::Public>,
	Authority::Public: Encode + UncheckedFrom<[u8; 32]>,
	OCallApi: ValidateerFetch + EnclaveOnChainOCallApi + EnclaveSidechainOCallApi + Send + 'static,
	NumberFor<ParentchainBlock>: BlockNumberOps,
	PEnvironment:
		Environment<ParentchainBlock, SignedSidechainBlock, Error = ConsensusError> + Send + Sync,
	BlockImportTrigger:
		TriggerParentchainBlockImport<SignedBlockType = SignedParentchainBlock<ParentchainBlock>>,
	ScheduledEnclave: ScheduledEnclaveUpdater,
	StateHandler: HandleState<StateT = SgxExternalities>,
{
	debug!("[Aura] Executing aura for slot: {:?}", slot);

	let mut aura =
		Aura::<_, ParentchainBlock, SignedSidechainBlock, PEnvironment, _, _, _, _>::new(
			authority,
			ocall_api.as_ref().clone(),
			block_import_trigger,
			proposer_environment,
			scheduled_enclave,
			state_handler,
		)
		.with_claim_strategy(SlotClaimStrategy::RoundRobin);

	let is_single_worker = match ocall_api.get_trusted_peers_urls() {
		Ok(urls) => urls.is_empty(),
		Err(e) => {
			warn!("Could not get trusted peers urls, error: {:?}", e);
			warn!("Falling back to non single worker mode");
			false
		},
	};

	let (blocks, xts): (Vec<_>, Vec<_>) =
		PerShardSlotWorkerScheduler::on_slot(&mut aura, slot, shards, is_single_worker)
			.into_iter()
			.map(|r| (r.block, r.parentchain_effects))
			.unzip();

	let opaque_calls: Vec<OpaqueCall> = xts.into_iter().flatten().collect();
	Ok((blocks, opaque_calls))
}

/// Broadcasts sidechain blocks to fellow peers and sends opaque calls as extrinsic to the parentchain.
pub(crate) fn send_blocks_and_extrinsics<
	ParentchainBlock,
	SignedSidechainBlock,
	OCallApi,
	ValidatorAccessor,
	ExtrinsicsFactory,
>(
	blocks: Vec<SignedSidechainBlock>,
	opaque_calls: Vec<OpaqueCall>,
	ocall_api: Arc<OCallApi>,
	validator_access: &ValidatorAccessor,
	extrinsics_factory: &ExtrinsicsFactory,
) -> Result<()>
where
	ParentchainBlock: BlockTrait,
	SignedSidechainBlock: SignedBlock + 'static,
	OCallApi: EnclaveSidechainOCallApi + EnclaveMetricsOCallApi,
	ValidatorAccessor: ValidatorAccess<ParentchainBlock> + Send + Sync + 'static,
	NumberFor<ParentchainBlock>: BlockNumberOps,
	ExtrinsicsFactory: CreateExtrinsics,
{
	let started = std::time::Instant::now();
	debug!("Proposing {} sidechain block(s) (broadcasting to peers)", blocks.len());
	ocall_api.propose_sidechain_blocks(blocks)?;
	if let Err(e) =
		ocall_api.update_metric(EnclaveMetric::SidechainBlockBroadcastingTime(started.elapsed()))
	{
		warn!("Failed to update metric for sidechain block broadcasting time: {:?}", e);
	};
	let xts = extrinsics_factory.create_extrinsics(opaque_calls.as_slice(), None)?;

	debug!("Sending sidechain block(s) confirmation extrinsic.. ");
	validator_access.execute_mut_on_validator(|v| v.send_extrinsics(xts))?;

	Ok(())
}

fn log_remaining_slot_duration<B: BlockTrait<Hash = H256>>(
	slot_info: &SlotInfo<B>,
	stage_name: &str,
) {
	match slot_info.duration_remaining() {
		None => {
			info!("No time remaining in slot (id: {:?}, stage: {})", slot_info.slot, stage_name);
		},
		Some(remainder) => {
			trace!(
				"Remaining time in slot (id: {:?}, stage {}): {} ms, {}% of slot time",
				slot_info.slot,
				stage_name,
				remainder.as_millis(),
				(remainder.as_millis() as f64 / slot_info.duration.as_millis() as f64) * 100f64
			);
		},
	};
}
