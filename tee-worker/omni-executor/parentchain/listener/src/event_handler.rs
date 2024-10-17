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

use crate::metadata::{MetadataProvider, SubxtMetadataProvider};
use crate::primitives::BlockEvent;
use crate::rpc_client::{SubstrateRpcClient, SubstrateRpcClientFactory};
use async_trait::async_trait;
use executor_core::event_handler::Error::RecoverableError;
use executor_core::event_handler::{Error, EventHandler};
use executor_core::intention_executor::IntentionExecutor;
use executor_core::key_store::KeyStore;
use executor_core::primitives::Intention;
use log::error;
use parity_scale_codec::Decode;
use std::marker::PhantomData;
use std::sync::RwLock;
use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::ext::scale_decode;
use subxt::ext::scale_decode::DecodeAsFields;
use subxt::ext::subxt_core::{metadata, tx};
use subxt::{Config, Metadata};
use subxt_core::config::DefaultExtrinsicParams;
use subxt_core::utils::{AccountId32, MultiAddress, MultiSignature};
use subxt_signer::sr25519::SecretKeyBytes;

pub struct IntentionEventHandler<
	MetadataT,
	MetadataProviderT: MetadataProvider<MetadataT>,
	EthereumIntentionExecutorT: IntentionExecutor,
	KeyStoreT: KeyStore<SecretKeyBytes>,
	RpcClient: SubstrateRpcClient,
	RpcClientFactory: SubstrateRpcClientFactory<RpcClient>,
> {
	metadata_provider: MetadataProviderT,
	ethereum_intention_executor: EthereumIntentionExecutorT,
	key_store: KeyStoreT,
	rpc_client_factory: RpcClientFactory,
	nonce: RwLock<u64>,
	phantom_data: PhantomData<(MetadataT, RpcClient)>,
}

impl<
		MetadataT,
		MetadataProviderT: MetadataProvider<MetadataT>,
		EthereumIntentionExecutorT: IntentionExecutor,
		KeyStoreT: KeyStore<SecretKeyBytes>,
		RpcClient: SubstrateRpcClient,
		RpcClientFactory: SubstrateRpcClientFactory<RpcClient>,
	>
	IntentionEventHandler<
		MetadataT,
		MetadataProviderT,
		EthereumIntentionExecutorT,
		KeyStoreT,
		RpcClient,
		RpcClientFactory,
	>
{
	pub fn new(
		metadata_provider: MetadataProviderT,
		ethereum_intention_executor: EthereumIntentionExecutorT,
		key_store: KeyStoreT,
		rpc_client_factory: RpcClientFactory,
	) -> Self {
		Self {
			metadata_provider,
			ethereum_intention_executor,
			key_store,
			rpc_client_factory,
			nonce: RwLock::new(0),
			phantom_data: Default::default(),
		}
	}
}

