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

#![cfg_attr(test, feature(assert_matches))]

#[cfg(feature = "teeracle")]
use crate::teeracle::start_interval_market_update;

#[cfg(not(feature = "dcap"))]
use crate::utils::check_files;

use crate::{
	account_funding::{setup_account_funding, EnclaveAccountInfoProvider},
	config::Config,
	error::Error,
	globals::tokio_handle::{GetTokioHandle, GlobalTokioHandle},
	initialized_service::{
		start_is_initialized_server, InitializationHandler, IsInitialized, TrackInitialization,
	},
	ocall_bridge::{
		bridge_api::Bridge as OCallBridge, component_factory::OCallBridgeComponentFactory,
	},
	parentchain_handler::{HandleParentchain, ParentchainHandler},
	prometheus_metrics::{start_metrics_server, EnclaveMetricsReceiver, MetricsHandler},
	sidechain_setup::{sidechain_init_block_production, sidechain_start_untrusted_rpc_server},
	sync_block_broadcaster::SyncBlockBroadcaster,
	utils::extract_shard,
	worker::Worker,
	worker_peers_updater::WorkerPeersUpdater,
};
use base58::ToBase58;
use clap::{load_yaml, App};
use codec::{Decode, Encode};
use enclave::{
	api::enclave_init,
	tls_ra::{enclave_request_state_provisioning, enclave_run_state_provisioning_server},
};
use itc_rpc_client::direct_client::{DirectApi, DirectClient};
use itp_enclave_api::{
	direct_request::DirectRequest,
	enclave_base::EnclaveBase,
	remote_attestation::{RemoteAttestation, TlsRemoteAttestation},
	sidechain::Sidechain,
	stf_task_handler::StfTaskHandler,
	teeracle_api::TeeracleApi,
	Enclave,
};
use itp_node_api::{
	api_client::{AccountApi, PalletTeerexApi, ParentchainApi},
	metadata::{event::print_event, NodeMetadata},
	node_api_factory::{CreateNodeApi, NodeApiFactory},
};
use itp_rpc::{RpcRequest, RpcResponse, RpcReturnValue};
use itp_settings::{
	files::SIDECHAIN_STORAGE_PATH,
	worker_mode::{ProvideWorkerMode, WorkerMode, WorkerModeProvider},
};

#[cfg(feature = "dcap")]
use itp_utils::hex::hex_encode;
#[cfg(feature = "dcap")]
use litentry_primitives::ParentchainHash as Hash;

use itp_types::DirectRequestStatus;
use its_peer_fetch::{
	block_fetch_client::BlockFetcher, untrusted_peer_fetch::UntrustedPeerFetcher,
};
use its_primitives::types::block::SignedBlock as SignedSidechainBlock;
use its_storage::{interface::FetchBlocks, BlockPruner, SidechainStorageLock};
use lc_data_providers::DataProvidersStatic;
use litentry_primitives::{ChallengeCode, Identity, ParentchainHeader as Header};
use log::*;
use serde_json::Value;
use sgx_types::*;

#[cfg(feature = "dcap")]
use sgx_verify::extract_tcb_info_from_raw_dcap_quote;

use sp_core::crypto::{AccountId32, Ss58Codec};
use sp_keyring::AccountKeyring;
use std::{
	env,
	fs::File,
	io::Read,
	path::PathBuf,
	str,
	sync::{mpsc::channel, Arc},
	thread,
	thread::sleep,
	time::Duration,
};
use substrate_api_client::{
	utils::{storage_key, FromHexString},
	Events, Header as HeaderTrait, StorageKey, XtStatus,
};
use teerex_primitives::{Enclave as TeerexEnclave, ShardIdentifier};
extern crate config as rs_config;

mod account_funding;
mod config;
mod enclave;
mod error;
mod globals;
mod initialized_service;
mod ocall_bridge;
mod parentchain_handler;
mod prometheus_metrics;
mod setup;
mod sidechain_setup;
mod sync_block_broadcaster;
mod sync_state;
#[cfg(feature = "teeracle")]
mod teeracle;
mod tests;
mod utils;
mod worker;
mod worker_peers_updater;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type EnclaveWorker =
	Worker<Config, NodeApiFactory, Enclave, InitializationHandler<WorkerModeProvider>>;

