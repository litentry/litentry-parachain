use bc_enclave_registry::EnclaveRegistryLookup;
use bc_musig2_ceremony::SignerId;
use itc_direct_rpc_client::{Response, RpcClient, RpcClientFactory};
use itp_rpc::RpcRequest;
use litentry_primitives::Address32;
use log::{error, info};
use std::{
	collections::HashMap,
	sync::{mpsc::Sender, Arc},
};

#[derive(Debug)]
pub enum ConnectError {
	UnknownSigner,
	ClientError,
}

// Responsible for interaction with other musig2 signers
pub struct Musig2Peers<ER, CF, C> {
	enclave_registry: Arc<ER>,
	client_factory: Arc<CF>,
	peers_clients: HashMap<SignerId, C>,
}

impl<ER, CF, C> Musig2Peers<ER, CF, C>
where
	ER: EnclaveRegistryLookup,
	CF: RpcClientFactory<Client = C>,
	C: RpcClient,
{
	pub fn new(enclave_registry: Arc<ER>, client_factory: Arc<CF>) -> Self {
		Self { enclave_registry, client_factory, peers_clients: Default::default() }
	}

	pub fn connect(
		&mut self,
		signer_id: &SignerId,
		responses_sender: &Sender<Response>,
	) -> Result<(), ConnectError> {
		if !self.peers_clients.contains_key(signer_id) {
			if let Some(signer_url) =
				self.enclave_registry.get_worker_url(&Address32::from(*signer_id))
			{
				if let Ok(client) =
					self.client_factory.create(&signer_url, responses_sender.clone())
				{
					self.peers_clients.insert(*signer_id, client);
					Ok(())
				} else {
					Err(ConnectError::ClientError)
				}
			} else {
				Err(ConnectError::UnknownSigner)
			}
		} else {
			Ok(())
		}
	}

	pub fn remove(&mut self, signer_id: &SignerId) {
		self.peers_clients.remove(signer_id);
	}

	pub fn send(&mut self, signer_id: &SignerId, request: &RpcRequest) -> Result<(), ()> {
		if let Some(client) = self.peers_clients.get_mut(signer_id) {
			return match client.send(request) {
				Err(_) => {
					error!("Could not send request to signer: {:?},", signer_id);
					Err(())
				},
				_ => {
					info!("Request successfully sent");
					Ok(())
				},
			}
		}
		Err(())
	}
}

#[cfg(test)]
pub mod tests {
	use crate::peers::{ConnectError, Musig2Peers};
	use bc_enclave_registry::{EnclaveRegistryLookup, EnclaveRegistryMap};
	use codec::alloc::sync::mpsc::Sender;
	use itc_direct_rpc_client::{Response, RpcClient, RpcClientFactory};
	use itp_rpc::{Id, RpcRequest};
	use litentry_primitives::Address32;
	use std::error::Error;

	fn enclave1_address() -> Address32 {
		Address32::from([0_u8; 32])
	}

	fn sample_rpc_request() -> RpcRequest {
		RpcRequest {
			jsonrpc: "2.0".to_string(),
			id: Id::Number(1),
			method: "test_method".to_string(),
			params: vec![],
		}
	}

	const ENCLAVE_1_URL: &str = "wss://localhost:2000";

	#[test]
	pub fn connect_should_connect_to_client() {
		// given
		let enclave_lookup =
			MockedEnclaveRegistry::init(vec![(enclave1_address(), ENCLAVE_1_URL.to_string())]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: false, failing_client: false };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());
		let (sender, _) = std::sync::mpsc::channel();

		// when
		let _ = peers.connect(enclave1_address().as_ref(), &sender);

