use crate::{self as pallet_halving_mint, OnTokenMinted};
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU64, Currency, ExistenceRequirement, Hooks},
	weights::Weight,
	PalletId,
};
use sp_core::ConstU32;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		HalvingMint: pallet_halving_mint,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 2;
	pub const MaxLocks: u32 = 10;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type MaxHolds = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

parameter_types! {
	pub const BeneficiaryId: PalletId = PalletId(*b"can/hlvm");
}

impl pallet_halving_mint::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = frame_system::EnsureRoot<u64>;
	type TotalIssuance = ConstU64<1000>;
	type HalvingInterval = ConstU32<10>;
	type BeneficiaryId = BeneficiaryId;
	type OnTokenMinted = TransferOnTokenMinted<Test>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	ext.execute_with(|| {
		System::set_block_number(1);
		let _ = Balances::deposit_creating(&HalvingMint::beneficiary_account(), 10);
	});
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		HalvingMint::on_initialize(System::block_number());
	}
}

pub struct TransferOnTokenMinted<T>(sp_std::marker::PhantomData<T>);

impl<T> OnTokenMinted<T::AccountId, T::Balance> for TransferOnTokenMinted<T>
where
	T: frame_system::Config<AccountId = u64> + pallet_balances::Config<Balance = u64>,
{
	fn token_minted(beneficiary: T::AccountId, amount: T::Balance) -> Weight {
		let _ = Balances::transfer(&beneficiary, &1, amount, ExistenceRequirement::AllowDeath);
		Weight::zero()
	}
}
