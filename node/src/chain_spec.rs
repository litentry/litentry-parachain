use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use litentry_parachain_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<litentry_parachain_runtime::GenesisConfig, Extensions>;

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

pub fn get_chain_spec_dev(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"litentry-dev",
		"litentry-dev",
		ChainType::Development,
		move || {
			default_genesis(
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
		Some("Litentry"),
		default_parachain_properties(),
		Extensions { relay_chain: "rococo-local".into(), para_id: id.into() },
	)
}

pub fn get_chain_spec_staging(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"litentry-staging",
		"litentry-staging",
		ChainType::Local,
		move || {
			default_genesis(
				// Staging keys are derivative keys based on a single master secret phrase:
				//
				// root: 	$SECRET
				// account:	$SECRET//collator//<id>
				// aura: 	$SECRET//collator//<id>//aura

				// 5DaB1AshD6NsRRq84rsLuR65aD8tTPxp2Ub7HnqVtirgWn4V
				hex!["42b5bbd733848b2207070115b0ed7479ea391f58c7c703cbdb960333005a4f67"].into(),
				vec![
					(
						// 5FZP2oqDBBWzaKp8STUQKTvSo2Y1UD68briboWreLiVAxJr1
						hex!["9a937224ffe6f9ec81301a63739e399836a77b77c5e7c59f9dcf75ee674e040b"]
							.into(),
						// 5HbUXue4BsoBmR1ZSWCurQMTdi2jrDdVzMQoKtc8ByMH9uEc
						hex!["f4a4ec8eca5abe1f2a84690e4f999fdc4ae0b95abad33fcd9ed222a3fba7876f"]
							.unchecked_into(),
					),
					(
						// 5EJLoe5Uaj8U7jTxJwiDCP1kNSnHu4Buw8pdkpm136QRiAEC
						hex!["62df08d3d47b89aa675268f30e516b3614e01fd888d92bb4d0d0733cc564f04d"]
							.into(),
						// 5E9ky6gxEMAHrVRLyubyL4UqCkdt5kJmHkB7HuxJ3Y5LDvm3
						hex!["5c532a810bd75624694109c1f2cb735c6b504b4c0ad5035e738415395272a73c"]
							.unchecked_into(),
					),
				],
				vec![
					hex!["9a937224ffe6f9ec81301a63739e399836a77b77c5e7c59f9dcf75ee674e040b"].into(),
					hex!["62df08d3d47b89aa675268f30e516b3614e01fd888d92bb4d0d0733cc564f04d"].into(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
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

// TODO: update this
const LITENTRY_ED: u128 = 100_000_000_000;

fn default_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> litentry_parachain_runtime::GenesisConfig {
	let num_endowed_accounts = endowed_accounts.len();

	litentry_parachain_runtime::GenesisConfig {
		system: litentry_parachain_runtime::SystemConfig {
			code: litentry_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: litentry_parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, LITENTRY_ED * 4096)).collect(),
		},
		sudo: litentry_parachain_runtime::SudoConfig { key: root_key },
		parachain_info: litentry_parachain_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: litentry_parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: LITENTRY_ED * 16,
			..Default::default()
		},
		session: litentry_parachain_runtime::SessionConfig {
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
		democracy: litentry_parachain_runtime::DemocracyConfig::default(),
		council: litentry_parachain_runtime::CouncilConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_committee: litentry_parachain_runtime::TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		treasury: Default::default(),
		vesting: Default::default(),
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
