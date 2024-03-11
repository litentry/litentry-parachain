// Copyright 2020-2024 Trust Computing GmbH.
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

use crate::{dynamic::repository::SmartContractRepository, *};
use ethabi::{decode, encode, ParamType, Token};
use evm::{
	backend::{MemoryBackend, MemoryVicinity},
	executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
	Config,
};
use itp_types::Assertion;
use lc_credentials::{assertion_logic::AssertionLogic, Credential};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::Identity;
use log::{error, info};
use precompiles::Precompiles;
use primitive_types::{H160, U256};
use std::{collections::BTreeMap, println};

mod precompiles;
pub mod repository;

pub fn build<SC: SmartContractRepository>(
	req: &AssertionBuildRequest,
	smart_contract_id: H160,
	repository: SC,
) -> Result<Credential> {
	let input = prepare_execute_call_input(&req.identities);

	let smart_contract_byte_code = repository.get(&smart_contract_id).unwrap();
	let (description, assertion_type, assertions, schema_url, result) =
		execute_smart_contract(smart_contract_byte_code, input);
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			let mut assertion_values: Vec<AssertionLogic> = vec![];
			for assertion in assertions {
				let logic: AssertionLogic = serde_json::from_str(&assertion).map_err(|e| {
					Error::RequestVCFailed(
						Assertion::Dynamic(smart_contract_id),
						ErrorDetail::StfError(ErrorString::truncate_from(format!("{}", e).into())),
					)
				})?;
				assertion_values.push(logic);
			}

			credential_unsigned.update_dynamic(
				description,
				assertion_type,
				assertion_values,
				schema_url,
				result,
			);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Dynamic(smart_contract_id),
				e.into_error_detail(),
			))
		},
	}
}

pub fn execute_smart_contract(
	smart_contract_byte_code: Vec<u8>,
	input: Vec<u8>,
) -> (String, String, Vec<String>, String, bool) {
	println!("Executing smart contract with input: {:?}", hex::encode(&input));

	// prepare EVM runtime
	let config = prepare_config();
	let vicinity = prepare_memory();
	let state = BTreeMap::new();
	let mut backend = MemoryBackend::new(&vicinity, state);
	let metadata = StackSubstateMetadata::new(u64::MAX, &config);
	let state = MemoryStackState::new(metadata, &mut backend);
	let precompiles = Precompiles {};
	let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

	// caller
	let caller = hash(5); //0x04

	// deploy smart contract
	let address = executor.create_address(evm::CreateScheme::Legacy { caller: hash(5) });
	let _create_result = executor.transact_create(
		caller,
		U256::zero(),
		smart_contract_byte_code,
		u64::MAX,
		Vec::new(),
	);
	// call function
	let call_reason =
		executor.transact_call(caller, address, U256::zero(), input, u64::MAX, Vec::new());

	info!("Contract execution result {:?}", &call_reason);
	decode_result(&call_reason.1)
}

fn decode_result(data: &[u8]) -> (String, String, Vec<String>, String, bool) {
	let types = vec![
		ParamType::String,
		ParamType::String,
		ParamType::Array(ParamType::String.into()),
		ParamType::String,
		ParamType::Bool,
	];
	let decoded = decode(&types, data).unwrap();
	(
		decoded[0].clone().into_string().unwrap(),
		decoded[1].clone().into_string().unwrap(),
		decoded[2]
			.clone()
			.into_array()
			.unwrap()
			.into_iter()
			.map(|p| p.into_string().unwrap())
			.collect(),
		decoded[3].clone().into_string().unwrap(),
		decoded[4].clone().into_bool().unwrap(),
	)
}

fn prepare_execute_call_input(identities: &[IdentityNetworkTuple]) -> Vec<u8> {
	let identities: Vec<Token> = identities.iter().map(identity_with_networks_to_token).collect();

	let encoded_identities = encode(&[Token::Array(identities)]);
	let encoded_identities_as_bytes = encode(&[Token::Bytes(encoded_identities)]);
	let function_hash = "09c5eabe";
	prepare_function_call_input(function_hash, encoded_identities_as_bytes)
}

pub fn identity_with_networks_to_token(identity: &IdentityNetworkTuple) -> Token {
	let (type_index, value) = match &identity.0 {
		Identity::Twitter(str) => (0, str.inner_ref().to_vec()),
		Identity::Discord(str) => (1, str.inner_ref().to_vec()),
		Identity::Github(str) => (2, str.inner_ref().to_vec()),
		Identity::Substrate(addr) => (3, addr.as_ref().to_vec()),
		Identity::Evm(addr) => (4, addr.as_ref().to_vec()),
		Identity::Bitcoin(addr) => (5, addr.as_ref().to_vec()),
		Identity::Solana(addr) => (6, addr.as_ref().to_vec()),
	};
	let networks: Vec<Token> = identity.1.iter().map(network_to_token).collect();
	Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value), Token::Array(networks)])
}