fn main() {
	// Setup logging
	env_logger::init();

	let yml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yml).get_matches();

	let config = Config::from(&matches);

	GlobalTokioHandle::initialize();

	// log this information, don't println because some python scripts for GA rely on the
	// stdout from the service
	#[cfg(feature = "production")]
	info!("*** Starting service in SGX production mode");
	#[cfg(not(feature = "production"))]
	info!("*** Starting service in SGX debug mode");

	info!("*** Running worker in mode: {:?} \n", WorkerModeProvider::worker_mode());

	let clean_reset = matches.is_present("clean-reset");
	if clean_reset {
		setup::purge_files_from_cwd().unwrap();
	}

	// build the entire dependency tree
	let tokio_handle = Arc::new(GlobalTokioHandle {});
	let sidechain_blockstorage = Arc::new(
		SidechainStorageLock::<SignedSidechainBlock>::new(PathBuf::from(&SIDECHAIN_STORAGE_PATH))
			.unwrap(),
	);
	let node_api_factory =
		Arc::new(NodeApiFactory::new(config.node_url(), AccountKeyring::Alice.pair()));
	let enclave = Arc::new(enclave_init(&config).unwrap());
	let initialization_handler = Arc::new(InitializationHandler::default());
	let worker = Arc::new(EnclaveWorker::new(
		config.clone(),
		enclave.clone(),
		node_api_factory.clone(),
		initialization_handler.clone(),
		Vec::new(),
	));
	let sync_block_broadcaster =
		Arc::new(SyncBlockBroadcaster::new(tokio_handle.clone(), worker.clone()));
	let peer_updater = Arc::new(WorkerPeersUpdater::new(worker));
	let untrusted_peer_fetcher = UntrustedPeerFetcher::new(node_api_factory.clone());
	let peer_sidechain_block_fetcher =
		Arc::new(BlockFetcher::<SignedSidechainBlock, _>::new(untrusted_peer_fetcher));
	let enclave_metrics_receiver = Arc::new(EnclaveMetricsReceiver {});

	// initialize o-call bridge with a concrete factory implementation
	OCallBridge::initialize(Arc::new(OCallBridgeComponentFactory::new(
		node_api_factory.clone(),
		sync_block_broadcaster,
		enclave.clone(),
		sidechain_blockstorage.clone(),
		peer_updater,
		peer_sidechain_block_fetcher,
		tokio_handle.clone(),
		enclave_metrics_receiver,
	)));

	if config.enable_mock_server {
		let enclave = enclave.clone();
		let trusted_server_url = format!("wss://localhost:{}", config.trusted_worker_port);
		let base58_enclave = enclave.get_mrenclave().unwrap().encode().to_base58();
		let mock_server_port = config
			.try_parse_mock_server_port()
			.expect("mock server port to be a valid port number");
		thread::spawn(move || {
			info!("*** Starting mock server");
			let getter = Arc::new(move |account: &AccountId32, identity: &Identity| {
				let client = DirectClient::new(trusted_server_url.clone());

				let mut entry_bytes = sp_core::twox_128("IdentityManagement".as_bytes()).to_vec();
				entry_bytes.extend(&sp_core::twox_128("ChallengeCodes".as_bytes())[..]);

				let encoded_account: &[u8] = &account.encode();
				let encoded_identity: &[u8] = &identity.encode();
				// Key1: Blake2_128Concat
				entry_bytes.extend(sp_core::blake2_128(encoded_account));
				entry_bytes.extend(encoded_account);
				// Key2: Blake2_128Concat
				entry_bytes.extend(sp_core::blake2_128(encoded_identity));
				entry_bytes.extend(encoded_identity);

				let request = RpcRequest {
					jsonrpc: "2.0".to_owned(),
					method: "state_getStorage".to_string(),
					params: vec![base58_enclave.clone(), format!("0x{}", hex::encode(entry_bytes))],
					id: 1,
				};

				match client.get(serde_json::to_string(&request).unwrap().as_str()) {
					Ok(response) => {
						let response: RpcResponse = serde_json::from_str(&response).unwrap();
						if let Ok(return_value) =
							<RpcReturnValue as itp_utils::FromHexPrefixed>::from_hex(
								&response.result,
							) {
							match return_value.status {
								DirectRequestStatus::Ok => {
									let mut value: &[u8] = &return_value.value;
									ChallengeCode::decode(&mut value).unwrap_or_default()
								},
								DirectRequestStatus::Error => {
									warn!("request status is error");
									if let Ok(value) =
										String::decode(&mut return_value.value.as_slice())
									{
										warn!("[Error] {}", value);
									}
									ChallengeCode::default()
								},
								DirectRequestStatus::TrustedOperationStatus(status) => {
									warn!("request status is: {:?}", status);
									ChallengeCode::default()
								},
							}
						} else {
							ChallengeCode::default()
						}
					},
					Err(e) => {
						error!("failed to send request: {:?}", e);
						ChallengeCode::default()
					},
				}
			});
			let _ = lc_mock_server::run(getter, mock_server_port);
		});
	}

	let data_provider_config = data_provider(&config);

	if let Some(run_config) = &config.run_config {
		let shard = extract_shard(&run_config.shard, enclave.as_ref());

		println!("Worker Config: {:?}", config);

		if clean_reset {
			setup::initialize_shard_and_keys(enclave.as_ref(), &shard).unwrap();
		}

		let node_api =
			node_api_factory.create_api().expect("Failed to create parentchain node API");

		if run_config.request_state {
			sync_state::sync_state::<_, _, WorkerModeProvider>(
				&node_api,
				&shard,
				enclave.as_ref(),
				run_config.skip_ra,
			);
		}

		start_worker::<_, _, _, _, WorkerModeProvider>(
			config,
			&shard,
			&data_provider_config,
			enclave,
			sidechain_blockstorage,
			node_api,
			tokio_handle,
			initialization_handler,
		);
	} else if let Some(smatches) = matches.subcommand_matches("request-state") {
		println!("*** Requesting state from a registered worker \n");
		let node_api =
			node_api_factory.create_api().expect("Failed to create parentchain node API");
		sync_state::sync_state::<_, _, WorkerModeProvider>(
			&node_api,
			&extract_shard(&smatches.value_of("shard").map(|s| s.to_string()), enclave.as_ref()),
			enclave.as_ref(),
			smatches.is_present("skip-ra"),
		);
	} else if matches.is_present("shielding-key") {
		setup::generate_shielding_key_file(enclave.as_ref());
	} else if matches.is_present("signing-key") {
		setup::generate_signing_key_file(enclave.as_ref());
		let tee_accountid = enclave_account(enclave.as_ref());
		println!("Enclave account: {:}", &tee_accountid.to_ss58check());
	} else if matches.is_present("dump-ra") {
		info!("*** Perform RA and dump cert to disk");
		#[cfg(not(feature = "dcap"))]
		enclave.dump_ias_ra_cert_to_disk().unwrap();
		#[cfg(feature = "dcap")]
		{
			let skip_ra = false;
			let dcap_quote = enclave.generate_dcap_ra_quote(skip_ra).unwrap();
			let (fmspc, _tcb_info) = extract_tcb_info_from_raw_dcap_quote(&dcap_quote).unwrap();
			enclave.dump_dcap_collateral_to_disk(fmspc).unwrap();
			enclave.dump_dcap_ra_cert_to_disk().unwrap();
		}
	} else if matches.is_present("mrenclave") {
		println!("{}", enclave.get_mrenclave().unwrap().encode().to_base58());
	} else if let Some(sub_matches) = matches.subcommand_matches("init-shard") {
		setup::init_shard(
			enclave.as_ref(),
			&extract_shard(&sub_matches.value_of("shard").map(|s| s.to_string()), enclave.as_ref()),
		);
	} else if let Some(sub_matches) = matches.subcommand_matches("test") {
		if sub_matches.is_present("provisioning-server") {
			println!("*** Running Enclave MU-RA TLS server\n");
			enclave_run_state_provisioning_server(
				enclave.as_ref(),
				sgx_quote_sign_type_t::SGX_UNLINKABLE_SIGNATURE,
				&config.mu_ra_url(),
				sub_matches.is_present("skip-ra"),
			);
			println!("[+] Done!");
		} else if sub_matches.is_present("provisioning-client") {
			println!("*** Running Enclave MU-RA TLS client\n");
			let shard = extract_shard(
				&sub_matches.value_of("shard").map(|s| s.to_string()),
				enclave.as_ref(),
			);
			enclave_request_state_provisioning(
				enclave.as_ref(),
				sgx_quote_sign_type_t::SGX_UNLINKABLE_SIGNATURE,
				&config.mu_ra_url_external(),
				&shard,
				sub_matches.is_present("skip-ra"),
			)
			.unwrap();
			println!("[+] Done!");
		} else {
			tests::run_enclave_tests(sub_matches);
		}
	} else if let Some(sub_matches) = matches.subcommand_matches("migrate-shard") {
		// This subcommand `migrate-shard` is only used for manual testing. Maybe deleted later.
		let old_shard = sub_matches
			.value_of("old-shard")
			.map(|value| {
				let mut shard = [0u8; 32];
				hex::decode_to_slice(value, &mut shard)
					.expect("shard must be hex encoded without 0x");
				ShardIdentifier::from_slice(&shard)
			})
			.unwrap();

		let new_shard: ShardIdentifier = sub_matches
			.value_of("new-shard")
			.map(|value| {
				let mut shard = [0u8; 32];
				hex::decode_to_slice(value, &mut shard)
					.expect("shard must be hex encoded without 0x");
				ShardIdentifier::from_slice(&shard)
			})
			.unwrap();

		if old_shard == new_shard {
			println!("old_shard should not be the same as new_shard");
		} else {
			setup::migrate_shard(enclave.as_ref(), &old_shard, &new_shard);
		}
	} else {
		println!("For options: use --help");
	}
}

