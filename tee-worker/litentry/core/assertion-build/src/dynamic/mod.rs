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
use itc_rest_client::http_client::{DefaultSend, HttpClient, SendHttpRequest};
use itc_rest_client::rest_client::Method;
use primitive_types::{H160, U256};
use serde_json::Value;
use ethabi::{encode, decode, Token, ParamType};
use log::info;
use std::result::{Result as StdResult};
use std::collections::BTreeMap;
use url::Url;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;


pub fn identity_to_token(identity: &Identity) -> Token {
    //remember to update correspondingy solidity library
    let (type_index, value) = match identity {
        Identity::Twitter(str) => (0, str.inner_ref().to_vec()),
        Identity::Discord(str) => (1, str.inner_ref().to_vec()),
        Identity::Github(str) => (2, str.inner_ref().to_vec()),
        Identity::Substrate(addr) => (3, addr.as_ref().to_vec()),
        Identity::Evm(addr) => (4, addr.as_ref().to_vec()),
        Identity::Bitcoin(addr) => (5, addr.as_ref().to_vec())
    };

    println!("Encoded single identity: {:?}", hex::encode(encode(&vec![Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value.clone().into())])])));

    Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value.into())])
}

pub fn build(
    req: &AssertionBuildRequest) -> Result<Credential> {

    let identities: Vec<Token> = req.identities.iter()
        .map(|identity| {
            identity_to_token(&identity.0)
        }).collect();

    println!("Identities: {:?}", identities);

    let mut encoded_identities = encode(&vec![Token::Array(identities)]);

    println!("encoded identities encoded: ${:?}", hex::encode(&encoded_identities));


    // input.append(&mut encoded_identities);





    let (description, assertion_type, assertion, result) = execute_smart_contract(encoded_identities);



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

pub fn execute_smart_contract(input: Vec<u8>) -> (String, String, String, bool) {
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
        hex::decode("608060405234801561001057600080fd5b50610373806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c806309c5eabe14610030575b600080fd5b61004361003e36600461019e565b61005c565b604051610053949392919061029c565b60405180910390f35b60608080600080806040519080825280602002602001820160405280156100aa57816020015b6040805180820190915260008152606060208201528152602001906001900390816100825790505b5090506100b6816100c6565b9450945094509450509193509193565b60608060606000806040518060600160405280603181526020016102ea6031913990506000604051806040016040528060148152602001732f646174612f332f656d706c6f7965655f61676560601b8152509050600060405180606001604052806023815260200161031b60239139604080518082018252600a81526904973206f7665722035360b41b602080830191909152825180840190935260088352670616765203e2035360c41b90830152919a919950975060019650945050505050565b634e487b7160e01b600052604160045260246000fd5b6000602082840312156101b057600080fd5b813567ffffffffffffffff808211156101c857600080fd5b818401915084601f8301126101dc57600080fd5b8135818111156101ee576101ee610188565b604051601f8201601f19908116603f0116810190838211818310171561021657610216610188565b8160405282815287602084870101111561022f57600080fd5b826020860160208301376000928101602001929092525095945050505050565b6000815180845260005b8181101561027557602081850181015186830182015201610259565b81811115610287576000602083870101525b50601f01601f19169290920160200192915050565b6080815260006102af608083018761024f565b82810360208401526102c1818761024f565b905082810360408401526102d5818661024f565b91505082151560608301529594505050505056fe68747470733a2f2f64756d6d792e726573746170696578616d706c652e636f6d2f6170692f76312f656d706c6f7965657349732074686520656d706c6f796565206f766572203530207965617273206f6c64203fa26469706673582212200e7325b98a537324b39e2a908aa47d723113cb3917dd3baa3fbf8c2a9189eba364736f6c63430008080033").unwrap(),
        u64::MAX,
        Vec::new(),
    );
    // call function
    let call_reason = executor.transact_call(
        caller,
        address,
        U256::zero(),
        input,
        u64::MAX,
        Vec::new(),
    );

    info!("Contract execution result {:?}", &call_reason);

    let types = vec![ParamType::String, ParamType::String, ParamType::String, ParamType::Bool];

    let decoded = decode(&types, &call_reason.1).unwrap();

    (decoded[0].clone().into_string().unwrap(), decoded[1].clone().into_string().unwrap(), decoded[2].clone().into_string().unwrap(), decoded[3].clone().into_bool().unwrap())
}


#[cfg(test)]
pub mod tests {
    use sp_core::crypto::AccountId32;
    use itp_types::Assertion;
    use lc_stf_task_sender::AssertionBuildRequest;
    use litentry_primitives::Identity;
    use litentry_primitives::IdentityString;
    use crate::dynamic::build;

    #[test]
    pub fn test_it() {

        let twitter_identity = Identity::Discord(IdentityString::new(vec![]));

        let request = AssertionBuildRequest {
            shard: Default::default(),
            signer: AccountId32::new([0;32]),
            who: Identity::Twitter(IdentityString::new(vec![])),
            assertion: Assertion::Dynamic,
            identities: vec![(twitter_identity, vec![])],
            top_hash: Default::default(),
            parachain_block_number: Default::default(),
            sidechain_block_number: Default::default(),
            maybe_key: None,
            req_ext_hash: Default::default()
        };


        build(&request).unwrap();
    }

}

