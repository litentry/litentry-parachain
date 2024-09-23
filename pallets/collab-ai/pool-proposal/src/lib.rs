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
//! # Pool Proposal Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The Pool Proposal handles the administration of proposed staking pool and pre-staking.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

use bitflags::bitflags;
use codec::{Decode, Encode};
use frame_support::{
	ensure,
	error::BadOrigin,
	traits::{Currency, EnsureOrigin, Get, LockIdentifier, LockableCurrency, ReservableCurrency},
	weights::Weight,
	BoundedVec,
};
use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
use orml_utilities::OrderedSet;
pub use pallet::*;
use pallet_collab_ai_common::*;
use sp_runtime::traits::CheckedAdd;
use sp_std::{cmp::Ordering, collections::vec_deque::VecDeque};

pub use types::*;

pub(crate) const POOL_DEMOCRACY_ID: LockIdentifier = *b"spdemocy";
pub(crate) const POOL_COMMITTEE_ID: LockIdentifier = *b"spcomtte";

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
pub type CallOf<T> = <T as frame_system::Config>::RuntimeCall;
pub type BoundedCallOf<T> = Bounded<CallOf<T>>;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

pub(crate) type InspectFungibles<T> = pallet_assets::Pallet<T>;
/// Balance type alias for balances of assets that implement the `fungibles` trait.
pub(crate) type AssetBalanceOf<T> =
	<InspectFungibles<T> as FsInspect<<T as frame_system::Config>::AccountId>>::Balance;
