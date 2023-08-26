// Copyright 2020-2023 Litentry Technologies GmbH.
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

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::tracing;

use cumulus_client_cli::CollatorOptions;
use cumulus_client_consensus_aura::{AuraConsensus, BuildAuraConsensusParams, SlotProportion};
use cumulus_client_consensus_common::{
	ParachainBlockImport as TParachainBlockImport, ParachainConsensus,
};
use cumulus_client_network::BlockAnnounceValidator;
use cumulus_client_service::{
	prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::ParaId;
use cumulus_primitives_parachain_inherent::{
	MockValidationDataInherentDataProvider, MockXcmConfig,
};
use cumulus_relay_chain_inprocess_interface::build_inprocess_relay_chain;
use cumulus_relay_chain_interface::{RelayChainError, RelayChainInterface, RelayChainResult};
use cumulus_relay_chain_minimal_node::build_minimal_relay_chain_node;

use futures::StreamExt;

use fc_consensus::FrontierBlockImport;
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use jsonrpsee::RpcModule;
use polkadot_service::CollatorPair;

use crate::{
	evm_tracing_types::{EthApi as EthApiCmd, EvmTracingConfig},
	rpc,
	standalone_block_import::StandaloneBlockImport,
	tracing::RpcRequesters,
};

pub use core_primitives::{AccountId, Balance, Block, Hash, Header, Index as Nonce};

use sc_client_api::BlockchainEvents;
use sc_consensus::{ImportQueue, LongestChain};
use sc_consensus_aura::StartAuraParams;
use sc_executor::NativeElseWasmExecutor;
use sc_network::NetworkService;
use sc_network_common::service::NetworkBlock;
use sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::ConstructRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::BlakeTwo256;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use substrate_prometheus_endpoint::Registry;

use fc_rpc::{EthBlockDataCacheTask, OverrideHandle};

#[cfg(not(feature = "runtime-benchmarks"))]
type HostFunctions =
	(sp_io::SubstrateHostFunctions, moonbeam_primitives_ext::moonbeam_ext::HostFunctions);

#[cfg(feature = "runtime-benchmarks")]
type HostFunctions = (
	sp_io::SubstrateHostFunctions,
	frame_benchmarking::benchmarking::HostFunctions,
	moonbeam_primitives_ext::moonbeam_ext::HostFunctions,
);

// Native executor instance.
pub struct LitentryParachainRuntimeExecutor;

impl sc_executor::NativeExecutionDispatch for LitentryParachainRuntimeExecutor {
	type ExtendHostFunctions = HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		litentry_parachain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		litentry_parachain_runtime::native_version()
	}
}

// Native executor instance.
pub struct LitmusParachainRuntimeExecutor;

impl sc_executor::NativeExecutionDispatch for LitmusParachainRuntimeExecutor {
	type ExtendHostFunctions = HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		litmus_parachain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		litmus_parachain_runtime::native_version()
	}
}

// Native executor instance.
pub struct RococoParachainRuntimeExecutor;

impl sc_executor::NativeExecutionDispatch for RococoParachainRuntimeExecutor {
	type ExtendHostFunctions = HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		rococo_parachain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		rococo_parachain_runtime::native_version()
	}
}

type ParachainClient<RuntimeApi, Executor> =
	TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;

type ParachainBackend = TFullBackend<Block>;

type MaybeSelectChain = Option<LongestChain<ParachainBackend, Block>>;

type ParachainBlockImport<RuntimeApi, Executor> = TParachainBlockImport<
	Block,
	FrontierBlockImport<
		Block,
		Arc<ParachainClient<RuntimeApi, Executor>>,
		ParachainClient<RuntimeApi, Executor>,
	>,
	ParachainBackend,
>;

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
pub fn new_partial<RuntimeApi, Executor, BIQ>(
	config: &Configuration,
	is_standalone: bool,
	build_import_queue: BIQ,
) -> Result<
	PartialComponents<
		ParachainClient<RuntimeApi, Executor>,
		ParachainBackend,
		MaybeSelectChain,
		sc_consensus::DefaultImportQueue<Block, ParachainClient<RuntimeApi, Executor>>,
		sc_transaction_pool::FullPool<Block, ParachainClient<RuntimeApi, Executor>>,
		(
			ParachainBlockImport<RuntimeApi, Executor>,
			Option<Telemetry>,
			Option<TelemetryWorkerHandle>,
			Arc<fc_db::Backend<Block>>,
		),
	>,
	sc_service::Error,
