// Copyright 2020-2024 Trust Computing GmbH.
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

use crate::{self as pallet_investing_pool};
use frame_support::{
	assert_ok, derive_impl, ord_parameter_types, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64},
	PalletId,
};
use frame_system::EnsureRoot;
use pallet_investing_pool::PoolSetting;
use sp_core::{ConstU16, H256};
use sp_runtime::{
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type AccountId = u64;
type Block = frame_system::mocking::MockBlock<Test>;

type Balance = u64;
/// Pool id
pub type PoolId = u128;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		InvestingPool: pallet_investing_pool,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
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
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU64<1>;
	type AssetAccountDeposit = ConstU64<10>;
	type MetadataDepositBase = ConstU64<1>;
	type MetadataDepositPerByte = ConstU64<1>;
	type ApprovalDeposit = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	type Extra = ();
	type RemoveItemsLimit = ConstU32<5>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub const InvestingPoolId: PalletId = PalletId(*b"can/stpl");
	pub const HavlingMintId: PalletId = PalletId(*b"can/hlvm");
}

impl pallet_investing_pool::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type InvestingPoolCommitteeOrigin = EnsureRoot<AccountId>;
	type PoolId = PoolId;
	type Fungibles = Assets;
	type Fungible = Balances;
	type StableTokenBeneficiaryId = InvestingPoolId;
	type CANBeneficiaryId = HavlingMintId;
	type PoolStringLimit = ConstU32<100>;
}

pub const ENDOWED_BALANCE: u64 = 100_000_000;
pub const USER_A: AccountId = 0x2;
pub const USER_B: AccountId = 0x3;
pub const USER_C: AccountId = 0x4;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let native_token_pool: u64 = HavlingMintId::get().into_account_truncating();
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(native_token_pool, ENDOWED_BALANCE),
			(USER_A, ENDOWED_BALANCE),
			(USER_B, ENDOWED_BALANCE),
			(USER_C, ENDOWED_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		frame_system::Pallet::<Test>::set_block_number(1);

		// Regist AIUSD
		// asset_id = 1, admin = USER_A
		assert_ok!(Assets::create(RuntimeOrigin::signed(USER_A), 1u32, USER_A, 1));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(USER_A), 1u32, USER_A, ENDOWED_BALANCE));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(USER_A), 1u32, USER_B, ENDOWED_BALANCE));
		assert_ok!(Assets::mint(RuntimeOrigin::signed(USER_A), 1u32, USER_C, ENDOWED_BALANCE));
		// Setup stable investing pallet
		assert_ok!(InvestingPool::regist_aiusd(RuntimeOrigin::root(), 1u32));
		assert_eq!(InvestingPool::aiusd_asset_id(), Some(1u32));
		// Create stable investing pool
		let pool_setup: PoolSetting<u64, u64> = PoolSetting {
			start_time: 100u64,
			epoch: 10u128,
			epoch_range: 100u64,
			setup_time: 200u64,
			pool_cap: 50_000_000u64,
		};
		assert_ok!(InvestingPool::create_investing_pool(RuntimeOrigin::root(), 1u128, pool_setup));
	});
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