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

// everything define in pallet mod must be public
pub use pallet::*;
use codec::{Codec, Encode, Decode};
use sp_core::crypto::KeyTypeId;

pub mod urls;
pub mod utils;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod tests;

const TOKEN_SERVER_URL: &str = "http://127.0.0.1:4000";
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw!");

#[frame_support::pallet]
pub mod pallet {
	/// Unique key for query
	#[derive(Encode, Decode, Default, Debug)]
	pub struct QueryKey<AccountId> {
		account: AccountId,
		data_source: urls::DataSource,
	}
	
	pub mod crypto {
		use super::KEY_TYPE;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify, MultiSignature, MultiSigner,
		};
		use sp_core::sr25519::Signature as Sr25519Signature;
		app_crypto!(sr25519, KEY_TYPE);
	
		pub struct TestAuthId;
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	
		impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	use crate::*;
	use frame_system::pallet_prelude::*;
	use core::{convert::TryInto,};
	use sp_std::{prelude::*, fmt::Debug, collections::btree_map::{BTreeMap, Entry,}};
	use frame_system::{
	ensure_signed,
	offchain::{CreateSignedTransaction, Signer, AppCrypto, SendSignedTransaction,},
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
		};
	
	use frame_support::{dispatch, traits::{Currency, OnUnbalanced, Imbalance},};
	use sp_runtime::offchain::{storage::StorageValueRef,};
	use sp_runtime::traits::{AtLeast32BitUnsigned, Member, MaybeSerializeDeserialize,};
	use weights::WeightInfo;

	type PositiveImbalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config + account_linker::Config + CreateSignedTransaction<Call<Self>> {
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
			MaybeSerializeDeserialize;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: From<Call<Self>>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type QueryTaskRedundancy: Get<u32>;
		type QuerySessionLength: Get<u32>;
		/// Currency type for this pallet.
		type Currency: Currency<Self::AccountId>;
		/// Handler for the unbalanced increment when rewarding (minting rewards)
		type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
		type OcwQueryReward: Get<<<Self as Config>::Currency as Currency<<Self as frame_system::Config>::AccountId>>::Balance>;
		type WeightInfo: weights::WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			debug::info!("ocw on_initialize {:?}.", block_number);
			1000
		}

		fn on_finalize(block_number: T::BlockNumber) {
			debug::info!("ocw on_finalize.{:?}.", block_number);

			let query_session_length: usize = T::QuerySessionLength::get() as usize;
			let index_in_session = TryInto::<usize>::try_into(block_number).map_or(query_session_length, |bn| bn % query_session_length);
			let last_block_number = query_session_length - 1;

			// Clear claim at the first block of a session
			if index_in_session == 0 {
				Self::clear_claim();
			// Do aggregation at last block of a session
			} else if index_in_session == last_block_number {
				Self::aggregate_query_result();
			}
		}

		// TODO block N offchain_worker will be called after block N+1 finalize
		// Trigger by offchain framework in each block
		fn offchain_worker(block_number: T::BlockNumber) {
			let query_session_length: usize = T::QuerySessionLength::get() as usize;

			let index_in_session = TryInto::<usize>::try_into(block_number).map_or(query_session_length, |bn| bn % query_session_length);

			// Start query at second block of a session
			if index_in_session == 1 {
				Self::start(block_number);
			}
		}
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", T::BlockNumber = "BlockNumber")]
	pub enum Event<T: Config> {
		BalanceGot(T::AccountId, T::BlockNumber, Option<u128>, Option<u128>),
	}
	
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error number parsing.
		InvalidNumber,
		/// Account already in claim list.
		AccountAlreadyInClaimlist,
		/// Invalid data source
		InvalidDataSource,
		/// Invalid commit block number
		InvalidCommitBlockNumber,
		/// Invalid commit slot
		InvalidCommitSlot,
		/// Invalid account index
		InvalidAccountIndex,
		/// Offchain worker index overflow
		OffchainWorkerIndexOverflow,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn total_claims)]
	pub(super) type TotalClaims<T: Config> = StorageValue<_, u64>;
	
	#[pallet::storage]
	#[pallet::getter(fn query_account_set)]
	pub(super) type ClaimAccountSet<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn claim_account_index)]
	pub(super) type ClaimAccountIndex<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, Option<u32>, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn account_balance)]
	pub(super) type AccountBalance<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, (Option<u128>, Option<u128>), ValueQuery>;

