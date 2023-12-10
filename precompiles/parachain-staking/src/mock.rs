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
	construct_runtime, parameter_types,
	traits::{ConstU64, Everything, GenesisBuild, OnFinalize, OnInitialize},
	weights::Weight,
};
use pallet_evm::{
	AddressMapping, EnsureAddressNever, EnsureAddressRoot, PrecompileResult, PrecompileSet,
};
use pallet_parachain_staking::{InflationInfo, Range};
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, Perbill, Percent,
};

pub type AccountId = AccountId32;
pub type Balance = u128;
pub type BlockNumber = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Evm: pallet_evm::{Pallet, Config<T>, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ParachainStaking: pallet_parachain_staking::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = Weight::from_parts(1024, 0);
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const SS58Prefix: u8 = 42;
}
impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
}
impl pallet_balances::Config for Test {
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 4];
	type MaxLocks = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type HoldIdentifier = ();
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}
parameter_types! {
	pub const MinBlocksPerRound: u32 = 3;
	pub const DefaultBlocksPerRound: u32 = 5;
	pub const LeaveCandidatesDelay: u32 = 2;
	pub const CandidateBondLessDelay: u32 = 2;
	pub const LeaveDelegatorsDelay: u32 = 2;
	pub const RevokeDelegationDelay: u32 = 2;
	pub const DelegationBondLessDelay: u32 = 2;
	pub const RewardPaymentDelay: u32 = 2;
	pub const MinSelectedCandidates: u32 = 5;
	pub const MaxTopDelegationsPerCandidate: u32 = 4;
	pub const MaxBottomDelegationsPerCandidate: u32 = 4;
	pub const MaxDelegationsPerDelegator: u32 = 4;
	pub const DefaultCollatorCommission: Perbill = Perbill::from_percent(20);
	pub const DefaultParachainBondReservePercent: Percent = Percent::from_percent(30);
	pub const MinCollatorStk: u128 = 10;
	pub const MinDelegatorStk: u128 = 5;
	pub const MinDelegation: u128 = 3;
}
impl pallet_parachain_staking::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
	type MinBlocksPerRound = MinBlocksPerRound;
	type DefaultBlocksPerRound = DefaultBlocksPerRound;
	type LeaveCandidatesDelay = LeaveCandidatesDelay;
	type CandidateBondLessDelay = CandidateBondLessDelay;
	type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
	type RevokeDelegationDelay = RevokeDelegationDelay;
	type DelegationBondLessDelay = DelegationBondLessDelay;
	type RewardPaymentDelay = RewardPaymentDelay;
	type MinSelectedCandidates = MinSelectedCandidates;
	type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
	type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
	type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
	type DefaultCollatorCommission = DefaultCollatorCommission;
	type DefaultParachainBondReservePercent = DefaultParachainBondReservePercent;
	type MinCollatorStk = MinCollatorStk;
	type MinCandidateStk = MinCollatorStk;
	type MinDelegatorStk = MinDelegatorStk;
	type MinDelegation = MinDelegation;
	type OnCollatorPayout = ();
	type OnNewRound = ();
	type WeightInfo = ();
	type IssuanceAdapter = ();
}

pub fn precompile_address() -> H160 {
	// 0x502d
	H160::from_low_u64_be(20480 + 45)
}

#[derive(Debug, Clone, Copy)]
pub struct ParachainStakingMockPrecompile<R>(PhantomData<R>);

impl<R> PrecompileSet for ParachainStakingMockPrecompile<R>
where
	R: pallet_evm::Config,
	ParachainStakingPrecompile<R>: Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			a if a == precompile_address() =>
				Some(ParachainStakingPrecompile::<R>::execute(handle)),
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

// silly for test purpose only
impl From<u8> for H160 {
	fn from(x: u8) -> H160 {
		H160::repeat_byte(x)
	}
}

