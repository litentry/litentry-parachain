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

#![allow(non_upper_case_globals)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	parameter_types, sp_runtime,
	traits::{Currency, OnUnbalanced},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_PER_SECOND},
		DispatchClass, Weight,
	},
};
use frame_system::limits;
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use primitives::BlockNumber;
use sp_runtime::{FixedPointNumber, Perbill, Perquintill};

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;

// Common constants used in all runtimes.
parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;

	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);

	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);

	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);

	/// Maximum length of block. Up to 5MB.
	pub BlockLength: limits::BlockLength = limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

	/// Block weights base values and limits.
	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	// The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have an extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
}

/// Parameterized slow adjusting fee updated based on
/// https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> =
	TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
	<R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
		}
	}
}

/// This macro expects the passed runtime constants to contain a `currency` module.
#[macro_export]
macro_rules! impl_runtime_transaction_payment_fees {
	($runtime:ident) => {
		use frame_support::traits::{Currency, Imbalance, OnUnbalanced};
		use runtime_common::ToAuthor;

		// Do i need to extract these constants to the common module?
		use $runtime::currency::{AUTHOR_PROPORTION, BURNED_PROPORTION, TREASURY_PROPORTION};

		// important !! The struct is used externally
		pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);

		impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
		where
			R: pallet_balances::Config + pallet_treasury::Config + pallet_authorship::Config,
			pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
			<R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
		{
			fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
				if let Some(fees) = fees_then_tips.next() {
					// for fees, (1) to treasury, (2) to author and (3) burned
					let (unburned, _) =
						fees.ration(TREASURY_PROPORTION + AUTHOR_PROPORTION, BURNED_PROPORTION);
					let mut split = unburned.ration(TREASURY_PROPORTION, AUTHOR_PROPORTION);

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
	};
}

use frame_support::traits::EnsureOneOf;
use frame_system::EnsureRoot;

pub use primitives::AccountId;

/// See https://github.com/paritytech/polkadot/blob/7096430edd116b1dc6d8337ab35b149e213cbfe9/runtime/common/src/lib.rs#L218
///
/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `fast-runtime` feature is selected).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
///     // Note that the env variable version parameter cannot be const.
///     pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "KSM_LAUNCH_PERIOD");
///     pub const VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
/// }
/// ```
#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}

/// Instance definition for council and technical committee
pub type CouncilInstance = pallet_collective::Instance1;
pub type TechnicalCommitteeInstance = pallet_collective::Instance2;
pub type CouncilMembershipInstance = pallet_membership::Instance1;
pub type TechnicalCommitteeMembershipInstance = pallet_membership::Instance2;

/// Type definition for various proportions of council and technical committee
/// Council
pub type EnsureRootOrAllCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 2>,
>;

pub type EnsureRootOrTwoThirdsCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 2, 3>,
>;

/// Technical Committee
pub type EnsureRootOrAllTechnicalCommittee = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 1>,
>;

pub type EnsureRootOrHalfTechnicalCommittee = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 2>,
>;

pub type EnsureRootOrTwoThirdsTechnicalCommittee = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 2, 3>,
>;
