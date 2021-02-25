//! # Offchain Worker 
//! The pallet is responsible for get the external assets claim from the extrinsic and then query and aggregate the 
//! balance (btc and eth) according to linked external accounts in account linker pallet. Offchain worker get the data
//! from most popular websire like etherscan, infura and blockinfo. After get the balance, Offchain worker emit the event
//! with balance info and store them on chain for on-chain query.
//! 
//! ## API token
//! The offchain worker need the API token to query data from third party data provider. Currently, offchain worker get 
//! the API tokens from a local server. Then store the API tokens in offchain worder local storage.
//! 

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{prelude::*, vec};
use core::fmt;
use frame_system::{
	ensure_signed, ensure_none,
	offchain::{CreateSignedTransaction, SubmitTransaction},
};
use frame_support::{
	debug, dispatch, decl_module, decl_storage, decl_event, decl_error,
	ensure, storage::IterableStorageMap,
};
use sp_core::crypto::KeyTypeId;

use sp_runtime::{
	transaction_validity::{
		ValidTransaction, InvalidTransaction, TransactionValidity, TransactionSource, TransactionLongevity,
	},
};
use sp_runtime::offchain::{http, storage::StorageValueRef,};
use codec::{Encode, Decode};

// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw!");
const TOKEN_SERVER_URL: &str = "http://127.0.0.1:4000";

/// Store all API tokens for offchain worker to send request to website
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct TokenInfo {
	/// API token for etherscan service
	#[serde(deserialize_with = "de_string_to_bytes")]
	etherscan: Vec<u8>,
	/// API token for infura service
	#[serde(deserialize_with = "de_string_to_bytes")]
	infura: Vec<u8>,
	/// API token for blockchain.info website
	#[serde(deserialize_with = "de_string_to_bytes")]
	blockchain: Vec<u8>,
}

/// Balances data embedded in etherscan response
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EtherScanBalance {
	/// Ethereum account
	#[serde(deserialize_with = "de_string_to_bytes")]
	account: Vec<u8>,
	/// Eth balance
	#[serde(deserialize_with = "de_string_to_bytes")]
	balance: Vec<u8>,
}

/// Response data from etherscan
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EtherScanResponse {
	/// Http response status
	#[serde(deserialize_with = "de_string_to_bytes")]
	status: Vec<u8>,
	/// Http response message
	#[serde(deserialize_with = "de_string_to_bytes")]
	message: Vec<u8>,
	/// Ethereum account and its balance
	result: Vec<EtherScanBalance>,
}

/// Balances data from Infura service
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct InfuraBalance {
	/// Json RPV version
	#[serde(deserialize_with = "de_string_to_bytes")]
	jsonrpc: Vec<u8>,
	/// Query ID
	id: u32,
	/// Balance data
	#[serde(deserialize_with = "de_string_to_bytes")]
	result: Vec<u8>,
}