>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>,
	sc_client_api::StateBackendFor<ParachainBackend, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	BIQ: FnOnce(
		Arc<ParachainClient<RuntimeApi, Executor>>,
		ParachainBlockImport<RuntimeApi, Executor>,
		&Configuration,
		Option<TelemetryHandle>,
		&TaskManager,
		bool,
	) -> Result<
		sc_consensus::DefaultImportQueue<Block, ParachainClient<RuntimeApi, Executor>>,
		sc_service::Error,
	>,
{
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = sc_executor::NativeElseWasmExecutor::<Executor>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let select_chain = if is_standalone { Some(LongestChain::new(backend.clone())) } else { None };

	let frontier_backend = crate::rpc::open_frontier_backend(client.clone(), config)?;
	let frontier_block_import =
		FrontierBlockImport::new(client.clone(), client.clone(), frontier_backend.clone());

	let block_import = ParachainBlockImport::new(frontier_block_import, backend.clone());

	let import_queue = build_import_queue(
		client.clone(),
		block_import.clone(),
		config,
		telemetry.as_ref().map(|telemetry| telemetry.handle()),
		&task_manager,
		is_standalone,
	)?;

	let params = PartialComponents {
		backend,
		client,
		import_queue,
		keystore_container,
		task_manager,
		transaction_pool,
		select_chain,
		other: (block_import, telemetry, telemetry_worker_handle, frontier_backend),
	};

	Ok(params)
}

