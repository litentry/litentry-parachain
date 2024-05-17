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

#![allow(clippy::unwrap_used)]

pub mod global_components;
pub mod parentchain;
use crate::{
	error::{Error, Result as EnclaveResult},
	get_node_metadata_repository_from_integritee_solo_or_parachain,
	get_validator_accessor_from_integritee_solo_or_parachain,
	initialization::global_components::{
		EnclaveGetterExecutor, EnclaveLightClientSeal, EnclaveOCallApi, EnclaveRpcResponder,
		EnclaveShieldingKeyRepository, EnclaveSidechainApi, EnclaveStateFileIo,
		EnclaveStateHandler, EnclaveStateInitializer, EnclaveStateObserver,
		EnclaveStateSnapshotRepository, EnclaveStfEnclaveSigner, EnclaveTopPool,
		EnclaveTopPoolAuthor, GLOBAL_ATTESTATION_HANDLER_COMPONENT,
		GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT, GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT,
		GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL, GLOBAL_OCALL_API_COMPONENT,
		GLOBAL_RPC_WS_HANDLER_COMPONENT, GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT,
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_HANDLER_COMPONENT,
		GLOBAL_STATE_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_OBSERVER_COMPONENT,
		GLOBAL_TARGET_A_PARENTCHAIN_LIGHT_CLIENT_SEAL,
		GLOBAL_TARGET_B_PARENTCHAIN_LIGHT_CLIENT_SEAL, GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
		GLOBAL_WEB_SOCKET_SERVER_COMPONENT,
	},
	ocall::OcallApi,
	rpc::{rpc_response_channel::RpcResponseChannel, worker_api_direct::public_api_rpc_handler},
	utils::get_extrinsic_factory_from_integritee_solo_or_parachain,
	Hash,
};
use base58::ToBase58;
use bc_enclave_registry::{EnclaveRegistryUpdater, GLOBAL_ENCLAVE_REGISTRY};
use bc_musig2_ceremony::{CeremonyRegistry, MuSig2Ceremony};
use bc_musig2_runner::init_ceremonies_thread;
use bc_relayer_registry::{RelayerRegistryUpdater, GLOBAL_RELAYER_REGISTRY};
use bc_signer_registry::{SignerRegistryUpdater, GLOBAL_SIGNER_REGISTRY};
use bc_task_receiver::{run_bit_across_handler_runner, BitAcrossTaskContext};
use codec::Encode;
use ita_stf::{Getter, TrustedCallSigned};
use itc_direct_rpc_client::DirectRpcClientFactory;
use itc_direct_rpc_server::{
	create_determine_watch, rpc_connection_registry::ConnectionRegistry,
	rpc_ws_handler::RpcWsHandler,
};

use bc_musig2_ceremony::{CeremonyCommandsRegistry, CeremonyId};
use itc_parentchain_light_client::{concurrent_access::ValidatorAccess, ExtrinsicSender};
use itc_tls_websocket_server::{
	certificate_generation::ed25519_self_signed_certificate, create_ws_server, ConnectionToken,
	WebSocketServer,
};
use itp_attestation_handler::{AttestationHandler, IntelAttestationHandler};
use itp_component_container::{ComponentGetter, ComponentInitializer};
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api_metadata::pallet_bitacross::BitAcrossCallIndexes;
use itp_node_api_metadata_provider::AccessNodeMetadata;
use itp_primitives_cache::GLOBAL_PRIMITIVES_CACHE;
use itp_settings::files::{
	LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, STATE_SNAPSHOTS_CACHE_SIZE,
	TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
};
use itp_sgx_crypto::{
	ecdsa::create_ecdsa_repository,
	get_aes_repository, get_ed25519_repository, get_rsa3072_repository,
	key_repository::{AccessKey, KeyRepository},
	schnorr::{create_schnorr_repository, Pair as SchnorrPair, Seal},
};

use itp_stf_state_handler::{
	file_io::StateDir, handle_state::HandleState, query_shard_state::QueryShardState,
	state_snapshot_repository::VersionedStateAccess,
	state_snapshot_repository_loader::StateSnapshotRepositoryLoader, StateHandler,
};
use itp_top_pool::pool::Options as PoolOptions;
use itp_top_pool_author::author::AuthorTopFilter;
use itp_types::{parentchain::ParentchainId, OpaqueCall, ShardIdentifier};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use log::*;
use sp_core::crypto::Pair;
use std::{collections::HashMap, path::PathBuf, string::String, sync::Arc};

use std::sync::SgxMutex as Mutex;