/// FIXME: needs some discussion (restructuring?)
#[allow(clippy::too_many_arguments)]
fn start_worker<E, T, D, InitializationHandler, WorkerModeProvider>(
	config: Config,
	shard: &ShardIdentifier,
	data_provider_config: &DataProvidersStatic,
	enclave: Arc<E>,
	sidechain_storage: Arc<D>,
	node_api: ParentchainApi,
	tokio_handle_getter: Arc<T>,
	initialization_handler: Arc<InitializationHandler>,
) where
	T: GetTokioHandle,
	E: EnclaveBase
		+ DirectRequest
		+ Sidechain
		+ RemoteAttestation
		+ TlsRemoteAttestation
		+ TeeracleApi
		+ StfTaskHandler
		+ Clone,
	D: BlockPruner + FetchBlocks<SignedSidechainBlock> + Sync + Send + 'static,
	InitializationHandler: TrackInitialization + IsInitialized + Sync + Send + 'static,
	WorkerModeProvider: ProvideWorkerMode,
{
	let run_config = config.run_config.clone().expect("Run config missing");
	let skip_ra = run_config.skip_ra;

	println!("Integritee Worker v{}", VERSION);
	info!("starting worker on shard {}", shard.encode().to_base58());
	// ------------------------------------------------------------------------
	// check for required files
	if !skip_ra {
		#[cfg(not(feature = "dcap"))]
		check_files();
	}
	// ------------------------------------------------------------------------
	// initialize the enclave
	let mrenclave = enclave.get_mrenclave().unwrap();
	println!("MRENCLAVE={}", mrenclave.to_base58());
	println!("MRENCLAVE in hex {:?}", hex::encode(mrenclave));

	// ------------------------------------------------------------------------
	// let new workers call us for key provisioning
	println!("MU-RA server listening on {}", config.mu_ra_url());
	let is_development_mode = run_config.dev;
	let ra_url = config.mu_ra_url();
	let enclave_api_key_prov = enclave.clone();
	thread::spawn(move || {
		enclave_run_state_provisioning_server(
			enclave_api_key_prov.as_ref(),
			sgx_quote_sign_type_t::SGX_UNLINKABLE_SIGNATURE,
			&ra_url,
			skip_ra,
		);
		info!("State provisioning server stopped.");
	});

	let tokio_handle = tokio_handle_getter.get_handle();

	#[cfg(feature = "teeracle")]
	let teeracle_tokio_handle = tokio_handle.clone();

	// ------------------------------------------------------------------------
	// Get the public key of our TEE.
	let tee_accountid = enclave_account(enclave.as_ref());
	println!("Enclave account {:} ", &tee_accountid.to_ss58check());

	// ------------------------------------------------------------------------
	// Start `is_initialized` server.
	let untrusted_http_server_port = config
		.try_parse_untrusted_http_server_port()
		.expect("untrusted http server port to be a valid port number");
	let initialization_handler_clone = initialization_handler.clone();
	tokio_handle.spawn(async move {
		if let Err(e) =
			start_is_initialized_server(initialization_handler_clone, untrusted_http_server_port)
				.await
		{
			error!("Unexpected error in `is_initialized` server: {:?}", e);
		}
	});

	// ------------------------------------------------------------------------
	// Start prometheus metrics server.
	if config.enable_metrics_server {
		let enclave_wallet =
			Arc::new(EnclaveAccountInfoProvider::new(node_api.clone(), tee_accountid.clone()));
		let metrics_handler = Arc::new(MetricsHandler::new(enclave_wallet));
		let metrics_server_port = config
			.try_parse_metrics_server_port()
			.expect("metrics server port to be a valid port number");
		tokio_handle.spawn(async move {
			if let Err(e) = start_metrics_server(metrics_handler, metrics_server_port).await {
				error!("Unexpected error in Prometheus metrics server: {:?}", e);
			}
		});
	}

	// ------------------------------------------------------------------------
	// Start trusted worker rpc server
	if WorkerModeProvider::worker_mode() == WorkerMode::Sidechain
		|| WorkerModeProvider::worker_mode() == WorkerMode::OffChainWorker
	{
		let direct_invocation_server_addr = config.trusted_worker_url_internal();
		let enclave_for_direct_invocation = enclave.clone();
		thread::spawn(move || {
			println!(
				"[+] Trusted RPC direct invocation server listening on {}",
				direct_invocation_server_addr
			);
			enclave_for_direct_invocation
				.init_direct_invocation_server(direct_invocation_server_addr)
				.unwrap();
			println!("[+] RPC direct invocation server shut down");
		});
	}

	// ------------------------------------------------------------------------
	// Start untrusted worker rpc server.
	// i.e move sidechain block importing to trusted worker.
	if WorkerModeProvider::worker_mode() == WorkerMode::Sidechain {
		sidechain_start_untrusted_rpc_server(
			&config,
			enclave.clone(),
			sidechain_storage.clone(),
			tokio_handle,
		);
	}

	// ------------------------------------------------------------------------
	// Init parentchain specific stuff. Needed for parentchain communication.
	let parentchain_handler = Arc::new(
		ParentchainHandler::new_with_automatic_light_client_allocation(
			node_api.clone(),
			enclave.clone(),
		)
		.unwrap(),
	);
	let last_synced_header = parentchain_handler.init_parentchain_components().unwrap();
	info!("Last synced parachain block = {:?}", &last_synced_header.number);
	let nonce = node_api.get_nonce_of(&tee_accountid).unwrap();
	info!("Enclave nonce = {:?}", nonce);
	enclave
		.set_nonce(nonce)
		.expect("Could not set nonce of enclave. Returning here...");

	let metadata = node_api.metadata.clone();
	let runtime_spec_version = node_api.runtime_version.spec_version;
	let runtime_transaction_version = node_api.runtime_version.transaction_version;
	enclave
		.set_node_metadata(
			NodeMetadata::new(metadata, runtime_spec_version, runtime_transaction_version).encode(),
		)
		.expect("Could not set the node metadata in the enclave");

	#[cfg(feature = "dcap")]
	register_collateral(&node_api, &*enclave, &tee_accountid, is_development_mode, skip_ra);

	let trusted_url = config.trusted_worker_url_external();
	#[cfg(feature = "dcap")]
	let marblerun_base_url =
		run_config.marblerun_base_url.unwrap_or("http://localhost:9944".to_owned());

	#[cfg(feature = "dcap")]
	fetch_marblerun_events_every_hour(
		node_api.clone(),
		enclave.clone(),
		tee_accountid.clone(),
		is_development_mode,
		trusted_url.clone(),
		marblerun_base_url.clone(),
	);

	// ------------------------------------------------------------------------
	// Perform a remote attestation and get an unchecked extrinsic back.

	if skip_ra {
		println!(
			"[!] skipping remote attestation. Registering enclave without attestation report."
		);
	} else {
		println!("[!] creating remote attestation report and create enclave register extrinsic.");
	};
	#[cfg(not(feature = "dcap"))]
	let uxt = enclave.generate_ias_ra_extrinsic(&trusted_url, skip_ra).unwrap();
	#[cfg(feature = "dcap")]
	let uxt = enclave.generate_dcap_ra_extrinsic(&trusted_url, skip_ra).unwrap();

	let mut xthex = hex::encode(uxt);
	xthex.insert_str(0, "0x");

	// Account funds
	if let Err(x) =
		setup_account_funding(&node_api, &tee_accountid, &xthex.clone(), is_development_mode)
	{
		error!("Starting worker failed: {:?}", x);
		// Return without registering the enclave. This will fail and the transaction will be banned for 30min.
		return
	}

	let mut register_enclave_xt_header: Option<Header> = None;
	let mut we_are_primary_validateer: bool = false;

	// litentry, Check if the enclave is already registered
	match node_api.get_keys(storage_key("Teerex", "EnclaveRegistry"), None) {
		Ok(Some(keys)) => {
			let trusted_url = trusted_url.as_bytes().to_vec();
			let mrenclave = mrenclave.to_vec();
			let mut found = false;
			for key in keys {
				let key = if key.starts_with("0x") {
					let bytes = &key.as_bytes()[b"0x".len()..];
					hex::decode(bytes).unwrap()
				} else {
					hex::decode(key.as_bytes()).unwrap()
				};
				match node_api.get_storage_by_key_hash::<TeerexEnclave<AccountId32, Vec<u8>>>(
					StorageKey(key.clone()),
					None,
				) {
					Ok(Some(value)) => {
						if value.mr_enclave.to_vec() == mrenclave && value.url == trusted_url {
							// After calling the perform_ra function, the nonce will be incremented by 1,
							// so enclave is already registered, we should reset the nonce_cache
							enclave
								.set_nonce(nonce)
								.expect("Could not set nonce of enclave. Returning here...");
							found = true;
							info!("fond enclave: {:?}", value);
							break
						}
					},
					Ok(None) => {
						warn!("not found from key: {:?}", key);
					},
					Err(_) => {},
				}
			}
			if !found {
				println!("[>] Register the enclave (send the extrinsic)");
				let register_enclave_xt_hash =
					node_api.send_extrinsic(xthex, XtStatus::Finalized).unwrap();
				println!("[<] Extrinsic got finalized. Hash: {:?}\n", register_enclave_xt_hash);
				register_enclave_xt_header = node_api.get_header(register_enclave_xt_hash).unwrap();
			}
		},
		_ => panic!("unknown error"),
	}

	if let Some(register_enclave_xt_header) = register_enclave_xt_header.clone() {
		we_are_primary_validateer =
			check_we_are_primary_validateer(&node_api, &register_enclave_xt_header).unwrap();
	}

	if we_are_primary_validateer {
		println!("[+] We are the primary validateer");
	} else {
		println!("[+] We are NOT the primary validateer");
	}

	initialization_handler.registered_on_parentchain();

	// ------------------------------------------------------------------------
	// Start stf task handler thread
	let enclave_api_stf_task_handler = enclave.clone();
	let data_provider_config = data_provider_config.clone();
	thread::spawn(move || {
		enclave_api_stf_task_handler.run_stf_task_handler(data_provider_config).unwrap();
	});

	// ------------------------------------------------------------------------
	// initialize teeracle interval
	#[cfg(feature = "teeracle")]
	if WorkerModeProvider::worker_mode() == WorkerMode::Teeracle {
		start_interval_market_update(
			&node_api,
			run_config.teeracle_update_interval,
			enclave.as_ref(),
			&teeracle_tokio_handle,
		);
	}

	if WorkerModeProvider::worker_mode() != WorkerMode::Teeracle {
		let parentchain_start_block = config
			.try_parse_parentchain_start_block()
			.expect("parentchain start block to be a valid number");
		println!("*** [+] Finished syncing light client, syncing parentchain...");
		println!(
			"*** [+] last_synced_header: {}, config.parentchain_start_block: {}",
			last_synced_header.number, parentchain_start_block
		);

		// Syncing all parentchain blocks, this might take a while..
		let mut last_synced_header = parentchain_handler
			.sync_parentchain(last_synced_header, parentchain_start_block)
			.unwrap();

		// ------------------------------------------------------------------------
		// Initialize the sidechain
		if WorkerModeProvider::worker_mode() == WorkerMode::Sidechain {
			last_synced_header = sidechain_init_block_production(
				enclave,
				register_enclave_xt_header,
				we_are_primary_validateer,
				parentchain_handler.clone(),
				sidechain_storage,
				&last_synced_header,
				parentchain_start_block,
			)
			.unwrap();
		}

		// ------------------------------------------------------------------------
		// start parentchain syncing loop (subscribe to header updates)
		thread::Builder::new()
			.name("parentchain_sync_loop".to_owned())
			.spawn(move || {
				if let Err(e) =
					subscribe_to_parentchain_new_headers(parentchain_handler, last_synced_header)
				{
					error!("Parentchain block syncing terminated with a failure: {:?}", e);
				}
				println!("[!] Parentchain block syncing has terminated");
			})
			.unwrap();
	}

	// ------------------------------------------------------------------------
	if WorkerModeProvider::worker_mode() == WorkerMode::Sidechain {
		spawn_worker_for_shard_polling(shard, node_api.clone(), initialization_handler);
	}

	// ------------------------------------------------------------------------
	// subscribe to events and react on firing
	println!("*** Subscribing to events");
	let (sender, receiver) = channel();
	let metadata = node_api.metadata.clone();
	let _ = thread::Builder::new()
		.name("event_subscriber".to_owned())
		.spawn(move || {
			node_api.subscribe_events(sender).unwrap();
		})
		.unwrap();

	println!("[+] Subscribed to events. waiting...");
	let timeout = Duration::from_secs(600);
	loop {
		if let Ok(events_str) = receiver.recv_timeout(timeout) {
			let event_bytes = Vec::from_hex(events_str).unwrap();
			let events = Events::new(metadata.clone(), Default::default(), event_bytes);

			for maybe_event_details in events.iter() {
				let event_details = maybe_event_details.unwrap();
				let _ = print_event(&event_details);
			}
		}
	}
}