pub async fn build_relay_chain_interface(
	polkadot_config: Configuration,
	parachain_config: &Configuration,
	telemetry_worker_handle: Option<TelemetryWorkerHandle>,
	task_manager: &mut TaskManager,
	collator_options: CollatorOptions,
	hwbench: Option<sc_sysinfo::HwBench>,
) -> RelayChainResult<(Arc<(dyn RelayChainInterface + 'static)>, Option<CollatorPair>)> {
	if !collator_options.relay_chain_rpc_urls.is_empty() {
		build_minimal_relay_chain_node(
			polkadot_config,
			task_manager,
			collator_options.relay_chain_rpc_urls,
		)
		.await
	} else {
		build_inprocess_relay_chain(
			polkadot_config,
			parachain_config,
			telemetry_worker_handle,
			task_manager,
			hwbench,
		)
	}
}

#[derive(Clone)]
/// To add additional config to start_xyz_node functions
pub struct AdditionalConfig {
	/// EVM tracing configuration
	pub evm_tracing_config: EvmTracingConfig,

	/// Whether EVM RPC be enabled
	pub enable_evm_rpc: bool,

	/// Maxium allowed block size limit to propose
	pub proposer_block_size_limit: usize,

	/// Soft deadline limit used by `Proposer`
	pub proposer_soft_deadline_percent: u8,
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor, RB, BIQ, BIC>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	collator_options: CollatorOptions,
	id: ParaId,
	additional_config: AdditionalConfig,
	_rpc_ext_builder: RB,
	build_import_queue: BIQ,
	build_consensus: BIC,
	hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient<RuntimeApi, Executor>>)>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	sc_client_api::StateBackendFor<ParachainBackend, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	RB: Fn(
			Arc<ParachainClient<RuntimeApi, Executor>>,
		) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>
		+ Send
		+ 'static,
	BIQ: FnOnce(
			Arc<ParachainClient<RuntimeApi, Executor>>,
			ParachainBlockImport<RuntimeApi, Executor>,
			&Configuration,
			Option<TelemetryHandle>,
			&TaskManager,
			bool,
		) -> Result<
			sc_consensus::DefaultImportQueue<Block, ParachainClient<RuntimeApi, Executor>>,
			sc_service::Error,
		> + 'static,
	BIC: FnOnce(
		Arc<ParachainClient<RuntimeApi, Executor>>,
		ParachainBlockImport<RuntimeApi, Executor>,
		Option<&Registry>,
		Option<TelemetryHandle>,
		&TaskManager,
		Arc<dyn RelayChainInterface>,
		Arc<sc_transaction_pool::FullPool<Block, ParachainClient<RuntimeApi, Executor>>>,
		Arc<NetworkService<Block, Hash>>,
		SyncCryptoStorePtr,
		bool,
	) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error>,
{
	let parachain_config = prepare_node_config(parachain_config);

	let params =
		new_partial::<RuntimeApi, Executor, BIQ>(&parachain_config, false, build_import_queue)?;
	let (block_import, mut telemetry, telemetry_worker_handle, frontier_backend) = params.other;

	let client = params.client.clone();
	let backend = params.backend.clone();

	let mut task_manager = params.task_manager;
	let (relay_chain_interface, collator_key) = build_relay_chain_interface(
		polkadot_config,
		&parachain_config,
		telemetry_worker_handle,
		&mut task_manager,
		collator_options.clone(),
		hwbench.clone(),
	)
	.await
	.map_err(|e| match e {
		RelayChainError::ServiceError(polkadot_service::Error::Sub(x)) => x,
		s => s.to_string().into(),
	})?;

	let block_announce_validator = BlockAnnounceValidator::new(relay_chain_interface.clone(), id);

	let force_authoring = parachain_config.force_authoring;
	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let transaction_pool = params.transaction_pool.clone();
	let import_queue_service = params.import_queue.service();
	let (network, system_rpc_tx, tx_handler_controller, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &parachain_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: params.import_queue,
			block_announce_validator_builder: Some(Box::new(|_| {
				Box::new(block_announce_validator)
			})),
			warp_sync_params: None,
		})?;

	let (
		filter_pool,
		fee_history_limit,
		fee_history_cache,
		block_data_cache,
		overrides,
		tracing_requesters,
		ethapi_cmd,
	) = start_node_evm_impl::<RuntimeApi, Executor>(
		client.clone(),
		backend.clone(),
		frontier_backend.clone(),
		&mut task_manager,
		&parachain_config,
		additional_config.evm_tracing_config.clone(),
	);

	let rpc_builder = {
		let client = client.clone();
		let network = network.clone();
		let transaction_pool = transaction_pool.clone();
		let rpc_config = rpc::EvmTracingConfig {
			tracing_requesters,
			trace_filter_max_count: additional_config.evm_tracing_config.ethapi_trace_max_count,
			enable_txpool: ethapi_cmd.contains(&EthApiCmd::TxPool),
		};

		Box::new(move |deny_unsafe, subscription| {
			let deps = rpc::FullDeps {
				client: client.clone(),
				pool: transaction_pool.clone(),
				graph: transaction_pool.pool().clone(),
				network: network.clone(),
				is_authority: validator,
				deny_unsafe,
				frontier_backend: frontier_backend.clone(),
				filter_pool: filter_pool.clone(),
				fee_history_limit,
				fee_history_cache: fee_history_cache.clone(),
				block_data_cache: block_data_cache.clone(),
				overrides: overrides.clone(),
				enable_evm_rpc: additional_config.enable_evm_rpc,
			};

			crate::rpc::create_full(deps, subscription, rpc_config.clone()).map_err(Into::into)
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: params.keystore_container.sync_keystore(),
		backend: backend.clone(),
		network: network.clone(),
		system_rpc_tx,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
	})?;

	if let Some(hwbench) = hwbench {
		sc_sysinfo::print_hwbench(&hwbench);

		if let Some(ref mut telemetry) = telemetry {
			let telemetry_handle = telemetry.handle();
			task_manager.spawn_handle().spawn(
				"telemetry_hwbench",
				None,
				sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
			);
		}
	}

	let announce_block = {
		let network = network.clone();
		Arc::new(move |hash, data| network.announce_block(hash, data))
	};

	let relay_chain_slot_duration = Duration::from_secs(6);

	let overseer_handle = relay_chain_interface
		.overseer_handle()
		.map_err(|e| sc_service::Error::Application(Box::new(e)))?;

	if validator {
		let parachain_consensus = build_consensus(
			client.clone(),
			block_import,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
			&task_manager,
			relay_chain_interface.clone(),
			transaction_pool,
			network,
			params.keystore_container.sync_keystore(),
			force_authoring,
		)?;

		let spawner = task_manager.spawn_handle();

		let params = StartCollatorParams {
			para_id: id,
			block_status: client.clone(),
			announce_block,
			client: client.clone(),
			task_manager: &mut task_manager,
			relay_chain_interface: relay_chain_interface.clone(),
			spawner,
			parachain_consensus,
			import_queue: import_queue_service,
			collator_key: collator_key.expect("Command line arguments do not allow this. qed"),
			relay_chain_slot_duration,
			recovery_handle: Box::new(overseer_handle),
		};

		start_collator(params).await?;
	} else {
		let params = StartFullNodeParams {
			client: client.clone(),
			announce_block,
			task_manager: &mut task_manager,
			para_id: id,
			relay_chain_interface,
			relay_chain_slot_duration,
			import_queue: import_queue_service,
			recovery_handle: Box::new(overseer_handle),
		};

		start_full_node(params)?;
	}

	start_network.start_network();

	Ok((task_manager, client))
}

/// Start a litmus/litentry/rococo node.
pub async fn start_node<RuntimeApi, Executor>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	collator_options: CollatorOptions,
	id: ParaId,
	additional_config: AdditionalConfig,
	hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient<RuntimeApi, Executor>>)>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ sp_consensus_aura::AuraApi<Block, AuraId>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	sc_client_api::StateBackendFor<ParachainBackend, Block>: sp_api::StateBackend<BlakeTwo256>,
{
	start_node_impl::<RuntimeApi, Executor, _, _, _>(
		parachain_config,
		polkadot_config,
		collator_options,
		id,
		additional_config.clone(),
		|_| Ok(RpcModule::new(())),
		build_import_queue::<RuntimeApi, Executor>,
		|client,
		 block_import,
		 prometheus_registry,
		 telemetry,
		 task_manager,
		 relay_chain_interface,
		 transaction_pool,
		 sync_oracle,
		 keystore,
		 force_authoring| {
			let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

			let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool,
				prometheus_registry,
				telemetry.clone(),
			);
			Ok(AuraConsensus::build::<sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _, _>(
				BuildAuraConsensusParams {
					proposer_factory,
					create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
						let relay_chain_interface = relay_chain_interface.clone();

						async move {
							let parachain_inherent =
							cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
								relay_parent,
								&relay_chain_interface,
								&validation_data,
								id,
							).await;
							let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

							let slot =
							sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
								*timestamp,
								slot_duration,
							);

							let parachain_inherent = parachain_inherent.ok_or_else(|| {
								Box::<dyn std::error::Error + Send + Sync>::from(
									"Failed to create parachain inherent",
								)
							})?;

							Ok((slot, timestamp, parachain_inherent))
						}
					},
					block_import,
					para_client: client,
					backoff_authoring_blocks: Option::<()>::None,
					sync_oracle,
					keystore,
					force_authoring,
					slot_duration,
					// We got around 500ms for proposing
					block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
					// And a maximum of 750ms if slots are skipped
					max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
					telemetry,
				},
			))
		},
		hwbench,
	)
	.await
}