pub(crate) fn init_enclave(
	mu_ra_url: String,
	untrusted_worker_url: String,
	base_dir: PathBuf,
) -> EnclaveResult<()> {
	let signing_key_repository = Arc::new(get_ed25519_repository(base_dir.clone())?);

	GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.initialize(signing_key_repository.clone());
	let signer = signing_key_repository.retrieve_key()?;
	info!("[Enclave initialized] Ed25519 prim raw : {:?}", signer.public().0);

	let bitcoin_key_repository = Arc::new(create_schnorr_repository(base_dir.clone(), "bitcoin")?);
	GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT.initialize(bitcoin_key_repository.clone());
	let bitcoin_key = bitcoin_key_repository.retrieve_key()?;
	info!("[Enclave initialized] Bitcoin public key raw : {:?}", bitcoin_key.public_bytes());

	let ethereum_key_repository = Arc::new(create_ecdsa_repository(base_dir.clone(), "ethereum")?);
	GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT.initialize(ethereum_key_repository.clone());
	let ethereum_key = ethereum_key_repository.retrieve_key()?;
	info!("[Enclave initialized] Ethereum public key raw : {:?}", ethereum_key.public_bytes());

	let shielding_key_repository = Arc::new(get_rsa3072_repository(base_dir.clone())?);
	GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.initialize(shielding_key_repository.clone());

	// Create the aes key that is used for state encryption such that a key is always present in tests.
	// It will be overwritten anyway if mutual remote attestation is performed with the primary worker.
	let state_key_repository = Arc::new(get_aes_repository(base_dir.clone())?);
	GLOBAL_STATE_KEY_REPOSITORY_COMPONENT.initialize(state_key_repository.clone());

	let integritee_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::Litentry,
	)?);
	GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(integritee_light_client_seal);

	let target_a_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::TargetA,
	)?);
	GLOBAL_TARGET_A_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(target_a_light_client_seal);

	let target_b_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::TargetB,
	)?);
	GLOBAL_TARGET_B_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(target_b_light_client_seal);

	let state_file_io =
		Arc::new(EnclaveStateFileIo::new(state_key_repository, StateDir::new(base_dir)));
	let state_initializer =
		Arc::new(EnclaveStateInitializer::new(shielding_key_repository.clone()));
	let state_snapshot_repository_loader = StateSnapshotRepositoryLoader::<
		EnclaveStateFileIo,
		EnclaveStateInitializer,
	>::new(state_file_io, state_initializer.clone());

	let state_snapshot_repository =
		state_snapshot_repository_loader.load_snapshot_repository(STATE_SNAPSHOTS_CACHE_SIZE)?;
	let state_observer = initialize_state_observer(&state_snapshot_repository)?;
	GLOBAL_STATE_OBSERVER_COMPONENT.initialize(state_observer.clone());

	let state_handler = Arc::new(StateHandler::load_from_repository(
		state_snapshot_repository,
		state_observer.clone(),
		state_initializer,
	)?);

	GLOBAL_STATE_HANDLER_COMPONENT.initialize(state_handler.clone());

	let ocall_api = Arc::new(OcallApi);
	GLOBAL_OCALL_API_COMPONENT.initialize(ocall_api.clone());

	// For debug purposes, list shards. no problem to panic if fails.
	#[allow(clippy::unwrap_used)]
	let shards = state_handler.list_shards().unwrap();
	debug!("found the following {} shards on disk:", shards.len());
	for s in shards {
		debug!("{}", s.encode().to_base58())
	}

	itp_primitives_cache::set_primitives(
		GLOBAL_PRIMITIVES_CACHE.as_ref(),
		mu_ra_url,
		untrusted_worker_url,
	)
	.map_err(Error::PrimitivesAccess)?;

	let watch_extractor = Arc::new(create_determine_watch::<Hash>());

	let connection_registry = Arc::new(ConnectionRegistry::<Hash, ConnectionToken>::new());

	// We initialize components for the public RPC / direct invocation server here, so we can start the server
	// before registering on the parentchain. If we started the RPC AFTER registering on the parentchain and
	// initializing the light-client, there is a period of time where a peer might want to reach us,
	// but the RPC server is not yet up and running, resulting in error messages or even in that
	// validateer completely breaking (IO PipeError).
	// Corresponding GH issues are #545 and #600.

	let response_channel = Arc::new(RpcResponseChannel::default());
	let rpc_responder =
		Arc::new(EnclaveRpcResponder::new(connection_registry.clone(), response_channel));

	let top_pool_author = create_top_pool_author(
		rpc_responder.clone(),
		state_handler,
		ocall_api.clone(),
		shielding_key_repository.clone(),
	);
	GLOBAL_TOP_POOL_AUTHOR_COMPONENT.initialize(top_pool_author.clone());

	let getter_executor = Arc::new(EnclaveGetterExecutor::new(state_observer));

	let ceremony_registry = Arc::new(Mutex::new(HashMap::<
		CeremonyId,
		MuSig2Ceremony<KeyRepository<SchnorrPair, Seal>>,
	>::new()));

	let pending_ceremony_commands = Arc::new(Mutex::new(CeremonyCommandsRegistry::new()));

	let attestation_handler =
		Arc::new(IntelAttestationHandler::new(ocall_api.clone(), signing_key_repository.clone()));
	GLOBAL_ATTESTATION_HANDLER_COMPONENT.initialize(attestation_handler);

	GLOBAL_RELAYER_REGISTRY.init().map_err(|e| Error::Other(e.into()))?;
	GLOBAL_ENCLAVE_REGISTRY.init().map_err(|e| Error::Other(e.into()))?;
	GLOBAL_SIGNER_REGISTRY.init().map_err(|e| Error::Other(e.into()))?;

	let io_handler = public_api_rpc_handler(
		top_pool_author,
		getter_executor,
		shielding_key_repository,
		ocall_api.clone(),
		signing_key_repository,
		bitcoin_key_repository,
		ethereum_key_repository,
		GLOBAL_SIGNER_REGISTRY.clone(),
	);
	let rpc_handler = Arc::new(RpcWsHandler::new(io_handler, watch_extractor, connection_registry));
	GLOBAL_RPC_WS_HANDLER_COMPONENT.initialize(rpc_handler);

	let ceremony_registry_cloned = ceremony_registry.clone();
	let pending_ceremony_commands_cloned = pending_ceremony_commands.clone();

	std::thread::spawn(move || {
		run_bit_across_handler(ceremony_registry, pending_ceremony_commands, signer.public().0)
			.unwrap()
	});

	let client_factory = DirectRpcClientFactory {};
	init_ceremonies_thread(
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get()?,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?,
		Arc::new(client_factory),
		GLOBAL_ENCLAVE_REGISTRY.clone(),
		ceremony_registry_cloned,
		pending_ceremony_commands_cloned,
		ocall_api,
		rpc_responder,
	);
	Ok(())
}