/// Start polling loop to wait until we have a worker for a shard registered on
/// the parentchain (TEEREX WorkerForShard). This is the pre-requisite to be
/// considered initialized and ready for the next worker to start (in sidechain mode only).
/// considered initialized and ready for the next worker to start.
fn spawn_worker_for_shard_polling<InitializationHandler>(
	shard: &ShardIdentifier,
	node_api: ParentchainApi,
	initialization_handler: Arc<InitializationHandler>,
) where
	InitializationHandler: TrackInitialization + Sync + Send + 'static,
{
	let shard_for_initialized = *shard;
	thread::spawn(move || {
		const POLL_INTERVAL_SECS: u64 = 2;

		loop {
			info!("Polling for worker for shard ({} seconds interval)", POLL_INTERVAL_SECS);
			if let Ok(Some(_)) = node_api.worker_for_shard(&shard_for_initialized, None) {
				// Set that the service is initialized.
				initialization_handler.worker_for_shard_registered();
				println!("[+] Found `WorkerForShard` on parentchain state");
				break
			}
			sleep(Duration::from_secs(POLL_INTERVAL_SECS));
		}
	});
}

#[cfg(feature = "dcap")]
fn fetch_marblerun_events_every_hour<E>(
	api: ParentchainApi,
	enclave: Arc<E>,
	accountid: AccountId32,
	is_development_mode: bool,
	url: String,
	marblerun_base_url: String,
) where
	E: RemoteAttestation + Clone + Sync + Send + 'static,
{
	let enclave = enclave.clone();
	let handle = thread::spawn(move || {
		const POLL_INTERVAL_5_MINUTES_IN_SECS: u64 = 5 * 60;
		loop {
			info!("Polling marblerun events for quotes to register");
			register_quotes_from_marblerun(
				&api,
				enclave.clone(),
				&accountid,
				is_development_mode,
				url.clone(),
				marblerun_base_url.clone(),
			);

			thread::sleep(Duration::from_secs(POLL_INTERVAL_5_MINUTES_IN_SECS));
		}
	});

	handle.join().unwrap()
}
#[cfg(feature = "dcap")]
fn register_quotes_from_marblerun(
	api: &ParentchainApi,
	enclave: Arc<dyn RemoteAttestation>,
	accountid: &AccountId32,
	is_development_mode: bool,
	url: String,
	marblerun_base_url: String,
) {
	let enclave = enclave.as_ref();
	let events = prometheus_metrics::fetch_marblerun_events(&marblerun_base_url)
		.map_err(|e| {
			info!("Fetching events from Marblerun failed with: {:?}, continuing with 0 events.", e);
		})
		.unwrap_or_default();
	let quotes: Vec<&[u8]> =
		events.iter().map(|event| event.get_quote_without_prepended_bytes()).collect();

	for quote in quotes {
		match enclave.generate_dcap_ra_extrinsic_from_quote(url.clone(), &quote) {
			Ok(xts) => {
				send_extrinsic(&xts, api, accountid, is_development_mode);
			},
			Err(e) => {
				error!("Extracting information from quote failed: {}", e)
			},
		}
	}
}
#[cfg(feature = "dcap")]
fn register_collateral(
	api: &ParentchainApi,
	enclave: &dyn RemoteAttestation,
	accountid: &AccountId32,
	is_development_mode: bool,
	skip_ra: bool,
) {
	let dcap_quote = enclave.generate_dcap_ra_quote(skip_ra).unwrap();
	let (fmspc, _tcb_info) = extract_tcb_info_from_raw_dcap_quote(&dcap_quote).unwrap();

	let uxt = enclave.generate_register_quoting_enclave_extrinsic(fmspc).unwrap();
	send_extrinsic(&uxt, api, accountid, is_development_mode);

	let uxt = enclave.generate_register_tcb_info_extrinsic(fmspc).unwrap();
	send_extrinsic(&uxt, api, accountid, is_development_mode);
}

