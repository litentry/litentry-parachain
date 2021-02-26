// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::*;
use crate as offchain_worker;
use frame_support::parameter_types;
use sp_core::{ H256, sr25519::Signature,};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, IdentityLookup, Extrinsic as ExtrinsicT,
		IdentifyAccount, Verify,
	},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the module, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		AccountLinker: account_linker::{Module, Call, Storage, Event<T>},
		OffchainWorker: offchain_worker::{Module, Call, Storage, Event<T>,},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	/// The type for recording an account's balance.
	type Balance = u128;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

type Extrinsic = TestXt<Call, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl account_linker::Config for Test {
	type Event = Event;
	type WeightInfo = ();
}

parameter_types! {
	pub const QueryTaskRedundancy: u32 = 3;
	pub const QuerySessionLength: u32 = 5;
	pub const OcwQueryReward: u128 = 1;
}

impl Config for Test {
	type AuthorityId = offchain_worker::crypto::TestAuthId;
	type Call = Call;
	type Event = Event;
	type Balance = u128;
	type QueryTaskRedundancy = QueryTaskRedundancy;
	type QuerySessionLength = QuerySessionLength;
	type Currency = Balances;
	type Reward = ();
	type OcwQueryReward = OcwQueryReward;
	type WeightInfo = ();
}


#[test]
fn test_chars_to_u128() {
	let correct_balance = vec!['5', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
	assert_eq!(Ok(500000000000000000_u128), utils::chars_to_u128(&correct_balance));

	let correct_balance = vec!['a', '2'];
	assert_eq!(Err("Wrong u128 balance data format"), utils::chars_to_u128(&correct_balance));

	let correct_balance = vec!['0', 'x', 'f', 'e'];
	assert_eq!(Ok(254_u128), utils::chars_to_u128(&correct_balance));

	// Corner case check
	let correct_balance = vec!['0', 'x'];
	assert_eq!(Ok(0_u128), utils::chars_to_u128(&correct_balance));
}


#[test]
fn test_parse_etherscan_balances() {
	let double_balances = r#"
	{
	"status": "1",
	"message": "OK",
	"result":
		[
			{"account":"0x742d35Cc6634C0532925a3b844Bc454e4438f44e","balance":"12"},
			{"account":"0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8","balance":"21"}
		]
	}"#;
	assert_eq!(Some(vec![12, 21]), urls::parse_etherscan_balances(double_balances));
}

#[test]
fn test_parse_etherscan_balances_2() {
	let double_balances = r#"
	{
	"status": "1",
	"message": "OK",
	"result":
		[
			{"account":"0x742d35Cc6634C0532925a3b844Bc454e4438f44e","balance":"12"},
			{"account":"0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8","balance":"21"}
		]
	}"#;

	let token_info: urls::EtherScanResponse = serde_json::from_str(&double_balances).unwrap();
	assert_eq!(token_info.status, "1".as_bytes().to_vec());
	assert_eq!(token_info.result[0].balance, "12".as_bytes().to_vec());
}

#[test]
fn test_parse_blockchain_info_balances() {
	let double_balances = r#"
	{
		"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":{"final_balance":30,"n_tx":2635,"total_received":6835384571},
		"15EW3AMRm2yP6LEF5YKKLYwvphy3DmMqN6":{"final_balance":1220,"n_tx":4,"total_received":310925609}
	}"#;
	let result = urls::parse_blockchain_info_balances(double_balances);
	assert_eq!(true, (Some(vec![30, 1220]) == result || Some(vec![1220, 30]) == result));

	// Test case should fail because fraction of the first balance value is non zero
	let double_balances = r#"
	{
		"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":{"final_balance":30.5,"n_tx":2635,"total_received":6835384571},
		"15EW3AMRm2yP6LEF5YKKLYwvphy3DmMqN6":{"final_balance":1220,"n_tx":4,"total_received":310925609}
	}"#;
	assert_eq!(Some(vec![1220]), urls::parse_blockchain_info_balances(double_balances));

	// Test case should fail because first balance value is negative
	let double_balances = r#"
	{
		"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa":{"final_balance":-30,"n_tx":2635,"total_received":6835384571},
		"15EW3AMRm2yP6LEF5YKKLYwvphy3DmMqN6":{"final_balance":1220,"n_tx":4,"total_received":310925609}
	}"#;
	assert_eq!(Some(vec![1220]), urls::parse_blockchain_info_balances(double_balances));
}

#[test]
fn test_parse_infura_balances() {
	let double_balances = r#"
	[
		{"jsonrpc":"2.0","id":1,"result":"0x4563918244f40000"},
		{"jsonrpc":"2.0","id":1,"result":"0xff"}
	]
	"#;

	assert_eq!(Some(vec![5000000000000000000, 255]), urls::parse_infura_balances(double_balances));
}

#[test]
fn test_parse_infura_balances_2() {
	let double_balances = r#"
	[
		{"jsonrpc":"2.0","id":1,"result":"0x4563918244f40000"},
		{"jsonrpc":"2.0","id":1,"result":"0xff"}
	]
	"#;
	let token_info: Vec<urls::InfuraBalance> = serde_json::from_str(double_balances).unwrap();
	assert_eq!(token_info[0].id, 1);

}

// fetch_balances only executed in offchain worker context, need investigate how to call it in test
// #[test]
// fn test_fetch_balances() {	
// 	let get = urls::HttpGet {
// 		blockchain: urls::BlockChainType::ETH,
// 		prefix: "https://api-ropsten.etherscan.io/api?module=account&action=balancemulti&address=0x",
// 		delimiter: ",0x",
// 		postfix: "&tag=latest&apikey=",
// 		api_token: "RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB",
// 	};

// 	let test_account = "4d88dc5D528A33E4b8bE579e9476715F60060582".as_bytes();
// 	let mut test_account_byte_array = [0u8; 20];
// 	test_account_byte_array.copy_from_slice(&test_account[0..20]);
	
// 	let mut accounts: Vec<[u8; 20]> = Vec::new();
// 	accounts.push(test_account_byte_array);

// 	sp_io::TestExternalities::default().execute_with(|| {
// 		match <Module<Test>>::fetch_balances(accounts, urls::HttpRequest::GET(get), &urls::parse_etherscan_balances) {
// 			Ok(b) => assert_eq!(500000000000000000_u128, b),
// 			Err(_) => panic!("Error occurs in test_fetch_balance!!"),
// 		};
// 	});
// }