pub(crate) fn finish_enclave_init() -> EnclaveResult<()> {
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;
	let mrenclave = attestation_handler.get_mrenclave()?;
	GLOBAL_SCHEDULED_ENCLAVE.init(mrenclave).map_err(|e| Error::Other(e.into()))?;

	Ok(())
}

pub(crate) fn publish_wallets() -> EnclaveResult<()> {
	let metadata_repository = get_node_metadata_repository_from_integritee_solo_or_parachain()?;
	let extrinsics_factory = get_extrinsic_factory_from_integritee_solo_or_parachain()?;
	let validator_accessor = get_validator_accessor_from_integritee_solo_or_parachain()?;

	let bitcoin_key_repository = GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT.get()?;
	let bitcoin_key = bitcoin_key_repository.retrieve_key()?;

	let bitcoin_call = metadata_repository
		.get_from_metadata(|m| m.btc_wallet_generated_indexes())
		.map_err(|e| Error::Other(e.into()))?
		.map_err(|e| Error::Other(format!("{:?}", e).into()))?;

	let bitcoin_opaque_call = OpaqueCall::from_tuple(&(bitcoin_call, bitcoin_key.public_bytes()));

	let ethereum_key_repository = GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT.get()?;
	let ethereum_key = ethereum_key_repository.retrieve_key()?;

	let ethereum_call = metadata_repository
		.get_from_metadata(|m| m.eth_wallet_generated_indexes())
		.map_err(|e| Error::Other(e.into()))?
		.map_err(|e| Error::Other(format!("{:?}", e).into()))?;

	let ethereum_opaque_call =
		OpaqueCall::from_tuple(&(ethereum_call, ethereum_key.public_bytes()));

	let xts = extrinsics_factory
		.create_extrinsics(&[bitcoin_opaque_call, ethereum_opaque_call], None)
		.map_err(|e| Error::Other(e.into()))?;
	validator_accessor
		.execute_mut_on_validator(|v| v.send_extrinsics(xts))
		.map_err(|e| Error::Other(e.into()))?;

	//todo: this should be called as late as possible P-727
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;
	let mrenclave = attestation_handler.get_mrenclave()?;
	GLOBAL_SCHEDULED_ENCLAVE.init(mrenclave).map_err(|e| Error::Other(e.into()))?;

	Ok(())
}

