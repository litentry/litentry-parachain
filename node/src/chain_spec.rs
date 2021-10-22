use cumulus_primitives_core::ParaId;
use litentry_parachain_runtime::{
	AccountId, AuraId, Balance, BalancesConfig, CollatorSelectionConfig, CouncilConfig,
	CouncilMembershipConfig, DemocracyConfig, GenesisConfig, ParachainInfoConfig, SessionConfig,
	Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	TechnicalCommitteeMembershipConfig, UNIT, WASM_BINARY,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate a crypto pair from seed
pub fn get_pair_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_pair_from_seed::<AuraId>(seed)
}

pub fn parachain_properties(symbol: &str, decimals: u32, ss58format: u32) -> Option<Properties> {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	Some(properties)
}

/// Get default parachain properties for Litentry which will be filled into chain spec
pub fn default_parachain_properties() -> Option<Properties> {
	parachain_properties("LIT", 12, 31)
}

const DEFAULT_CANDIDACY_BOND: Balance = 16 * UNIT;
const DEFAULT_ENDOWED_ACCOUNT_BALANCE: Balance = 1000 * UNIT;

/// GenesisInfo struct to store the parsed genesis_info JSON
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GenesisInfo {
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	candidacy_bond: String,
	endowed_accounts: Vec<(AccountId, String)>,
	council_membership: Vec<AccountId>,
	technical_committee_membership: Vec<AccountId>,
	boot_nodes: Vec<String>,
	telemetry_endpoints: Vec<String>,
}

pub fn get_chain_spec_dev(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Litentry-dev",
		"Litentry-dev",
		ChainType::Development,
		move || {
			generate_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				DEFAULT_CANDIDACY_BOND,
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
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_ENDOWED_ACCOUNT_BALANCE,
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
				],
				id,
			)
		},
		vec![],
		None,
		Some("Litentry"),
		default_parachain_properties(),
		Extensions { relay_chain: "rococo-local".into(), para_id: id.into() },
	)
}

pub fn get_chain_spec_staging(id: ParaId) -> ChainSpec {
	get_chain_spec_from_genesis_info(
		include_bytes!("../res/genesis_info/staging.json"),
		"Litentry-staging",
		"Litentry-staging",
		ChainType::Local,
		"rococo-local".into(),
		id,
	)
}

pub fn get_chain_spec_prod(id: ParaId) -> ChainSpec {
	get_chain_spec_from_genesis_info(
		include_bytes!("../res/genesis_info/prod.json"),
		"Litentry",
		"Litentry",
		ChainType::Live,
		"polkadot".into(),
		id,
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
				u128::from_str(&genesis_info_cloned.candidacy_bond)
					.expect("Bad candicy bond; qed."),
				genesis_info_cloned
					.endowed_accounts
					.into_iter()
					.map(|(k, b)| (k, u128::from_str(&b).expect("Bad endowed balance; qed.")))
					.collect(),
				genesis_info_cloned.council_membership,
				genesis_info_cloned.technical_committee_membership,
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
		Some("Litentry"),
		default_parachain_properties(),
		Extensions { relay_chain: relay_chain_name, para_id: para_id.into() },
	)
}

fn generate_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	candicy_bond: Balance,
	endowed_accounts: Vec<(AccountId, Balance)>,
	council_membership: Vec<AccountId>,
	technical_committee_membership: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig { balances: endowed_accounts },
		sudo: SudoConfig { key: root_key },
		parachain_info: ParachainInfoConfig { parachain_id: id },
		collator_selection: CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: candicy_bond,
			..Default::default()
		},
		session: SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                                      // account id
						acc.clone(),                                      // validator id
						litentry_parachain_runtime::SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		democracy: DemocracyConfig::default(),
		council: CouncilConfig::default(),
		council_membership: CouncilMembershipConfig {
			members: council_membership,
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig::default(),
		technical_committee_membership: TechnicalCommitteeMembershipConfig {
			members: technical_committee_membership,
			phantom: Default::default(),
		},
		treasury: Default::default(),
		vesting: Default::default(),
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
