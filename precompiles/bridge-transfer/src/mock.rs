// Copyright 2020-2023 Trust Computing GmbH.
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

use super::*;
use fp_evm::IsPrecompileResult;
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU32, ConstU64, SortedMembers},
	weights::Weight,
};
use hex_literal::hex;
use pallet_evm::{
	AddressMapping, EnsureAddressNever, EnsureAddressRoot, PrecompileResult, PrecompileSet,
};
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

pub type AccountId = AccountId32;
pub type Balance = u128;
pub type BlockNumber = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub const MAXIMUM_ISSURANCE: u64 = 20_000_000_000_000;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Evm: pallet_evm::{Pallet, Config, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Bridge: pallet_bridge::{Pallet, Call, Storage, Event<T>},
		BridgeTransfer: pallet_bridge_transfer::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type PalletInfo = PalletInfo;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<100>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type HoldIdentifier = ();
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}

parameter_types! {
	pub const TestChainId: u8 = 5;
	pub const ProposalLifetime: u64 = 100;
	pub const TreasuryAccount:u64 = 0x8;
}

impl pallet_bridge::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BridgeCommitteeOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Proposal = RuntimeCall;
	type BridgeChainId = TestChainId;
	type Currency = Balances;
	type ProposalLifetime = ProposalLifetime;
	type TreasuryAccount = TreasuryAccount;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaximumIssuance: u64 = MAXIMUM_ISSURANCE;
	pub const ExternalTotalIssuance: u64 = MAXIMUM_ISSURANCE;
	// bridge::derive_resource_id(1, &bridge::hashing::blake2_128(b"LIT"));
	pub const NativeTokenResourceId: [u8; 32] = hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
}

pub struct TransferNativeAnyone;
impl SortedMembers<AccountId> for TransferNativeAnyone {
	fn sorted_members() -> Vec<AccountId> {
		vec![]
	}

	fn contains(_who: &AccountId) -> bool {
		true
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(_: &AccountId) {
		unimplemented!()
	}
}

impl pallet_bridge_transfer::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BridgeOrigin = pallet_bridge::EnsureBridge<Test>;
	type TransferNativeMembers = TransferNativeAnyone;
	type SetMaximumIssuanceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type NativeTokenResourceId = NativeTokenResourceId;
	type DefaultMaximumIssuance = MaximumIssuance;
	type ExternalTotalIssuance = ExternalTotalIssuance;
	type WeightInfo = ();
}

parameter_types! {
	pub const VerifyPRuntime: bool = false;
	pub const VerifyRelaychainGenesisBlockHash: bool = false;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

pub fn precompile_address() -> H160 {
	// 0x502d
	H160::from_low_u64_be(20480 + 45)
}

#[derive(Debug, Clone, Copy)]
pub struct BridgeTransferMockPrecompile<R>(PhantomData<R>);

impl<R> PrecompileSet for BridgeTransferMockPrecompile<R>
where
	R: pallet_evm::Config,
	BridgeTransferPrecompile<R>: Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			a if a == precompile_address() => Some(BridgeTransferPrecompile::<R>::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: sp_core::H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer { is_precompile: address == precompile_address(), extra_cost: 0 }
	}
}

pub struct TruncatedAddressMapping;
impl AddressMapping<AccountId> for TruncatedAddressMapping {
	fn into_account_id(address: H160) -> AccountId {
		let mut data = [0u8; 32];
		data[0..20].copy_from_slice(&address[..]);
		AccountId::from(Into::<[u8; 32]>::into(data))
	}
}

pub trait IntoH160 {
	fn into_h160(&self) -> H160;
}
// silly for test purpose only
impl IntoH160 for u8 {
	fn into_h160(x: u8) -> H160 {
		H160::repeat_byte(x)
	}
}

pub trait IntoAccountId {
	fn into_account_id(&self) -> AccountId;
}
impl IntoAccountId for u8 {
	fn from(x: u8) -> AccountId {
		TruncatedAddressMapping::into_account_id(x.into_h160())
	}
}

parameter_types! {
	pub PrecompilesValue: BridgeTransferMockPrecompile<Test> = BridgeTransferMockPrecompile(Default::default());
	pub WeightPerGas: Weight = Weight::from_parts(1, 0);
}

impl pallet_evm::Config for Test {
	type FeeCalculator = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type CallOrigin = EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = EnsureAddressNever<AccountId>;
	type AddressMapping = TruncatedAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = BridgeTransferMockPrecompile<Test>;
	type PrecompilesValue = PrecompilesValue;
	type Timestamp = Timestamp;
	type ChainId = ();
	type OnChargeTransaction = ();
	type BlockGasLimit = ();
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type FindAuthor = ();
	type OnCreate = ();
	type WeightInfo = ();
	type GasLimitPovSizeRatio = ConstU64<4>;
}

pub const ENDOWED_BALANCE: Balance = 100_000_000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let bridge_id: AccountId = 0u8.into_account_id();
	let treasury_account: AccountId = 8u8.into_account_id();
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(bridge_id, ENDOWED_BALANCE),
			(1u8.into_account_id(), ENDOWED_BALANCE),
			(treasury_account, ENDOWED_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| frame_system::Pallet::<Test>::set_block_number(1));
	ext
}

// Checks events against the latest. A contiguous set of events must be provided. They must
// include the most recent event, but do not have to include every past event.
pub fn assert_events(mut expected: Vec<RuntimeEvent>) {
	let mut actual: Vec<RuntimeEvent> =
		frame_system::Pallet::<Test>::events().iter().map(|e| e.event.clone()).collect();

	expected.reverse();

	for evt in expected {
		let next = actual.pop().expect("event expected");
		assert_eq!(next, evt, "Events don't match");
	}
}
