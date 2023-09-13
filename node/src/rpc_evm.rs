// Copyright 2020-2023 Trust Computing GmbH.
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

#![warn(missing_docs)]

use std::sync::Arc;

use core_primitives::{AccountId, Balance, Block, Hash, Index as Nonce};
use fc_rpc::{
	Eth, EthApiServer, EthBlockDataCacheTask, EthFilter, EthFilterApiServer, EthPubSub,
	EthPubSubApiServer, Net, NetApiServer, OverrideHandle, Web3, Web3ApiServer,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use sc_client_api::{AuxStore, Backend, BlockchainEvents, StateBackend, StorageProvider};
use sc_network::NetworkService;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_runtime::traits::BlakeTwo256;

use moonbeam_rpc_debug::{Debug, DebugServer};
use moonbeam_rpc_trace::{Trace, TraceServer};
use moonbeam_rpc_txpool::{TxPool, TxPoolServer};
/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

use crate::tracing;

#[derive(Clone)]
pub struct EvmTracingConfig {
	pub tracing_requesters: tracing::RpcRequesters,
	pub trace_filter_max_count: u32,
	pub enable_txpool: bool,
}

// TODO This is copied from frontier. It should be imported instead after
// https://github.com/paritytech/frontier/issues/333 is solved
pub fn open_frontier_backend<C>(
	client: Arc<C>,
	config: &sc_service::Configuration,
) -> Result<Arc<fc_db::Backend<Block>>, String>
where
	C: sp_blockchain::HeaderBackend<Block>,
{
	let config_dir = config
		.base_path
		.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			sc_service::BasePath::from_project("", "", "astar").config_dir(config.chain_spec.id())
		});
	let path = config_dir.join("frontier").join("db");

	Ok(Arc::new(fc_db::Backend::<Block>::new(
		client,
		&fc_db::DatabaseSettings { source: fc_db::DatabaseSource::RocksDb { path, cache_size: 0 } },
	)?))
}

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
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Frontier Backend.
	pub frontier_backend: Arc<fc_db::Backend<Block>>,
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
pub fn create_full<C, P, BE, A>(
	deps: FullDeps<C, P, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
	tracing_config: EvmTracingConfig,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
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
	let FullDeps {
		client,
		pool,
		graph,
		network,
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

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

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
			network.clone(),
			Default::default(),
			overrides.clone(),
			frontier_backend.clone(),
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_limit,
			// Allow 10x max allowed weight for non-transactional calls
			10,
		)
		.into_rpc(),
	)?;

	let max_past_logs: u32 = 10_000;
	let max_stored_filters: usize = 500;
	module.merge(
		EthFilter::new(
			client.clone(),
			frontier_backend,
			filter_pool,
			max_stored_filters,
			max_past_logs,
			block_data_cache,
		)
		.into_rpc(),
	)?;

	module.merge(Net::new(client.clone(), network.clone(), true).into_rpc())?;

	module.merge(Web3::new(client.clone()).into_rpc())?;

	module.merge(
		EthPubSub::new(pool, client.clone(), network, subscription_task_executor, overrides)
			.into_rpc(),
	)?;

	if tracing_config.enable_txpool {
		module.merge(TxPool::new(Arc::clone(&client), graph).into_rpc())?;
	}

	if let Some(trace_filter_requester) = tracing_config.tracing_requesters.trace {
		module.merge(
			Trace::new(client, trace_filter_requester, tracing_config.trace_filter_max_count)
				.into_rpc(),
		)?;
	}

	if let Some(debug_requester) = tracing_config.tracing_requesters.debug {
		module.merge(Debug::new(debug_requester).into_rpc())?;
	}

	Ok(module)
}
