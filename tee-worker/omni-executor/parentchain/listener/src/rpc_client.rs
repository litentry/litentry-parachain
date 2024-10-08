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

use crate::primitives::EventId;
use async_trait::async_trait;
use std::marker::PhantomData;
use subxt::backend::legacy::LegacyRpcMethods;
use subxt::backend::BlockRef;
use subxt::config::Header;
use subxt::events::EventsClient;
use subxt::{Config, OnlineClient};

pub struct BlockEvent {
	pub id: EventId,
	pub pallet_name: String,
	pub variant_name: String,
	pub field_bytes: Vec<u8>,
}

impl BlockEvent {
	pub fn new(
		id: EventId,
		pallet_name: String,
		variant_name: String,
		field_bytes: Vec<u8>,
	) -> Self {
		Self { id, pallet_name, variant_name, field_bytes }
	}
}

/// For fetching data from Substrate RPC node
#[async_trait]
pub trait SubstrateRpcClient {
	async fn get_last_finalized_block_num(&mut self) -> Result<u64, ()>;
	async fn get_block_events(&mut self, block_num: u64) -> Result<Vec<BlockEvent>, ()>;
}

pub struct SubxtClient<ChainConfig: Config> {
	legacy: LegacyRpcMethods<ChainConfig>,
	events: EventsClient<ChainConfig, OnlineClient<ChainConfig>>,
}

impl<ChainConfig: Config> SubxtClient<ChainConfig> {}

#[async_trait]
impl<ChainConfig: Config> SubstrateRpcClient for SubxtClient<ChainConfig> {
	async fn get_last_finalized_block_num(&mut self) -> Result<u64, ()> {
		let finalized_header = self.legacy.chain_get_finalized_head().await.map_err(|_| ())?;
		match self.legacy.chain_get_header(Some(finalized_header)).await.map_err(|_| ())? {
			Some(header) => Ok(header.number().into()),
			None => Err(()),
		}
	}
	async fn get_block_events(&mut self, block_num: u64) -> Result<Vec<BlockEvent>, ()> {
		match self.legacy.chain_get_block_hash(Some(block_num.into())).await.map_err(|_| ())? {
			Some(hash) => {
				let events = self.events.at(BlockRef::from_hash(hash)).await.map_err(|_| ())?;

				Ok(events
					.iter()
					.enumerate()
					.map(|(i, event)| {
						let event = event.unwrap();
						BlockEvent::new(
							EventId::new(block_num, i as u64),
							event.pallet_name().to_string(),
							event.variant_name().to_string(),
							event.field_bytes().to_vec(),
						)
					})
					.collect())
			},
			None => Err(()),
		}
	}
}

pub struct MockedRpcClient {
	block_num: u64,
}

#[async_trait]
impl SubstrateRpcClient for MockedRpcClient {
	async fn get_last_finalized_block_num(&mut self) -> Result<u64, ()> {
		Ok(self.block_num)
	}

	async fn get_block_events(&mut self, _block_num: u64) -> Result<Vec<BlockEvent>, ()> {
		Ok(vec![])
	}
}

#[async_trait]
pub trait SubstrateRpcClientFactory<RpcClient: SubstrateRpcClient> {
	async fn new_client(&self) -> Result<RpcClient, ()>;
}

pub struct SubxtClientFactory<ChainConfig: Config> {
	url: String,
	_phantom: PhantomData<ChainConfig>,
}

impl<ChainConfig: Config> SubxtClientFactory<ChainConfig> {
	pub fn new(url: &str) -> Self {
		Self { url: url.to_string(), _phantom: PhantomData }
	}
}

#[async_trait]
impl<ChainConfig: Config> SubstrateRpcClientFactory<SubxtClient<ChainConfig>>
	for SubxtClientFactory<ChainConfig>
{
	async fn new_client(&self) -> Result<SubxtClient<ChainConfig>, ()> {
		let rpc_client = subxt::backend::rpc::RpcClient::from_insecure_url(self.url.clone())
			.await
			.map_err(|e| {
				log::error!("Could not create RpcClient: {:?}", e);
			})?;
		let legacy = LegacyRpcMethods::new(rpc_client);

		let online_client =
			OnlineClient::from_insecure_url(self.url.clone()).await.map_err(|e| {
				log::error!("Could not create OnlineClient: {:?}", e);
			})?;
		let events = online_client.events();

		Ok(SubxtClient { legacy, events })
	}
}