#[cfg(feature = "dcap")]
fn send_extrinsic(
	extrinsic: &[u8],
	api: &ParentchainApi,
	accountid: &AccountId32,
	is_development_mode: bool,
) -> Option<Hash> {
	let xthex = hex_encode(extrinsic);
	// Account funds
	if let Err(x) = setup_account_funding(api, accountid, &xthex, is_development_mode) {
		error!("Starting worker failed: {:?}", x);
		// Return without registering the enclave. This will fail and the transaction will be banned for 30min.
		return None
	}

	println!("[>] Register the TCB info (send the extrinsic)");
	let register_qe_xt_hash = api.send_extrinsic(xthex, XtStatus::Finalized).unwrap();
	println!("[<] Extrinsic got finalized. Hash: {:?}\n", register_qe_xt_hash);
	register_qe_xt_hash
}

/// Subscribe to the node API finalized heads stream and trigger a parent chain sync
/// upon receiving a new header.
fn subscribe_to_parentchain_new_headers<E: EnclaveBase + Sidechain>(
	parentchain_handler: Arc<ParentchainHandler<ParentchainApi, E>>,
	mut last_synced_header: Header,
) -> Result<(), Error> {
	let (sender, receiver) = channel();
	//TODO: this should be implemented by parentchain_handler directly, and not via
	// exposed parentchain_api. Blocked by https://github.com/scs/substrate-api-client/issues/267.
	parentchain_handler
		.parentchain_api()
		.subscribe_finalized_heads(sender)
		.map_err(Error::ApiClient)?;

	// TODO(Kai@Litentry):
	// originally we had an outer loop to try to handle the disconnection,
	// see https://github.com/litentry/litentry-parachain/commit/b8059d0fad928e4bba99178451cd0d473791c437
	// but I reverted it because:
	// - no graceful shutdown, we could have many mpsc channel when it doesn't go right
	// - we might have multiple `sync_parentchain` running concurrently, which causes chaos in enclave side
	// - I still feel it's only a workaround, not a perfect solution
	//
	// TODO: now the sync will panic if disconnected - it heavily relys on the worker-restart to work (even manually)
	loop {
		let new_header: Header = match receiver.recv() {
			Ok(header_str) => serde_json::from_str(&header_str).map_err(Error::Serialization),
			Err(e) => Err(Error::ApiSubscriptionDisconnected(e)),
		}?;

		println!(
			"[+] Received finalized header update ({}), syncing parent chain...",
			new_header.number
		);

		// the overriden_start_block shouldn't matter here
		last_synced_header = parentchain_handler.sync_parentchain(last_synced_header, 0)?;
	}
}