// 		/// Record account's btc and ethereum balance
	#[pallet::storage]
	#[pallet::getter(fn commit_account_balance)]
	pub(super) type CommitAccountBalance<T: Config> =  StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, QueryKey<T::AccountId>, Option<u128>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn ocw_account_index)]
	pub(super) type OcwAccountIndex<T: Config> =  StorageMap<_, Blake2_128Concat, T::AccountId, Option<u32>, ValueQuery>;


	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(<T as pallet::Config>::WeightInfo::asset_claim())]
		pub fn asset_claim(origin: OriginFor<T>,) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			// If the same claim already in set
			ensure!(!<ClaimAccountSet<T>>::contains_key(&account), Error::<T>::AccountAlreadyInClaimlist);

			<ClaimAccountSet<T>>::insert(&account, ());

			Ok(().into())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::submit_balance())]
		fn submit_balance(origin: OriginFor<T>, account: T::AccountId, block_number: T::BlockNumber, data_source: urls::DataSource, balance: u128)-> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Check data source
			Self::valid_data_source(data_source)?;

			// Check block number
			Self::valid_commit_block_number(block_number, <frame_system::Module<T>>::block_number())?;

			// Check the commit slot
			Self::valid_commit_slot(account.clone(), Self::get_ocw_index(Some(&account)), data_source)?;

			// put query result on chain
			CommitAccountBalance::<T>::insert(&sender, &QueryKey{account, data_source}, Some(balance));

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		// Main entry for ocw
		fn query(block_number: T::BlockNumber, info: &urls::TokenInfo) {
			// Get my ocw account for submit query result
			let offchain_worker_account = StorageValueRef::persistent(b"offchain-worker::account");

			// Get my ocw index
			let ocw_account_index = match offchain_worker_account.get::<T::AccountId>() {
				Some(Some(account)) => Self::get_ocw_index(Some(&account)),
				_ => Self::get_ocw_index(None),
			};

			// ocw length
			let mut ocw_length = Self::get_ocw_length();
			if ocw_length == 0 {
				// No ocw in last round, set it as one, then new ocw query for all accounts and all data source
				ocw_length = 1;
			}

			// Loop for each account
			for item in <ClaimAccountIndex<T>>::iter() {
				let account: T::AccountId = item.0;
				match item.1 {
					Some(account_index) => {

						let mut source_index = 0;
						for source in &urls::DATA_SOURCE_LIST {
							let task_index = urls::TOTAL_DATA_SOURCE_NUMBER * account_index + source_index;
							if task_index % ocw_length == ocw_account_index {
								match source {
									urls::DataSource::EthEtherScan => {
										match Self::get_balance_from_etherscan(&account, info) {
											Some(balance) => Self::offchain_signed_tx(account.clone(), block_number, urls::DataSource::EthEtherScan, balance),
											None => ()
										}
									},
									urls::DataSource::EthInfura => {
										match Self::get_balance_from_infura(&account, info) {
											Some(balance) => Self::offchain_signed_tx(account.clone(), block_number, urls::DataSource::EthInfura, balance),
											None => ()
										}
									},
									urls::DataSource::BtcBlockChain => {
										match Self::get_balance_from_blockchain_info(&account, info) {
											Some(balance) => Self::offchain_signed_tx(account.clone(), block_number, urls::DataSource::BtcBlockChain, balance),
											None => ()
										}
									},
									_ => (),
								};
							}
							source_index = source_index + 1;
					}	
				},
				None => (),
			}
		}
	}

		// Clear claim accounts in last session
		fn clear_claim() {
			// Remove all account index in last session
			<ClaimAccountIndex<T>>::remove_all();

			let accounts: Vec<T::AccountId> = <ClaimAccountSet::<T>>::iter().map(|(k, _)| k).collect();

			// Set account index
			for (index, account) in accounts.iter().enumerate() {
				<ClaimAccountIndex<T>>::insert(&account, Some(index as u32));
			}

			// Remove all claimed accounts
			<ClaimAccountSet::<T>>::remove_all();
		}

		// Start new round of offchain worker
		fn start(block_number: T::BlockNumber) {
			let local_token = StorageValueRef::persistent(b"offchain-worker::token");

			match local_token.get::<urls::TokenInfo>() {
				Some(Some(token)) => {
					Self::query(block_number, &token);
				},
				_ => {
					// Get token from local server
					let _ = urls::get_token();
				},
			};
		}

		// Aggregate query result and then record on chain
		fn aggregate_query_result() {
			let mut result_map: BTreeMap<(T::AccountId, urls::BlockChainType, u128), u32> = BTreeMap::new();
			let mut result_key: BTreeMap<(T::AccountId, urls::BlockChainType), Vec<u128>> = BTreeMap::new();
			// Statistics for result
			for result in <CommitAccountBalance<T>>::iter() {

				let account: T::AccountId = result.1.account;
				let data_source: urls::DataSource = result.1.data_source;
				let block_type: urls::BlockChainType = urls::data_source_to_block_chain_type(data_source);

				match result.2 {
					Some(balance) => {
						let map_key = (account.clone(), block_type, balance);

						result_map.entry(map_key.clone()).or_insert(1_32);
		
						match result_map.entry(map_key.clone()) {
							Entry::Occupied(mut entry) => {
								*entry.get_mut() = entry.get() + 1;
							},
							Entry::Vacant(v) => {v.insert(1_u32);} ,
						};
		
						let key_key = (account, block_type);
						match result_key.get(&key_key) {
							Some(balance_vec) => {
								let mut found = false;
								for item in balance_vec.iter() {
									if *item == balance {
										found = true;
										break;
									}
								}
								if !found {
									let mut new_balance_vec: Vec<u128> = balance_vec.clone();
									new_balance_vec.push(balance);
									result_key.insert(key_key, new_balance_vec);
								}
							},
							None => {result_key.insert(key_key, vec![balance]);},
						};
					},
					None => (),
				}
			}

			// Store on chain, record_map will used to reward ocw.
			let mut record_map: BTreeMap<(T::AccountId, urls::BlockChainType), u128> = BTreeMap::new();
			for result in result_key.iter() {
				let account: T::AccountId = result.0.0.clone();
				let block_type: urls::BlockChainType = result.0.1;

				let mut most_value = 0_u128;
				let mut most_times = 0_u32;

				for balance in result.1 {
					let key = (account.clone(), block_type, *balance);
					match result_map.get(&key) {
						Some(frequence) => {
							if *frequence > most_times {
								most_times = *frequence;
								most_value = *balance;
							}
						},
						None => {},
					}
				}
				record_map.insert((account.clone(), block_type), most_value);

				// Update balance on chain
				if block_type == urls::BlockChainType::ETH {
					<AccountBalance<T>>::mutate(account,
						|value| value.1 = Some(most_value)
					);
					Self::increment_total_claims();
				} else if block_type == urls::BlockChainType::BTC {
					<AccountBalance<T>>::mutate(account,
						|value| value.0 = Some(most_value)
					);
					Self::increment_total_claims();
				}
			}

			// Remove all old ocw index
			<OcwAccountIndex<T>>::remove_all();

			let mut account_index = 0_u32;
			let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

			// Put account into index map for next session
			for result in <CommitAccountBalance<T>>::iter() {
				let ocw_account: T::AccountId = result.0;
				let query_account: T::AccountId = result.1.account;
				let data_source: urls::DataSource = result.1.data_source;
				let block_type: urls::BlockChainType = urls::data_source_to_block_chain_type(data_source);

				match result.2 {
					Some(committed_balance) => {
						// reward the ocw
						match record_map.get(&(query_account, block_type)) {
							Some(balance) => {
								// balance matched
								if *balance == committed_balance {
									let r = T::Currency::deposit_into_existing(&ocw_account, T::OcwQueryReward::get()).ok();
									total_imbalance.maybe_subsume(r);
								}
							},
							None => {},
						}
						// update index for next session
						match Self::ocw_account_index(ocw_account.clone()) {
							Some(_) => {},
							None => {
								<OcwAccountIndex<T>>::insert(ocw_account, Some(account_index));
								account_index = account_index + 1;
							},
						}
					},
					None => (),
				}				
			}

			T::Reward::on_unbalanced(total_imbalance);

			// Remove all ocw commit in this session after aggregation
			<CommitAccountBalance<T>>::remove_all();
		}

		fn increment_total_claims() {
			match Self::total_claims() {
				Some(claims) => TotalClaims::<T>::put(claims + 1),
				None => TotalClaims::<T>::put(1),
			}
		}

		fn valid_commit_slot(account: T::AccountId, ocw_index: u32, data_source: urls::DataSource) -> dispatch::DispatchResult {
			// account claimed the asset query
			let ocw_account_index = Self::get_account_index(account)?;

			// ocw length
			let ocw_length = Self::get_ocw_length();
			// if no ocw works in last session, then all new ocw valid for all accounts with all data source
			if ocw_length == 0 {
				return Ok(())
			}

			// ensure ocw index is valid
			ensure!(ocw_index <= ocw_length, <Error<T>>::OffchainWorkerIndexOverflow);

			// ensure data source is valid
			ensure!(data_source != urls::DataSource::Invalid, <Error<T>>::InvalidDataSource);

			// get data source index
			let data_source_index = urls::data_source_to_index(data_source);

			// query task rounds
			let query_task_redudancy: u32 = T::QueryTaskRedundancy::get();

			// task number per round
			let total_task_per_round = urls::TOTAL_DATA_SOURCE_NUMBER * Self::get_claim_account_length();

			// task index in the first round
			let task_base_index = data_source_index + ocw_account_index * urls::TOTAL_DATA_SOURCE_NUMBER;

			let mut round: u32 = 0;
			while round < query_task_redudancy {
				// task index in n round
				let task_index = task_base_index + round * total_task_per_round;

				if task_index >= ocw_index {
					// if index match return Ok
					if (task_index - ocw_index) % ocw_length == 0 {
						return Ok(())
					}
				}
				round = round + 1;
			}

			// no match found, return error
			Err(<Error<T>>::InvalidCommitSlot.into())
		}

		// get claim account index
		fn get_account_index(account: T::AccountId) -> Result<u32, Error<T>> {
			match Self::claim_account_index(account) {
				Some(index) => Ok(index),
				None => Err(<Error<T>>::InvalidAccountIndex.into()),
			}
		}

		// Check data source
		fn valid_data_source(data_source: urls::DataSource) -> dispatch::DispatchResult {
			match data_source {
				urls::DataSource::Invalid => Err(<Error<T>>::InvalidDataSource.into()),
				_ => Ok(()),
			}
		}

		// Check the block number
		fn valid_commit_block_number(commit_block_number: T::BlockNumber, current_block_number: T::BlockNumber) -> dispatch::DispatchResult {
			let zero_block: u32 = 0;
			let commit_block_number: u32 = TryInto::<usize>::try_into(commit_block_number).map_or(zero_block, |block_number| block_number as u32);
			let current_block_number: u32 = TryInto::<usize>::try_into(current_block_number).map_or(zero_block, |block_number| block_number as u32);

			// Basic check for both block number
			if commit_block_number == 0 || current_block_number == 0 {
				return Err(<Error<T>>::InvalidCommitBlockNumber.into());
			}

			// Compute the scope of session
			let sesseion_start_block = commit_block_number -  commit_block_number % T::QuerySessionLength::get() ;
			let sesseion_end_block = sesseion_start_block + T::QuerySessionLength::get();

			// If commit block number out of the scope of session.
			if current_block_number >= sesseion_end_block || current_block_number <= sesseion_start_block {
				return Err(<Error<T>>::InvalidCommitBlockNumber.into());
			}

			Ok(())
		}

		// Get index from map or use length of map for new ocw
		fn get_ocw_index(account: Option<&T::AccountId>) -> u32 {
			match account {
				Some(account) => match Self::ocw_account_index(account) {
					Some(index_in_map) => index_in_map,
					None => Self::get_ocw_length(),
				},
				None => Self::get_ocw_length(),
			}
		}

		// Get the length of accounts
		fn get_ocw_length() -> u32 {
			<OcwAccountIndex::<T>>::iter().collect::<Vec<_>>().len() as u32
		}

		// Get the length of accounts
		fn get_claim_account_length() -> u32 {
			<ClaimAccountIndex::<T>>::iter().collect::<Vec<_>>().len() as u32
		}

		fn get_balance_from_etherscan(account: &T::AccountId, info: &urls::TokenInfo) -> Option<u128> {
			if info.etherscan.len() == 0 {
				None
			} else {
				match core::str::from_utf8(&info.etherscan) {
					Ok(token) => {
						let get = urls::HttpGet {
							blockchain: urls::BlockChainType::ETH,
							prefix: "https://api-ropsten.etherscan.io/api?module=account&action=balancemulti&address=0x",
							delimiter: ",0x",
							postfix: "&tag=latest&apikey=",
							api_token: token,
						};

						Self::fetch_balances(
							<account_linker::Module<T>>::eth_addresses(account),
							urls::HttpRequest::GET(get),
							&urls::parse_etherscan_balances).ok()
					},
					Err(_) => None,
				}
			}
		}

		fn get_balance_from_infura(account: &T::AccountId, info: &urls::TokenInfo) -> Option<u128> {

			if info.infura.len() == 0 {
				None
			} else {
				match core::str::from_utf8(&info.infura) {
					Ok(token) => {
						let post = urls::HttpPost {
							url_main: "https://ropsten.infura.io/v3/",
							blockchain: urls::BlockChainType::ETH,
							prefix: r#"[{"jsonrpc":"2.0","method":"eth_getBalance","id":1,"params":["0x"#,
							delimiter: r#"","latest"]},{"jsonrpc":"2.0","method":"eth_getBalance","id":1,"params":["0x"#,
							postfix: r#"","latest"]}]"#,
							api_token: token,
						};
						Self::fetch_balances(
							<account_linker::Module<T>>::eth_addresses(account),
							urls::HttpRequest::POST(post),
							&urls::parse_blockchain_info_balances).ok()
					},
					Err(_) => None,
				}
			}
		}

		// TODO account not input request parameter
		fn get_balance_from_blockchain_info(_account: &T::AccountId, info: &urls::TokenInfo) -> Option<u128> {
			if info.blockchain.len() == 0 {
				None
			} else {
				match core::str::from_utf8(&info.blockchain) {
					Ok(token) => {
						let get = urls::HttpGet {
								blockchain: urls::BlockChainType::BTC,
								prefix: "https://blockchain.info/balance?active=",
								delimiter: "%7C",
								postfix: "",
								api_token: token,
						};
						Self::fetch_balances(Vec::new(),
							urls::HttpRequest::GET(get),
							&urls::parse_blockchain_info_balances).ok()
					},
					Err(_) => None,
				}
			}
		}

		// Sign the query result
		fn offchain_signed_tx(account: T::AccountId, block_number: T::BlockNumber, data_source: urls::DataSource, balance: u128) {
			debug::info!("ocw sign tx: account {:?}, block number {:?}, data_source {:?}, balance {:?}",
				account.clone(), block_number, data_source, balance);
			// Get signer from ocw
			let signer = Signer::<T, T::AuthorityId>::any_account();

			let result = signer.send_signed_transaction(|_acct|
				// This is the on-chain function
				Call::submit_balance(account.clone(), block_number, data_source, balance)
			);

			// Display error if the signed tx fails.
			if let Some((acc, res)) = result {
				if res.is_err() {
					debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				} else {
					debug::info!("successful: offchain_signed_tx: tx sent: {:?} index is {:?}", acc.id, acc.index);
				}

				// Record the account in local storage then we can know my index
				let account = StorageValueRef::persistent(b"offchain-worker::account");
				account.set(&acc.id);
			} else {
				debug::error!("No local account available");
			}
		}

		// Generic function to fetch balance for specific link type
		pub fn fetch_balances(wallet_accounts: Vec<[u8; 20]>, request: urls::HttpRequest,
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

						link.extend(utils::address_to_string(each_account));
					}
					link.extend(get_req.postfix.as_bytes());
					link.extend(get_req.api_token.as_bytes());

					// Fetch json response via http get
					urls::fetch_json_http_get(&link[..]).map_err(|_| Error::<T>::InvalidNumber)?
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

						body.extend(utils::address_to_string(each_account));
					}
					body.extend(post_req.postfix.as_bytes());

					// Fetch json response via http post
					urls::fetch_json_http_post(&link[..], &body[..]).map_err(|_| Error::<T>::InvalidNumber)?
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
	}
}