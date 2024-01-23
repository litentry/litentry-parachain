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
    Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value.into())])
}

pub fn build(
    req: &AssertionBuildRequest) -> Result<Credential> {

    let identities: Vec<Token> = req.identities.iter()
        .map(|identity| {
            identity_to_token(&identity.0)
        }).collect();

    let mut input = hex::decode("0x09c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let mut encoded_identities = encode(&vec![Token::Array(identities)]);

    // input.append(&mut encoded_identities);





    let (description, assertion_type, assertion, result) = execute_smart_contract(input);



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
        hex::decode("608060405234801561001057600080fd5b50610969806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c806309c5eabe14610030575b600080fd5b61004a600480360381019061004591906103fd565b610063565b60405161005a94939291906104e9565b60405180910390f35b60608060606000808580602001905181019061007f919061074b565b905061008a8161009a565b9450945094509450509193509193565b60608060606000806040518060600160405280603181526020016108e060319139905060006040518060400160405280601481526020017f2f646174612f332f656d706c6f7965655f616765000000000000000000000000815250905060006101038383610267565b9050600060405180606001604052806023815260200161091160239139905060006040518060400160405280600a81526020017f4973206f76657220353000000000000000000000000000000000000000000000815250905060006040518060400160405280600881526020017f616765203e2035300000000000000000000000000000000000000000000000008152509050600073__$332172407589552215ef22f863f58abd45$__63e99a875e8d6000815181106101c6576101c5610794565b5b60200260200101516040518263ffffffff1660e01b81526004016101ea9190610864565b60206040518083038186803b15801561020257600080fd5b505af4158015610216573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061023a91906108b2565b15610248576001905061024d565b600090505b838383839a509a509a509a50505050505050509193509193565b60008083518351604051858701602082828a60006002600019f161028a57600080fd5b6020820160405281519450505050508091505092915050565b6000604051905090565b600080fd5b600080fd5b600080fd5b600080fd5b6000601f19601f8301169050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b61030a826102c1565b810181811067ffffffffffffffff82111715610329576103286102d2565b5b80604052505050565b600061033c6102a3565b90506103488282610301565b919050565b600067ffffffffffffffff821115610368576103676102d2565b5b610371826102c1565b9050602081019050919050565b82818337600083830152505050565b60006103a061039b8461034d565b610332565b9050828152602081018484840111156103bc576103bb6102bc565b5b6103c784828561037e565b509392505050565b600082601f8301126103e4576103e36102b7565b5b81356103f484826020860161038d565b91505092915050565b600060208284031215610413576104126102ad565b5b600082013567ffffffffffffffff811115610431576104306102b2565b5b61043d848285016103cf565b91505092915050565b600081519050919050565b600082825260208201905092915050565b60005b83811015610480578082015181840152602081019050610465565b8381111561048f576000848401525b50505050565b60006104a082610446565b6104aa8185610451565b93506104ba818560208601610462565b6104c3816102c1565b840191505092915050565b60008115159050919050565b6104e3816104ce565b82525050565b600060808201905081810360008301526105038187610495565b905081810360208301526105178186610495565b9050818103604083015261052b8185610495565b905061053a60608301846104da565b95945050505050565b600067ffffffffffffffff82111561055e5761055d6102d2565b5b602082029050602081019050919050565b600080fd5b600080fd5b600080fd5b600063ffffffff82169050919050565b6105978161057e565b81146105a257600080fd5b50565b6000815190506105b48161058e565b92915050565b60006105cd6105c88461034d565b610332565b9050828152602081018484840111156105e9576105e86102bc565b5b6105f4848285610462565b509392505050565b600082601f830112610611576106106102b7565b5b81516106218482602086016105ba565b91505092915050565b6000604082840312156106405761063f610574565b5b61064a6040610332565b9050600061065a848285016105a5565b600083015250602082015167ffffffffffffffff81111561067e5761067d610579565b5b61068a848285016105fc565b60208301525092915050565b60006106a96106a484610543565b610332565b905080838252602082019050602084028301858111156106cc576106cb61056f565b5b835b8181101561071357805167ffffffffffffffff8111156106f1576106f06102b7565b5b8086016106fe898261062a565b855260208501945050506020810190506106ce565b5050509392505050565b600082601f830112610732576107316102b7565b5b8151610742848260208601610696565b91505092915050565b600060208284031215610761576107606102ad565b5b600082015167ffffffffffffffff81111561077f5761077e6102b2565b5b61078b8482850161071d565b91505092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b6107cc8161057e565b82525050565b600081519050919050565b600082825260208201905092915050565b60006107f9826107d2565b61080381856107dd565b9350610813818560208601610462565b61081c816102c1565b840191505092915050565b600060408301600083015161083f60008601826107c3565b506020830151848203602086015261085782826107ee565b9150508091505092915050565b6000602082019050818103600083015261087e8184610827565b905092915050565b61088f816104ce565b811461089a57600080fd5b50565b6000815190506108ac81610886565b92915050565b6000602082840312156108c8576108c76102ad565b5b60006108d68482850161089d565b9150509291505056fe68747470733a2f2f64756d6d792e726573746170696578616d706c652e636f6d2f6170692f76312f656d706c6f7965657349732074686520656d706c6f796565206f766572203530207965617273206f6c64203fa264697066735822122096f25027ab7a1861f29a84a6bc4456919f4262a2fd650c094093eb84b514c05d64736f6c63430008080033").unwrap(),
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
        let request = AssertionBuildRequest {
            shard: Default::default(),
            signer: AccountId32::new([0;32]),
            who: Identity::Twitter(IdentityString::new(vec![])),
            assertion: Assertion::Dynamic,
            identities: vec![],
            top_hash: Default::default(),
            parachain_block_number: Default::default(),
            sidechain_block_number: Default::default(),
            maybe_key: None,
            req_ext_hash: Default::default()
        };


        build(&request).unwrap();
    }

}