fn initialize_state_observer(
	snapshot_repository: &EnclaveStateSnapshotRepository,
) -> EnclaveResult<Arc<EnclaveStateObserver>> {
	let shards = snapshot_repository.list_shards()?;
	let mut states_map = HashMap::<
		ShardIdentifier,
		<EnclaveStateSnapshotRepository as VersionedStateAccess>::StateType,
	>::new();
	for shard in shards.into_iter() {
		let state = snapshot_repository.load_latest(&shard)?;
		states_map.insert(shard, state);
	}
	Ok(Arc::new(EnclaveStateObserver::from_map(states_map)))
}

fn run_bit_across_handler(
	musig2_ceremony_registry: Arc<Mutex<CeremonyRegistry<KeyRepository<SchnorrPair, Seal>>>>,
	musig2_ceremony_pending_commands: Arc<Mutex<CeremonyCommandsRegistry>>,
	signing_key_pub: [u8; 32],
) -> Result<(), Error> {
	let author_api = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let state_observer = GLOBAL_STATE_OBSERVER_COMPONENT.get()?;
	let relayer_registry_lookup = GLOBAL_RELAYER_REGISTRY.clone();
	let enclave_registry_lookup = GLOBAL_ENCLAVE_REGISTRY.clone();
	let signer_registry_lookup = GLOBAL_SIGNER_REGISTRY.clone();

	let shielding_key_repository = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?;
	let ethereum_key_repository = GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT.get()?;
	let bitcoin_key_repository = GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT.get()?;

	#[allow(clippy::unwrap_used)]
	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api.clone(),
		shielding_key_repository.clone(),
		author_api,
	));

	let stf_task_context = BitAcrossTaskContext::new(
		shielding_key_repository,
		ethereum_key_repository,
		bitcoin_key_repository,
		stf_enclave_signer,
		state_handler,
		ocall_api,
		relayer_registry_lookup,
		musig2_ceremony_registry,
		enclave_registry_lookup,
		signer_registry_lookup,
		musig2_ceremony_pending_commands,
		signing_key_pub,
	);
	run_bit_across_handler_runner(Arc::new(stf_task_context));
	Ok(())
}

pub(crate) fn init_direct_invocation_server(server_addr: String) -> EnclaveResult<()> {
	let rpc_handler = GLOBAL_RPC_WS_HANDLER_COMPONENT.get()?;
	let signer = GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get()?.retrieve_key()?;

	let cert =
		ed25519_self_signed_certificate(signer, "Enclave").map_err(|e| Error::Other(e.into()))?;

	// Serialize certificate(s) and private key to PEM.
	// PEM format is needed as a certificate chain can only be serialized into PEM.
	let pem_serialized = cert.serialize_pem().map_err(|e| Error::Other(e.into()))?;
	let private_key = cert.serialize_private_key_pem();

	let web_socket_server =
		create_ws_server(server_addr.as_str(), &private_key, &pem_serialized, rpc_handler);

	GLOBAL_WEB_SOCKET_SERVER_COMPONENT.initialize(web_socket_server.clone());

	match web_socket_server.run() {
		Ok(()) => {},
		Err(e) => {
			error!("Web socket server encountered an unexpected error: {:?}", e)
		},
	}

	Ok(())
}

pub(crate) fn init_shard(shard: ShardIdentifier) -> EnclaveResult<()> {
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let _ = state_handler.initialize_shard(shard)?;
	Ok(())
}

pub(crate) fn migrate_shard(
	old_shard: ShardIdentifier,
	new_shard: ShardIdentifier,
) -> EnclaveResult<()> {
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let _ = state_handler.migrate_shard(old_shard, new_shard)?;
	Ok(())
}

/// Initialize the TOP pool author component.
pub fn create_top_pool_author(
	rpc_responder: Arc<EnclaveRpcResponder>,
	state_handler: Arc<EnclaveStateHandler>,
	ocall_api: Arc<EnclaveOCallApi>,
	shielding_key_repository: Arc<EnclaveShieldingKeyRepository>,
) -> Arc<EnclaveTopPoolAuthor> {
	let side_chain_api = Arc::new(EnclaveSidechainApi::new());
	let top_pool =
		Arc::new(EnclaveTopPool::create(PoolOptions::default(), side_chain_api, rpc_responder));

	Arc::new(EnclaveTopPoolAuthor::new(
		top_pool,
		AuthorTopFilter::<TrustedCallSigned, Getter>::new(),
		state_handler,
		shielding_key_repository,
		ocall_api,
	))
}