/// Get the public signing key of the TEE.
fn enclave_account<E: EnclaveBase>(enclave_api: &E) -> AccountId32 {
	let tee_public = enclave_api.get_ecc_signing_pubkey().unwrap();
	trace!("[+] Got ed25519 account of TEE = {}", tee_public.to_ss58check());
	AccountId32::from(*tee_public.as_array_ref())
}

/// Checks if we are the first validateer to register on the parentchain.
fn check_we_are_primary_validateer(
	node_api: &ParentchainApi,
	register_enclave_xt_header: &Header,
) -> Result<bool, Error> {
	let enclave_count_of_previous_block =
		node_api.enclave_count(Some(*register_enclave_xt_header.parent_hash()))?;
	Ok(enclave_count_of_previous_block == 0)
}

fn data_provider(config: &Config) -> DataProvidersStatic {
	let built_in_modes = vec!["dev", "staging", "prod", "mock"];
	let built_in_config: Value =
		serde_json::from_slice(include_bytes!("running-mode-config.json")).unwrap();

	let mut data_provider_config = if built_in_modes.contains(&config.running_mode.as_str()) {
		let config = built_in_config.get(config.running_mode.as_str()).unwrap();
		serde_json::from_value::<DataProvidersStatic>(config.clone()).unwrap()
	} else {
		let file_path = config.running_mode.as_str();
		let mut file = File::open(file_path)
			.map_err(|e| format!("{:?}, file:{}", e, file_path))
			.unwrap();
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();
		serde_json::from_str::<DataProvidersStatic>(data.as_str()).unwrap()
	};
	if let Ok(v) = env::var("TWITTER_OFFICIAL_URL") {
		data_provider_config.set_twitter_official_url(v);
	}
	if let Ok(v) = env::var("TWITTER_LITENTRY_URL") {
		data_provider_config.set_twitter_litentry_url(v);
	}
	// Bearer Token is as same as App only Access Token on Twitter (https://developer.twitter.com/en/docs/authentication/oauth-2-0/application-only),
	// that is for developers that just need read-only access to public information.
	if let Ok(v) = env::var("TWITTER_AUTH_TOKEN_V1_1") {
		data_provider_config.set_twitter_auth_token_v1_1(v);
	}
	if let Ok(v) = env::var("TWITTER_AUTH_TOKEN_V2") {
		data_provider_config.set_twitter_auth_token_v2(v);
	}
	if let Ok(v) = env::var("DISCORD_OFFICIAL_URL") {
		data_provider_config.set_discord_official_url(v);
	}
	if let Ok(v) = env::var("DISCORD_LITENTRY_URL") {
		data_provider_config.set_discord_litentry_url(v);
	}
	if let Ok(v) = env::var("DISCORD_AUTH_TOKEN") {
		data_provider_config.set_discord_auth_token(v);
	}
	if let Ok(v) = env::var("GRAPHQL_URL") {
		data_provider_config.set_graphql_url(v);
	}
	if let Ok(v) = env::var("GRAPHQL_AUTH_KEY") {
		data_provider_config.set_graphql_auth_key(v);
	}
	if let Ok(v) = env::var("CREDENTIAL_ENDPOINT") {
		data_provider_config.set_credential_endpoint(v);
	}

	data_provider_config
}
