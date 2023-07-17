#![cfg(test)]
use crate as pallet_evm_address;
use codec::Encode;
use fp_evm::GenesisAccount;
use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, FindAuthor, GenesisBuild},
	weights::Weight,
	ConsensusEngineId,
};
use frame_system::RawOrigin;
use hex_literal::hex;
use sp_core::{H160, U256};
use sp_runtime::{
	generic,
	traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};
use sp_std::vec;
use std::{collections::BTreeMap, str::FromStr};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		// Frontier
		EVM: pallet_evm::{Pallet, Call, Config, Storage, Event<T>},
		Ethereum: pallet_ethereum::{Pallet, Call, Storage, Event, Origin},
		EVMAddress: pallet_evm_address::{Pallet, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = u64;
	/// The index type for blocks.
	type BlockNumber = u64;
	/// The type for hashing blocks and tries.
	type Hash = sp_core::H256;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<u64, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = ();
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<u128>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = ();
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = ();
	/// The maximum length of a block (in bytes).
	type BlockLength = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = ();
	/// The action to take on a Runtime Upgrade
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
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

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

use pallet_evm::{EnsureAddressOrigin, FeeCalculator};
pub struct EnsureAddressEqualAndStore<T>(sp_std::marker::PhantomData<T>);
impl<T, OuterOrigin> EnsureAddressOrigin<OuterOrigin> for EnsureAddressEqualAndStore<T>
where
	T: pallet_evm_address::Config<EVMId = H160>,
	OuterOrigin: Into<Result<RawOrigin<T::AccountId>, OuterOrigin>> + From<RawOrigin<T::AccountId>>,
{
	type Success = ();

	fn try_address_origin(address: &H160, origin: OuterOrigin) -> Result<(), OuterOrigin> {
		origin.into().and_then(|o| match o {
			RawOrigin::Root => Ok(()),
			RawOrigin::Signed(account_id) => {
				// AddressMapping revert logic check here
				if H160::from_slice(&account_id.encode()[0..20]) == *address {
					match pallet_evm_address::Pallet::<T>::add_address_mapping(
						*address,
						account_id.clone(),
					) {
						Ok(_) => Ok(()),
						Err(_) => Err(OuterOrigin::from(RawOrigin::Signed(account_id))),
					}
				} else {
					Err(OuterOrigin::from(RawOrigin::Signed(account_id)))
				}
			},
			r => Err(OuterOrigin::from(r)),
		})
	}
}

parameter_types! {
	pub WeightPerGas: Weight = Weight::from_ref_time(20_000);
	// It will be the best if we can implement this in a more professional way
	pub ChainId: u64 = 2106u64;
	pub BlockGasLimit: U256 = U256::max_value();
}
use pallet_evm::AddressMapping;
pub struct EVMAddressMapping<T>(sp_std::marker::PhantomData<T>);
impl<T> AddressMapping<T::AccountId> for EVMAddressMapping<T>
where
	T: pallet_evm_address::Config<EVMId = H160> + frame_system::Config<AccountId = AccountId>,
{
	fn into_account_id(address: H160) -> T::AccountId {
		match pallet_evm_address::Pallet::<T>::get_address_mapped(address) {
			Some(r) => r,
			None => TruncatedAddressMapping::into_account_id(address),
		}
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

pub struct FindAuthorTruncated;
impl FindAuthor<H160> for FindAuthorTruncated {
	fn find_author<'a, I>(_digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		Some(H160::from_str("1234500000000000000000000000000000000000").unwrap())
	}
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		// Return some meaningful gas price and weight
		(1_000_000_000u128.into(), Weight::from_ref_time(7u64))
	}
}

impl pallet_evm::Config for Test {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressEqualAndStore<Test>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	// From evm address to parachain address
	type AddressMapping = EVMAddressMapping<Self>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	// Minimal effort, no precompile for now
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type ChainId = ChainId;
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated;
}

use pallet_ethereum::PostLogContent;
parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
}

impl pallet_evm_address::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type EVMId = H160;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut accounts = BTreeMap::new();
	pub const ALICE: AccountId = sp_runtime::AccountId32::new(hex![
		"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
	]);
	let alice_evm = H160::from_slice(&ALICE.encode()[0..20]);
	accounts.insert(
		alice_evm,
		GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(10000000000000000u128),
			storage: Default::default(),
			code: vec![],
		},
	);
	pallet_balances::GenesisConfig::<Test> {
		// Create the block author account with some balance.
		balances: vec![(ALICE, 80000000000000000u128)],
	}
	.assimilate_storage(&mut t)
	.expect("Pallet balances storage can be assimilated");

	GenesisBuild::<Test>::assimilate_storage(&pallet_evm::GenesisConfig { accounts }, &mut t)
		.expect("Pallet evm storage can be assimilated");

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}
