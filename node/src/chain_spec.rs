use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachain_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

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

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
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
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		None,
		Extensions { relay_chain: "westend".into(), para_id: id.into() },
	)
}

pub fn staging_test_net(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
				vec![
					(
						// $secret//one
						hex!["aad9fa2249f87a210a0f93400b7f90e47b810c6d65caa0ca3f5af982904c2a33"]
							.into(),
						hex!["aad9fa2249f87a210a0f93400b7f90e47b810c6d65caa0ca3f5af982904c2a33"]
							.unchecked_into(),
					),
					(
						// $secret//two
						hex!["d47753f0cca9dd8da00c70e82ec4fc5501a69c49a5952a643d18802837c88212"]
							.into(),
						hex!["d47753f0cca9dd8da00c70e82ec4fc5501a69c49a5952a643d18802837c88212"]
							.unchecked_into(),
					)
				],
				vec![
					hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into()
				],
				id,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Extensions { relay_chain: "westend".into(), para_id: id.into() },
	)
}

// TODO: update this
const LITENTRY_ED: u128 = 100_000_000_000;

fn testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	let num_endowed_accounts = endowed_accounts.len();

	parachain_runtime::GenesisConfig {
		system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: parachain_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, LITENTRY_ED * 4096))
				.collect(),
		},
		sudo: parachain_runtime::SudoConfig { key: root_key },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: LITENTRY_ED * 16,
			..Default::default()
		},
		session: parachain_runtime::SessionConfig {
			keys: invulnerables
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),                  // account id
						acc.clone(),                  // validator id
						parachain_runtime::SessionKeys{aura}, // session keys
					)
				})
				.collect(),
		},
		democracy: parachain_runtime::DemocracyConfig::default(),
		council: parachain_runtime::CouncilConfig::default(),
		technical_committee: parachain_runtime::TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		treasury: Default::default(),
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
