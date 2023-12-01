#![allow(warnings)]

extern crate litentry_cli;
use litentry_cli::{
	base_cli::BaseCommand,
	commands::Commands,
	get_layer_two_nonce,
	trusted_base_cli::TrustedBaseCommand,
	trusted_cli::{TrustedCli, TrustedCommand},
	trusted_command_utils::get_identifiers,
	trusted_operation::{perform_direct_operation, perform_trusted_operation},
	Cli,
};
use log::debug;
use std::time::Instant;

use ita_stf::{
	Identity, Index, LitentryMultiSignature, RequestAesKey, TrustedCall, TrustedCallSigned,
	TrustedOperation, H256,
};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::{aes_encrypt_nonce, Assertion, Web3CommonValidationData, Web3Network};

use codec::{Decode, Encode};
use ita_stf::TrustedCallSigning;
use itp_stf_primitives::types::ShardIdentifier;
use sp_core::{blake2_256, sr25519, Pair};
use std::sync::Arc;

pub const COMMON_SEED: &[u8] =
	b"crouch whisper apple ladder skull blouse ridge oven despair cloth pony";

pub const COMMON_AES: &[u8] = b"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12";

pub struct User {
	pub shielding_key: Option<[u8; 32]>,
	pub identity: Identity,
	pub substrate_identity: Option<Identity>,
	pub evm_identity: Option<Identity>,
	pub nonce: u32,
	pub prime_pair: sr25519::Pair,
	pub substrate_pair: Option<sr25519::Pair>,
	pub evm_pair: Option<sr25519::Pair>,
	pub user_shielding_key_set: bool,
	pub link_identity: bool,
}

impl User {
	pub fn generate_user_with_prime_junction(prime_junction: u32) -> User {
		let derive_junction_pair = format!("{:?}//{}", COMMON_SEED, prime_junction);
		// Creating additional blake256 layer for hashing the correct seed layer
		let seed = blake2_256(derive_junction_pair.as_bytes()).to_vec();
		assert_eq!(seed.len(), 32);
		let pair = sr25519::Pair::from_seed_slice(&seed)
			.expect("Failed to create a key pair from the provided seed");
		// Generate User
		User::new(None, pair.public().into(), None, None, pair, None, None)
	}

	pub fn new(
		shielding_key: Option<RequestAesKey>,
		identity: Identity,
		substrate_identity: Option<Identity>,
		evm_identity: Option<Identity>,
		prime_pair: sr25519::Pair,
		substrate_pair: Option<sr25519::Pair>,
		evm_pair: Option<sr25519::Pair>,
	) -> Self {
		Self {
			shielding_key,
			identity,
			substrate_identity,
			evm_identity,
			nonce: 0,
			prime_pair,
			substrate_pair,
			evm_pair,
			user_shielding_key_set: false,
			link_identity: false,
		}
	}

	pub fn create_substrate_identity_with_junction(self, junction: u32) -> User {
		let derive_junction_pair = format!("{:?}//{}", COMMON_SEED, junction);
		let seed = blake2_256(derive_junction_pair.as_bytes()).to_vec();
		let pair = sr25519::Pair::from_seed_slice(&seed)
			.expect("Failed to create a key pair from the provided seed");
		User::new(
			self.shielding_key,
			self.identity,
			Some(pair.public().into()),
			self.evm_identity,
			self.prime_pair,
			Some(pair),
			None,
		)
	}

	pub fn create_evm_identity_with_junction(self, junction: u32) -> User {
		let derive_junction_pair = format!("{:?}//{}", COMMON_SEED, junction);
		let seed = blake2_256(derive_junction_pair.as_bytes()).to_vec();
		let pair = sr25519::Pair::from_seed_slice(&seed)
			.expect("Failed to create a key pair from the provided seed");
		User::new(
			self.shielding_key,
			self.identity,
			self.substrate_identity,
			Some(pair.public().into()),
			self.prime_pair,
			None,
			Some(pair),
		)
	}

	// TODO: Construct request_vc function
	pub fn request_vc(&self, cli: &Cli, trusted_cli: &TrustedCli) -> Result<(), String> {
		let alice = self.prime_pair.clone();
		let id = self.identity.clone();

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);
		let assertion = Assertion::A1;

		let mut key = RequestAesKey::default();
		hex::decode_to_slice(
			"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
			&mut key,
		)
		.expect("decoding shielding_key failed");

		let top = TrustedCall::request_vc(
			alice.public().into(),
			id,
			assertion,
			Some(key),
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);

		let start = Instant::now();

		// TODO: P-177, print actual VC content to stdout
		perform_direct_operation(cli, trusted_cli, &top, key)
			.map_err(|e| format!("Failed to perform direct operation: {:?}", e))?;

		let duration = start.elapsed();
		println!("The time taken is: {:?}", duration);

		Ok(())
	}

	pub fn increment_nonce(&mut self) {
		self.nonce += 1;
	}
}

pub fn generate_dummy_cli() -> Cli {
	Cli {
		node_url: "ws://127.0.0.1".to_string(),
		node_port: "9944".to_string(),
		worker_url: "wss://127.0.0.1".to_string(),
		trusted_worker_port: "2000".to_string(),
		command: Commands::Base(BaseCommand::PrintMetadata),
	}
}

pub fn generate_dummy_trusted_cli() -> TrustedCli {
	TrustedCli {
		mrenclave: None,
		shard: None,
		xt_signer: "//Alice".to_string(),
		direct: true,
		command: TrustedCommand::BaseTrusted(TrustedBaseCommand::ListAccounts),
	}
}

#[tokio::main]
async fn main() -> Result<(), String> {
	println!("Starting smoke test!");
	let cli = generate_dummy_cli();
	let trusted_cli = generate_dummy_trusted_cli();
	let cli = Arc::new(cli);
	let trusted_cli = Arc::new(trusted_cli);
	for x in 0..10 {
		let copied_cli = cli.clone();
		let copied_trusted_cli = trusted_cli.clone();
		// We are not awaiting, just to bombard it with requests
		let user = User::generate_user_with_prime_junction(x);
		user.request_vc(&copied_cli, &copied_trusted_cli);
	}
	Ok(())
}
