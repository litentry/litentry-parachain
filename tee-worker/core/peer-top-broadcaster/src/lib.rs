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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

extern crate alloc;
extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use alloc::vec;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[cfg(feature = "std")]
use std::sync::Mutex;

use itc_direct_rpc_client::{DirectRpcClientFactory, RpcClient, RpcClientFactory};
use itc_direct_rpc_server::{
	response_channel::ResponseChannel, rpc_responder::RpcResponder, RpcConnectionRegistry,
	SendRpcResponse,
};
use itp_rpc::Id;
use itp_stf_primitives::types::Hash;
use itp_types::{DirectRequestStatus, TrustedOperationStatus, H256};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use std::{
	collections::HashMap,
	string::{String, ToString},
	sync::Arc,
	time::Duration,
	vec::Vec,
};

pub trait PeerUpdater {
	fn update(&self, peers: Vec<String>);
}

pub struct DirectRpcBroadcaster<ClientFactory>
where
	ClientFactory: RpcClientFactory,
{
	peers: Mutex<HashMap<String, ClientFactory::RpcClient>>,
	factory: ClientFactory,
}

impl<ClientFactory> DirectRpcBroadcaster<ClientFactory>
where
	ClientFactory: RpcClientFactory,
{
	pub fn new(peers: &[&str], client_factory: ClientFactory) -> Self {
		let mut peers_map = HashMap::new();
		for peer in peers {
			match client_factory.create(peer) {
				Ok(client) => {
					peers_map.insert(peer.to_string(), client);
				},
				Err(e) => log::error!("Could not connect to peer {}, reason: {:?}", peer, e),
			}
		}

		DirectRpcBroadcaster { peers: Mutex::new(peers_map), factory: client_factory }
	}

	pub fn broadcast<Hash: ToHexPrefixed>(&self, hash: Hash, params: Vec<String>) {
		if let Ok(mut peers) = self.peers.lock() {
			peers.values_mut().for_each(|peer| {
				if let Err(e) = peer.send(hash.to_hex(), params.clone()) {
					log::warn!("Could not send top to peer reason: {:?}", e);
				}
			});
		}
	}

	pub fn collect_responses(&self) -> Vec<(Id, TrustedOperationStatus, bool)> {
		if let Ok(mut peers) = self.peers.lock() {
			peers
				.values_mut()
				.flat_map(|peer| match peer.read_response() {
					Ok(response) =>
						if let Some(response) = response {
							match response.1.status {
								DirectRequestStatus::TrustedOperationStatus(status, _) =>
									Some((response.0, status, response.1.do_watch)),
								DirectRequestStatus::Ok | DirectRequestStatus::Error => {
									log::warn!(
										"Got unexpected direct request status: {:?}",
										response.1.status
									);
									None
								},
							}
						} else {
							None
						},
					Err(e) => {
						log::warn!("Could not reed response from peer, reason: {:?}", e);
						None
					},
				})
				.collect()
		} else {
			vec![]
		}
	}
}

pub fn id_to_hash(id: &Id) -> Option<Hash> {
	match id {
		Id::Text(id) => H256::from_hex(id).ok(),
		Id::Number(id) => {
			log::error!("Got response with id {}", id);
			None
		},
	}
}

#[allow(clippy::type_complexity)]
pub fn init<Registry, ResponseChannelType>(
	rpc_responder: Arc<RpcResponder<Registry, Hash, ResponseChannelType>>,
) -> (
	std::sync::mpsc::SyncSender<(Hash, Vec<String>)>,
	Arc<DirectRpcBroadcaster<DirectRpcClientFactory>>,
)
where
	Registry: RpcConnectionRegistry<Hash = Hash> + 'static,
	ResponseChannelType: ResponseChannel<Registry::Connection> + 'static,
{
	let (sender, receiver) = std::sync::mpsc::sync_channel::<(Hash, Vec<String>)>(1000);

	let peers = vec![];

	let client_factory = DirectRpcClientFactory {};

	let rpc_broadcaster = Arc::new(DirectRpcBroadcaster::new(&peers, client_factory));
	let cloned_rpc_broadcaster = rpc_broadcaster.clone();
	let return_rpc_broadcaster = rpc_broadcaster.clone();

	std::thread::spawn(move || {
		for received in receiver {
			rpc_broadcaster.broadcast(received.0, received.1);
		}
	});

	std::thread::spawn(move || {
		loop {
			let responses = cloned_rpc_broadcaster.collect_responses();
			for response in responses {
				//we need to map Id to hash in order to correlate it with connection
				let hash = match id_to_hash(&response.0) {
					Some(hash) => hash,
					None => continue,
				};
				match response.1 {
					// this will come from every peer so do not flood the client
					TrustedOperationStatus::Submitted => {},
					// this needs to come before block is imported, otherwise it's going to be ignored because TOP will be removed from the pool after block import
					TrustedOperationStatus::TopExecuted(ref value) => {
						match rpc_responder.update_connection_state(hash, value.clone(), response.2)
						{
							Ok(_) => {},
							Err(e) =>
								log::error!("Could not set connection {}, reason: {:?}", hash, e),
						};
						if let Err(_e) = rpc_responder.update_status_event(hash, response.1) {};
					},
					_ => {
						match rpc_responder.update_force_wait(hash, response.2) {
							Ok(_) => {},
							Err(e) =>
								log::error!("Could not set connection {}, reason: {:?}", hash, e),
						};
						if let Err(_e) = rpc_responder.update_status_event(hash, response.1) {};
					},
				}
			}
			std::thread::sleep(Duration::from_millis(50))
		}
	});
	(sender, return_rpc_broadcaster)
}

