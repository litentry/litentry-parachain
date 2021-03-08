use sp_std::{prelude::*};
use core::{fmt};
use frame_support::{
	debug, 
};
use sp_runtime::offchain::{http, storage::StorageValueRef,};
use codec::{Encode, Decode};
use alt_serde::{Deserialize, Deserializer};
use super::utils;

/// Asset type
#[derive(Encode, Decode, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum BlockChainType {
    /// invalid
    Invalid,
    /// eth token
    ETH,
    /// bitcoin
    BTC,
}

impl Default for BlockChainType {
    fn default() -> Self {BlockChainType::Invalid}
}

/// Eth source enum
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq)]
pub enum DataSource {
    /// invalid
    Invalid,
    /// etherscan
    EthEtherScan,
    /// infura
    EthInfura,
    /// blockchain
    BtcBlockChain,
}

pub const TOTAL_DATA_SOURCE_NUMBER: u32 = 3;
pub const DATA_SOURCE_LIST: [DataSource; TOTAL_DATA_SOURCE_NUMBER as usize] = [
        DataSource::EthEtherScan, 
        DataSource::EthInfura, 
        DataSource::BtcBlockChain,
    ];

impl Default for DataSource {
    fn default() -> Self {DataSource::Invalid}
}

/// Data source to blockchain type
pub fn data_source_to_index(data_source: DataSource) -> u32 {
    match data_source {
        DataSource::Invalid => u32::MAX, 
        DataSource::EthEtherScan => 0,
        DataSource::EthInfura => 1,
        DataSource::BtcBlockChain => 2,
    }
}

/// Data source to blockchain type
pub fn data_source_to_block_chain_type(data_source: DataSource) -> BlockChainType {
    match data_source {
        DataSource::Invalid => BlockChainType::Invalid, 
        DataSource::EthEtherScan => BlockChainType::ETH,
        DataSource::EthInfura => BlockChainType::ETH,
        DataSource::BtcBlockChain => BlockChainType::BTC,
    }
}

/// Http Get URL structure
pub struct HttpGet<'a> {
    pub blockchain: BlockChainType,
    // URL affix
    pub prefix: &'a str,
    pub delimiter: &'a str,
    pub postfix: &'a str,
    pub api_token: &'a str,
}

/// Http Post URL structure
pub struct HttpPost<'a> {
    pub blockchain: BlockChainType,
    // URL affix
    pub url_main: &'a str,
    pub api_token: &'a str,
    // Body affix
    pub prefix: &'a str,
    pub delimiter: &'a str,
    pub postfix: &'a str,
}

/// Request enum to wrap up both get and post method
pub enum HttpRequest<'a> {
    GET(HttpGet<'a>),
    POST(HttpPost<'a>),
}

/// Store all API tokens for offchain worker to send request to website
#[derive(Deserialize, Encode, Decode, Default)]
#[serde(crate = "alt_serde")]
pub struct TokenInfo {
	/// API token for etherscan service
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub etherscan: Vec<u8>,
	/// API token for infura service
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub infura: Vec<u8>,
	/// API token for blockchain.info website
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub blockchain: Vec<u8>,
}

/// Balances data embedded in etherscan response
#[derive(Deserialize, Encode, Decode, Default)]
#[serde(crate = "alt_serde")]
pub struct EtherScanBalance {
	/// Ethereum account
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub account: Vec<u8>,
	/// Eth balance
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub balance: Vec<u8>,
}

/// Response data from etherscan
#[derive(Deserialize, Encode, Decode, Default)]
#[serde(crate = "alt_serde")]
pub struct EtherScanResponse {
	/// Http response status
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub status: Vec<u8>,
	/// Http response message
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub message: Vec<u8>,
	/// Ethereum account and its balance
	pub result: Vec<EtherScanBalance>,
}

/// Balances data from Infura service
#[derive(Deserialize, Encode, Decode, Default)]
#[serde(crate = "alt_serde")]
pub struct InfuraBalance {
	/// Json RPV version
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub jsonrpc: Vec<u8>,
	/// Query ID
	pub id: u32,
	/// Balance data
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub result: Vec<u8>,
}

/// Response from Infura
#[derive(Deserialize, Encode, Decode, Default)]
#[serde(crate = "alt_serde")]
pub struct InfuraResponse {
	/// Response vector for several Ethreum account
	pub response: Vec<InfuraBalance>,
}

/// Deserialize string to Vec<u8>
pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}

/// Implement Debug trait for print TokenInfo
impl fmt::Debug for TokenInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{{ etherscan: {}, infura: {}, blockchain: {} }}",
			sp_std::str::from_utf8(&self.etherscan).map_err(|_| fmt::Error)?,
			sp_std::str::from_utf8(&self.infura).map_err(|_| fmt::Error)?,
			sp_std::str::from_utf8(&self.blockchain).map_err(|_| fmt::Error)?,
		)
	}
}

