#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

mod precompile;
pub mod repository;

use crate::*;
use ethabi::{decode, encode, ParamType, Token};
use evm::{
	backend::{MemoryBackend, MemoryVicinity},
	executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
	Config,
};
use itp_types::Assertion;
use lc_credentials::Credential;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::Identity;
use log::{error, info};
use precompile::Precompiles;
use primitive_types::{H160, U256};
use std::{collections::BTreeMap, println};
use crate::dynamic::repository::SmartContractRepository;

pub fn build<SC: SmartContractRepository>(req: &AssertionBuildRequest, smart_contract_id: H160, repository: SC) -> Result<Credential> {
	// let smart_contract_byte_code = hex::decode("608060405234801561001057600080fd5b5061077f806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806309c5eabe1461003b578063c66e70f014610067575b600080fd5b61004e6100493660046103cd565b61008a565b60405161005e9493929190610466565b60405180910390f35b61007a6100753660046104c8565b6100c1565b604051901515815260200161005e565b6060806060600080858060200190518101906100a69190610548565b90506100b1816100e6565b9450945094509450509193509193565b6000816000015163ffffffff16600414156100de57506001919050565b506000919050565b60608060606000806040518060800160405280604581526020016106ce60459139905060006040518060400160405280601b81526020017f4261736963204964656e7469747920566572696669636174696f6e00000000008152509050600060405180606001604052806037815260200161071360379139905060008080805b8b518110156101dd576101918c82815181106101845761018461068e565b60200260200101516101fc565b1561019f57600191506101cb565b6101c18c82815181106101b4576101b461068e565b602002602001015161022b565b156101cb57600192505b806101d5816106a4565b915050610166565b508080156101e85750815b959b949a5092985093965091945050505050565b600061020782610254565b8061021657506102168261026b565b80610225575061022582610288565b92915050565b6000610236826102a5565b806102455750610245826100c1565b806102255750610225826102c2565b805160009063ffffffff166100de57506001919050565b6000816000015163ffffffff16600114156100de57506001919050565b6000816000015163ffffffff16600214156100de57506001919050565b6000816000015163ffffffff16600314156100de57506001919050565b6000816000015163ffffffff16600514156100de57506001919050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610318576103186102df565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715610347576103476102df565b604052919050565b600067ffffffffffffffff821115610369576103696102df565b50601f01601f191660200190565b600082601f83011261038857600080fd5b813561039b6103968261034f565b61031e565b8181528460208386010111156103b057600080fd5b816020850160208301376000918101602001919091529392505050565b6000602082840312156103df57600080fd5b813567ffffffffffffffff8111156103f657600080fd5b61040284828501610377565b949350505050565b60005b8381101561042557818101518382015260200161040d565b83811115610434576000848401525b50505050565b6000815180845261045281602086016020860161040a565b601f01601f19169290920160200192915050565b608081526000610479608083018761043a565b828103602084015261048b818761043a565b9050828103604084015261049f818661043a565b915050821515606083015295945050505050565b63ffffffff811681146104c557600080fd5b50565b6000602082840312156104da57600080fd5b813567ffffffffffffffff808211156104f257600080fd5b908301906040828603121561050657600080fd5b61050e6102f5565b8235610519816104b3565b815260208301358281111561052d57600080fd5b61053987828601610377565b60208301525095945050505050565b6000602080838503121561055b57600080fd5b825167ffffffffffffffff8082111561057357600080fd5b818501915085601f83011261058757600080fd5b815181811115610599576105996102df565b8060051b6105a885820161031e565b91825283810185019185810190898411156105c257600080fd5b86860192505b83831015610681578251858111156105e05760008081fd5b86016040818c03601f19018113156105f85760008081fd5b6106006102f5565b8983015161060d816104b3565b815282820151888111156106215760008081fd5b8084019350508c603f8401126106375760008081fd5b898301516106476103968261034f565b8181528e8483870101111561065c5760008081fd5b61066b828d830186880161040a565b828c0152508452505091860191908601906105c8565b9998505050505050505050565b634e487b7160e01b600052603260045260246000fd5b60006000198214156106c657634e487b7160e01b600052601160045260246000fd5b506001019056fe596f75277665206964656e746966696564206174206c65617374206f6e65206163636f756e742f6164647265737320696e20626f7468205765623220616e6420576562332e246861735f776562325f6163636f756e74203d3d207472756520616e6420246861735f776562335f6163636f756e74203d3d2074727565a26469706673582212208271aba6061226250b47b78fd11d6561b035772536a30a99688373d21cf3a9c464736f6c63430008080033").unwrap();
	let input = prepare_execute_call_input(&req.identities);

	let smart_contract_byte_code = repository.get(&smart_contract_id).unwrap();
	let (description, assertion_type, assertion, result) =
		execute_smart_contract(smart_contract_byte_code, input);
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_dynamic(description, assertion_type, assertion, result);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::Dynamic(smart_contract_id), e.into_error_detail()))
		},
	}
}

pub fn execute_smart_contract(
	smart_contract_byte_code: Vec<u8>,
	input: Vec<u8>,
) -> (String, String, String, bool) {
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
	let caller = hash(4);

	// deploy smart contract
	let address = executor.create_address(evm::CreateScheme::Legacy { caller: hash(4) });
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

fn decode_result(data: &[u8]) -> (String, String, String, bool) {
	let types = vec![ParamType::String, ParamType::String, ParamType::String, ParamType::Bool];
	let decoded = decode(&types, data).unwrap();
	(
		decoded[0].clone().into_string().unwrap(),
		decoded[1].clone().into_string().unwrap(),
		decoded[2].clone().into_string().unwrap(),
		decoded[3].clone().into_bool().unwrap(),
	)
}

fn prepare_execute_call_input(identities: &[IdentityNetworkTuple]) -> Vec<u8> {
	let identities: Vec<Token> =
		identities.iter().map(|identity| identity_to_token(&identity.0)).collect();

	let encoded_identities = encode(&[Token::Array(identities)]);
	let encoded_identities_as_bytes = encode(&[Token::Bytes(encoded_identities)]);
	let function_hash = "09c5eabe";
	prepare_function_call_input(function_hash, encoded_identities_as_bytes)
}

pub fn identity_to_token(identity: &Identity) -> Token {
	let (type_index, value) = match identity {
		Identity::Twitter(str) => (0, str.inner_ref().to_vec()),
		Identity::Discord(str) => (1, str.inner_ref().to_vec()),
		Identity::Github(str) => (2, str.inner_ref().to_vec()),
		Identity::Substrate(addr) => (3, addr.as_ref().to_vec()),
		Identity::Evm(addr) => (4, addr.as_ref().to_vec()),
		Identity::Bitcoin(addr) => (5, addr.as_ref().to_vec()),
	};
	Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value)])
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
	use crate::dynamic::build;
	use itp_types::Assertion;
	use lc_stf_task_sender::AssertionBuildRequest;
	use litentry_primitives::{Address32, Identity, IdentityString};
	use sp_core::crypto::AccountId32;
	use crate::dynamic::repository::InMemorySmartContractRepo;
	use sp_core::H160;

	#[test]
	pub fn test_it() {
		let twitter_identity = Identity::Discord(IdentityString::new(vec![]));
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
		};

		let repository = InMemorySmartContractRepo::new();

		let credential = build(&request, hash(0), repository).unwrap();

		// assert!(credential.credential_subject.values.iter().all(|v| v == true))
	}

	fn hash(a: u64) -> H160 {
		H160::from_low_u64_be(a)
	}
}