#[async_trait]
impl<
		ChainConfig: Config<
			ExtrinsicParams = DefaultExtrinsicParams<ChainConfig>,
			AccountId = AccountId32,
			Address = MultiAddress<AccountId32, u32>,
			Signature = MultiSignature,
		>,
		EthereumIntentionExecutorT: IntentionExecutor + Send + Sync,
		KeyStoreT: KeyStore<SecretKeyBytes> + Send + Sync,
		RpcClient: SubstrateRpcClient + Send + Sync,
		RpcClientFactory: SubstrateRpcClientFactory<RpcClient> + Send + Sync,
	> EventHandler<BlockEvent>
	for IntentionEventHandler<
		Metadata,
		SubxtMetadataProvider<ChainConfig>,
		EthereumIntentionExecutorT,
		KeyStoreT,
		RpcClient,
		RpcClientFactory,
	>
{
	async fn handle(&self, event: BlockEvent) -> Result<(), Error> {
		log::debug!("Got event: {:?}, variant name: {}", event.id, event.variant_name);

		if event.pallet_name != "OmniAccount" || event.variant_name != "IntentionRequested" {
			// we are not interested in this event
			log::debug!("Not interested in this event");
			return Ok(());
		}

		log::debug!("Got IntentionRequested event: {:?}", event.id);

		let metadata = self.metadata_provider.get(event.id.block_num).await;

		let pallet = metadata.pallet_by_name(&event.pallet_name).ok_or_else(move || {
			log::error!(
				"No pallet metadata found for event {} and pallet {} ",
				event.id.block_num,
				event.pallet_name
			);
			Error::NonRecoverableError
		})?;
		let variant = pallet.event_variant_by_index(event.variant_index).ok_or_else(move || {
			log::error!(
				"No event variant metadata found for event {} and variant {}",
				event.id.block_num,
				event.variant_index
			);
			Error::NonRecoverableError
		})?;

		let mut fields = variant
			.fields
			.iter()
			.map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));

		let decoded =
			crate::litentry_rococo::omni_account::events::IntentionRequested::decode_as_fields(
				&mut event.field_bytes.clone().as_slice(),
				&mut fields.clone(),
				metadata.types(),
			)
			.map_err(|e| {
				log::error!("Could not decode event {:?}, reason: {:?}", event.id, e);
				Error::NonRecoverableError
			})?;

		let intention = match decoded.intention {
			crate::litentry_rococo::runtime_types::core_primitives::intention::Intention::CallEthereum(call_ethereum) => {
				Intention::CallEthereum(call_ethereum.address.to_fixed_bytes(), call_ethereum.input.0)
			},
			crate::litentry_rococo::runtime_types::core_primitives::intention::Intention::TransferEthereum(transfer) => {
				Intention::TransferEthereum(transfer.to.to_fixed_bytes(), transfer.value)
			}
		};

		//to explicitly handle all intention variants
		match intention {
			Intention::CallEthereum(_, _) => {
				self.ethereum_intention_executor.execute(intention).await.map_err(|e| {
					// assume for now we can easily recover
					log::error!("Error executing intention");
					Error::RecoverableError
				})?;
			},
			Intention::TransferEthereum(_, _) => {
				self.ethereum_intention_executor.execute(intention).await.map_err(|e| {
					// assume for now we can easily recover
					log::error!("Error executing intention");
					Error::RecoverableError
				})?;
			},
		}

		log::debug!("Intention executed, publishing result");

		// todo: the whole signing part should be encapsulated in separate component like `TransactionSigner`
		//we need to report back to parachain intention result
		let decoded =
			crate::litentry_rococo::omni_account::events::IntentionRequested::decode_as_fields(
				&mut event.field_bytes.as_slice(),
				&mut fields,
				metadata.types(),
			)
			.map_err(|_| {
				log::error!("Could not decode event {:?}", event.id);
				Error::NonRecoverableError
			})?;

		let execution_result =
			crate::litentry_rococo::omni_account::calls::types::intention_executed::Result::Success;

		let call = crate::litentry_rococo::tx().omni_account().intention_executed(
			decoded.who,
			decoded.intention,
			execution_result,
		);

		let secret_key_bytes = self
			.key_store
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

		let mut client = self.rpc_client_factory.new_client().await.map_err(|e| {
			error!("Could not create RPC client: {:?}", e);
			RecoverableError
		})?;
		let runtime_version = client.runtime_version().await.map_err(|e| {
			error!("Could not get runtime version: {:?}", e);
			RecoverableError
		})?;
		let genesis_hash = client.get_genesis_hash().await.map_err(|e| {
			error!("Could not get genesis hash: {:?}", e);
			RecoverableError
		})?;
		let nonce = *self.nonce.read().map_err(|e| {
			error!("Could not read nonce: {:?}", e);
			RecoverableError
		})?;
		let params = DefaultExtrinsicParamsBuilder::<ChainConfig>::new().nonce(nonce).build();
		*self.nonce.write().map_err(|e| {
			error!("Could not write nonce: {:?}", e);
			RecoverableError
		})? = nonce + 1;

		let state = tx::ClientState::<ChainConfig> {
			metadata: { metadata },
			genesis_hash: ChainConfig::Hash::decode(&mut genesis_hash.as_slice()).unwrap(),
			runtime_version: tx::RuntimeVersion {
				spec_version: runtime_version.spec_version,
				transaction_version: runtime_version.transaction_version,
			},
		};
		let signed_call = tx::create_signed(&call, &state, &signer, params).unwrap();
		client.submit_tx(signed_call.encoded()).await.map_err(|e| {
			error!("Error while submitting tx: {:?}", e);
			RecoverableError
		})?;
		log::debug!("Result published");
		Ok(())
	}
}
