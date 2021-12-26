// Copyright 2020-2021 Litentry Technologies GmbH.
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

//! A pallet for managing drop3 reward pools
//!
//! lifecycles of a reward pool:
//!  1. any user can propose a reward pool with needed parameters.
//!  2. the admin can approve or reject the proposal, a proportion of `total`
//!     amount will be slashed upon rejection.
//!  3. once approved, either the pool owner or admin can start/stop the
//! 	pool multiple times, or close the pool to remove it from the map storage,
//!     where the remaining amount will be unreserved.
//!	 4. when the pool is actively running, the owner can send the reward to any other user,
//!     the amount will be deducted directly from reserved balance.
//!
//! Some notes:
//!  - the admin account can only be set by SetAdminOrigin, which will be bound at runtime.
//!  - a user can propose/own multiple reward pools.
//!  - the events and errors are relatively in verbose level.
//!  - about the usage of reversed balance: see the checks around `slash_reserved` and
//!    `repatriated_reversed`, theoretically the desired amount could always be slashed/moved, as
//!    `total` amount is already reserved upon creation of a reward pool. However, we are playing it
//!    safe and we make sure the emitted event always contains the actual amount, since we don't
//!    know if user's balance is unexpectedly unreserved somewhere outside this pallet.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::{
	pallet_prelude::*,
	traits::{BalanceStatus, ReservableCurrency, StorageVersion},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, Saturating, Zero},
	Percent,
};

use scale_info::TypeInfo;

