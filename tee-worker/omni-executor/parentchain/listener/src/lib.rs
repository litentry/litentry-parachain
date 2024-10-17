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
mod key_store;
mod listener;
mod metadata;
mod primitives;
mod rpc_client;

use crate::event_handler::IntentionEventHandler;
use crate::fetcher::Fetcher;
use crate::key_store::SubstrateKeyStore;
use crate::listener::ParentchainListener;
use crate::metadata::SubxtMetadataProvider;
use crate::rpc_client::{SubxtClient, SubxtClientFactory};
use executor_core::intention_executor::IntentionExecutor;
use executor_core::key_store::KeyStore;
use executor_core::listener::Listener;
use executor_core::sync_checkpoint_repository::FileCheckpointRepository;
use log::{error, info};
use scale_encode::EncodeAsType;
use subxt::config::signed_extensions;
use subxt::Config;
use subxt_core::utils::AccountId32;
use subxt_core::utils::MultiAddress::Address32;
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
	type Address = subxt::utils::MultiAddress<Self::AccountId, u32>;
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
pub async fn create_listener<EthereumIntentionExecutorT: IntentionExecutor + Send + Sync>(
	id: &str,
	handle: Handle,
	ws_rpc_endpoint: &str,
	ethereum_intention_executor: EthereumIntentionExecutorT,
	stop_signal: Receiver<()>,
) -> Result<
	ParentchainListener<
		SubxtClient<CustomConfig>,
		SubxtClientFactory<CustomConfig>,
		FileCheckpointRepository,
		CustomConfig,
		EthereumIntentionExecutorT,
	>,
	(),
> {
	let client_factory: SubxtClientFactory<CustomConfig> = SubxtClientFactory::new(ws_rpc_endpoint);

	let fetcher = Fetcher::new(client_factory);
	let last_processed_log_repository =
		FileCheckpointRepository::new("data/parentchain_last_log.bin");

	let metadata_provider = SubxtMetadataProvider::new(SubxtClientFactory::new(ws_rpc_endpoint));
	let key_store = SubstrateKeyStore::new("/data/parentchain_key.bin".to_string());
	let secret_key_bytes = key_store
		.read()
		.map_err(|e| {
			error!("Could not unseal key: {:?}", e);
		})
		.unwrap();
	let signer = subxt_signer::sr25519::Keypair::from_secret_key(secret_key_bytes)
		.map_err(|e| {
			error!("Could not create secret key: {:?}", e);
		})
		.unwrap();

	info!("Substrate signer address: {}", AccountId32::from(signer.public_key()));

	let intention_event_handler = IntentionEventHandler::new(
		metadata_provider,
		ethereum_intention_executor,
		key_store,
		SubxtClientFactory::new(ws_rpc_endpoint),
	);

	Listener::new(
		id,
		handle,
		fetcher,
		intention_event_handler,
		stop_signal,
		last_processed_log_repository,
	)
}