// Fetch json result from remote URL with get method
pub fn fetch_json_http_get<'a>(remote_url: &'a [u8]) -> Result<Vec<u8>, &'static str> {
    let remote_url_str = core::str::from_utf8(remote_url)
        .map_err(|_| "Error in converting remote_url to string")?;

    let pending = http::Request::get(remote_url_str).send()
        .map_err(|_| "Error in sending http GET request")?;

    let response = pending.wait()
        .map_err(|_| "Error in waiting http response back")?;

    if response.code != 200 {
        debug::warn!("Unexpected status code: {}", response.code);
        return Err("Non-200 status code returned from http request");
    }

    let json_result: Vec<u8> = response.body().collect::<Vec<u8>>();

    let balance =
        core::str::from_utf8(&json_result).map_err(|_| "JSON result cannot convert to string")?;

    Ok(balance.as_bytes().to_vec())
}

// Fetch json result from remote URL with post method
pub fn fetch_json_http_post<'a>(remote_url: &'a [u8], body: &'a [u8]) -> Result<Vec<u8>, &'static str> {
    let remote_url_str = core::str::from_utf8(remote_url)
        .map_err(|_| "Error in converting remote_url to string")?;

    debug::info!("Offchain Worker post request url is {}.", remote_url_str);

    let pending = http::Request::post(remote_url_str, vec![body]).send()
        .map_err(|_| "Error in sending http POST request")?;

    let response = pending.wait()
        .map_err(|_| "Error in waiting http response back")?;

    if response.code != 200 {
        debug::warn!("Unexpected status code: {}", response.code);
        return Err("Non-200 status code returned from http request");
    }

    let json_result: Vec<u8> = response.body().collect::<Vec<u8>>();

    let balance =
        core::str::from_utf8(&json_result).map_err(|_| "JSON result cannot convert to string")?;

    Ok(balance.as_bytes().to_vec())
}

// Send request to local server for query api tokens
pub fn send_get_token() -> Result<Vec<u8>, &'static str> {
    let pending = http::Request::get(super::TOKEN_SERVER_URL).send()
        .map_err(|_| "Error in sending http GET request")?;

    let response = pending.wait()
        .map_err(|_| "Error in waiting http response back")?;

    if response.code != 200 {
        debug::warn!("Unexpected status code: {}", response.code);
        return Err("Non-200 status code returned from http request");
    }

    let json_result: Vec<u8> = response.body().collect::<Vec<u8>>();

    Ok(json_result)
}

// Get the API tokens from local server
pub fn get_token() {
    match send_get_token() {
        Ok(json_result) => {
            match core::str::from_utf8(&json_result) {
                Ok(balance) => parse_store_tokens(balance),
                Err(_) => {},
            }
        },
        Err(_) => {},
    }
}

#[allow(dead_code)]
// Parse the balance from etherscan response
pub fn parse_etherscan_balances(price_str: &str) -> Option<Vec<u128>> {
    // {
    // "status": "1",
    // "message": "OK",
    // "result":
    //   [
    //     {"account":"0x742d35Cc6634C0532925a3b844Bc454e4438f44e","balance":"3804372455842738500000001"},
    //     {"account":"0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8","balance":"2571179226430511381996287"}
    //   ]
    // }
    debug::info!("Offchain Worker response from etherscan is {:?}", price_str);

    let token_info: EtherScanResponse = serde_json::from_str(price_str).ok()?;
    let result: Vec<u128> = token_info.result.iter().map(|item| match utils::chars_to_u128(&item.balance.iter().map(|i| *i as char).collect()) {
        Ok(balance) => balance,
        Err(_) => 0_u128,
    }).collect();
    Some(result)
}

#[allow(dead_code)]
// Parse balances from blockchain info response
pub fn parse_blockchain_info_balances(price_str: &str) -> Option<Vec<u128>>{
    // {
    //	"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":{"final_balance":6835384571,"n_tx":2635,"total_received":6835384571},
    //  "15EW3AMRm2yP6LEF5YKKLYwvphy3DmMqN6":{"final_balance":0,"n_tx":4,"total_received":310925609}
    // }
    let mut balance_vec: Vec<u128> = Vec::new();

    let value: serde_json::Value = serde_json::from_str(price_str).ok()?;

    match value {
        serde_json::Value::Object(map_data) => {
            for (_, v) in map_data.iter() {
                match v["final_balance"].as_u64() {
                Some(balance) =>  balance_vec.push(balance as u128),
                None => (),    
                }
            }
        },
        _ => (),
    };

    Some(balance_vec)
}

#[allow(dead_code)]
// Parse the balance from infura response
pub fn parse_infura_balances(price_str: &str) -> Option<Vec<u128>> {
    //[
    //  {"jsonrpc":"2.0","id":1,"result":"0x4563918244f40000"},
    //  {"jsonrpc":"2.0","id":1,"result":"0xff"}
    //]

    let token_info: Vec<InfuraBalance> = serde_json::from_str(price_str).ok()?;
    let result: Vec<u128> = token_info.iter().map(|item| match utils::chars_to_u128(&item.result.iter().map(|i| *i as char).collect()) {
        Ok(balance) => balance,
        Err(_) => 0_u128,
    }).collect();
    Some(result)
}

// Parse the token from local server
pub fn parse_store_tokens(resp_str: &str) {
    let token_info: Result<TokenInfo, _> = serde_json::from_str(&resp_str);

    match token_info {
        Ok(info) => {
            let s_info = StorageValueRef::persistent(b"offchain-worker::token");
            s_info.set(&info);
            debug::info!("Token info get from local server is {:?}.", &info);
        },
        Err(_) => {},
    }
}