/// a single reward pool
#[derive(PartialEq, Eq, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RewardPool<PoolId, AccountId, Balance, BlockNumber> {
	id: PoolId,
	name: Vec<u8>,
	owner: AccountId,
	total: Balance,  // total amount of token that will be reserved upon creation
	remain: Balance, // remaining amount of token
	create_at: BlockNumber,
	start_at: BlockNumber,
	end_at: BlockNumber,
	started: bool,
	approved: bool,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type PoolId: Default
			+ Copy
			+ PartialEq
			+ core::fmt::Debug
			+ codec::FullCodec
			+ AtLeast32BitUnsigned
			+ From<u64>
			+ TypeInfo;

		/// The origin who can set the admin account
		type SetAdminOrigin: EnsureOrigin<Self::Origin>;

		/// percent of the total amount slashed when proposal gets rejected
		#[pallet::constant]
		type SlashPercent: Get<Percent>;

		/// The maximum length a name of proposed reward pool can be
		#[pallet::constant]
		type MaxNameLength: Get<u32>;
	}

	/// The reward pool admin account
	/// The reason why such an account is needed (other than just using ROOT) is for
	/// fast processing of reward proposals, imagine later when sudo is removed
	#[pallet::storage]
	#[pallet::getter(fn admin)]
	pub type Admin<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	// current maximum pool id starting from 1
	// used as starting point as long as it hasn't reached max_value()
	// see get_next_pool_id()
	#[pallet::storage]
	#[pallet::getter(fn current_max_pool_id)]
	pub type CurrentMaxPoolId<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

	/// Map for PoolId <> RewardPool
	#[pallet::storage]
	#[pallet::getter(fn reward_pools)]
	pub type RewardPools<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		RewardPool<T::PoolId, T::AccountId, T::Balance, T::BlockNumber>,
		ValueQuery,
	>;

	/// Map for PoolId <> RewardPoolOwner
	#[pallet::storage]
	#[pallet::getter(fn reward_pool_owners)]
	pub type RewardPoolOwners<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Admin acccount was changed, the \[ old admin \] is provided
		AdminChanged { old_admin: Option<T::AccountId> },
		/// An \[ amount \] balance of \[ who \] was slashed
		BalanceSlashed { who: T::AccountId, amount: T::Balance },
		/// A reward pool with \[ id \] was approved by admin
		RewardPoolApproved { id: T::PoolId },
		/// A reward pool with \[ id \] was rejected by admin
		RewardPoolRejected { id: T::PoolId },
		/// A reward pool with \[ id \] was started, either by admin or owner
		RewardPoolStarted { id: T::PoolId },
		/// A reward pool with \[ id \] was stopped, either by admin or owner
		RewardPoolStopped { id: T::PoolId },
		/// A reward pool with \[ id, name, owner \] was removed, either by admin or owner
		RewardPoolRemoved { id: T::PoolId, name: Vec<u8>, owner: T::AccountId },
		/// A reward pool with \[ id, name, owner \] was proposed
		RewardPoolProposed { id: T::PoolId, name: Vec<u8>, owner: T::AccountId },
		/// An \[ amount \] of reward was sent to \[ to \]
		RewardSent { to: T::AccountId, amount: T::Balance },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error when the caller account is not the admin
		RequireAdmin,
		/// Error when the caller account is not the reward pool owner
		RequireRewardPoolOwner,
		/// Error when the caller account is not the reward pool owner or admin
		RequireAdminOrRewardPoolOwner,
		/// Error when a reward pool can't be found
		NoSuchRewardPool,
		/// Error when the sender doesn't have enough reserved balance
		InsufficientReservedBalance,
		/// Error when `total` amount is 0 when proposing reward pool
		InvalidTotalBalance,
		/// Error when the remaning of a reward pool is not enough
		InsufficientRemain,
		/// Error when start_at < end_at when proposing reward pool
		InvalidProposedBlock,
		/// Error when proposed reward pool name is too long
		ProposedNameTooLong,
		/// Error when the reward pool is unapproved
		RewardPoolUnapproved,
		/// Error when the reward pool is first approved then rejected
		RewardPoolAlreadyApproved,
		/// Error when the reward pool is stopped
		RewardPoolStopped,
		/// Error when the reward pool is runing before `start_at`
		RewardPoolRanTooEarly,
		/// Error when the reward pool is runing after `end_at`
		RewardPoolRanTooLate,
		/// Error of unexpected unmoved amount when calling repatriate_reserved
		UnexpectedUnMovedAmount,
		/// Error when no vacant PoolId can be acquired
		NoVacantPoolId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Change the admin account
		/// similar to sudo.set_key, the old account will be supplied in event
		#[pallet::weight(50_000_000)]
		pub fn set_admin(origin: OriginFor<T>, new: T::AccountId) -> DispatchResultWithPostInfo {
			T::SetAdminOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::AdminChanged { old_admin: Self::admin() });
			<Admin<T>>::put(new);
			// Do not pay a fee
			Ok(Pays::No.into())
		}

		/// Approve a RewardPool proposal, must be called from admin
		#[pallet::weight(50_000_000)]
		pub fn approve_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::admin().unwrap(), Error::<T>::RequireAdmin);
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);

			RewardPools::<T>::mutate(id, |pool| {
				pool.approved = true;
			});
			Self::deposit_event(Event::RewardPoolApproved { id });
			Ok(().into())
		}

		/// Reject a RewardPool proposal, must be called from admin
		#[pallet::weight(50_000_000)]
		pub fn reject_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::admin().unwrap(), Error::<T>::RequireAdmin);
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);

			let pool = Self::reward_pools(id);
			// in case the reward pool was approved earlier, it can't be rejected (it can only be
			// closed)
			ensure!(!pool.approved, Error::<T>::RewardPoolAlreadyApproved);

			// slash a portion from reserved balance
			let to_slash = T::SlashPercent::get() * pool.total;
			let (_, unslashed) =
				<pallet_balances::Pallet<T> as ReservableCurrency<_>>::slash_reserved(
					&pool.owner,
					to_slash,
				);
			// theoretically unslashed should be always 0, but just to play it safe
			// no error is thrown even if unslashed is non-zero.
			let actual_slashed = to_slash - unslashed;
			RewardPools::<T>::mutate(id, |pool| {
				pool.remain = pool.remain.saturating_sub(actual_slashed);
			});

			Self::deposit_event(Event::RewardPoolRejected { id });
			Self::deposit_event(Event::BalanceSlashed {
				who: pool.owner.clone(),
				amount: actual_slashed,
			});

			Self::unreserve_and_close_reward_pool(id);
			Ok(().into())
		}

		/// Start a reward pool, can be called by admin or reward pool owner
		#[pallet::weight(50_000_000)]
		pub fn start_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);
			let mut pool = RewardPools::<T>::get(id);
			ensure!(
				sender == Self::admin().unwrap() || sender == pool.owner,
				Error::<T>::RequireAdminOrRewardPoolOwner
			);
			ensure!(pool.approved, Error::<T>::RewardPoolUnapproved);

			pool.started = true;
			RewardPools::<T>::insert(id, pool);
			Self::deposit_event(Event::RewardPoolStarted { id });
			Ok(().into())
		}

		/// Stop a reward pool, can be called by admin or reward pool owner
		#[pallet::weight(50_000_000)]
		pub fn stop_reward_pool(origin: OriginFor<T>, id: T::PoolId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);
			let mut pool = RewardPools::<T>::get(id);
			ensure!(
				sender == Self::admin().unwrap() || sender == pool.owner,
				Error::<T>::RequireAdminOrRewardPoolOwner
			);
			ensure!(pool.approved, Error::<T>::RewardPoolUnapproved);

			pool.started = false;
			RewardPools::<T>::insert(id, pool);
			Self::deposit_event(Event::RewardPoolStopped { id });
			Ok(().into())
		}

		/// Close a reward pool, can be called by admin or reward pool owner
		///
		/// Note here `approved` state is not required, which gives the owner a
		/// chance to close it before the admin evaluates the proposal
		#[pallet::weight(50_000_000)]
		pub fn close_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);
			ensure!(
				sender == Self::admin().unwrap() || sender == Self::reward_pools(id).owner,
				Error::<T>::RequireAdminOrRewardPoolOwner
			);

			Self::unreserve_and_close_reward_pool(id);
			Ok(().into())
		}

		/// Create a RewardPool proposal, can be called by any signed account
		#[pallet::weight(50_000_000)]
		pub fn propose_reward_pool(
			origin: OriginFor<T>,
			name: Vec<u8>,
			total: T::Balance,
			start_at: T::BlockNumber,
			end_at: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			// a few sanity checks
			let sender = ensure_signed(origin)?;
			ensure!(total > 0u32.into(), Error::<T>::InvalidTotalBalance);
			ensure!(
				<pallet_balances::Pallet<T> as ReservableCurrency<_>>::can_reserve(&sender, total),
				Error::<T>::InsufficientReservedBalance
			);
			ensure!(end_at >= start_at, Error::<T>::InvalidProposedBlock);
			ensure!(
				name.len() <= T::MaxNameLength::get() as usize,
				Error::<T>::ProposedNameTooLong
			);

			// reserve the owner's balance
			let _ = <pallet_balances::Pallet<T> as ReservableCurrency<_>>::reserve(&sender, total)?;
			let next_id: T::PoolId = Self::get_next_pool_id()?;
			// create the reward pool
			let new_reward_pool = RewardPool::<_, _, _, _> {
				id: next_id,
				name: name.clone(),
				owner: sender.clone(),
				total,
				remain: total,
				create_at: <frame_system::Pallet<T>>::block_number(),
				start_at,
				end_at,
				started: false,
				approved: false,
			};

			RewardPools::<T>::insert(next_id, new_reward_pool);
			RewardPoolOwners::<T>::insert(next_id, sender.clone());
			Self::deposit_event(Event::RewardPoolProposed { id: next_id, name, owner: sender });
			Ok(().into())
		}

		/// transfer an amount of reserved balance to some other user
		/// must be called by reward pool owner
		#[pallet::weight(50_000_000)]
		pub fn send_reward(
			origin: OriginFor<T>,
			id: T::PoolId,
			to: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(RewardPools::<T>::contains_key(id), Error::<T>::NoSuchRewardPool);
			let pool = Self::reward_pools(id);
			ensure!(sender == pool.owner, Error::<T>::RequireRewardPoolOwner);
			ensure!(pool.approved, Error::<T>::RewardPoolUnapproved);
			ensure!(pool.started, Error::<T>::RewardPoolStopped);

			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now >= pool.start_at, Error::<T>::RewardPoolRanTooEarly);
			ensure!(now <= pool.end_at, Error::<T>::RewardPoolRanTooLate);

			// ensure the remaining amount of a pool >= amount to be sent
			ensure!(pool.remain >= amount, Error::<T>::InsufficientRemain);
			// ensure the reserved balance of the pool owner >= amount of a pool
			// theoretically this should be always true, even when one account has multiple reward
			// pools and the balance is reserved/unreserved here and there.
			//
			// If it somehow fails, it implies somewhere the code logic is wrong or the the reserved
			// balance was (unexpectedly) unreserved
			//
			// we don't care so much about the comparison between reserved balance and pool.remain,
			// the worst case is pool.remain is higher than reserved balance when unreserving, which
			// is not so bad as up to 'reserved balance' will be unreserved anyway.
			ensure!(
				<pallet_balances::Pallet<T> as ReservableCurrency<_>>::reserved_balance(
					&pool.owner
				) >= amount,
				Error::<T>::InsufficientReservedBalance
			);
			// do the transfer from the reserved balance
			// we shall make sure the correct amount is included in event even when
			// unmoved is non-zero (which should not happen)
			let unmoved =
				<pallet_balances::Pallet<T> as ReservableCurrency<_>>::repatriate_reserved(
					&pool.owner,
					&to,
					amount,
					BalanceStatus::Free,
				)?;
			let actual_moved = amount - unmoved;
			Self::deposit_event(Event::RewardSent { to, amount: actual_moved });
			RewardPools::<T>::mutate(id, |pool| {
				pool.remain = pool.remain.saturating_sub(actual_moved);
			});
			ensure!(unmoved == Zero::zero(), Error::<T>::UnexpectedUnMovedAmount);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// close the reward pool, remove it from the map
		/// and unreserve all the remaining token to the pool owner
		fn unreserve_and_close_reward_pool(id: T::PoolId) {
			let pool = RewardPools::<T>::take(id);
			// we don't care if reserved balance is less than pool.remain
			let _ = <pallet_balances::Pallet<T> as ReservableCurrency<_>>::unreserve(
				&pool.owner,
				pool.remain,
			);
			RewardPoolOwners::<T>::remove(pool.id);

			Self::deposit_event(Event::RewardPoolRemoved {
				id: pool.id,
				name: pool.name.clone(),
				owner: pool.owner.clone(),
			});
		}

		pub fn get_sorted_pool_ids() -> Vec<T::PoolId> {
			let mut ids = RewardPools::<T>::iter_keys().collect::<Vec<T::PoolId>>();
			ids.sort();
			ids
		}

		#[cfg(test)]
		// propose a default reward pool, but with given id
		// mainly used to test get_next_pool_id()
		pub fn propose_default_reward_pool(id: T::PoolId, should_change_current_max: bool) {
			RewardPools::<T>::insert(id, RewardPool::default());
			RewardPoolOwners::<T>::insert(id, T::AccountId::default());
			if should_change_current_max {
				CurrentMaxPoolId::<T>::put(id);
			}
		}

		fn get_next_pool_id() -> Result<T::PoolId, pallet::Error<T>> {
			// if CurrentMaxPoolId hasn't reached max, increment and return it
			if Self::current_max_pool_id() < T::PoolId::max_value() {
				return CurrentMaxPoolId::<T>::mutate(|id| {
					*id += 1u64.into();
					Ok(*id)
				})
			}

			// otherwise find the vacant id from the beginning
			let sorted_ids = Self::get_sorted_pool_ids();
			let len = sorted_ids.len();
			for i in 0..len {
				// we start from 1, the addition should never overflow
				let expected_id: T::PoolId = (i as u64).checked_add(1u64).unwrap().into();
				if sorted_ids[i] != expected_id {
					return Ok(expected_id)
				}
			}

			Err(Error::<T>::NoVacantPoolId)
		}
	}
}
