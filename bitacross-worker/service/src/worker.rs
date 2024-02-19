/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

///! Integritee worker. Inspiration for this design came from parity's substrate Client.
///
/// This should serve as a proof of concept for a potential refactoring design. Ultimately, everything
/// from the main.rs should be covered by the worker struct here - hidden and split across
/// multiple traits.
use crate::{config::Config, error::Error, initialized_service::TrackInitialization};
use async_trait::async_trait;
use codec::{Decode, Encode};
use itc_rpc_client::direct_client::{DirectApi, DirectClient as DirectWorkerApi};
use itp_enclave_api::enclave_base::EnclaveBase;
use itp_node_api::{api_client::PalletTeebagApi, node_api_factory::CreateNodeApi};
use jsonrpsee::{
	types::{to_json_value, traits::Client},
	ws_client::WsClientBuilder,
};
use litentry_primitives::WorkerType;
use log::*;
use std::{
	collections::HashSet,
	sync::{Arc, RwLock},
};

pub type WorkerResult<T> = Result<T, Error>;
pub type Url = String;

#[derive(Clone, Hash, Eq, PartialEq, Encode, Decode, Debug)]
pub struct PeerUrls {
	pub trusted: Url,
	pub untrusted: Url,
	pub me: bool,
}

impl PeerUrls {
	pub fn new(trusted: Url, untrusted: Url, me: bool) -> Self {
		PeerUrls { trusted, untrusted, me }
	}
}

pub struct Worker<Config, NodeApiFactory, Enclave, InitializationHandler> {
	_config: Config,
	// unused yet, but will be used when more methods are migrated to the worker
	_enclave_api: Arc<Enclave>,
	node_api_factory: Arc<NodeApiFactory>,
	initialization_handler: Arc<InitializationHandler>,
	peer_urls: RwLock<HashSet<PeerUrls>>,
}

impl<Config, NodeApiFactory, Enclave, InitializationHandler>
	Worker<Config, NodeApiFactory, Enclave, InitializationHandler>
{
	pub fn new(
		config: Config,
		enclave_api: Arc<Enclave>,
		node_api_factory: Arc<NodeApiFactory>,
		initialization_handler: Arc<InitializationHandler>,
		peer_urls: HashSet<PeerUrls>,
	) -> Self {
		Self {
			_config: config,
			_enclave_api: enclave_api,
			node_api_factory,
			initialization_handler,
			peer_urls: RwLock::new(peer_urls),
		}
	}
}

/// Looks for new peers and updates them.
pub trait UpdatePeers {
	fn search_peers(&self) -> WorkerResult<HashSet<PeerUrls>>;

	fn set_peers_urls(&self, peers: HashSet<PeerUrls>) -> WorkerResult<()>;

	fn update_peers(&self) -> WorkerResult<()> {
		let peers = self.search_peers()?;
		self.set_peers_urls(peers)
	}
}

pub trait GetPeers {
	fn read_peers_urls(&self) -> WorkerResult<HashSet<PeerUrls>>;
}

impl<NodeApiFactory, Enclave, InitializationHandler> GetPeers
	for Worker<Config, NodeApiFactory, Enclave, InitializationHandler>
where
	NodeApiFactory: CreateNodeApi + Send + Sync,
	Enclave: EnclaveBase + itp_enclave_api::remote_attestation::TlsRemoteAttestation,
{
	fn read_peers_urls(&self) -> WorkerResult<HashSet<PeerUrls>> {
		if let Ok(peer_urls) = self.peer_urls.read() {
			Ok(peer_urls.clone())
		} else {
			Err(Error::Custom("Encountered poisoned lock for peers".into()))
		}
	}
}

impl<NodeApiFactory, Enclave, InitializationHandler> UpdatePeers
	for Worker<Config, NodeApiFactory, Enclave, InitializationHandler>
where
	NodeApiFactory: CreateNodeApi + Send + Sync,
	Enclave: EnclaveBase + itp_enclave_api::remote_attestation::TlsRemoteAttestation,
{
	fn search_peers(&self) -> WorkerResult<HashSet<PeerUrls>> {
		let worker_url_external = self._config.trusted_worker_url_external();
		let node_api = self
			.node_api_factory
			.create_api()
			.map_err(|e| Error::Custom(format!("Failed to create NodeApi: {:?}", e).into()))?;
		let enclaves = node_api.all_enclaves(WorkerType::BitAcross, None)?;
		let mut peer_urls = HashSet::<PeerUrls>::new();
		for enclave in enclaves {
			// FIXME: This is temporary only, as block broadcasting should be moved to trusted ws server.
			let enclave_url = String::from_utf8_lossy(enclave.url.as_slice()).to_string();
			let worker_api_direct = DirectWorkerApi::new(enclave_url.clone());
			match worker_api_direct.get_untrusted_worker_url() {
				Ok(untrusted_worker_url) => {
					let is_me = enclave_url == worker_url_external;
					peer_urls.insert(PeerUrls::new(enclave_url, untrusted_worker_url, is_me));
				},
				Err(e) => {
					warn!("Failed to get untrusted worker url (enclave: {}): {:?}", enclave_url, e);
				},
			}
		}
		Ok(peer_urls)
	}

	fn set_peers_urls(&self, peers: HashSet<PeerUrls>) -> WorkerResult<()> {
		let peers_vec: Vec<PeerUrls> = peers.clone().into_iter().collect();
		info!("Setting peers urls: {:?}", peers_vec);

		let mut peer_urls = self.peer_urls.write().map_err(|e| {
			Error::Custom(format!("Encountered poisoned lock for peers urls: {:?}", e).into())
		})?;
		*peer_urls = peers;
		Ok(())
	}
}
