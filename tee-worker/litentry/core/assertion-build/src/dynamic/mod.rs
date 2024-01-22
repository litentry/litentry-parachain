use log::debug;
use itp_types::Assertion;
use lc_credentials::Credential;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::BnbDigitDomainType;
use crate::nodereal::bnb_domain::BnbDomainInfo;
use crate::transpose_identity;
use crate::*;
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
use std::result::{Result as StdResult};
use std::collections::BTreeMap;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;


pub fn build(
    req: &AssertionBuildRequest) -> Result<Credential> {

    let identities = &req.identities.iter()
        .map(|identity| {
            identity.
        })





    let (description, assertion_type, assertion, result) = execute_smart_contract();



    match Credential::new(&req.who, &req.shard) {
        Ok(mut credential_unsigned) => {
            credential_unsigned.update_dynamic(description, assertion_type, assertion, result);
            Ok(credential_unsigned)
        },
        Err(e) => {
            error!("Generate unsigned credential failed {:?}", e);
            Err(Error::RequestVCFailed(
                Assertion::Dynamic,
                e.into_error_detail(),
            ))
        },
    }
}

pub type PrecompileResult = StdResult<PrecompileOutput, PrecompileFailure>;

pub struct InputReader {
    input: Vec<u8>,
    position: usize
}

impl InputReader {

    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            position: 0
        }
    }

    pub fn read_string(&mut self) -> String {
        let word_size = 32;
        let str_len = self.read_string_len();
        let end = self.position + str_len;
        let value = String::from_utf8_lossy(&self.input[(self.position)..end]).to_string();
        self.position += ((str_len / word_size) +1) * word_size;
        value
    }

    fn read_string_len(&mut self) -> usize {
        let word_size = 32;
        // first word contains information about string size,
        let end = self.position + word_size;
        let size: usize = U256::from_big_endian(&self.input[(self.position)..end]).try_into().expect("Could not convert size");
        self.position += word_size;
        size
    }
}


/* This is precompile contract for making http get requests.
It will send HTTP GET request to hardcoded URL, parse JSON response, extract value using JSON Pointer and pass it back to calle contract.
`input` can be used to customize URL, JSON pointer and authorization.
Currently this contract return only integers, but it may be possible to return any data as byte array but handling code on calling side
will be more complex. It may also require more flexible parsing or JSON handling in Solidity (jsmnSol) */
pub struct HttpGetI64Precompile;


impl HttpGetI64Precompile {
    // 256 bytes for url, 256 bytes for json pointer, total 512 bytes
    fn execute(input: Vec<u8>) -> PrecompileResult {
        let mut reader = InputReader::new(input);

        let url = reader.read_string();
        let pointer = reader.read_string();



        let client = HttpClient::new(DefaultSend, true, Some(Duration::from_secs(10)), None, None);
        let resp = client
            .send_request_raw(
                Url::parse(&url).unwrap(),
                Method::GET,
                None,
            )
            .unwrap();
        let value: Value = serde_json::from_slice(&resp.1).unwrap();
        let result = value.pointer(&pointer).unwrap();
        let encoded = encode(&[Token::Uint(result.as_i64().unwrap().into())]);
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: encoded,
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
            a if a == hash(2) => Some(HttpGetI64Precompile::execute(handle.input().to_vec())),
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

pub fn execute_smart_contract() -> (String, String, String, bool) {
    let config = prepare_config();
    let vicinity = prepare_memory();
    let mut state = BTreeMap::new();
    let mut backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let mut state = MemoryStackState::new(metadata, &mut backend);
    let mut precompiles = Precompiles {};
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    // caller
    let caller = hash(4);

    // deploy smart contract
    let address = executor.create_address(evm::CreateScheme::Legacy { caller: hash(4) });
    let create_result = executor.transact_create(
        caller,
        U256::zero(),
        hex::decode("608060405234801561001057600080fd5b5061053b806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063b1976a021461003b578063c9cefe8f1461005c575b600080fd5b61004361008c565b604051610053949392919061033f565b60405180910390f35b61007660048036038101906100719190610261565b6101bd565b6040516100839190610324565b60405180910390f35b60608060606000806040518060600160405280603181526020016104b260319139905060006040518060400160405280601481526020017f2f646174612f332f656d706c6f7965655f616765000000000000000000000000815250905060006100f583836101bd565b905060006040518060600160405280602381526020016104e360239139905060006040518060400160405280600a81526020017f4973206f76657220353000000000000000000000000000000000000000000000815250905060006040518060400160405280600881526020017f616765203e2035300000000000000000000000000000000000000000000000008152509050600060328560070b131561019f57600190506101a4565b600090505b838383839a509a509a509a505050505050505090919293565b60008083518351604051858701602082828a60006002600019f16101e057600080fd5b6020820160405281519450505050508091505092915050565b600061020c610207846103ca565b610399565b90508281526020810184848401111561022457600080fd5b61022f84828561042f565b509392505050565b600082601f83011261024857600080fd5b81356102588482602086016101f9565b91505092915050565b6000806040838503121561027457600080fd5b600083013567ffffffffffffffff81111561028e57600080fd5b61029a85828601610237565b925050602083013567ffffffffffffffff8111156102b757600080fd5b6102c385828601610237565b9150509250929050565b6102d681610416565b82525050565b6102e581610422565b82525050565b60006102f6826103fa565b6103008185610405565b935061031081856020860161043e565b610319816104a0565b840191505092915050565b600060208201905061033960008301846102dc565b92915050565b6000608082019050818103600083015261035981876102eb565b9050818103602083015261036d81866102eb565b9050818103604083015261038181856102eb565b905061039060608301846102cd565b95945050505050565b6000604051905081810181811067ffffffffffffffff821117156103c0576103bf610471565b5b8060405250919050565b600067ffffffffffffffff8211156103e5576103e4610471565b5b601f19601f8301169050602081019050919050565b600081519050919050565b600082825260208201905092915050565b60008115159050919050565b60008160070b9050919050565b82818337600083830152505050565b60005b8381101561045c578082015181840152602081019050610441565b8381111561046b576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6000601f19601f830116905091905056fe68747470733a2f2f64756d6d792e726573746170696578616d706c652e636f6d2f6170692f76312f656d706c6f7965657349732074686520656d706c6f796565206f766572203530207965617273206f6c64203fa2646970667358221220d8bf4c201a96af8578dd6d22940b71b051ac2fb0ba22891a5d98175349ee415a64736f6c63430008000033").unwrap(),
        u64::MAX,
        Vec::new(),
    );
    // call function
    let call_reason = executor.transact_call(
        caller,
        address,
        U256::zero(),
        hex::decode("b1976a02").unwrap(),
        u64::MAX,
        Vec::new(),
    );

    info!("Contract execution result {:?}", &call_reason);

    let types = vec![ParamType::String, ParamType::String, ParamType::String, ParamType::Bool];

    let decoded = decode(&types, &call_reason.1).unwrap();

    (decoded[0].clone().into_string().unwrap(), decoded[1].clone().into_string().unwrap(), decoded[2].clone().into_string().unwrap(), decoded[3].clone().into_bool().unwrap())
}

