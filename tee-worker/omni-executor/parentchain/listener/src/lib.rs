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

mod event_handler;
mod fetcher;
mod listener;
mod metadata;
mod primitives;
mod rpc_client;

use crate::event_handler::IntentionEventHandler;
use crate::fetcher::Fetcher;
use crate::listener::ParentchainListener;
use crate::metadata::SubxtMetadataProvider;
use crate::rpc_client::{SubxtClient, SubxtClientFactory};
use executor_core::intention_executor::IntentionExecutor;
use executor_core::listener::Listener;
use executor_core::sync_checkpoint_repository::FileCheckpointRepository;
use scale_encode::EncodeAsType;
use subxt::config::signed_extensions;
use subxt::Config;
use tokio::runtime::Handle;
use tokio::sync::oneshot::Receiver;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/rococo-omni-account.scale")]
pub mod litentry_rococo {}

// We don't need to construct this at runtime,
// so an empty enum is appropriate:
#[derive(EncodeAsType)]
pub enum CustomConfig {}

//todo: adjust if needed
impl Config for CustomConfig {
	type Hash = subxt::utils::H256;
	type AccountId = subxt::utils::AccountId32;
	type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
	type Signature = subxt::utils::MultiSignature;
	type Hasher = subxt::config::substrate::BlakeTwo256;
	type Header = subxt::config::substrate::SubstrateHeader<u32, Self::Hasher>;
	type ExtrinsicParams = signed_extensions::AnyOf<
		Self,
		(
			// Load in the existing signed extensions we're interested in
			// (if the extension isn't actually needed it'll just be ignored):
			signed_extensions::CheckSpecVersion,
			signed_extensions::CheckTxVersion,
			signed_extensions::CheckNonce,
			signed_extensions::CheckGenesis<Self>,
			signed_extensions::CheckMortality<Self>,
			signed_extensions::ChargeAssetTxPayment<Self>,
			signed_extensions::ChargeTransactionPayment,
			signed_extensions::CheckMetadataHash,
		),
	>;
	type AssetId = u32;
}

/// Creates parentchain listener
pub async fn create_listener<
	ChainConfig: Config,
	EthereumIntentionExecutorT: IntentionExecutor + Send + Sync,
>(
	id: &str,
	handle: Handle,
	ws_rpc_endpoint: &str,
	ethereum_intention_executor: EthereumIntentionExecutorT,
	stop_signal: Receiver<()>,
) -> Result<
	ParentchainListener<
		SubxtClient<ChainConfig>,
		SubxtClientFactory<ChainConfig>,
		FileCheckpointRepository,
		ChainConfig,
		EthereumIntentionExecutorT,
	>,
	(),
> {
	let client_factory: SubxtClientFactory<ChainConfig> = SubxtClientFactory::new(ws_rpc_endpoint);

	let fetcher = Fetcher::new(client_factory);
	let last_processed_log_repository =
		FileCheckpointRepository::new("data/parentchain_last_log.bin");

	let metadata_provider = SubxtMetadataProvider::new(SubxtClientFactory::new(ws_rpc_endpoint));
	let intention_event_handler =
		IntentionEventHandler::new(metadata_provider, ethereum_intention_executor);

	Listener::new(
		id,
		handle,
		fetcher,
		intention_event_handler,
		stop_signal,
		last_processed_log_repository,
	)
}
