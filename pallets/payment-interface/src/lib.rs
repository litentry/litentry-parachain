#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
// use codec::alloc::string::String;
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// linear ratio of transaction fee distribution
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, sp_runtime::RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RatioOf {
	treasury: u32,
	author: u32,
	burned: u32,
}

impl Default for RatioOf {
	fn default() -> Self {
		RatioOf { treasury: 0, author: 0, burned: 1 }
	}
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_ratio)]
	pub type Ratio<T: Config> = StorageValue<_, RatioOf, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn fix_block_reward)]
	pub type FixBlockReward<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		ratio: RatioOf,
		fix_block_reward: u32,
		_phantom: sp_std::marker::PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				ratio: Default::default(),
				fix_block_reward: Default::default(),
				_phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Ratio<T>>::put(self.ratio);
			<FixBlockReward<T>>::put(self.fix_block_reward);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetRatio(RatioOf),
		SetFixBlockReward(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		RatioOverflow,
		BlockRewardTooLow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10)]
		pub fn set_ratio(origin: OriginFor<T>, ratio: RatioOf) -> DispatchResult {
			ensure_root(origin)?;
			if ratio.treasury + ratio.author + ratio.burned > 0 {
				<Ratio<T>>::put(ratio);
				Self::deposit_event(Event::<T>::SetRatio(ratio));
				Ok(())
			} else {
				Err(Error::<T>::RatioOverflow.into())
			}
		}

		#[pallet::weight(10)]
		pub fn set_fix_block_reward(
			origin: OriginFor<T>,
			// block_reward: <T::Currency as Currency<T::AccountId>>::Balance,
			block_reward: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Reward meet minimum_balance requirement of account existence
			if 0 < block_reward &&
				BalanceOf::<T>::from(block_reward) <
					<T::Currency as Currency<T::AccountId>>::minimum_balance()
			{
				return Err(Error::<T>::BlockRewardTooLow.into())
			}

			<FixBlockReward<T>>::put(block_reward);
			Self::deposit_event(Event::<T>::SetFixBlockReward(block_reward));
			Ok(())
		}
	}
}

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
	<R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		let numeric_amount = amount.peek();
		let author = <pallet_authorship::Pallet<R>>::author();
		<pallet_balances::Pallet<R>>::resolve_creating(
			&<pallet_authorship::Pallet<R>>::author(),
			amount,
		);
		<frame_system::Pallet<R>>::deposit_event(pallet_balances::Event::Deposit(
			author,
			numeric_amount,
		));
	}
}

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: Config + pallet_balances::Config + pallet_treasury::Config + pallet_authorship::Config,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
	<R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, (1) to treasury, (2) to author and (3) burned
			let ratio = Pallet::<R>::get_ratio();
			let (unburned, _) = fees.ration(ratio.treasury + ratio.author, ratio.burned);
			let mut split = unburned.ration(ratio.treasury, ratio.author);

			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut split.1);
			}
			use pallet_treasury::Pallet as Treasury;
			<Treasury<R> as OnUnbalanced<_>>::on_unbalanced(split.0);
			<ToAuthor<R> as OnUnbalanced<_>>::on_unbalanced(split.1);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_payment_interface;

	use std::cell::RefCell;

	// use codec::Encode;
	use smallvec::smallvec;

	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup, SignedExtension},
		Perbill,
	};

	use frame_support::{
		assert_ok, parameter_types,
		traits::{FindAuthor, Get},
		weights::{
			DispatchClass, DispatchInfo, PostDispatchInfo, Weight, WeightToFeeCoefficient,
			WeightToFeeCoefficients, WeightToFeePolynomial,
		},
		PalletId,
	};

	use pallet_balances::Call as BalancesCall;

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
	type Block = frame_system::mocking::MockBlock<Runtime>;

	frame_support::construct_runtime!(
		pub enum Runtime where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Authorship: pallet_authorship::{Pallet, Call, Storage},
			Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
			PaymentInterface: pallet_payment_interface::{Pallet, Call, Storage, Event<T>},
		}
	);

	const CALL: &<Runtime as frame_system::Config>::Call =
		&Call::Balances(BalancesCall::transfer { dest: 2, value: 69 });

	thread_local! {
		static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
	}

	pub struct BlockWeights;
	impl Get<frame_system::limits::BlockWeights> for BlockWeights {
		fn get() -> frame_system::limits::BlockWeights {
			frame_system::limits::BlockWeights::builder()
				.base_block(0)
				.for_class(DispatchClass::all(), |weights| {
					weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow()).into();
				})
				.for_class(DispatchClass::non_mandatory(), |weights| {
					weights.max_total = 1024.into();
				})
				.build_or_panic()
		}
	}

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
	}

	impl frame_system::Config for Runtime {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = BlockWeights;
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Call = Call;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}

	parameter_types! {
		pub const UncleGenerations: u64 = 5;
	}

	impl pallet_authorship::Config for Runtime {
		type FindAuthor = AuthorGiven;
		type UncleGenerations = UncleGenerations;
		type FilterUncle = ();
		type EventHandler = ();
	}

	pub type ConsensusEngineId = [u8; 4];
	const TEST_ID: ConsensusEngineId = [1, 2, 3, 4];

	pub struct AuthorGiven;
	impl FindAuthor<u64> for AuthorGiven {
		fn find_author<'a, I>(digests: I) -> Option<u64>
		where
			I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
		{
			for (id, data) in digests {
				if id == TEST_ID {
					return u64::decode(&mut &data[..]).ok()
				}
			}

			None
		}
	}

	impl pallet_balances::Config for Runtime {
		type Balance = u64;
		type Event = Event;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type WeightInfo = ();
	}

	parameter_types! {
		pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	}

	impl pallet_treasury::Config for Runtime {
		type PalletId = TreasuryPalletId;
		type Currency = Balances;
		type ApproveOrigin = frame_system::EnsureSigned<u64>;
		type RejectOrigin = frame_system::EnsureSigned<u64>;
		type Event = Event;
		type OnSlash = ();
		type ProposalBond = ();
		type ProposalBondMinimum = ();
		type SpendPeriod = ();
		type Burn = ();
		type BurnDestination = ();
		type SpendFunds = ();
		type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
		type MaxApprovals = ();
	}

	impl WeightToFeePolynomial for WeightToFee {
		type Balance = u64;

		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				coeff_frac: Perbill::zero(),
				coeff_integer: WEIGHT_TO_FEE.with(|v| *v.borrow()),
				negative: false,
			}]
		}
	}

	impl pallet_payment_interface::Config for Runtime {
		type Event = Event;
		type Currency = Balances;
	}

	parameter_types! {
		pub static TransactionByteFee: u64 = 1;
		pub static WeightToFee: u64 = 1;
		pub static OperationalFeeMultiplier: u8 = 5;
	}

	impl pallet_transaction_payment::Config for Runtime {
		type OnChargeTransaction =
			pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
		type TransactionByteFee = TransactionByteFee;
		type OperationalFeeMultiplier = OperationalFeeMultiplier;
		type WeightToFee = WeightToFee;
		type FeeMultiplierUpdate = ();
	}

	pub struct ExtBuilder {
		balance_factor: u64,
		base_weight: u64,
		byte_fee: u64,
		weight_to_fee: u64,
	}

	impl Default for ExtBuilder {
		fn default() -> Self {
			Self { balance_factor: 1, base_weight: 0, byte_fee: 1, weight_to_fee: 1 }
		}
	}

	impl ExtBuilder {
		pub fn base_weight(mut self, base_weight: u64) -> Self {
			self.base_weight = base_weight;
			self
		}
		pub fn balance_factor(mut self, factor: u64) -> Self {
			self.balance_factor = factor;
			self
		}
		fn set_constants(&self) {
			EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow_mut() = self.base_weight);
			TRANSACTION_BYTE_FEE.with(|v| *v.borrow_mut() = self.byte_fee);
			WEIGHT_TO_FEE.with(|v| *v.borrow_mut() = self.weight_to_fee);
		}
		pub fn build(self) -> sp_io::TestExternalities {
			self.set_constants();
			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
			pallet_balances::GenesisConfig::<Runtime> {
				balances: if self.balance_factor > 0 {
					vec![(1, 100 * self.balance_factor)]
				} else {
					vec![]
				},
			}
			.assimilate_storage(&mut t)
			.unwrap();
			t.into()
		}
	}

	/// create a transaction info struct from weight. Handy to avoid building the whole struct.
	pub fn info_from_weight(w: Weight) -> DispatchInfo {
		// pays_fee: Pays::Yes -- class: DispatchClass::Normal
		DispatchInfo { weight: w, ..Default::default() }
	}

	fn post_info_from_weight(w: Weight) -> PostDispatchInfo {
		PostDispatchInfo { actual_weight: Some(w), pays_fee: Default::default() }
	}

	#[test]
	fn signed_extension_transaction_payment_work() {
		ExtBuilder::default()
			.balance_factor(10)
			.base_weight(5)
			.build()
			.execute_with(|| {
				assert_ok!(PaymentInterface::set_ratio(
					Origin::root(),
					RatioOf { treasury: 50, author: 30, burned: 20 }
				));
				let mut sender_balance = Balances::free_balance(1);
				let mut treasury_balance = Balances::free_balance(Treasury::account_id());
				let len = 10;
				let pre = pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0)
					.pre_dispatch(&1, CALL, &info_from_weight(85), len)
					.unwrap();
				// 1: initial 1000 balance, withdraw 5 base fee, 85 weight fee, 10 len fee
				// Treasury unchanged
				assert_eq!(sender_balance - Balances::free_balance(1), 5 + 85 + 10);
				assert_eq!(Balances::free_balance(Treasury::account_id()) - treasury_balance, 0);
				sender_balance = Balances::free_balance(1);
				treasury_balance = Balances::free_balance(Treasury::account_id());
				assert_ok!(
					pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::post_dispatch(
						pre,
						&info_from_weight(85),
						// so acutal weight is 35 + 5 + 10 = 50
						&post_info_from_weight(35),
						len,
						&Ok(())
					)
				);
				// 1: balance refund 50
				assert_eq!(Balances::free_balance(1) - sender_balance, 50);
				// treasury pallet account get distribution 50 out of (50+30+20) proprtion of 50
				// actual weight
				assert_eq!(
					Balances::free_balance(Treasury::account_id()) - treasury_balance,
					50 * 50 / (50 + 30 + 20)
				);
				// TODO: author account get distribution

				// change the ratio setting and repeat the pre/post dispatch above
				assert_ok!(PaymentInterface::set_ratio(
					Origin::root(),
					RatioOf { treasury: 20, author: 30, burned: 50 }
				));
				let mut sender_balance = Balances::free_balance(1);
				let mut treasury_balance = Balances::free_balance(Treasury::account_id());
				let len = 10;
				let pre = pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0)
					.pre_dispatch(&1, CALL, &info_from_weight(85), len)
					.unwrap();
				// 1: withdraw 5 base fee, 85 weight fee, 10 len fee
				// Treasury unchanged
				assert_eq!(sender_balance - Balances::free_balance(1), 5 + 85 + 10);
				assert_eq!(Balances::free_balance(Treasury::account_id()) - treasury_balance, 0);
				sender_balance = Balances::free_balance(1);
				treasury_balance = Balances::free_balance(Treasury::account_id());
				assert_ok!(
					pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::post_dispatch(
						pre,
						&info_from_weight(85),
						// so acutal weight is 35 + 5 + 10 = 50
						&post_info_from_weight(35),
						len,
						&Ok(())
					)
				);
				// 1: balance refund 50
				assert_eq!(Balances::free_balance(1) - sender_balance, 50);
				// treasury pallet account get distribution 20 out of (50+30+20) proprtion of 50
				// actual weight
				assert_eq!(
					Balances::free_balance(Treasury::account_id()) - treasury_balance,
					20 * 50 / (50 + 30 + 20)
				);
				// TODO: author account get distribution
			});
	}
}