/// Build the import queue for the litmus/litentry/rococo runtime.
pub fn build_import_queue<RuntimeApi, Executor>(
	client: Arc<ParachainClient<RuntimeApi, Executor>>,
	block_import: ParachainBlockImport<RuntimeApi, Executor>,
	config: &Configuration,
	telemetry: Option<TelemetryHandle>,
	task_manager: &TaskManager,
	is_standalone: bool,
) -> Result<
	sc_consensus::DefaultImportQueue<Block, ParachainClient<RuntimeApi, Executor>>,
	sc_service::Error,
>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ sp_consensus_aura::AuraApi<Block, AuraId>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>,
	sc_client_api::StateBackendFor<ParachainBackend, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	if is_standalone {
		// aura import queue
		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let client_for_cidp = client.clone();

		sc_consensus_aura::import_queue::<sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _>(
			sc_consensus_aura::ImportQueueParams {
				block_import,
				justification_import: None,
				client,
				create_inherent_data_providers: move |block: Hash, ()| {
					let current_para_block = client_for_cidp
						.number(block)
						.expect("Header lookup should succeed")
						.expect("Header passed in as parent should be present in backend.");
					let client_for_xcm = client_for_cidp.clone();

					async move {
						let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

						let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
								*timestamp,
								slot_duration,
							);

						let mocked_parachain = MockValidationDataInherentDataProvider {
							current_para_block,
							relay_offset: 1000,
							relay_blocks_per_para_block: 2,
							para_blocks_per_relay_epoch: 0,
							relay_randomness_config: (),
							xcm_config: MockXcmConfig::new(
								&*client_for_xcm,
								block,
								Default::default(),
								Default::default(),
							),
							raw_downward_messages: vec![],
							raw_horizontal_messages: vec![],
						};

						Ok((slot, timestamp, mocked_parachain))
					}
				},
				spawner: &task_manager.spawn_essential_handle(),
				registry: config.prometheus_registry(),
				check_for_equivocation: Default::default(),
				telemetry,
				compatibility_mode: Default::default(),
			},
		)
		.map_err(Into::into)
	} else {
		let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

		cumulus_client_consensus_aura::import_queue::<
			sp_consensus_aura::sr25519::AuthorityPair,
			_,
			_,
			_,
			_,
			_,
		>(cumulus_client_consensus_aura::ImportQueueParams {
			block_import,
			client,
			create_inherent_data_providers: move |_, _| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
					sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						*timestamp,
						slot_duration,
					);

				Ok((slot, timestamp))
			},
			registry: config.prometheus_registry(),
			spawner: &task_manager.spawn_essential_handle(),
			telemetry,
		})
		.map_err(Into::into)
	}
}

