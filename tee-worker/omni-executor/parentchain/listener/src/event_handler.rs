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
use async_trait::async_trait;
use executor_core::event_handler::{Error, EventHandler};
use executor_core::intention_executor::IntentionExecutor;
use executor_core::primitives::Intention;
use std::marker::PhantomData;
use subxt::ext::scale_decode;
use subxt::ext::scale_decode::DecodeAsFields;
use subxt::{Config, Metadata};

pub struct IntentionEventHandler<
	MetadataT,
	MetadataProviderT: MetadataProvider<MetadataT>,
	EthereumIntentionExecutorT: IntentionExecutor,
> {
	metadata_provider: MetadataProviderT,
	ethereum_intention_executor: EthereumIntentionExecutorT,
	phantom_data: PhantomData<MetadataT>,
}

impl<
		MetadataT,
		MetadataProviderT: MetadataProvider<MetadataT>,
		EthereumIntentionExecutorT: IntentionExecutor,
	> IntentionEventHandler<MetadataT, MetadataProviderT, EthereumIntentionExecutorT>
{
	pub fn new(
		metadata_provider: MetadataProviderT,
		ethereum_intention_executor: EthereumIntentionExecutorT,
	) -> Self {
		Self { metadata_provider, ethereum_intention_executor, phantom_data: Default::default() }
	}
}

#[async_trait]
impl<ChainConfig: Config, EthereumIntentionExecutorT: IntentionExecutor + Send + Sync>
	EventHandler<BlockEvent>
	for IntentionEventHandler<Metadata, SubxtMetadataProvider<ChainConfig>, EthereumIntentionExecutorT>
{
	async fn handle(&self, event: BlockEvent) -> Result<(), Error> {
		log::debug!("Handling block event: {:?}", event.id);
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
				&mut event.field_bytes.as_slice(),
				&mut fields,
				metadata.types(),
			)
			.map_err(|_| {
				log::error!("Could not decode event {:?}", event.id);
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
		Ok(())
	}
}