pub fn network_to_token(network: &Web3Network) -> Token {
	Token::Uint(
		match network {
			Web3Network::Polkadot => 0,
			Web3Network::Kusama => 1,
			Web3Network::Litentry => 2,
			Web3Network::Litmus => 3,
			Web3Network::LitentryRococo => 4,
			Web3Network::Khala => 5,
			Web3Network::SubstrateTestnet => 6,
			Web3Network::Ethereum => 7,
			Web3Network::Bsc => 8,
			Web3Network::BitcoinP2tr => 9,
			Web3Network::BitcoinP2pkh => 10,
			Web3Network::BitcoinP2sh => 11,
			Web3Network::BitcoinP2wpkh => 12,
			Web3Network::BitcoinP2wsh => 13,
			Web3Network::Polygon => 14,
			Web3Network::Arbitrum => 15,
			Web3Network::Solana => 16,
		}
		.into(),
	)
}

fn prepare_function_call_input(function_hash: &str, mut input: Vec<u8>) -> Vec<u8> {
	let mut call_input = hex::decode(function_hash).unwrap();
	call_input.append(&mut input);
	call_input
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

fn prepare_config() -> Config {
	let mut config = Config::frontier();
	config.has_bitwise_shifting = true;
	config.err_on_call_with_more_gas = false;
	config
}

fn prepare_memory() -> MemoryVicinity {
	MemoryVicinity {
		gas_price: U256::zero(),
		origin: H160::default(),
		block_hashes: Vec::new(),
		block_number: Default::default(),
		block_coinbase: Default::default(),
		block_timestamp: Default::default(),
		block_difficulty: Default::default(),
		block_gas_limit: Default::default(),
		chain_id: U256::one(),
		block_base_fee_per_gas: U256::zero(),
		block_randomness: None,
	}
}

#[cfg(test)]
pub mod tests {
	use crate::dynamic::{
		build, identity_with_networks_to_token, repository::InMemorySmartContractRepo, U256,
	};
	use ethabi::Token;
	use itp_types::Assertion;
	use lc_stf_task_sender::AssertionBuildRequest;
	use litentry_primitives::{Address32, Identity, IdentityString, Web3Network};
	use sp_core::{crypto::AccountId32, H160};

	#[test]
	pub fn should_tokenize_identity_with_networks() {
		// given
		let identity = Identity::Substrate(Address32::from([0u8; 32]));
		let networks = vec![Web3Network::Polkadot, Web3Network::Litentry];

		// when
		let token = identity_with_networks_to_token(&(identity, networks));

		// then
		match token {
			Token::Tuple(tokens) => {
				assert_eq!(tokens.len(), 3);
				match tokens.get(0).unwrap() {
					Token::Uint(value) => {
						assert_eq!(value, &Into::<U256>::into(3))
					},
					_ => panic!("Expected Token::Uint"),
				};
				match tokens.get(1).unwrap() {
					Token::Bytes(value) => {
						assert_eq!(value, &[0u8; 32].to_vec())
					},
					_ => panic!("Expected Token::Bytes"),
				}
				match tokens.get(2).unwrap() {
					Token::Array(network_tokens) => {
						assert_eq!(network_tokens.len(), 2);
						match network_tokens.get(0).unwrap() {
							Token::Uint(value) => {
								assert_eq!(value, &Into::<U256>::into(0))
							},
							_ => panic!("Expected Token::Uint"),
						}
						match network_tokens.get(1).unwrap() {
							Token::Uint(value) => {
								assert_eq!(value, &Into::<U256>::into(2))
							},
							_ => panic!("Expected Token::Uint"),
						}
					},
					_ => panic!("Expected Token::Array"),
				}
			},
			_ => panic!("Expected Token::Tuple"),
		}
	}

	#[test]
	pub fn test_a1_true() {
		let _ = env_logger::builder().is_test(true).try_init();
		// given
		let twitter_identity = Identity::Twitter(IdentityString::new(vec![]));
		let substrate_identity = Identity::Substrate(AccountId32::new([0; 32]).into());

		let request = AssertionBuildRequest {
			shard: Default::default(),
			signer: AccountId32::new([0; 32]),
			who: Identity::Twitter(IdentityString::new(vec![])),
			assertion: Assertion::Dynamic(hash(0)),
			identities: vec![(twitter_identity, vec![]), (substrate_identity, vec![])],
			top_hash: Default::default(),
			parachain_block_number: Default::default(),
			sidechain_block_number: Default::default(),
			maybe_key: None,
			req_ext_hash: Default::default(),
			should_create_id_graph: Default::default(),
		};

		let repository = InMemorySmartContractRepo::new();

		// when
		let credential = build(&request, hash(0), repository).unwrap();

		// then
		assert!(credential.credential_subject.values[0]);
	}

	#[test]
	pub fn test_a1_false() {
		let _ = env_logger::builder().is_test(true).try_init();
		// given
		let twitter_identity = Identity::Twitter(IdentityString::new(vec![]));

		let request = AssertionBuildRequest {
			shard: Default::default(),
			signer: AccountId32::new([0; 32]),
			who: Identity::Twitter(IdentityString::new(vec![])),
			assertion: Assertion::Dynamic(hash(0)),
			identities: vec![(twitter_identity, vec![])],
			top_hash: Default::default(),
			parachain_block_number: Default::default(),
			sidechain_block_number: Default::default(),
			maybe_key: None,
			req_ext_hash: Default::default(),
			should_create_id_graph: Default::default(),
		};

		let repository = InMemorySmartContractRepo::new();

		// when
		let credential = build(&request, hash(0), repository).unwrap();

		// then
		assert!(!credential.credential_subject.values[0]);
	}

	fn hash(a: u64) -> H160 {
		H160::from_low_u64_be(a)
	}
}