// start a standalone node which doesn't need to connect to relaychain
pub async fn start_standalone_node<RuntimeApi, Executor>(
	config: Configuration,
	evm_tracing_config: crate::evm_tracing_types::EvmTracingConfig,
) -> Result<TaskManager, sc_service::Error>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ sp_consensus_aura::AuraApi<Block, AuraId>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	sc_client_api::StateBackendFor<ParachainBackend, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain: maybe_select_chain,
		transaction_pool,
		other: (_, _, _, frontier_backend),
	} = new_partial::<RuntimeApi, Executor, _>(
		&config,
		true,
		build_import_queue::<RuntimeApi, Executor>,
	)?;

	let (network, system_rpc_tx, tx_handler_controller, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
		})?;

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;

	let (
		filter_pool,
		fee_history_limit,
		fee_history_cache,
		block_data_cache,
		overrides,
		tracing_requesters,
		ethapi_cmd,
	) = start_node_evm_impl::<RuntimeApi, Executor>(
		client.clone(),
		backend.clone(),
		frontier_backend.clone(),
		&mut task_manager,
		&config,
		evm_tracing_config.clone(),
	);

	let select_chain = maybe_select_chain
		.expect("In `standalone` mode, `new_partial` will return some `select_chain`; qed");

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			None,
			None,
		);
		// aura
		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let client_for_cidp = client.clone();

		let aura = sc_consensus_aura::start_aura::<
			sp_consensus_aura::sr25519::AuthorityPair,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
		>(StartAuraParams {
			slot_duration: sc_consensus_aura::slot_duration(&*client)?,
			client: client.clone(),
			select_chain,
			block_import: StandaloneBlockImport::new(client.clone()),
			proposer_factory,
			create_inherent_data_providers: move |block: Hash, ()| {
				let current_para_block = client_for_cidp
					.number(block)
					.expect("Header lookup should succeed")
					.expect("Header passed in as parent should be present in backend.");
				let client_for_xcm = client_for_cidp.clone();

				async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

					let mocked_parachain = MockValidationDataInherentDataProvider {
						current_para_block,
						relay_offset: 1000,
						relay_blocks_per_para_block: 2,
						para_blocks_per_relay_epoch: 0,
						relay_randomness_config: (),
						xcm_config: MockXcmConfig::new(
							&*client_for_xcm,
							block,
							Default::default(),
							Default::default(),
						),
						raw_downward_messages: vec![],
						raw_horizontal_messages: vec![],
					};

					Ok((slot, timestamp, mocked_parachain))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			keystore: keystore_container.sync_keystore(),
			sync_oracle: network.clone(),
			justification_sync_link: network.clone(),
			// We got around 500ms for proposing
			block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
			// And a maximum of 750ms if slots are skipped
			max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
			telemetry: None,
			compatibility_mode: Default::default(),
		})?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("aura", Some("block-authoring"), aura);
	}

	let rpc_builder = {
		let client = client.clone();
		let network = network.clone();
		let transaction_pool = transaction_pool.clone();
		let rpc_config = rpc::EvmTracingConfig {
			tracing_requesters,
			trace_filter_max_count: evm_tracing_config.ethapi_trace_max_count,
			enable_txpool: ethapi_cmd.contains(&EthApiCmd::TxPool),
		};

		Box::new(move |deny_unsafe, subscription| {
			let deps = rpc::FullDeps {
				client: client.clone(),
				pool: transaction_pool.clone(),
				graph: transaction_pool.pool().clone(),
				network: network.clone(),
				is_authority: role.is_authority(),
				deny_unsafe,
				frontier_backend: frontier_backend.clone(),
				filter_pool: filter_pool.clone(),
				fee_history_limit,
				fee_history_cache: fee_history_cache.clone(),
				block_data_cache: block_data_cache.clone(),
				overrides: overrides.clone(),
				// enable EVM RPC for dev node by default
				enable_evm_rpc: true,
			};

			crate::rpc::create_full(deps, subscription, rpc_config.clone()).map_err(Into::into)
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client,
		transaction_pool,
		task_manager: &mut task_manager,
		config,
		keystore: keystore_container.sync_keystore(),
		backend,
		network,
		system_rpc_tx,
		tx_handler_controller,
		telemetry: None,
	})?;

	start_network.start_network();

	Ok(task_manager)
}

