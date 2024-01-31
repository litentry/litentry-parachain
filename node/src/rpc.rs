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

// For maintaining a node code without implementing Frontier EVM.
// This File should be safe to delete once All parachain matrix are EVM impl.
#![warn(missing_docs)]

use core_primitives::reuse;

use std::sync::Arc;

use core_primitives::{AccountId, Balance, Block, Hash, Index as Nonce};
use fc_rpc::{
	Eth, EthApiServer, EthBlockDataCacheTask, EthFilter, EthFilterApiServer, EthPubSub,
	EthPubSubApiServer, Net, NetApiServer, OverrideHandle, TxPool, TxPoolApiServer, Web3,
	Web3ApiServer,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use moonbeam_rpc_debug::{Debug, DebugServer};
use moonbeam_rpc_trace::{Trace, TraceServer};
use moonbeam_rpc_txpool::{TxPool as MoonbeamTxPool, TxPoolServer};
use sc_client_api::{AuxStore, Backend, BlockchainEvents, StateBackend, StorageProvider};
use sc_network::NetworkService;
use sc_network_sync::SyncingService;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_runtime::traits::BlakeTwo256;

use crate::tracing;

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

#[reuse(evm, no_evm)]
pub mod __ {
	use super::*;

	#[evm]
	#[derive(Clone)]
	pub struct EvmTracingConfig {
		pub tracing_requesters: tracing::RpcRequesters,
		pub trace_filter_max_count: u32,
		pub enable_txpool: bool,
	}

	// TODO This is copied from frontier. It should be imported instead after
	// https://github.com/paritytech/frontier/issues/333 is solved
	#[evm]
	pub fn open_frontier_backend<C>(
		client: Arc<C>,
		config: &sc_service::Configuration,
	) -> Result<Arc<fc_db::kv::Backend<Block>>, String>
	where
		C: sp_blockchain::HeaderBackend<Block>,
	{
		let config_dir = config
			.base_path
			.as_ref()
			.map(|base_path| base_path.config_dir(config.chain_spec.id()))
			.unwrap_or_else(|| {
				sc_service::BasePath::from_project("", "", "litentry")
					.config_dir(config.chain_spec.id())
			});
		let path = config_dir.join("frontier").join("db");

		Ok(Arc::new(fc_db::kv::Backend::<Block>::new(
			client,
			&fc_db::kv::DatabaseSettings {
				source: fc_db::DatabaseSource::RocksDb { path, cache_size: 0 },
			},
		)?))
	}

	/// Full client dependencies
	#[no_evm]
	pub struct FullDeps<C, P> {
		/// The client instance to use.
		pub client: Arc<C>,
		/// Transaction pool instance.
		pub pool: Arc<P>,
		/// Whether to deny unsafe calls
		pub deny_unsafe: DenyUnsafe,
	}

	#[evm]
	/// Full client dependencies
	pub struct FullDeps<C, P, A: ChainApi> {
		/// The client instance to use.
		pub client: Arc<C>,
		/// Transaction pool instance.
		pub pool: Arc<P>,
		/// Graph pool instance.
		pub graph: Arc<Pool<A>>,
		/// Network service
		pub network: Arc<NetworkService<Block, Hash>>,
		/// Chain syncing service
		pub sync: Arc<SyncingService<Block>>,
		/// Whether to deny unsafe calls
		pub deny_unsafe: DenyUnsafe,
		/// The Node authority flag
		pub is_authority: bool,
		/// Frontier Backend.
		pub frontier_backend: Arc<dyn fc_db::BackendReader<Block> + Send + Sync>,
		/// EthFilterApi pool.
		pub filter_pool: FilterPool,
		/// Maximum fee history cache size.
		pub fee_history_limit: u64,
		/// Fee history cache.
		pub fee_history_cache: FeeHistoryCache,
		/// Ethereum data access overrides.
		pub overrides: Arc<OverrideHandle<Block>>,
		/// Cache for Ethereum block data.
		pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
		/// Enable EVM RPC servers
		pub enable_evm_rpc: bool,
	}

	/// Instantiate all RPC extensions.
	#[no_evm]
	pub fn create_full<C, P>(
		deps: FullDeps<C, P>,
	) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
	where
		C: ProvideRuntimeApi<Block>
			+ HeaderBackend<Block>
			+ AuxStore
			+ HeaderMetadata<Block, Error = BlockChainError>
			+ Send
			+ Sync
			+ 'static,
		C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
		C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
		C::Api: BlockBuilder<Block>,
		P: TransactionPool + Sync + Send + 'static,
	{
		__
	}

	/// Instantiate all RPC extensions.
	#[evm]
	pub fn create_full<C, P, BE, A>(
		deps: FullDeps<C, P, A>,
		subscription_task_executor: SubscriptionTaskExecutor,
		pubsub_notification_sinks: Arc<
			fc_mapping_sync::EthereumBlockNotificationSinks<
				fc_mapping_sync::EthereumBlockNotification<Block>,
			>,
		>,
		tracing_config: EvmTracingConfig,
	) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
	where
		C: ProvideRuntimeApi<Block>
			+ HeaderBackend<Block>
			+ CallApiAt<Block>
			+ AuxStore
			+ StorageProvider<Block, BE>
			+ HeaderMetadata<Block, Error = BlockChainError>
			+ BlockchainEvents<Block>
			+ Send
			+ Sync
			+ 'static,
		C: sc_client_api::BlockBackend<Block>,
		C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
			+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
			+ fp_rpc::ConvertTransactionRuntimeApi<Block>
			+ fp_rpc::EthereumRuntimeRPCApi<Block>
			+ BlockBuilder<Block>
			+ moonbeam_rpc_primitives_debug::DebugRuntimeApi<Block>
			+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>,
		P: TransactionPool<Block = Block> + Sync + Send + 'static,
		BE: Backend<Block> + 'static,
		BE::State: StateBackend<BlakeTwo256>,
		BE::Blockchain: BlockchainBackend<Block>,
		A: ChainApi<Block = Block> + 'static,
	{
		use frame_rpc_system::{System, SystemApiServer};
		use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};

		let mut module = RpcExtension::new(());
		#[no_evm]
		let FullDeps { client, pool, deny_unsafe } = deps;
		#[evm]
		let FullDeps {
			client,
			pool,
			graph,
			network,
			sync,
			deny_unsafe,
			is_authority,
			frontier_backend,
			filter_pool,
			fee_history_limit,
			fee_history_cache,
			overrides,
			block_data_cache,
			enable_evm_rpc,
		} = deps;

		#[evm]
		let cloned = (client.clone(), pool.clone());
		module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
		module.merge(TransactionPayment::new(client).into_rpc())?;

		#[evm]
		{
			let (client, pool) = cloned;
			if !enable_evm_rpc {
				return Ok(module)
			}

			let no_tx_converter: Option<fp_rpc::NoTransactionConverter> = None;

			module.merge(
				Eth::new(
					client.clone(),
					pool.clone(),
					graph.clone(),
					no_tx_converter,
					sync.clone(),
					Default::default(),
					overrides.clone(),
					frontier_backend.clone(),
					is_authority,
					block_data_cache.clone(),
					fee_history_cache,
					fee_history_limit,
					// Allow 10x max allowed weight for non-transactional calls
					10,
					None,
				)
				.into_rpc(),
			)?;

			let max_past_logs: u32 = 10_000;
			let max_stored_filters: usize = 500;
			let tx_pool = TxPool::new(client.clone(), graph.clone());
			module.merge(
				EthFilter::new(
					client.clone(),
					frontier_backend,
					tx_pool.clone(),
					filter_pool,
					max_stored_filters,
					max_past_logs,
					block_data_cache,
				)
				.into_rpc(),
			)?;

			module.merge(Net::new(client.clone(), network, true).into_rpc())?;

			module.merge(Web3::new(client.clone()).into_rpc())?;

			module.merge(
				EthPubSub::new(
					pool,
					client.clone(),
					sync,
					subscription_task_executor,
					overrides,
					pubsub_notification_sinks,
				)
				.into_rpc(),
			)?;

			module.merge(tx_pool.into_rpc())?;

			if tracing_config.enable_txpool {
				module.merge(MoonbeamTxPool::new(Arc::clone(&client), graph).into_rpc())?;
			}

			if let Some(trace_filter_requester) = tracing_config.tracing_requesters.trace {
				module.merge(
					Trace::new(
						client,
						trace_filter_requester,
						tracing_config.trace_filter_max_count,
					)
					.into_rpc(),
				)?;
			}

			if let Some(debug_requester) = tracing_config.tracing_requesters.debug {
				module.merge(Debug::new(debug_requester).into_rpc())?;
			}
		}

		Ok(module)
	}
}
