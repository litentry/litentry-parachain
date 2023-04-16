// Copyright 2020-2023 Litentry Technologies GmbH.
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

use super::*;
use cumulus_primitives_core::ParaId;
use rococo_parachain_runtime::{
	AccountId, AuraId, Balance, BalancesConfig, CouncilMembershipConfig, GenesisConfig,
	ParachainInfoConfig, ParachainStakingConfig, PolkadotXcmConfig, SessionConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeMembershipConfig, TeerexConfig, VCManagementConfig, UNIT,
	WASM_BINARY,
};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::Deserialize;
use sp_core::sr25519;

const DEFAULT_PARA_ID: u32 = 2106;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Get default parachain properties for Rococo which will be filled into chain spec
/// Currently, we use 131 as the SS58Prefix (same as Litmus)
fn default_parachain_properties() -> Option<Properties> {
	parachain_properties("LIT", 12, 131)
}

const DEFAULT_ENDOWED_ACCOUNT_BALANCE: Balance = 1000 * UNIT;

/// GenesisInfo struct to store the parsed genesis_info JSON
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GenesisInfo {
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, String)>,
	council: Vec<AccountId>,
	technical_committee: Vec<AccountId>,
	boot_nodes: Vec<String>,
	telemetry_endpoints: Vec<String>,
}

pub fn get_chain_spec_dev(is_standalone: bool) -> ChainSpec {
	let id = if is_standalone { "standalone" } else { "litentry-rococo-dev" };
	ChainSpec::from_genesis(
		"Litentry-rococo-dev",
		id,
		ChainType::Development,
		move || {
			generate_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
				)],
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				DEFAULT_PARA_ID.into(),
			)
		},
		Vec::new(),
		None,
		Some("litentry-rococo"),
		None,
		default_parachain_properties(),
		Extensions { relay_chain: "rococo-local".into(), para_id: DEFAULT_PARA_ID },
	)
}

pub fn get_chain_spec_staging() -> ChainSpec {
	// Staging keys are derivative keys based on a single master secret phrase:
	//
	// root: 	$SECRET
	// account:	$SECRET//collator//<id>
	// aura: 	$SECRET//collator//<id>//aura
	get_chain_spec_from_genesis_info(
		include_bytes!("../../res/genesis_info/staging.json"),
		"Litentry-rococo-staging",
		"litentry-rococo-staging",
		ChainType::Local,
		"rococo-local".into(),
		DEFAULT_PARA_ID.into(),
	)
}

pub fn get_chain_spec_prod() -> ChainSpec {
	get_chain_spec_from_genesis_info(
		include_bytes!("../../res/genesis_info/rococo.json"),
		"Litentry-rococo",
		"litentry-rococo",
		ChainType::Live,
		"rococo".into(),
		DEFAULT_PARA_ID.into(),
	)
}

/// Private function to get a ChainSpec from a `genesis_info_json_file`,
/// used in both staging and prod env.
fn get_chain_spec_from_genesis_info(
	genesis_info_bytes: &[u8],
	name: &str,
	id: &str,
	chain_type: ChainType,
	relay_chain_name: String,
	para_id: ParaId,
) -> ChainSpec {
	let genesis_info: GenesisInfo =
		serde_json::from_slice(genesis_info_bytes).expect("Invalid GenesisInfo; qed.");

	let boot_nodes = genesis_info.boot_nodes.clone();
	let telemetry_endpoints = genesis_info.telemetry_endpoints.clone();

	ChainSpec::from_genesis(
		name,
		id,
		chain_type,
		move || {
			use std::str::FromStr;
			let genesis_info_cloned = genesis_info.clone();
			generate_genesis(
				genesis_info_cloned.root_key,
				genesis_info_cloned.invulnerables,
				genesis_info_cloned
					.endowed_accounts
					.into_iter()
					.map(|(k, b)| (k, u128::from_str(&b).expect("Bad endowed balance; qed.")))
					.collect(),
				genesis_info_cloned.council,
				genesis_info_cloned.technical_committee,
				para_id,
			)
		},
		boot_nodes
			.into_iter()
			.map(|k| k.parse().expect("Wrong bootnode format; qed."))
			.collect(),
		Some(
			TelemetryEndpoints::new(
				telemetry_endpoints
					.into_iter()
					.map(|k| (k, 0)) // 0 is verbose level
					.collect(),
			)
			.expect("Invalid telemetry URL; qed."),
		),
		Some("litentry-rococo"),
		None,
		default_parachain_properties(),
		Extensions { relay_chain: relay_chain_name, para_id: para_id.into() },
	)
}

fn generate_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	council_members: Vec<AccountId>,
	technical_committee_members: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
		},
		balances: BalancesConfig { balances: endowed_accounts },
		sudo: SudoConfig { key: Some(root_key.clone()) },
		parachain_info: ParachainInfoConfig { parachain_id: id },
		parachain_staking: ParachainStakingConfig {
			candidates: invulnerables.iter().cloned().map(|(acc, _)| (acc, 50 * UNIT)).collect(),
			..Default::default()
		},
		session: SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                                    // account id
						acc,                                            // validator id
						rococo_parachain_runtime::SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		democracy: Default::default(),
		council: Default::default(),
		council_membership: CouncilMembershipConfig {
			members: council_members.try_into().expect("error convert to BoundedVec"),
			phantom: Default::default(),
		},
		technical_committee: Default::default(),
		technical_committee_membership: TechnicalCommitteeMembershipConfig {
			members: technical_committee_members.try_into().expect("error convert to BoundedVec"),
			phantom: Default::default(),
		},
		treasury: Default::default(),
		vesting: Default::default(),
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: PolkadotXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },
		// use sudo key as genesis admin for teerex and VCMP
		teerex: TeerexConfig { allow_sgx_debug_mode: true, admin: Some(root_key.clone()) },
		vc_management: VCManagementConfig { admin: Some(root_key) },
		transaction_payment: Default::default(),
		tokens: Default::default(),
	}
}