/// Type alias for Asset IDs.
pub(crate) type AssetIdOf<T> =
	<InspectFungibles<T> as FsInspect<<T as frame_system::Config>::AccountId>>::AssetId;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::OptionQuery};
	use orml_utilities::ordered_set;

	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Scheduler.
		type Scheduler: ScheduleNamed<BlockNumberFor<Self>, CallOf<Self>, Self::PalletsOrigin>;

		/// Currency type for this pallet.
		type Currency: ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>;

		// Declare the asset id of AIUSD
		type AIUSDAssetId: Get<AssetIdOf<Self>>;

		/// Period of time between proposal ended and pool start
		#[pallet::constant]
		type OfficialGapPeriod: Get<BlockNumberFor<Self>>;

		/// Minimum period of time for proposal voting end/expired
		#[pallet::constant]
		type MinimumProposalLastTime: Get<BlockNumberFor<Self>>;

		/// The minimum amount to be used as a deposit for creating a pool curator
		#[pallet::constant]
		type MinimumPoolDeposit: Get<BalanceOf<Self>>;

		/// The maximum amount of allowed pool proposed by a single curator
		#[pallet::constant]
		type MaximumPoolProposed: Get<u32>;

		/// Origin who can make a pool proposal
		type ProposalOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// Origin who can make a pool proposal pass public vote check
		type PublicVotingOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// System Account holding pre-staking assets
		type PreStakingPool: Get<Self::AccountId>;
	}

	/// The next free Pool Proposal index, aka the number of pool proposed so far.
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_count)]
	pub type PoolProposalCount<T> = StorageValue<_, PoolProposalIndex, ValueQuery>;

	/// Those who have a reserve for his pool proposal.
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal_deposit_of)]
	pub type PoolProposalDepositOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		OrderedSet<Bond<PoolProposalIndex, BalanceOf<T>>, T::MaxDeposits>,
		OptionQuery,
	>;

	// Pending pool proposal status of staking pools
	// Ordered by expired time
	#[pallet::storage]
	#[pallet::getter(fn pending_pool_proposal_status)]
	pub type PendingPoolProposalStatus<T: Config> =
		StorageValue<_, VecDeque<PoolProposalStatus<BlockNumberFor<T>>>, ValueQuery>;

	// Pool proposal content
	// This storage is not allowed to update once any ProposalStatusFlags passed
	// Yet root is allowed to do that
	#[pallet::storage]
	#[pallet::getter(fn pool_proposal)]
	pub type PoolProposal<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PoolProposalIndex,
		PoolProposalInfo<InfoHash, AssetBalanceOf<T>, BlockNumberFor<T>, T::AccountId>,
		OptionQuery,
	>;

	// Prestaking of pool proposal
	// This storage will be modified/delete correspondingly when solving pending pool
	#[pallet::storage]
	#[pallet::getter(fn staking_pool_pre_stakings)]
	pub type StakingPoolPrestakings<T: Config> = StorageMap<
		_,
		Twox64Concat,
		PoolProposalIndex,
		PoolProposalPreStaking<
			T::AccountId,
			AssetBalanceOf<T>,
			BlockNumberFor<T>,
			T::MaximumPoolProposed,
		>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A motion has been proposed by a public account.
		PoolProposed { proposer: T::AccountId, pool_proposal_index: PoolProposalIndex },
		/// A pre staking becomes valid
		PoolPreStaked {
			user: T::AccountId,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		},
		/// A pre staking queued
		PoolPreStakeQueued {
			user: T::AccountId,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		},
		/// A queued pre staking becomes a valid pre staking
		PoolQueuedStaked {
			user: T::AccountId,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		},
		/// Some amount of pre staking regardless of queue or pre staked, withdrawed (Withdraw queue ones first)
		PoolWithdrawed {
			user: T::AccountId,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		},
		/// A public vote result of proposal get passed
		ProposalPublicVoted { pool_proposal_index: PoolProposalIndex, vote_result: bool },
	}

	#[pallet::error]
	pub enum Error<T> {
		PreStakingOverflow,
		ProposalExpired,
		ProposalPreStakingLocked,
		ProposalPublicTimeTooShort,
		ProposalNotExist,
		StakingPoolOversized,
		InsufficientPreStaking,
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
		/// proposal_last_time: How does the proposal lasts for voting/prestaking.
		///                     All ProposalStatusFlags must be satisfied after this period passed, which is also
		/// 					the approximate date when pool begins.
		/// pool_last_time: How long does the staking pool last if passed
		/// estimated_epoch_reward: This number is only for displaying purpose without any techinical meaning
		/// pool_info_hash: Hash of pool info for including pool details
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		#[transactional]
		pub fn propose_staking_pool(
			origin: OriginFor<T>,
			max_pool_size: AssetBalanceOf<T>,
			proposal_last_time: BlockNumberFor<T>,
			pool_last_time: BlockNumberFor<T>,
			estimated_epoch_reward: AssetBalanceOf<T>,
			pool_info_hash: InfoHash,
		) -> DispatchResult {
			let who = T::ProposalOrigin::ensure_origin(origin)?;

			let current_block = frame_system::Pallet::<T>::block_number();
			ensure!(
				proposal_last_time >= MinimumProposalLastTime::get(),
				Error::<T>::ProposalPublicTimeTooShort
			);

			let proposal_end_time =
				current_block.checked_add(proposal_last_time).ok_or(ArithmeticError::Overflow)?;

			let pool_start_time = proposal_end_time
				.checked_add(OfficialGapPeriod::get())
				.ok_or(ArithmeticError::Overflow)?;

			let new_proposal_info = PoolProposalInfo {
				proposer: who,
				pool_info_hash,
				max_pool_size,
				pool_start_time,
				pool_end_time: pool_start_time
					.checked_add(pool_last_time)
					.ok_or(ArithmeticError::Overflow)?,
				estimated_epoch_reward,
				proposal_status_flags: ProposalStatusFlags::empty(),
			};

			let next_proposal_index = PoolProposalCount::<T>::get();
			PoolProposal::<T>::insert(next_proposal_index, new_proposal_info);
			PublicCuratorToIndex::<T>::insert(&who, next_curator_index);
			PoolProposalDepositOf::<T>::try_mutate_exists(
				&who,
				|maybe_ordered_set| -> Result<(), DispatchError> {
					let reserved_amount = MinimumPoolDeposit::get();
					let _ = T::Currency::reserve(&who, reserved_amount)?;
					// We should not care about duplicating since the proposal index is auto-increment
					match maybe_ordered_set.as_mut() {
						Some(ordered_set) => {
							ordered_set.insert(Bond {
								owner: next_proposal_index,
								value: reserved_amount,
							});
						},
						None => {
							let new_ordered_set = OrderedSet::new().insert(Bond {
								owner: next_proposal_index,
								value: reserved_amount,
							});
							*maybe_ordered_set = Some(new_ordered_set)
						},
					}
				},
			);
			<PendingPoolProposalStatus<T>>::mutate(|pending_porposals| {
				let new_proposal_status = PoolProposalStatus {
					pool_proposal_index: next_proposal_index,
					proposal_expire_time: proposal_end_time,
				};
				pending_porposals.push_back(new_proposal_status);
				// Make sure the first element has earlies effective time
				pending_porposals
					.make_contiguous()
					.sort_by(|a, b| a.proposal_expire_time.cmp(&b.proposal_expire_time));
			});
			PoolProposalCount::<T>::put(
				next_proposal_index.checked_add(1u32.into()).ok_or(ArithmeticError::Overflow)?,
			);
			Self::deposit_event(Event::PoolProposed {
				proposer: who,
				pool_proposal_index: next_proposal_index,
			});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(W{195_000_000})]
		#[transactional]
		pub fn pre_stake_proposal(
			origin: OriginFor<T>,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let asset_actual_transfer_amount: AssetBalanceOf<T> = <InspectFungibles<T>>::transfer(
				AIUSDAssetId::get(),
				who,
				PreStakingPool::get(),
				amount,
				Preservation::Expendable,
			)?;

			let mut pool_proposal_pre_staking =
				<StakingPoolPrestakings<T>>::take(pool_proposal_index)
					.unwrap_or(PoolProposalPreStaking::new());

			// Check pool maximum size limit and make pool size limit flag change accordingly
			let mut pool_proposal =
				PoolProposal::get(pool_proposal_index).ok_or(Error::<T>::ProposalNotExist)?;
			// Proposal not expired
			ensure!(
				!pool_proposal
					.proposal_status_flags
					.contains(ProposalStatusFlags::PROPOSAL_EXPIRED),
				Error::<T>::ProposalExpired
			);
			// If proposal is fully pre-staked or partial oversized after this stake

			// Check BoundedVec limit
			ensure!(
				!pool_proposal_pre_staking.pre_stakings.is_full
					&& !pool_proposal_pre_staking.queued_pre_stakings.is_full,
				Error::<T>::StakingPoolOversized
			);

			let target_pre_staked_amount = pool_proposal_pre_staking
				.total_pre_staked_amount
				.checked_add(asset_actual_transfer_amount)
				.ok_or(ArithmeticError::Overflow)?;
			if (target_pre_staked_amount <= pool_proposal.max_pool_size) {
				// take all pre-staking into valid pre-staking line
				pool_proposal_pre_staking
					.add_pre_staking::<T>(who, asset_actual_transfer_amount)?;

				// Emit event only
				Self::deposit_event(Event::PoolPreStaked {
					user: who,
					pool_proposal_index,
					amount: asset_actual_transfer_amount,
				});
				// Flag proposal status if pool is just fully staked
				if (target_pre_staked_amount == pool_proposal.max_pool_size) {
					pool_proposal.proposal_status_flags = pool_proposal.proposal_status_flags
						| ProposalStatusFlags::STAKE_AMOUNT_PASSED;
					PoolProposal::put(pool_proposal_index, pool_proposal);
				}
			} else {
				// Partially
				let queued_pre_staked_amount = target_pre_staked_amount
					.checked_sub(pool_proposal.max_pool_size)
					.ok_or(ArithmeticError::Overflow)?;
				pool_proposal_pre_staking.add_queued_staking::<T>(
					who,
					queued_pre_staked_amount,
					frame_system::Pallet::<T>::block_number(),
				)?;

				// If pool not already full, flag proposal status
				if (asset_actual_transfer_amount > queued_pre_staked_amount) {
					let actual_pre_staked_amount = asset_actual_transfer_amount
						.checked_sub(queued_pre_staked_amount)
						.ok_or(ArithmeticError::Overflow)?;
					pool_proposal_pre_staking
						.add_pre_staking::<T>(who, actual_pre_staked_amount)?;

					Self::deposit_event(Event::PoolPreStaked {
						user: who,
						pool_proposal_index,
						amount: actual_pre_staked_amount,
					});

					pool_proposal.proposal_status_flags = pool_proposal.proposal_status_flags
						| ProposalStatusFlags::STAKE_AMOUNT_PASSED;
					PoolProposal::put(pool_proposal_index, pool_proposal);
				}

				// Emit events
				Self::deposit_event(Event::PoolPreStakeQueued {
					user: who,
					pool_proposal_index,
					amount: queued_pre_staked_amount,
				});
			}

			<StakingPoolPrestakings<T>>::put(pool_proposal_index, pool_proposal_pre_staking);
		}

		// Withdraw is not allowed when proposal has STAKE_AMOUNT_PASSED flag
		// unless there is queued amount pending
		#[pallet::call_index(2)]
		#[pallet::weight(W{195_000_000})]
		#[transactional]
		pub fn withdraw_pre_staking(
			origin: OriginFor<T>,
			pool_proposal_index: PoolProposalIndex,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut pool_proposal_pre_staking =
				<StakingPoolPrestakings<T>>::take(pool_proposal_index)
					.unwrap_or(PoolProposalPreStaking::new());

			// Either staking pool has not locked yet,
			// Or queued amount is enough to replace the withdrawal
			ensure!(
				!pool_proposal
					.proposal_status_flags
					.contains(ProposalStatusFlags::STAKE_AMOUNT_PASSED)
					|| (pool_proposal_pre_staking.total_queued_amount >= amount),
				Error::<T>::ProposalPreStakingLocked
			);

			let _ = pool_proposal_pre_staking.withdraw::<T>(who, amount)?;
			Self::deposit_event(Event::PoolWithdrawed { user: who, pool_proposal_index, amount });

			let mut pool_proposal =
				PoolProposal::get(pool_proposal_index).ok_or(Error::<T>::ProposalNotExist)?;
			// Make queued amount fill the missing staked amount if pool staked flag ever reached
			if ((pool_proposal_pre_staking.total_pre_staked_amount < pool_proposal.max_pool_size)
				&& (pool_proposal
					.proposal_status_flags
					.contains(ProposalStatusFlags::STAKE_AMOUNT_PASSED)))
			{
				let moved_bonds = pool_proposal_pre_staking
					.move_queued_to_pre_staking_until::<T>(pool_proposal.max_pool_size)?;
				for i in moved_bonds.iter() {
					// Emit events
					Self::deposit_event(Event::PoolQueuedStaked {
						user: i.owner,
						pool_proposal_index,
						amount: i.amount,
					});
				}
			}

			// Return funds
			let asset_actual_transfer_amount: AssetBalanceOf<T> = <InspectFungibles<T>>::transfer(
				AIUSDAssetId::get(),
				PreStakingPool::get(),
				who,
				amount,
				Preservation::Expendable,
			)?;

			<StakingPoolPrestakings<T>>::put(pool_proposal_index, pool_proposal_pre_staking);

			Ok(())
		}

		// This is democracy/committe passing check for staking pool proposal
		// TODO: Related logic with "pallet-conviction-voting"
		#[pallet::call_index(3)]
		#[pallet::weight(W{195_000_000})]
		pub fn public_vote_proposal(
			origin: OriginFor<T>,
			pool_proposal_index: PoolProposalIndex,
			vote: bool,
		) -> DispatchResult {
			T::PublicVotingOrigin::ensure_origin(origin)?;
			let mut pool_proposal =
				PoolProposal::get(pool_proposal_index).ok_or(Error::<T>::ProposalNotExist)?;

			if vote {
				pool_proposal.proposal_status_flags =
					pool_proposal.proposal_status_flags | ProposalStatusFlags::PUBLIC_VOTE_PASSED;
			} else {
				pool_proposal.proposal_status_flags =
					pool_proposal.proposal_status_flags & !ProposalStatusFlags::PUBLIC_VOTE_PASSED;
			}
			PoolProposal::put(pool_proposal_index, pool_proposal);

			Self::deposit_event(Event::ProposalPublicVoted {
				pool_proposal_index,
				vote_result: vote,
			});
			Ok(())
		}
	}
}