impl<ClientFactory> PeerUpdater for DirectRpcBroadcaster<ClientFactory>
where
	ClientFactory: RpcClientFactory,
{
	fn update(&self, peers: Vec<String>) {
		log::debug!("Updating peers: {:?}", &peers);
		for peer in peers {
			if let Ok(mut peers) = self.peers.lock() {
				if !peers.contains_key(&peer) {
					log::info!("Adding a peer: {}", peer.clone());
					match self.factory.create(&peer) {
						Ok(client) => {
							peers.insert(peer.to_string(), client);
						},
						Err(e) =>
							log::error!("Could not connect to peer {}, reason: {:?}", peer, e),
					}
				}
			}
		}
	}
}

#[cfg(test)]
pub mod tests {
	use crate::{DirectRpcBroadcaster, PeerUpdater};
	use itc_direct_rpc_client::{MaybeResponse, RpcClient, RpcClientFactory};
	use itp_rpc::{Id, RpcReturnValue};
	use itp_stf_primitives::types::Hash;
	use itp_types::{DirectRequestStatus, TrustedOperationStatus, H256};
	use std::{collections::HashMap, error::Error};

	#[derive(Default)]
	pub struct MockedRpcClient {
		pub sent_requests: u64,
		pub response: Option<(Id, RpcReturnValue)>,
	}

	impl RpcClient for MockedRpcClient {
		fn send(
			&mut self,
			_request_id: String,
			_params: Vec<String>,
		) -> Result<(), Box<dyn Error>> {
			self.sent_requests = self.sent_requests + 1;
			Ok(())
		}

		fn read_response(&mut self) -> Result<MaybeResponse, Box<dyn Error>> {
			Ok(self.response.take())
		}
	}

	impl MockedRpcClient {
		pub fn set_response(&mut self, response: (Id, RpcReturnValue)) {
			self.response = Some(response)
		}
	}

	pub struct MockedRpcClientFactory {}

	impl RpcClientFactory for MockedRpcClientFactory {
		type RpcClient = MockedRpcClient;

		fn create(&self, _url: &str) -> Result<Self::RpcClient, Box<dyn Error>> {
			Ok(MockedRpcClient::default())
		}
	}

	#[test]
	pub fn creates_initial_peers() {
		//given
		let factory = MockedRpcClientFactory {};

		//when
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec!["localhost"], factory);

