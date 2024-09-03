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
use sp_core::H256;
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

pub type InfoHash = H256;
pub type CuratorIndex = u128;
pub type PoolProposalIndex = u128;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct PoolMetadata<BoundedString> {
	/// The user friendly name of this staking pool. Limited in length by `PoolStringLimit`.
	pub name: BoundedString,
	/// The short description for this staking pool. Limited in length by `PoolStringLimit`.
	pub description: BoundedString,
}

bitflags! {
	/// Flags used to record the status of pool proposal
	pub struct ProposalStatusFlags: u32 {
		/// Whether the minimum staked amount proposed by curator is satisfied.
		///
		/// # Note
		///
		/// Once a pool is satisfied this requirement, all staked amount can no longer be withdrawed
		/// unless the pool is later denied passing by voting or until the end of pool maturity.
		/// 
		/// Otherwise, the pool will be refunded.
		const MINIMUM_STAKE_PASSED = 0b0000_0001;
		/// Whether the pool proposal passing the committee voting.
		///
		/// # Note
		///
		/// A valid pool must passing committee's audit procedure regarding legal files and other pool parameters.
		const COMMITTEE_VOTE_PASSED = 0b0000_0010;
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

		/// The Legal file storage
		type FileStorage: QueryPreimage + StorePreimage;

		/// Currency type for this pallet.
		type Currency: ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		/// The minimum amount to be used as a deposit for a curator
		#[pallet::constant]
		type MinimumCuratorDeposit: Get<BalanceOf<Self>>;

		/// Origin from curator legal file verified by
		type CuratorJudgeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

	}

	/// The number of (public) curator that have been made so far.
	#[pallet::storage]
	#[pallet::getter(fn public_curator_count)]
	pub type PublicCuratorCount<T> = StorageValue<_, CuratorIndex, ValueQuery>;

	/// The public curator to index
	#[pallet::storage]
	#[pallet::getter(fn public_curator_to_index)]
	pub type PublicCuratorToIndex<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		CuratorIndex,
		OptionQuery,
	>;

	/// Curator index to hash and update time. Info Hash is current used curator legal file hash.
	#[pallet::storage]
	#[pallet::getter(fn curator_index_to_info)]
	pub type CuratorIndexToInfo<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CuratorIndex,
		(InfoHash, BlockNumberFor<T>, T::AccountId, CandidateStatus),
		OptionQuery,
	>;

	/// The next free Pool Proposal index, aka the number of pool proposed so far.
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_count)]
	pub type PoolProposalCount<T> = StorageValue<_, PoolProposalIndex, ValueQuery>;

	/// Those who have a reserve for his pool proposal.
	///
	/// TWOX-NOTE: Safe, as increasing integer keys are safe.
	#[pallet::storage]
	#[pallet::getter(fn pool_deposit_of)]
	pub type PoolDepositOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		CuratorIndex,
		BoundedVec<(PoolProposalIndex, BalanceOf<T>), T::MaxDeposits>,
	>;

	// Metadata of staking pools
	#[pallet::storage]
	#[pallet::getter(fn staking_pool_metadata)]
	pub type StakingPoolStatus<T: Config> = StorageMap<
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

		CuratorRegisted {
			curator: T::AccountId,
			curator_index: CuratorIndex,
			info_hash: InfoHash, 
		},
		CuratorUpdated {
			curator: T::AccountId,
			curator_index: CuratorIndex,
			info_hash: InfoHash, 
		},
		CuratorCleaned {
			curator: T::AccountId,
			curator_index: CuratorIndex,
		},
		CuratorStatusUpdated {
			curator: T::AccountId,
			curator_index: CuratorIndex,
			status: CandidateStatus,
		}
		/// A motion has been proposed by a public account.
		Proposed { proposal_index: PropIndex, deposit: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		CuratorAlreadyRegistered,
		CuratorNotRegistered,
		CuratorIndexNotExist,
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
		/// Registing a curator legal info
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn regist_curator(
			origin: OriginFor<T>,
			info_hash: InfoHash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure curator not existing yet
			ensure!(!PublicCuratorToIndex::<T>::contains_key(&who), Error::<T>::CuratorAlreadyRegistered);
			// New registed curator need to make a balance reserve
			T::Currency::reserve(&who, MinimumCuratorDeposit::get())?;
			Self::update_curator(who, Some(info_hash))
		}

		/// Updating a curator legal info
		#[pallet::call_index(1)]
		#[pallet::weight(W{195_000_000})]
		pub fn update_curator(
			origin: OriginFor<T>,
			info_hash: InfoHash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure existing
			ensure!(PublicCuratorToIndex::<T>::contains_key(&who), Error::<T>::CuratorNotRegistered);

			Self::update_curator(who, Some(info_hash))
		}

		/// Clean a curator legal info
		/// Impossible when there is a staking pool proposal ongoing
		#[pallet::call_index(2)]
		#[pallet::weight(W{195_000_000})]
		pub fn clean_curator(
			origin: OriginFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure existing
			ensure!(PublicCuratorToIndex::<T>::contains_key(&who), Error::<T>::CuratorNotRegistered);

			// New registed curator need to make a balance reserve
			T::Currency::unreserve(&who, MinimumCuratorDeposit::get())?;
			Self::update_curator(who, None)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(W{195_000_000})]
		pub fn judge_curator_status(
			origin: OriginFor<T>,
			curator: T::AccountId,
			status: CandidateStatus,
		) -> DispatchResult {
			T::CuratorJudgeOrigin::ensure_origin(origin)?;
			let curator_index = PublicCuratorToIndex::<T>::get(curator).ok_or(Error::<T>::CuratorNotRegistered)?;
			CuratorIndexToInfo::<T>::try_mutate_exists(curator_index, |maybe_info| -> Result<(), DispatchError> {
				let mut info = maybe_info.as_mut().ok_or(Error::<T>::CuratorIndexNotExist)?;
				// Update block number
				info.1 = frame_system::Pallet::<T>::block_number();
				// Update status
				info.3 = status;
				Self::deposit_event(Event::CuratorStatusUpdated { 
					curator,
					curator_index,
					status,
				});
			})?;
		}





		/// Curator propose a staking pool
		///
		
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn propose_staking_pool(
			origin: OriginFor<T>,
			pool_setup: PoolSetting<BlockNumberFor<T>, BalanceOf<T>>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;


		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn update_curator(who: T::AccountId, info_hash: Option<InfoHash>) -> DispatchResult {
		if Some(hash) = info_hash {
			let current_block = frame_system::Pallet::<T>::block_number();
			let next_curator_index = PublicCuratorCount::<T>::get();

			PublicCuratorToIndex::<T>::insert(&who, next_curator_index);
			CuratorIndexToInfo::<T>::insert(&next_curator_index, (info_hash, current_block, who.clone(), CandidateStatus::Unverified));
			
			PublicCuratorCount::<T>::put(next_curator_index.checked_add(1u32.into())?);
		} else {
			// i.e. info_hash == None
			let index = PublicCuratorToIndex::<T>::take(&who);
			CuratorIndexToInfo::<T>::remove(&index);
		}
		Ok(())
	}
}
