// Copyright 2020-2023 Litentry Technologies GmbH.
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

#![cfg(test)]

use crate as pallet_vc_management;
use frame_support::{
	assert_ok,
	pallet_prelude::EnsureOrigin,
	parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64, Everything},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};
use sp_std::marker::PhantomData;

pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;

type SystemAccountId = <Test as frame_system::Config>::AccountId;

pub struct EnsureEnclaveSigner<T>(PhantomData<T>);
impl<T> EnsureOrigin<T::RuntimeOrigin> for EnsureEnclaveSigner<T>
where
	T: frame_system::Config + pallet_teerex::Config + pallet_timestamp::Config<Moment = u64>,
	<T as frame_system::Config>::AccountId: From<[u8; 32]>,
	<T as frame_system::Config>::Hash: From<[u8; 32]>,
{
	type Success = T::AccountId;
	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		o.into().and_then(|o| match o {
			frame_system::RawOrigin::Signed(who)
				if pallet_teerex::Pallet::<T>::ensure_registered_enclave(&who) == Ok(()) =>
				Ok(who),
			r => Err(T::RuntimeOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		use test_utils::ias::{
			consts::{TEST8_MRENCLAVE, TEST8_SIGNER_PUB},
			TestEnclave,
		};
		let signer: <T as frame_system::Config>::AccountId =
			test_utils::get_signer(TEST8_SIGNER_PUB);
		if !pallet_teerex::EnclaveIndex::<T>::contains_key(signer.clone()) {
			assert_ok!(pallet_teerex::Pallet::<T>::add_enclave(
				&signer,
				&teerex_primitives::Enclave::test_enclave(signer.clone())
					.with_mr_enclave(TEST8_MRENCLAVE),
			));
		}
		Ok(frame_system::RawOrigin::Signed(signer).into())
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Teerex: pallet_teerex,
		Timestamp: pallet_timestamp,
		VCManagement: pallet_vc_management,
		VCMPExtrinsicWhitelist: pallet_group,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
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
	type SS58Prefix = ConstU16<31>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<10000>;
	type WeightInfo = ();
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

impl pallet_vc_management::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type TEECallOrigin = EnsureEnclaveSigner<Self>;
	type SetAdminOrigin = EnsureRoot<Self::AccountId>;
	type ExtrinsicWhitelistOrigin = VCMPExtrinsicWhitelist;
}

parameter_types! {
	pub const MomentsPerDay: u64 = 86_400_000; // [ms/d]
}

impl pallet_teerex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MomentsPerDay = MomentsPerDay;
	type WeightInfo = ();
	type SetAdminOrigin = EnsureRoot<Self::AccountId>;
}

impl pallet_group::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type GroupManagerOrigin = frame_system::EnsureRoot<Self::AccountId>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let alice: SystemAccountId = test_utils::get_signer(&[1u8; 32]);

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		let _ = VCManagement::set_admin(RuntimeOrigin::root(), alice);

		use test_utils::ias::consts::{TEST8_CERT, TEST8_SIGNER_PUB, TEST8_TIMESTAMP, URL};
		Timestamp::set_timestamp(TEST8_TIMESTAMP);
		let teerex_signer: SystemAccountId = test_utils::get_signer(TEST8_SIGNER_PUB);
		if !pallet_teerex::EnclaveIndex::<Test>::contains_key(teerex_signer.clone()) {
			assert_ok!(Teerex::register_enclave(
				RuntimeOrigin::signed(teerex_signer),
				TEST8_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
			));
		}
	});
	ext
}
