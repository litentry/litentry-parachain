use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::time::Duration;
use evm::backend::{MemoryBackend, MemoryVicinity};
use evm::executor::stack::{IsPrecompileResult, MemoryStackState, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileSet, StackExecutor, StackSubstateMetadata};
use evm::{Config, ExitSucceed};
use itc_direct_rpc_client::sgx_reexport_prelude::url::Url;
use itc_rest_client::http_client::{DefaultSend, HttpClient, SendHttpRequest};
use itc_rest_client::rest_client::Method;
use primitive_types::{H160, U256};
use serde_json::Value;
use ethabi::{encode, decode, Token, ParamType};
use log::info;

pub type PrecompileResult = Result<PrecompileOutput, PrecompileFailure>;

/* This is precompile contract for making http get requests.
It will send HTTP GET request to hardcoded URL, parse JSON response, extract value using JSON Pointer and pass it back to calle contract.
`input` can be used to customize URL, JSON pointer and authorization.
Currently this contract return only integers, but it may be possible to return any data as byte array but handling code on calling side
will be more complex. It may also require more flexible parsing or JSON handling in Solidity (jsmnSol) */
pub struct HttpGetPrecompile;

impl HttpGetPrecompile {
    fn execute(input: Vec<u8>) -> PrecompileResult {
        let client = HttpClient::new(DefaultSend, true, Some(Duration::from_secs(10)), None, None);
        let resp = client
            .send_request_raw(
                Url::parse("https://dummy.restapiexample.com/api/v1/employees").unwrap(),
                Method::GET,
                None,
            )
            .unwrap();
        let value: Value = serde_json::from_slice(&resp.1).unwrap();
        let result = value.pointer("/data/0/employee_age").unwrap();
        let encoded = encode(&[Token::Uint(result.as_i64().unwrap().into())]);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: vec![],
        })
    }
}

pub struct Precompiles();

impl Precompiles {
    pub fn new() -> Self {
        Self()
    }
}

impl PrecompileSet for Precompiles {
    fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
        match handle.code_address() {
            a if a == hash(2) => Some(HttpGetPrecompile::execute(handle.input().to_vec())),
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160, remaining_gas: u64) -> IsPrecompileResult {
        match address {
            a if a == hash(2) => IsPrecompileResult::Answer {
                is_precompile: false,
                extra_cost: 0,
            },
            _ => IsPrecompileResult::Answer {
                is_precompile: true,
                extra_cost: 0,
            },
        }
    }
}

fn hash(a: u64) -> H160 {
    let hash = H160::from_low_u64_be(a);
    hash
}

fn precompile_example<'config, 'precompiles, 'backend, 'vicinity>(
    executor: &mut StackExecutor<
        'config,
        'precompiles,
        MemoryStackState<'backend, 'config, MemoryBackend<'vicinity>>,
        Precompiles,
    >,
) {
    let caller = hash(4);

    // deploy smart contract
    let address = executor.create_address(evm::CreateScheme::Legacy { caller: hash(4) });
    let create_result = executor.transact_create(
        caller,
        U256::zero(),
        hex::decode("608060405234801561001057600080fd5b50610288806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c8063b1976a0214610030575b600080fd5b610038610051565b6040516100489493929190610169565b60405180910390f35b606080606060008060405160208160008360006002600019f161007357600080fd5b80519150506040518060600160405280602381526020016102306023913994506040518060400160405280600a81526020017f4973206f7665722035300000000000000000000000000000000000000000000081525093506040518060400160405280600881526020017f616765203e20353000000000000000000000000000000000000000000000000081525092506032811315610115576001915061011a565b600091505b5090919293565b61012a816101df565b82525050565b600061013b826101c3565b61014581856101ce565b93506101558185602086016101eb565b61015e8161021e565b840191505092915050565b600060808201905081810360008301526101838187610130565b905081810360208301526101978186610130565b905081810360408301526101ab8185610130565b90506101ba6060830184610121565b95945050505050565b600081519050919050565b600082825260208201905092915050565b60008115159050919050565b60005b838110156102095780820151818401526020810190506101ee565b83811115610218576000848401525b50505050565b6000601f19601f830116905091905056fe49732074686520656d706c6f796565206f766572203530207965617273206f6c64203fa2646970667358221220d83444a13e15907f8783a5456c5406291b8a67895ce34fc0fc696d7bd77a7a1564736f6c63430008000033").unwrap(),
        u64::MAX,
        Vec::new(),
    );
    // call another method
    let another_reason = executor.transact_call(
        caller,
        address,
        U256::zero(),
        hex::decode("b1976a02").unwrap(),
        u64::MAX,
        Vec::new(),
    );

    println!("Contract execution result {:?}", &another_reason);

    let types = vec![ParamType::String, ParamType::String, ParamType::String, ParamType::Bool];

    let decoded = decode(&types, &another_reason.1).unwrap();

    decoded.iter().for_each(|t| info!("Got type: {:?}", t));
}

pub fn test() {
    let mut config = Config::frontier();
    config.has_bitwise_shifting = true;
    config.err_on_call_with_more_gas = false;
    let vicinity = MemoryVicinity {
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
    };
    let mut state = BTreeMap::new();
    let mut backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let mut state = MemoryStackState::new(metadata, &mut backend);
    let mut precompiles = Precompiles {};
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    // run example
    precompile_example(&mut executor);
}