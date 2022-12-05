// Copyright 2020-2022 Litentry Technologies GmbH.
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

#![allow(clippy::borrowed_box)]

use crate::{
	chain_specs,
	cli::{Cli, RelayChainCli, Subcommand},
	service::{
		new_partial, Block, LitentryParachainRuntimeExecutor, LitmusParachainRuntimeExecutor,
		RococoParachainRuntimeExecutor,
	},
};
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::{info, warn};
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
	config::{BasePath, PrometheusConfig},
	TaskManager,
};
use sp_core::{hexdisplay::HexDisplay, Encode};
use sp_runtime::traits::AccountIdConversion;
use std::net::SocketAddr;

const UNSUPPORTED_CHAIN_MESSAGE: &str = "Unsupported chain spec, please use litmus* or litentry*";

trait IdentifyChain {
	fn is_litentry(&self) -> bool;
	fn is_litmus(&self) -> bool;
	fn is_rococo(&self) -> bool;
	fn is_dev(&self) -> bool;
	fn is_standalone(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_litentry(&self) -> bool {
		// we need the combined condition as the id in our rococo spec starts with `litentry-rococo`
		// simply renaming `litentry-rococo` to `rococo` everywhere would have an impact on the
		// existing litentry-rococo chain
		self.id().starts_with("litentry") && !self.id().starts_with("litentry-rococo")
	}
	fn is_litmus(&self) -> bool {
		self.id().starts_with("litmus")
	}
	fn is_rococo(&self) -> bool {
		self.id().starts_with("litentry-rococo")
	}
	fn is_dev(&self) -> bool {
		self.id().ends_with("dev")
	}
	fn is_standalone(&self) -> bool {
		self.id().eq("standalone") || self.id().eq("dev")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_litentry(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_litentry(self)
	}
	fn is_litmus(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_litmus(self)
	}
	fn is_rococo(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_rococo(self)
	}
	fn is_dev(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_dev(self)
	}
	fn is_standalone(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_standalone(self)
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		// `--chain=standalone or --chain=dev` to start a standalone node with rococo-dev chain spec
		// mainly based on Acala's `dev` implementation
		"dev" | "standalone" => Box::new(chain_specs::rococo::get_chain_spec_dev(true)),
		// Litentry
		"litentry-dev" => Box::new(chain_specs::litentry::get_chain_spec_dev()),
		"litentry-staging" => Box::new(chain_specs::litentry::get_chain_spec_staging()),
		"litentry" => Box::new(chain_specs::litentry::ChainSpec::from_json_bytes(
			&include_bytes!("../res/chain_specs/litentry.json")[..],
		)?),
		// Litmus
		"litmus-dev" => Box::new(chain_specs::litmus::get_chain_spec_dev()),
		"litmus-staging" => Box::new(chain_specs::litmus::get_chain_spec_staging()),
		"litmus" => Box::new(chain_specs::litmus::ChainSpec::from_json_bytes(
			&include_bytes!("../res/chain_specs/litmus.json")[..],
		)?),
		// Rococo
		"rococo-dev" => Box::new(chain_specs::rococo::get_chain_spec_dev(false)),
		"rococo-staging" => Box::new(chain_specs::rococo::get_chain_spec_staging()),
		"rococo" => Box::new(chain_specs::rococo::ChainSpec::from_json_bytes(
			&include_bytes!("../res/chain_specs/rococo-170000.json")[..],
		)?),
		// Generate res/chain_specs/litentry.json
		"generate-litentry" => Box::new(chain_specs::litentry::get_chain_spec_prod()),
		// Generate res/chain_specs/litmus.json
		"generate-litmus" => Box::new(chain_specs::litmus::get_chain_spec_prod()),
		// Generate res/chain_specs/rococo.json
		// Deprecated: for rococo we are using a new chain spec which was restored from an old state
		//             see https://github.com/paritytech/subport/issues/337#issuecomment-1137882912
		"generate-rococo" => Box::new(chain_specs::rococo::get_chain_spec_prod()),
		path => {
			let chain_spec = chain_specs::ChainSpec::from_json_file(path.into())?;
			if chain_spec.is_litmus() {
				Box::new(chain_specs::litmus::ChainSpec::from_json_file(path.into())?)
			} else if chain_spec.is_rococo() {
				Box::new(chain_specs::rococo::ChainSpec::from_json_file(path.into())?)
			} else {
				// Fallback: use Litentry chain spec
				Box::new(chain_spec)
			}
		},
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Litentry node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		"Litentry node\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		litentry-collator <parachain-args> -- <relay-chain-args>"
			.into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/litentry/litentry-parachain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if chain_spec.is_litmus() {
			&litmus_parachain_runtime::VERSION
		} else if chain_spec.is_rococo() {
			&rococo_parachain_runtime::VERSION
		} else {
			// By default litentry is used
			&litentry_parachain_runtime::VERSION
		}
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Litentry node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		"Litentry node\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		litentry-collator <parachain-args> -- <relay-chain-args>"
			.into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/litentry/litentry-parachain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}

/// Creates partial components for the runtimes that are supported by the benchmarks.
macro_rules! construct_benchmark_partials {
	($config:expr, |$partials:ident| $code:expr) => {
		if $config.chain_spec.is_litmus() {
			let $partials = new_partial::<litmus_parachain_runtime::RuntimeApi, _>(
				&$config,
				false,
				crate::service::build_import_queue::<litmus_parachain_runtime::RuntimeApi>,
			)?;
			$code
		} else if $config.chain_spec.is_litentry() {
			let $partials = new_partial::<litentry_parachain_runtime::RuntimeApi, _>(
				&$config,
				false,
				crate::service::build_import_queue::<litentry_parachain_runtime::RuntimeApi>,
			)?;
			$code
		} else if $config.chain_spec.is_rococo() {
			let $partials = new_partial::<rococo_parachain_runtime::RuntimeApi, _>(
				&$config,
				false,
				crate::service::build_import_queue::<rococo_parachain_runtime::RuntimeApi>,
			)?;
			$code
		} else {
			panic!("{}", UNSUPPORTED_CHAIN_MESSAGE)
		}
	};
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;

		if runner.config().chain_spec.is_litmus() {
			runner.async_run(|$config| {
				let $components = new_partial::<
					litmus_parachain_runtime::RuntimeApi,
					_
				>(
					&$config,
					false,
					crate::service::build_import_queue::<litmus_parachain_runtime::RuntimeApi>,
				)?;
				let task_manager = $components.task_manager;
				{ $( $code )* }.map(|v| (v, task_manager))
			})
		} else if runner.config().chain_spec.is_litentry() {
			runner.async_run(|$config| {
				let $components = new_partial::<
					litentry_parachain_runtime::RuntimeApi,
					_
				>(
					&$config,
					false,
					crate::service::build_import_queue::<litentry_parachain_runtime::RuntimeApi>,
				)?;
				let task_manager = $components.task_manager;
				{ $( $code )* }.map(|v| (v, task_manager))
			})
		} else if runner.config().chain_spec.is_rococo() {
			runner.async_run(|$config| {
				let $components = new_partial::<
					rococo_parachain_runtime::RuntimeApi,
					_
				>(
					&$config,
					false,
					crate::service::build_import_queue::<rococo_parachain_runtime::RuntimeApi>,
				)?;
				let task_manager = $components.task_manager;
				{ $( $code )* }.map(|v| (v, task_manager))
			})
		} else {
			panic!("{}", UNSUPPORTED_CHAIN_MESSAGE)
		}
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::Revert(cmd)) => construct_async_run!(|components, cli, cmd, config| {
			Ok(cmd.run(components.client, components.backend, None))
		}),

		Some(Subcommand::Key(cmd)) => cmd.run(&cli),

		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				let state_version = Cli::native_runtime_version(&spec).state_version();
				cmd.run::<Block>(&*spec, state_version)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						if !runner.config().chain_spec.is_dev() {
							return Err("Only dev chain should be used in benchmark".into())
						}

						runner.sync_run(|config| {
							if config.chain_spec.is_litmus() {
								cmd.run::<Block, LitmusParachainRuntimeExecutor>(config)
							} else if config.chain_spec.is_litentry() {
								cmd.run::<Block, LitentryParachainRuntimeExecutor>(config)
							} else if config.chain_spec.is_rococo() {
								cmd.run::<Block, RococoParachainRuntimeExecutor>(config)
							} else {
								Err(UNSUPPORTED_CHAIN_MESSAGE.into())
							}
						})
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
						You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, |partials| cmd.run(partials.client))
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) =>
					return Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)
					.into()),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, |partials| {
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();

						cmd.run(config, partials.client.clone(), db, storage)
					})
				}),
				BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
				BenchmarkCmd::Machine(cmd) =>
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())),
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		Some(Subcommand::TryRuntime(cmd)) => {
			if cfg!(feature = "try-runtime") {
				let runner = cli.create_runner(cmd)?;

				// grab the task manager.
				let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					TaskManager::new(runner.config().tokio_handle.clone(), *registry)
						.map_err(|e| format!("Error: {:?}", e))?;

				if runner.config().chain_spec.is_litmus() {
					runner.async_run(|config| {
						Ok((cmd.run::<Block, LitmusParachainRuntimeExecutor>(config), task_manager))
					})
				} else if runner.config().chain_spec.is_litentry() {
					runner.async_run(|config| {
						Ok((
							cmd.run::<Block, LitentryParachainRuntimeExecutor>(config),
							task_manager,
						))
					})
				} else if runner.config().chain_spec.is_rococo() {
					runner.async_run(|config| {
						Ok((cmd.run::<Block, RococoParachainRuntimeExecutor>(config), task_manager))
					})
				} else {
					Err(UNSUPPORTED_CHAIN_MESSAGE.into())
				}
			} else {
				Err("Try-runtime must be enabled by `--features try-runtime`.".into())
			}
		},
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();
			let is_standalone = runner.config().chain_spec.is_standalone();

			runner.run_node_until_exit(|config| async move {
				if is_standalone {
					return crate::service::start_standalone_node::<rococo_parachain_runtime::RuntimeApi>(
						config
					)
					.await
					.map_err(Into::into)
				}

				let hwbench = if !cli.no_hardware_benchmarks {
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})
				} else {
					None
				};

				let para_id = chain_specs::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);

				let state_version = Cli::native_runtime_version(&config.chain_spec).state_version();
				let block: Block = generate_genesis_block(&*config.chain_spec, state_version)
					.map_err(|e| format!("{:?}", e))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header.encode()));

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Parachain genesis state: {}", genesis_state);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				if collator_options.relay_chain_rpc_url.is_some() && !cli.relay_chain_args.is_empty() {
					warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
				}
				if config.chain_spec.is_litmus() {
					crate::service::start_node::<litmus_parachain_runtime::RuntimeApi>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into)
				} else if config.chain_spec.is_litentry() {
					crate::service::start_node::<litentry_parachain_runtime::RuntimeApi>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into)
				} else if config.chain_spec.is_rococo() {
					crate::service::start_node::<rococo_parachain_runtime::RuntimeApi>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into)
				} else {
					Err(UNSUPPORTED_CHAIN_MESSAGE.into())
				}
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_ws_listen_port() -> u16 {
		9945
	}

	fn rpc_http_listen_port() -> u16 {
		9934
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_ws(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}
