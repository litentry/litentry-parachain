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
//
//! # Curator Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The Curator pallet handles the administration of general curator and proposed staking pool.
//!
//! 
#![cfg_attr(not(feature = "std"), no_std)]
use bitflags::bitflags;
use codec::{Decode, Encode};
use frame_support::{
	ensure,
	error::BadOrigin,
	traits::{
		defensive_prelude::*,
		schedule::{v3::Named as ScheduleNamed, DispatchTime},
		Bounded, Currency, EnsureOrigin, Get, Hash as PreimageHash, LockIdentifier,
		LockableCurrency, OnUnbalanced, QueryPreimage, ReservableCurrency, StorePreimage,
		WithdrawReasons,
	},
	weights::Weight,
};
use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
use orml_utilities::OrderedSet;
use pallet_collab_ai_common::*;
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
pub type CallOf<T> = <T as frame_system::Config>::RuntimeCall;
pub type BoundedCallOf<T> = Bounded<CallOf<T>>;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

bitflags! {
	/// Flags used to record the status of pool proposal
	pub struct ProposalStatusFlags: u32 {
		/// Whether the pool proposal passing the committee voting.
		///
		/// # Note
		///
		/// A valid pool must passing committee's audit procedure regarding legal files and other pool parameters.
		const COMMITTEE_VOTE_PASSED = 0b0000_0001;
				/// Whether the minimum staked amount proposed by curator is satisfied.
		///
		/// # Note
		///
		/// Once a pool is satisfied this requirement, all staked amount can no longer be withdrawed
		/// unless the pool is later denied passing by voting or until the end of pool maturity.
		/// 
		/// Otherwise, the pool will be refunded.
		const MINIMUM_STAKE_PASSED = 0b0000_0010;
		/// Whether the pool proposal passing the global democracy voting.
		///
		/// # Note
		///
		/// A valid pool must passing committee's audit procedure regarding legal files and other pool parameters.
		const DEMOCRACY_VOTE_PASSED = 0b0000_0100;
		/// Whether the pool guardian has been selected
		///
		/// # Note
		///
		/// A valid pool must have guardian or a default one will be used (committee)
		const GUARDIAN_SELECTED = 0b0000_1000;
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalStatus<BlockNumber, Balance> {
	pub pool_status_flags: ProposalStatusFlags,
	pub proposal_expire_time: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalPreStake<AccountId, Balance> {
	pub amount: Balance,
	pub owner: AccountId,
}

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalPreStaking<AccountId, Balance, S: Get<u32>> {
	pub total_pre_staked_amount: Balance,
	pub pre_staking: OrderedSet<PoolProposalPreStake<AccountId, Balance>, S>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + Sized {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Scheduler.
		type Scheduler: ScheduleNamed<BlockNumberFor<Self>, CallOf<Self>, Self::PalletsOrigin>;

		/// Currency type for this pallet.
		type Currency: ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		/// The minimum amount to be used as a deposit for creating a pool curator
		#[pallet::constant]
		type MinimumPoolDeposit: Get<BalanceOf<Self>>;

		/// Origin who can make a pool proposal 
		type ProposalOrigin: EnsureOrigin<Self::RuntimeOrigin>;

	}

	/// The next free Pool Proposal index, aka the number of pool proposed so far.
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_count)]
	pub type PoolProposalCount<T> = StorageValue<_, PoolProposalIndex, ValueQuery>;

	/// Those who have a reserve for his pool proposal.
	///
	/// TWOX-NOTE: Safe, as increasing integer keys are safe.
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_deposit_of)]
	pub type PoolProposalDepositOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<(PoolProposalIndex, BalanceOf<T>), T::MaxDeposits>,
	>;

	// Metadata of staking pools
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_status)]
	pub type PoolProposalStatus<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PoolProposalIndex,
		(InfoHash),
		OptionQuery,
	>;

	// Metadata of staking pools
	#[pallet::storage]
	#[pallet::getter(fn staking_pool_metadata)]
	pub type StakingPoolMetadata<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PoolProposalIndex,
		PoolMetadata<BoundedVec<u8, T::PoolStringLimit>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A motion has been proposed by a public account.
		Proposed { proposal_index: PropIndex, deposit: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Weight: see `begin_block`
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			Self::begin_block(n)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Curator propose a staking pool
		/// 
		/// max_pool_size: At most this amount of raised money curator/staking pool willing to take
		/// min_pool_size: At least this amount of raised money require for curator willing to fulfill contract 
		/// proposal_end_time: All ProposalStatusFlags must be satisfied before this date, this is also the approximate 
		/// 				   date when pool begins.
		/// pool_last_time: How long does the staking pool last if passed
		/// estimated_epoch_reward: This number is only for displaying purpose without any techinical meaning
		/// pool_info_hash: Hash of pool info for including pool details
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn propose_staking_pool(
			origin: OriginFor<T>,
			max_pool_size: BalanceOf<T>,
			min_pool_size: BalanceOf<T>,
			proposal_end_time: BlockNumberFor<T>,
			pool_last_time: BlockNumberFor<T>,
			estimated_epoch_reward: BalanceOf<T>,
			pool_info_hash: InfoHash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;


		}
	}
}