/// Response from Infura
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct InfuraResponse {
	/// Response vector for several Ethreum account
	response: Vec<InfuraBalance>,
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

/// URL mod to define request data structure
mod urls {
	/// Asset type
	pub enum BlockChainType {
		/// eth token
		ETH,
		/// bitcoin
		BTC,
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
}

//pub mod crypto {
//	use super::KEY_TYPE;
//	use sp_runtime::{
//		app_crypto::{app_crypto, sr25519},
//		traits::Verify,
//	};
//	use sp_core::sr25519::Signature as Sr25519Signature;
//	app_crypto!(sr25519, KEY_TYPE);
//
//	pub struct TestAuthId;
//	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
//		type RuntimeAppPublic = Public;
//		type GenericSignature = sp_core::sr25519::Signature;
//		type GenericPublic = sp_core::sr25519::Public;
//	}
//}

pub trait Config: frame_system::Config + account_linker::Config + CreateSignedTransaction<Call<Self>> {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Call: From<Call<Self>>;
}

decl_storage! {
	trait Store for Module<T: Config> as OffchainWorkerModule {
		/// Record how many claims from Litentry user
		TotalClaims get(fn total_claims): u64;
		/// Record the accounts send claims in latest block
		ClaimAccountSet get(fn query_account_set): map hasher(blake2_128_concat) T::AccountId => ();
		/// Record account's btc and ethereum balance
		AccountBalance get(fn account_balance): map hasher(blake2_128_concat) T::AccountId => (u128, u128);
	}
}

decl_event!(
	pub enum Event<T> where	AccountId = <T as frame_system::Config>::AccountId,
					BlockNumber = <T as frame_system::Config>::BlockNumber, {
		/// Event for account and its ethereum balance
		BalanceGot(AccountId, BlockNumber, u128, u128),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Error number parsing.
		InvalidNumber,
		/// Account already in claim list.
		AccountAlreadyInClaimlist,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		// Request offchain worker to get balance of linked external account
		#[weight = 10_000]
		pub fn asset_claim(origin,) -> dispatch::DispatchResult {
			let account = ensure_signed(origin)?;

			// If the same claim not processed yet
			ensure!(!ClaimAccountSet::<T>::contains_key(&account), Error::<T>::AccountAlreadyInClaimlist);

			<ClaimAccountSet<T>>::insert(&account, ());

			Ok(())
		}

		// Clear claimed account list
		#[weight = 10_000]
		fn clear_claim(origin, block: T::BlockNumber)-> dispatch::DispatchResult {
			// Ensuring this is an unsigned tx
			ensure_none(origin)?;
			// Remove all claimed accounts
			<ClaimAccountSet::<T>>::remove_all();

			Ok(())
		}

		// Record the balance on chain
		#[weight = 10_000]
		fn record_balance(
			origin,
			account: T::AccountId,
			block: T::BlockNumber,
			btc_balance: u128,
			eth_balance: u128,
		) -> dispatch::DispatchResult {
			// Ensuring this is an unsigned tx
			ensure_none(origin)?;
			// Record the total claims processed
			TotalClaims::put(Self::total_claims() + 1);
			// Set balance 
			<AccountBalance<T>>::insert(account.clone(), (btc_balance, eth_balance));
			// Spit out an event and Add to storage
			Self::deposit_event(RawEvent::BalanceGot(account, block, btc_balance, eth_balance));

			Ok(())
		}

		// Trigger by offchain framework in each block
		fn offchain_worker(block: T::BlockNumber) {
			// Get the all accounts who ask for asset claims
			let accounts: Vec<T::AccountId> = <ClaimAccountSet::<T>>::iter().map(|(k, _)| k).collect();

			let s_info = StorageValueRef::persistent(b"offchain-worker::token");
			match s_info.get::<TokenInfo>() {
				Some(Some(info)) => {
					// Try to remove claims via tx
					if accounts.len() > 0 {
						let call = Call::clear_claim(block);
						let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
						.map_err(|_| {
							debug::error!("Failed in offchain_unsigned_tx");
							<Error<T>>::InvalidNumber
						});
					}

					match Self::update(accounts, block, &info) {
						Ok(()) => debug::info!("Offchain Worker end successfully."),
						Err(err) => debug::info!("Offchain Worker end with err {:?}.", err),
					}
				},
				_ => {
					debug::info!("Offchain Worker to get token from local server.");
					let _ = Self::get_token();
					return ;
				},
			};
		}
	}
}

impl<T: Config> Module<T> {
	// Fetch all claimed accounts
	fn update(account_vec: Vec<T::AccountId>, block: T::BlockNumber, info: &TokenInfo) ->  Result<(), Error<T>> {
		for (_, account) in account_vec.iter().enumerate() {
			// Get balance from etherscan
			let eth_balance = {
				if info.etherscan.len() == 0 {
					Err(Error::<T>::InvalidNumber)
				} else {
					match core::str::from_utf8(&info.etherscan) {
						Ok(token) => Self::fetch_balances(
							<account_linker::Module<T>>::eth_addresses(account), 
							urls::HttpRequest::GET(urls::HttpGet {
								blockchain: urls::BlockChainType::ETH,
								prefix: "https://api-ropsten.etherscan.io/api?module=account&action=balancemulti&address=0x",
								delimiter: ",0x",
								postfix: "&tag=latest&apikey=",
								api_token: token,
								}), 
							&Self::parse_etherscan_balances),
						Err(_) => Err(Error::<T>::InvalidNumber),
					}
				}
			};
			
			// Get balance from blockchain.info
			let btc_balance = {
				if info.blockchain.len() == 0 {
					Err(Error::<T>::InvalidNumber)
				} else {
					match core::str::from_utf8(&info.blockchain) {
						Ok(token) => Self::fetch_balances(Vec::new(), 
							urls::HttpRequest::GET(urls::HttpGet {
								blockchain: urls::BlockChainType::BTC,
								prefix: "https://blockchain.info/balance?active=",
								delimiter: "%7C",
								postfix: "",
								api_token: token,
								}), 
							&Self::parse_blockchain_info_balances),
						Err(_) => Err(Error::<T>::InvalidNumber),
					}
				}
			};

			// Get the balance from Infura
			let etc_balance_infura = {
				if info.infura.len() == 0 {
					Err(Error::<T>::InvalidNumber)
				} else {
					match core::str::from_utf8(&info.infura) {
						Ok(token) => Self::fetch_balances(
							<account_linker::Module<T>>::eth_addresses(account), 
							urls::HttpRequest::POST(urls::HttpPost {
								url_main: "https://ropsten.infura.io/v3/",
								blockchain: urls::BlockChainType::ETH,
								prefix: r#"[{"jsonrpc":"2.0","method":"eth_getBalance","id":1,"params":["0x"#,
								delimiter: r#"","latest"]},{"jsonrpc":"2.0","method":"eth_getBalance","id":1,"params":["0x"#,
								postfix: r#"","latest"]}]"#,
								api_token: token,
								}), 
							&Self::parse_infura_balances),
						Err(_) => Err(Error::<T>::InvalidNumber),
					}
				}
			};

			debug::info!("Offchain Worker etherscan balance got is {:?}", eth_balance);

			// Store balance on chain after offchain worker query done
			match (btc_balance, eth_balance) {
				(Ok(btc), Ok(eth)) => {
					let call = Call::record_balance(account.clone(), block, btc, eth);
					let result = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
					if result.is_err() {
						debug::info!("Offchain Worker failed to submit record balance transaction");
					}
					// TODO Test code
					if eth == etc_balance_infura? {
						debug::info!("Infura returned balance equals to etherscan returned balance.");
					} else {
						debug::error!("Infura returned balance does NOT equal to etherscan returned balance!");
					}
				},
				(Ok(btc), _) => {
					let call = Call::record_balance(account.clone(), block, btc, 0_u128);
					let result = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
					if result.is_err() {
						debug::info!("Offchain Worker failed to submit record balance transaction");
					}
				},
				(_, Ok(eth)) => {
					let call = Call::record_balance(account.clone(), block, 0_u128, eth);
					let result = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
					if result.is_err() {
						debug::info!("Offchain Worker failed to submit record balance transaction");
					}
					// TODO Test code
					if eth == etc_balance_infura? {
						debug::info!("Infura returned balance equals to etherscan returned balance.");
					} else {
						debug::error!("Infura returned balance does NOT equal to etherscan returned balance!");
					}
				},
				_ => (),
			}
		}
		Ok(())
	}

	// Generic function to fetch balance for specific link type
	fn fetch_balances(wallet_accounts: Vec<[u8; 20]>, request: urls::HttpRequest, 
		parser: &dyn Fn(&str) -> Option<Vec<u128>>) -> Result<u128, Error<T>> {
		// Return if no account linked
		if wallet_accounts.len() == 0 {
			return Ok(0_u128)
		}

		let result: Vec<u8> = match request {
			urls::HttpRequest::GET(get_req) => {
				// Compose the get request URL 
				let mut link: Vec<u8> = Vec::new();
				link.extend(get_req.prefix.as_bytes());

				for (i, each_account) in wallet_accounts.iter().enumerate() {
					// Append delimiter if there are more than one accounts in the account_vec
					if i >=1 {
						link.extend(get_req.delimiter.as_bytes());
					};

					link.extend(Self::address_to_string(each_account));
				}
				link.extend(get_req.postfix.as_bytes());
				link.extend(get_req.api_token.as_bytes());

				// Fetch json response via http get
				Self::fetch_json_http_get(&link[..]).map_err(|_| Error::<T>::InvalidNumber)?
			},

			urls::HttpRequest::POST(post_req) => {
				// Compose the post request URL
				let mut link: Vec<u8> = Vec::new();
				link.extend(post_req.url_main.as_bytes());
				link.extend(post_req.api_token.as_bytes());

				// Batch multiple JSON-RPC calls for multiple getBalance operations within one post
				let mut body: Vec<u8> = Vec::new();
				body.extend(post_req.prefix.as_bytes());

				for (i, each_account) in wallet_accounts.iter().enumerate() {
					// Append delimiter if there are more than one accounts in the account_vec
					if i >=1 {
						body.extend(post_req.delimiter.as_bytes());
					};

					body.extend(Self::address_to_string(each_account));
				}
				body.extend(post_req.postfix.as_bytes());

				// Fetch json response via http post 
				Self::fetch_json_http_post(&link[..], &body[..]).map_err(|_| Error::<T>::InvalidNumber)?
			},
		};
		
		let response = sp_std::str::from_utf8(&result).map_err(|_| Error::<T>::InvalidNumber)?;
		let balances = parser(response);

		match balances {
			Some(data) => {
				let mut total_balance: u128 = 0;
				// Sum up the balance
				for balance in data {
					total_balance = total_balance + balance;
				}
				Ok(total_balance)
			},
			None => Ok(0_u128),
		}
	}

	// Fetch json result from remote URL with get method
	fn fetch_json_http_get<'a>(remote_url: &'a [u8]) -> Result<Vec<u8>, &'static str> {
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
	fn fetch_json_http_post<'a>(remote_url: &'a [u8], body: &'a [u8]) -> Result<Vec<u8>, &'static str> {
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

	// Parse the balance from etherscan response
	fn parse_etherscan_balances(price_str: &str) -> Option<Vec<u128>> {
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
		let result: Vec<u128> = token_info.result.iter().map(|item| match Self::chars_to_u128(&item.balance.iter().map(|i| *i as char).collect()) {
			Ok(balance) => balance,
			Err(_) => 0_u128,
		}).collect();
		Some(result)
	}

	// Parse balances from blockchain info response
	fn parse_blockchain_info_balances(price_str: &str) -> Option<Vec<u128>>{
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

	// Parse the balance from infura response
	fn parse_infura_balances(price_str: &str) -> Option<Vec<u128>> {
		//[
		//  {"jsonrpc":"2.0","id":1,"result":"0x4563918244f40000"},
		//  {"jsonrpc":"2.0","id":1,"result":"0xff"}
		//]

		let token_info: Vec<InfuraBalance> = serde_json::from_str(price_str).ok()?;
		let result: Vec<u128> = token_info.iter().map(|item| match Self::chars_to_u128(&item.result.iter().map(|i| *i as char).collect()) {
			Ok(balance) => balance,
			Err(_) => 0_u128,
		}).collect();
		Some(result)
	}

	// u128 number string to u128
	pub fn chars_to_u128(vec: &Vec<char>) -> Result<u128, &'static str> {
		// Check if the number string is decimal or hexadecimal (whether starting with 0x or not) 
		let base = if vec.len() >= 2 && vec[0] == '0' && vec[1] == 'x' {
			// This is a hexadecimal number
			16
		} else {
			// This is a decimal number
			10
		};

		let mut result: u128 = 0;
		for (i, item) in vec.iter().enumerate() {
			// Skip the 0 and x digit for hex. 
			// Using skip here instead of a new vec build to avoid an unnecessary copy operation
			if base == 16 && i < 2 {
				continue;
			}

			let n = item.to_digit(base);
			match n {
				Some(i) => {
					let i_64 = i as u128; 
					result = result * base as u128 + i_64;
					if result < i_64 {
						return Err("Wrong u128 balance data format");
					}
				},
				None => return Err("Wrong u128 balance data format"),
			}
		}
		return Ok(result)
	}

	// number byte to string byte
	fn u8_to_str_byte(a: u8) -> u8{
		if a < 10 {
			return a + 48 as u8;
		}
		else {
			return a + 87 as u8;
		}
	}

	// address to string bytes
	fn address_to_string(address: &[u8; 20]) -> Vec<u8> {

		let mut vec_result: Vec<u8> = Vec::new();
		for item in address {
			let a: u8 = item & 0x0F;
			let b: u8 = item >> 4;
			vec_result.push(Self::u8_to_str_byte(b));
			vec_result.push(Self::u8_to_str_byte(a));
		}
		return vec_result;
	}

	// Get the API tokens from local server
	fn get_token<'a>() -> Result<(), &'static str> {
	
		let pending = http::Request::get(TOKEN_SERVER_URL).send()
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

		debug::info!("Token json from local server is {:?}.", &balance);

		let _ = Self::parse_store_tokens(balance);

		Ok(())
	}

	// Parse the balance from infura response
	fn parse_store_tokens(resp_str: &str) -> Result<(), Error<T>> {
		let token_info: TokenInfo = serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::InvalidNumber)?;

		let s_info = StorageValueRef::persistent(b"offchain-worker::token");

		s_info.set(&token_info);

		debug::info!("Token info get from local server is {:?}.", &token_info);

		Ok(())
	}
}

#[allow(deprecated)]
impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	#[allow(deprecated)]
	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {

		match call {
		Call::record_balance(account, block, btc_balance, eth_balance) => Ok(ValidTransaction {
			priority: 0,
			requires: vec![],
			provides: vec![(account, block, btc_balance, eth_balance).encode()],
			longevity: TransactionLongevity::max_value(),
			propagate: true,
		}),

		Call::clear_claim(block) => Ok(ValidTransaction {
			priority: 0,
			requires: vec![],
			provides: vec![(block).encode()],
			longevity: TransactionLongevity::max_value(),
			propagate: true,
		}),
		_ => InvalidTransaction::Call.into()
		}
	}
}