		//then
		assert_eq!(broadcaster.peers.lock().unwrap().len(), 1);
	}

	#[test]
	pub fn update_creates_new_peer_if_not_exists() {
		//given
		let factory = MockedRpcClientFactory {};
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec!["localhost"], factory);
		assert_eq!(broadcaster.peers.lock().unwrap().len(), 1);

		//when
		broadcaster.update(vec!["127.0.0.1".to_string()]);

		//then
		assert_eq!(broadcaster.peers.lock().unwrap().len(), 2);
	}

	#[test]
	pub fn update_doesnt_create_new_peer_if_exists() {
		//given
		let factory = MockedRpcClientFactory {};
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec!["localhost"], factory);
		assert_eq!(broadcaster.peers.lock().unwrap().len(), 1);

		//when
		broadcaster.update(vec!["localhost".to_string()]);

		//then
		assert_eq!(broadcaster.peers.lock().unwrap().len(), 1);
	}

	#[test]
	pub fn broadcast_sends_to_all_peers() {
		//given
		let factory = MockedRpcClientFactory {};
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec!["localhost", "localhost2"], factory);

		//when
		broadcaster.broadcast(Hash::random(), vec![]);
		broadcaster.broadcast(Hash::random(), vec![]);

		//then
		let peers = broadcaster.peers.lock().unwrap();
		for peer in peers.iter() {
			assert_eq!(peer.1.sent_requests, 2u64)
		}
	}

	#[test]
	pub fn collect_responses_from_all_peers() {
		// given
		let factory = MockedRpcClientFactory {};
		let local_host = "localhost";
		let another_host = "anotherhost";
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec![local_host, another_host], factory);
		let resp_1_id = Id::Text(
			"0x0000000000000000000000000000000000000000000000000000000000000001".to_string(),
		);
		let resp_2_id = Id::Text(
			"0x0000000000000000000000000000000000000000000000000000000000000002".to_string(),
		);
		let block_hash = H256::random();
		let resp_2_top_hash = H256::random();

		let mut expected_responses: HashMap<Id, (Id, TrustedOperationStatus, bool)> =
			HashMap::new();
		expected_responses.insert(
			resp_1_id.clone(),
			(resp_1_id.clone(), TrustedOperationStatus::TopExecuted(vec![]), true),
		);
		expected_responses.insert(
			resp_2_id.clone(),
			(resp_2_id.clone(), TrustedOperationStatus::InSidechainBlock(block_hash), false),
		);

		//wrapped in the inner scope in order to release the lock before `collect_responses` is called
		{
			let mut peers = broadcaster.peers.lock().unwrap();

			let peer_1 = peers.get_mut(local_host).unwrap();
			peer_1.set_response((resp_1_id.clone(), prepare_top_executed_rpc_return_value()));

			let peer_2 = peers.get_mut(another_host).unwrap();
			peer_2.set_response((
				resp_2_id.clone(),
				prepare_in_sidechain_block_return_value(resp_2_top_hash, block_hash),
			));
		}
		//when
		let collected_responses = broadcaster.collect_responses();

		//then
		assert_eq!(collected_responses.len(), expected_responses.len());

		//the order of responses is not deterministic so we need to get expected value from map by id in order to make test result deterministic
		let expected_response_1 = expected_responses.get(&collected_responses[0].0).unwrap();

		assert_eq!(collected_responses[0].0, expected_response_1.0);
		assert_eq!(collected_responses[0].1, expected_response_1.1);
		assert_eq!(collected_responses[0].2, expected_response_1.2);

		let expected_response_2 = expected_responses.get(&collected_responses[1].0).unwrap();

		assert_eq!(collected_responses[1].0, expected_response_2.0);
		assert_eq!(collected_responses[1].1, expected_response_2.1);
		assert_eq!(collected_responses[1].2, expected_response_2.2);
	}

	#[test]
	pub fn collect_responses_ignores_ok_value() {
		// given
		let factory = MockedRpcClientFactory {};
		let local_host = "localhost";
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec![local_host], factory);

		//wrapped in the inner scope in order to release the lock before `collect_responses` is called
		{
			let mut peers = broadcaster.peers.lock().unwrap();
			let peer = peers.get_mut(local_host).unwrap();
			peer.set_response((Id::Text("1".to_string()), prepare_ok_rpc_return_value()));
		}

		//when
		let responses = broadcaster.collect_responses();

		//then
		assert_eq!(responses.len(), 0);
	}

	#[test]
	pub fn collect_ignores_error_status() {
		// given
		let factory = MockedRpcClientFactory {};
		let local_host = "localhost";
		let broadcaster: DirectRpcBroadcaster<MockedRpcClientFactory> =
			DirectRpcBroadcaster::new(&vec![local_host], factory);

		{
			let mut peers = broadcaster.peers.lock().unwrap();
			let peer = peers.get_mut(local_host).unwrap();
			peer.set_response((Id::Text("1".to_string()), prepare_error_rpc_return_value()));
		}

		//when
		let responses = broadcaster.collect_responses();

		//then
		assert_eq!(responses.len(), 0);
	}

	fn prepare_error_rpc_return_value() -> RpcReturnValue {
		RpcReturnValue::new(vec![], true, DirectRequestStatus::Error)
	}

	fn prepare_top_executed_rpc_return_value() -> RpcReturnValue {
		RpcReturnValue::new(
			vec![],
			true,
			DirectRequestStatus::TrustedOperationStatus(
				TrustedOperationStatus::TopExecuted(vec![]),
				H256::random(),
			),
		)
	}

	fn prepare_in_sidechain_block_return_value(top_hash: H256, block_hash: H256) -> RpcReturnValue {
		RpcReturnValue::new(
			vec![],
			false,
			DirectRequestStatus::TrustedOperationStatus(
				TrustedOperationStatus::InSidechainBlock(block_hash),
				top_hash,
			),
		)
	}

	fn prepare_ok_rpc_return_value() -> RpcReturnValue {
		RpcReturnValue::new(vec![], true, DirectRequestStatus::Ok)
	}
}
