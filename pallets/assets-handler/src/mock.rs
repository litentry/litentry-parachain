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

use crate::{self as pallet_assets_handler};
use frame_support::{
	assert_ok, derive_impl, ord_parameter_types, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, ConstU64, SortedMembers},
	PalletId,
};
use hex_literal::hex;
use pallet_assets_handler::AssetInfo;
use sp_core::{ConstU16, H256};
use sp_runtime::{
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
	BuildStorage,
};
pub const TEST_THRESHOLD: u32 = 2;
type Block = frame_system::mocking::MockBlock<Test>;

type Balance = u64;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Bridge: pallet_bridge,
		Assets: pallet_assets,
		AssetsHandler: pallet_assets_handler,
		BridgeTransfer: pallet_bridge_transfer,
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
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
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

parameter_types! {
	pub const TestChainId: u8 = 5;
	pub const ProposalLifetime: u64 = 50;
	pub const TreasuryAccount:u64 = 0x8;
}

impl pallet_bridge::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BridgeCommitteeOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Proposal = RuntimeCall;
	type BridgeChainId = TestChainId;
	type Balance = Balance;
	type ProposalLifetime = ProposalLifetime;
	type WeightInfo = ();
}

parameter_types! {
	// bridge::derive_resource_id(1, &bridge::hashing::blake2_128(b"LIT"));
	pub const NativeTokenResourceId: [u8; 32] = hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
	// transfernativemembers
	static MembersProviderTestvalue:Vec<u64> = vec![RELAYER_A, RELAYER_B, RELAYER_C];
}

pub struct MembersProvider;
impl SortedMembers<u64> for MembersProvider {
	fn sorted_members() -> Vec<u64> {
		MembersProviderTestvalue::get()
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	fn contains(who: &u64) -> bool {
		Self::sorted_members().contains(who)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(_: &u64) {
		unimplemented!()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn contains(_who: &u64) -> bool {
		true
	}
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

impl pallet_assets_handler::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TreasuryAccount = TreasuryAccount;
}

impl pallet_bridge_transfer::Config for Test {
	type BridgeOrigin = pallet_bridge::EnsureBridge<Test>;
	type TransferNativeMembers = MembersProvider;
	type BridgeHandler = AssetsHandler;
	type WeightInfo = ();
}

pub const RELAYER_A: u64 = 0x2;
pub const RELAYER_B: u64 = 0x3;
pub const RELAYER_C: u64 = 0x4;
pub const ENDOWED_BALANCE: u64 = 100_000_000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let bridge_id = PalletId(*b"litry/bg").into_account_truncating();
	let treasury_account: u64 = 0x8;
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(bridge_id, ENDOWED_BALANCE),
			(RELAYER_A, ENDOWED_BALANCE),
			(treasury_account, ENDOWED_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		frame_system::Pallet::<Test>::set_block_number(1);
		let resource_id = NativeTokenResourceId::get();
		let native_token_asset_info: AssetInfo<
			<Test as pallet_assets::Config>::AssetId,
			<Test as pallet_assets::Config>::Balance,
		> = AssetInfo { fee: 0u64, asset: None };
		// Setup asset handler
		assert_ok!(AssetsHandler::set_resource(
			RuntimeOrigin::root(),
			resource_id,
			native_token_asset_info
		));
	});
	ext
}

pub fn new_test_ext_initialized(
	src_id: pallet_bridge::BridgeChainId,
	r_id: pallet_bridge::ResourceId,
	asset: AssetInfo<
		<Test as pallet_assets::Config>::AssetId,
		<Test as pallet_assets::Config>::Balance,
	>,
) -> sp_io::TestExternalities {
	let mut t = new_test_ext();
	t.execute_with(|| {
		// Set and check threshold
		assert_ok!(Bridge::set_threshold(RuntimeOrigin::root(), TEST_THRESHOLD));
		assert_eq!(Bridge::relayer_threshold(), TEST_THRESHOLD);
		// Add relayers
		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_A));
		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_B));
		assert_ok!(Bridge::add_relayer(RuntimeOrigin::root(), RELAYER_C));
		// Whitelist chain
		assert_ok!(Bridge::whitelist_chain(RuntimeOrigin::root(), src_id));

		// Setup asset handler
		assert_ok!(AssetsHandler::set_resource(RuntimeOrigin::root(), r_id, asset));
	});
	t
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
