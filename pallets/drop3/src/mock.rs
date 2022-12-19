// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate as pallet_drop3;
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32},
};
use frame_system as system;
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Percent,
};
use sp_std::vec;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;
pub type PoolId = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Drop3: pallet_drop3::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
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
	type SS58Prefix = ConstU16<31>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance; // the type that is relevant to us
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const SlashPercent: Percent = Percent::from_percent(20);
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl pallet_drop3::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PoolId = PoolId;
	type SetAdminOrigin = EnsureSignedBy<One, u64>;
	type Currency = Balances;
	type WeightInfo = ();
	type SlashPercent = SlashPercent;
	type MaximumNameLength = ConstU32<16>;
}

// propose a default reward pool with the given id
pub(crate) fn propose_default_reward_pool(
	id: <Test as pallet_drop3::Config>::PoolId,
	should_change_current_max: bool,
) {
	let default_reward_pool = pallet_drop3::RewardPool::<_, _, _, _, _> {
		id,
		name: vec![].try_into().unwrap(),
		owner: <Test as frame_system::Config>::AccountId::from(0u32),
		total: pallet_drop3::BalanceOf::<Test>::default(),
		remain: pallet_drop3::BalanceOf::<Test>::default(),
		create_at: <Test as frame_system::Config>::BlockNumber::default(),
		start_at: <Test as frame_system::Config>::BlockNumber::default(),
		end_at: <Test as frame_system::Config>::BlockNumber::default(),
		started: false,
		approved: false,
	};

	pallet_drop3::RewardPools::<Test>::insert(id, default_reward_pool);
	pallet_drop3::RewardPoolOwners::<Test>::insert(
		id,
		<Test as frame_system::Config>::AccountId::from(0u32),
	);
	if should_change_current_max {
		pallet_drop3::CurrentMaxPoolId::<Test>::put(id);
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		// set 1 as admin account
		let _ = Drop3::set_admin(RuntimeOrigin::signed(1), 1);
	});
	ext
}
