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

use crate::{self as pallet_omni_account, EnsureOmniAccount};
use core_primitives::Identity;
use frame_support::{
	assert_ok,
	pallet_prelude::EnsureOrigin,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use frame_system::EnsureRoot;
pub use pallet_teebag::test_util::get_signer;
use pallet_teebag::test_util::{TEST8_CERT, TEST8_SIGNER_PUB, TEST8_TIMESTAMP, URL};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage,
};
use sp_std::marker::PhantomData;

pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u64;
pub type SystemAccountId = <TestRuntime as frame_system::Config>::AccountId;

pub struct EnsureEnclaveSigner<T>(PhantomData<T>);
impl<T> EnsureOrigin<T::RuntimeOrigin> for EnsureEnclaveSigner<T>
where
	T: frame_system::Config + pallet_teebag::Config + pallet_timestamp::Config<Moment = u64>,
	<T as frame_system::Config>::AccountId: From<[u8; 32]>,
	<T as frame_system::Config>::Hash: From<[u8; 32]>,
{
	type Success = T::AccountId;
	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		o.into().and_then(|o| match o {
			frame_system::RawOrigin::Signed(who)
				if pallet_teebag::EnclaveRegistry::<T>::contains_key(&who) =>
			{
				Ok(who)
			},
			r => Err(T::RuntimeOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		use pallet_teebag::test_util::{get_signer, TEST8_MRENCLAVE, TEST8_SIGNER_PUB};
		let signer: <T as frame_system::Config>::AccountId = get_signer(TEST8_SIGNER_PUB);
		if !pallet_teebag::EnclaveRegistry::<T>::contains_key(signer.clone()) {
			assert_ok!(pallet_teebag::Pallet::<T>::add_enclave(
				&signer,
				&pallet_teebag::Enclave::default().with_mrenclave(TEST8_MRENCLAVE),
			));
		}
		Ok(frame_system::RawOrigin::Signed(signer).into())
	}
}

pub fn alice() -> AccountId {
	AccountKeyring::Alice.to_account_id()
}

pub fn bob() -> AccountId {
	AccountKeyring::Bob.to_account_id()
}

pub fn charlie() -> AccountId {
	AccountKeyring::Charlie.to_account_id()
}

frame_support::construct_runtime!(
	pub enum TestRuntime
	{
		System: frame_system,
		Balances: pallet_balances,
		Teebag: pallet_teebag,
		Timestamp: pallet_timestamp,
		Utility: pallet_utility,
		OmniAccount: pallet_omni_account,
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Block = frame_system::mocking::MockBlock<TestRuntime>;
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
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

impl pallet_balances::Config for TestRuntime {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
}

impl pallet_timestamp::Config for TestRuntime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<10000>;
	type WeightInfo = ();
}

impl pallet_utility::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

impl pallet_teebag::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type MomentsPerDay = ConstU64<86_400_000>; // [ms/d]
	type SetAdminOrigin = EnsureRoot<Self::AccountId>;
	type MaxEnclaveIdentifier = ConstU32<3>;
	type MaxAuthorizedEnclave = ConstU32<3>;
	type WeightInfo = ();
}

pub struct IdentityToAccountIdConverter;

impl pallet_omni_account::AccountIdConverter<TestRuntime> for IdentityToAccountIdConverter {
	fn convert(identity: &Identity) -> Option<AccountId> {
		identity.to_account_id()
	}
}

impl pallet_omni_account::Config for TestRuntime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type TEECallOrigin = EnsureEnclaveSigner<Self>;
	type MaxIDGraphLength = ConstU32<3>;
	type AccountIdConverter = IdentityToAccountIdConverter;
	type OmniAccountOrigin = EnsureOmniAccount<Self::AccountId>;
}

pub fn get_tee_signer() -> SystemAccountId {
	get_signer(TEST8_SIGNER_PUB)
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<TestRuntime> { balances: vec![(alice(), 10)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| {
		System::set_block_number(1);
		let signer = get_tee_signer();
		assert_ok!(Teebag::set_admin(RuntimeOrigin::root(), signer.clone()));
		assert_ok!(Teebag::set_mode(
			RuntimeOrigin::signed(signer.clone()),
			pallet_teebag::OperationalMode::Development
		));

		Timestamp::set_timestamp(TEST8_TIMESTAMP);
		if !pallet_teebag::EnclaveRegistry::<TestRuntime>::contains_key(signer.clone()) {
			assert_ok!(Teebag::register_enclave(
				RuntimeOrigin::signed(signer),
				pallet_teebag::WorkerType::Identity,
				pallet_teebag::WorkerMode::Sidechain,
				TEST8_CERT.to_vec(),
				URL.to_vec(),
				None,
				None,
				pallet_teebag::AttestationType::Ias,
			));
		}
	});
	ext
}