		// then
		assert!(peers.peers_clients.contains_key(enclave1_address().as_ref()));
	}

	#[test]
	pub fn should_not_connect_to_unknown_signer() {
		// given
		let enclave_lookup = MockedEnclaveRegistry::init(vec![]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: false, failing_client: false };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());
		let (sender, _) = std::sync::mpsc::channel();

		// when
		let result = peers.connect(enclave1_address().as_ref(), &sender);

		// then
		assert!(result.is_err());
		assert!(matches!(result, Err(ConnectError::UnknownSigner)));
		assert!(!peers.peers_clients.contains_key(enclave1_address().as_ref()));
	}

	#[test]
	pub fn should_not_connect_if_cannot_create_client() {
		// given
		let enclave_lookup =
			MockedEnclaveRegistry::init(vec![(enclave1_address(), ENCLAVE_1_URL.to_string())]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: true, failing_client: false };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());
		let (sender, _) = std::sync::mpsc::channel();

		// when
		let result = peers.connect(enclave1_address().as_ref(), &sender);

		// then
		assert!(result.is_err());
		assert!(matches!(result, Err(ConnectError::ClientError)));
		assert!(!peers.peers_clients.contains_key(enclave1_address().as_ref()));
	}

	#[test]
	pub fn should_not_send_request_to_non_existing_signer() {
		// given
		let enclave_lookup =
			MockedEnclaveRegistry::init(vec![(enclave1_address(), ENCLAVE_1_URL.to_string())]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: false, failing_client: false };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());

		// when
		let result = peers.send(enclave1_address().as_ref(), &sample_rpc_request());

		// then
		assert!(result.is_err());
	}

	#[test]
	pub fn should_send_request_to_connected_signer() {
		// given
		let enclave_lookup =
			MockedEnclaveRegistry::init(vec![(enclave1_address(), ENCLAVE_1_URL.to_string())]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: false, failing_client: false };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());
		let (sender, _) = std::sync::mpsc::channel();
		let _ = peers.connect(enclave1_address().as_ref(), &sender);

		// when
		let result = peers.send(enclave1_address().as_ref(), &sample_rpc_request());

		// then
		assert!(result.is_ok());
	}

	#[test]
	pub fn send_should_should_return_error_if_client_fails_to_send_request() {
		// given
		let enclave_lookup =
			MockedEnclaveRegistry::init(vec![(enclave1_address(), ENCLAVE_1_URL.to_string())]);
		let client_factory =
			MockedRpcClientFactory { failing_factory: false, failing_client: true };
		let mut peers = Musig2Peers::new(enclave_lookup.into(), client_factory.into());
		let (sender, _) = std::sync::mpsc::channel();
		let _ = peers.connect(enclave1_address().as_ref(), &sender);

		// when
		let result = peers.send(enclave1_address().as_ref(), &sample_rpc_request());

		// then
		assert!(result.is_err());
	}

	#[derive(Default)]
	pub struct MockedEnclaveRegistry {
		pub map: EnclaveRegistryMap,
	}

	impl MockedEnclaveRegistry {
		pub fn init(items: Vec<(Address32, String)>) -> Self {
			let mut registry = MockedEnclaveRegistry::default();
			for item in items {
				registry.map.insert(item.0, item.1);
			}
			registry
		}
	}

	impl EnclaveRegistryLookup for MockedEnclaveRegistry {
		fn contains_key(&self, account: &Address32) -> bool {
			self.map.contains_key(account)
		}
		fn get_all(&self) -> Vec<(Address32, String)> {
			self.map.iter().map(|(k, v)| (*k, v.clone())).collect()
		}
		fn get_worker_url(&self, account: &Address32) -> Option<String> {
			self.map.get(account).cloned()
		}
	}

	pub struct MockedRpcClient {
		failing: bool,
	}

	impl RpcClient for MockedRpcClient {
		fn send(&mut self, _request: &RpcRequest) -> Result<(), Box<dyn Error>> {
			if self.failing {
				Err("Could not send request".into())
			} else {
				Ok(())
			}
		}
	}

	pub struct MockedRpcClientFactory {
		failing_factory: bool,
		failing_client: bool,
	}

	impl RpcClientFactory for MockedRpcClientFactory {
		type Client = MockedRpcClient;

		fn create(
			&self,
			_url: &str,
			_response_sink: Sender<Response>,
		) -> Result<Self::Client, Box<dyn Error>> {
			if self.failing_factory {
				Err("Could not create client".into())
			} else {
				Ok(MockedRpcClient { failing: self.failing_client })
			}
		}
	}
}