impl From<u8> for AccountId {
	fn from(x: u8) -> AccountId {
		TruncatedAddressMapping::into_account_id(Into::<H160>::into(x))
	}
}
parameter_types! {
	pub PrecompilesValue: ParachainStakingMockPrecompile<Test> = ParachainStakingMockPrecompile(Default::default());
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
	type PrecompilesType = ParachainStakingMockPrecompile<Test>;
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

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

pub(crate) struct ExtBuilder {
	// endowed accounts with balances
	balances: Vec<(AccountId, Balance)>,
	// [collator, amount]
	collators: Vec<(AccountId, Balance)>,
	// [delegator, collator, delegation_amount]
	delegations: Vec<(AccountId, AccountId, Balance, Percent)>,
	// inflation config
	inflation: InflationInfo<Balance>,
}

impl Default for ExtBuilder {
	fn default() -> ExtBuilder {
		ExtBuilder {
			balances: vec![],
			delegations: vec![],
			collators: vec![],
			inflation: InflationInfo {
				expect: Range { min: 700, ideal: 700, max: 700 },
				// not used
				annual: Range {
					min: Perbill::from_percent(50),
					ideal: Perbill::from_percent(50),
					max: Perbill::from_percent(50),
				},
				// unrealistically high parameterization, only for testing
				round: Range {
					min: Perbill::from_percent(5),
					ideal: Perbill::from_percent(5),
					max: Perbill::from_percent(5),
				},
			},
		}
	}
}

impl ExtBuilder {
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub(crate) fn with_candidates(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
		self.collators = collators;
		self
	}

	pub(crate) fn with_delegations(
		mut self,
		delegations: Vec<(AccountId, AccountId, Balance)>,
	) -> Self {
		self.delegations =
			delegations.into_iter().map(|d| (d.0, d.1, d.2, Percent::zero())).collect();
		self
	}

	pub(crate) fn with_auto_compounding_delegations(
		mut self,
		delegations: Vec<(AccountId, AccountId, Balance, Percent)>,
	) -> Self {
		self.delegations = delegations;
		self
	}

	#[allow(dead_code)]
	pub(crate) fn with_inflation(mut self, inflation: InflationInfo<Balance>) -> Self {
		self.inflation = inflation;
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.expect("Frame system builds valid default genesis config");

		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.expect("Pallet balances storage can be assimilated");
		pallet_parachain_staking::GenesisConfig::<Test> {
			candidates: self.collators,
			delegations: self.delegations,
			inflation_config: self.inflation,
		}
		.assimilate_storage(&mut t)
		.expect("Parachain Staking's storage can be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

/// Rolls forward one block. Returns the new block number.
pub(crate) fn roll_one_block() -> u64 {
	ParachainStaking::on_finalize(System::block_number());
	Balances::on_finalize(System::block_number());
	System::on_finalize(System::block_number());
	System::set_block_number(System::block_number() + 1);
	System::on_initialize(System::block_number());
	Balances::on_initialize(System::block_number());
	ParachainStaking::on_initialize(System::block_number());
	System::block_number()
}

/// Rolls to the desired block. Returns the number of blocks played.
pub(crate) fn roll_to(n: u64) -> u64 {
	let mut num_blocks = 0;
	let mut block = System::block_number();
	while block < n {
		block = roll_one_block();
		num_blocks += 1;
	}
	num_blocks
}

/// Rolls block-by-block to the beginning of the specified round.
/// This will complete the block in which the round change occurs.
/// Returns the number of blocks played.
pub(crate) fn roll_to_round_begin(round: u64) -> u64 {
	let block = (round - 1) * DefaultBlocksPerRound::get() as u64;
	roll_to(block)
}

/// Rolls block-by-block to the end of the specified round.
/// The block following will be the one in which the specified round change occurs.
pub(crate) fn roll_to_round_end(round: u64) -> u64 {
	let block = round * DefaultBlocksPerRound::get() as u64 - 1;
	roll_to(block)
}

pub(crate) fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}

/// Assert input equal to the last event emitted
#[macro_export]
macro_rules! assert_last_event {
	($event:expr) => {
		match &$event {
			e => assert_eq!(*e, $crate::mock::last_event()),
		}
	};
}

pub(crate) fn events() -> Vec<pallet_parachain_staking::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(
			|e| if let RuntimeEvent::ParachainStaking(inner) = e { Some(inner) } else { None },
		)
		.collect::<Vec<_>>()
}
