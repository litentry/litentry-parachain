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
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use alloc::vec;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[cfg(feature = "std")]
use std::sync::Mutex;

use itc_direct_rpc_client::DirectRpcClient;
use itc_direct_rpc_server::{
	response_channel::ResponseChannel, rpc_responder::RpcResponder, RpcConnectionRegistry,
	SendRpcResponse,
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt};
use itp_stf_primitives::types::Hash;
use itp_types::{DirectRequestStatus, TrustedOperationStatus};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use std::{
	collections::HashMap,
	string::{String, ToString},
	sync::Arc,
	time::Duration,
	vec::Vec,
};

pub struct DirectRpcBroadcaster<ShieldingKeyRepository>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt,
{
	peers: HashMap<String, DirectRpcClient<ShieldingKeyRepository>>,
}

impl<ShieldingKeyRepository> DirectRpcBroadcaster<ShieldingKeyRepository>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt,
{
	pub fn new(peers: &[&str], key_repository: Arc<ShieldingKeyRepository>) -> Self {
		let mut peers_map = HashMap::new();

		for peer in peers {
			match DirectRpcClient::new(peer, key_repository.clone()) {
				Ok(client) => {
					peers_map.insert(peer.to_string(), client);
				},
				Err(e) => log::error!("Could not connect to peer {}, reason: {:?}", peer, e),
			}
		}

		DirectRpcBroadcaster { peers: peers_map }
	}

	pub fn broadcast<Hash: ToHexPrefixed>(&mut self, hash: Hash, params: Vec<String>) {
		self.peers.values_mut().for_each(|peer| {
			if let Err(e) = peer.send(hash.to_hex(), params.clone()) {
				log::warn!("Could not send top to peer reason: {:?}", e);
			}
		});
	}

	pub fn collect_responses<T: FromHexPrefixed>(
		&mut self,
	) -> Vec<(T::Output, TrustedOperationStatus, bool)> {
		self.peers
			.values_mut()
			.flat_map(|peer| {
				if let Ok(response) = peer.read_response::<T>() {
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
					}
				} else {
					log::warn!("Could not reed response from peer");
					None
				}
			})
			.collect()
	}
}

pub fn init<ShieldingKeyRepository, Registry, ResponseChannelType>(
	shielding_key_repository: Arc<ShieldingKeyRepository>,
	rpc_responder: Arc<RpcResponder<Registry, Hash, ResponseChannelType>>,
) -> std::sync::mpsc::SyncSender<(Hash, Vec<String>)>
where
	ShieldingKeyRepository: AccessKey + Send + Sync + 'static,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt,
	Registry: RpcConnectionRegistry<Hash = Hash> + 'static,
	ResponseChannelType: ResponseChannel<Registry::Connection> + 'static,
{
	let (sender, receiver) = std::sync::mpsc::sync_channel::<(Hash, Vec<String>)>(1000);

	let peers = vec!["localhost:2010"];
	let rpc_broadcaster =
		Arc::new(Mutex::new(DirectRpcBroadcaster::new(&peers, shielding_key_repository)));
	let cloned_rpc_broadcaster = rpc_broadcaster.clone();

	std::thread::spawn(move || {
		for received in receiver {
			if let Ok(mut locked) = rpc_broadcaster.lock() {
				locked.broadcast(received.0, received.1);
			} else {
				log::error!("Could not broadcast hash {} message", received.0)
			}
		}
	});

	std::thread::spawn(move || {
		loop {
			if let Ok(mut locked) = cloned_rpc_broadcaster.lock() {
				let responses = locked.collect_responses::<Hash>();
				for response in responses {
					match response.1 {
						// this will come from every peer so do not flood the client
						TrustedOperationStatus::Submitted => {},
						// this needs to come before block is imported, otherwise it's going to be ignored because TOP will be removed from the pool after block import
						TrustedOperationStatus::TopExecuted(ref value) => {
							match rpc_responder.update_connection_state(
								response.0,
								value.clone(),
								response.2,
							) {
								Ok(_) => {},
								Err(e) => log::error!(
									"Could not set connection {}, reason: {:?}",
									response.0,
									e
								),
							};
							if let Err(_e) =
								rpc_responder.update_status_event(response.0, response.1)
							{};
						},
						_ => {
							match rpc_responder.update_force_wait(response.0, response.2) {
								Ok(_) => {},
								Err(e) => log::error!(
									"Could not set connection {}, reason: {:?}",
									response.0,
									e
								),
							};
							if let Err(_e) =
								rpc_responder.update_status_event(response.0, response.1)
							{};
						},
					}
				}
			} else {
				log::warn!("Could not collect responses from peers")
			}
			std::thread::sleep(Duration::from_millis(10))
		}
	});
	sender
}