pub fn start_node_evm_impl<RuntimeApi, Executor>(
	client: Arc<ParachainClient<RuntimeApi, Executor>>,
	backend: Arc<ParachainBackend>,
	frontier_backend: Arc<fc_db::Backend<Block>>,
	task_manager: &mut TaskManager,
	config: &Configuration,
	evm_tracing_config: crate::evm_tracing_types::EvmTracingConfig,
) -> (
	FilterPool,
	u64,
	FeeHistoryCache,
	Arc<EthBlockDataCacheTask<Block>>,
	Arc<OverrideHandle<Block>>,
	RpcRequesters,
	Vec<EthApiCmd>,
)
where
	RuntimeApi:
		ConstructRuntimeApi<Block, ParachainClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<ParachainBackend, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
		+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	let prometheus_registry = config.prometheus_registry().cloned();
	let filter_pool: FilterPool = Arc::new(std::sync::Mutex::new(BTreeMap::new()));
	let fee_history_cache: FeeHistoryCache = Arc::new(std::sync::Mutex::new(BTreeMap::new()));
	let overrides = fc_storage::overrides_handle(client.clone());

	// TODO: Not for 0.9.39
	// // Sinks for pubsub notifications.
	// // Everytime a new subscription is created, a new mpsc channel is added to the sink pool.
	// // The MappingSyncWorker sends through the channel on block import and the subscription emits
	// a notification to the subscriber on receiving a message through this channel. // This way we
	// avoid race conditions when using native substrate block import notification stream.
	// let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
	//     fc_mapping_sync::EthereumBlockNotification<Block>,
	// > = Default::default();
	// let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

	let ethapi_cmd = evm_tracing_config.ethapi.clone();
	let tracing_requesters =
		if ethapi_cmd.contains(&EthApiCmd::Debug) || ethapi_cmd.contains(&EthApiCmd::Trace) {
			tracing::spawn_tracing_tasks(
				&evm_tracing_config,
				tracing::SpawnTasksParams {
					task_manager,
					client: client.clone(),
					substrate_backend: backend.clone(),
					frontier_backend: frontier_backend.clone(),
					filter_pool: Some(filter_pool.clone()),
					overrides: overrides.clone(),
				},
			)
		} else {
			tracing::RpcRequesters { debug: None, trace: None }
		};

	// Frontier offchain DB task. Essential.
	// Maps emulated ethereum data to substrate native data.
	task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		Some("frontier"),
		fc_mapping_sync::MappingSyncWorker::new(
			client.import_notification_stream(),
			Duration::new(6, 0),
			client.clone(),
			backend,
			overrides.clone(),
			frontier_backend,
			3,
			0,
			fc_mapping_sync::SyncStrategy::Parachain,
		)
		.for_each(|()| futures::future::ready(())),
	);

	// Frontier `EthFilterApi` maintenance. Manages the pool of user-created Filters.
	// Each filter is allowed to stay in the pool for 100 blocks.
	const FILTER_RETAIN_THRESHOLD: u64 = 100;
	task_manager.spawn_essential_handle().spawn(
		"frontier-filter-pool",
		Some("frontier"),
		fc_rpc::EthTask::filter_pool_task(
			client.clone(),
			filter_pool.clone(),
			FILTER_RETAIN_THRESHOLD,
		),
	);

	const FEE_HISTORY_LIMIT: u64 = 2048;
	task_manager.spawn_essential_handle().spawn(
		"frontier-fee-history",
		Some("frontier"),
		fc_rpc::EthTask::fee_history_task(
			client,
			overrides.clone(),
			fee_history_cache.clone(),
			FEE_HISTORY_LIMIT,
		),
	);

	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		50,
		50,
		prometheus_registry,
	));

	(
		filter_pool,
		FEE_HISTORY_LIMIT,
		fee_history_cache,
		block_data_cache,
		overrides,
		tracing_requesters,
		ethapi_cmd,
	)
}
