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

//! A pallet for managing drop3 reward pools
//!
//! lifecycles of a reward pool:
//!  1. any user can propose a reward pool with needed parameters.
//!  2. the admin can approve or reject the proposal, a proportion of `total`
//!     amount will be slashed upon rejection.
//!  3. once approved, either the pool owner or admin can start/stop the
//!     pool multiple times, or close the pool to remove it from the map storage,
//!     where the remaining amount will be unreserved.
//!  4. when the pool is actively running, the owner can send the reward to any other user,
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

#![allow(clippy::type_complexity)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use frame_support::{
	pallet_prelude::*,
	traits::{BalanceStatus, Currency, ReservableCurrency, StorageVersion},
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, Saturating, Zero},
	Percent,
};

use scale_info::TypeInfo;
use sp_std::vec::Vec;
pub use weights::WeightInfo;
/// a single reward pool
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RewardPool<PoolId, BoundedString, AccountId, Balance, BlockNumber> {
	// unique pool id
	id: PoolId,
	// a bounded string whose length is limited by `MaximumNameLength`
	name: BoundedString,
	// account id of the pool owner
	owner: AccountId,
	// total amount of token that will be reserved upon creation
	total: Balance,
	// remaining amount of token that can be sent as reward for this pool
	remain: Balance,
	// block height where the pool was created
	create_at: BlockNumber,
	// start block height which is defined when proposing,
	// calling `send_reward` prior to this height would fail
	start_at: BlockNumber,
	// end block height which is defined when proposing,
	// calling `send_reward` after this height would fail
	end_at: BlockNumber,
	// if the pool is started
	started: bool,
	// if the pool is approved
	approved: bool,
}

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A unique id representing a single reward pool
		type PoolId: Default
			+ Copy
			+ PartialEq
			+ core::fmt::Debug
			+ codec::FullCodec
			+ AtLeast32BitUnsigned
			+ From<u64>
			+ TypeInfo;

		/// Currency mechanism
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The origin who can set the admin account
		type SetAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weights
		type WeightInfo: WeightInfo;

		/// percent of the total amount slashed when proposal gets rejected
		#[pallet::constant]
		type SlashPercent: Get<Percent>;

		/// The maximum length a on-chain string can be
		#[pallet::constant]
		type MaximumNameLength: Get<u32>;
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
		RewardPool<
			T::PoolId,
			BoundedVec<u8, T::MaximumNameLength>,
			T::AccountId,
			BalanceOf<T>,
			T::BlockNumber,
		>,
		OptionQuery,
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
		BalanceSlashed { who: T::AccountId, amount: BalanceOf<T> },
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
		RewardSent { to: T::AccountId, amount: BalanceOf<T> },
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
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_admin())]
		pub fn set_admin(origin: OriginFor<T>, new: T::AccountId) -> DispatchResultWithPostInfo {
			T::SetAdminOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::AdminChanged { old_admin: Self::admin() });
			<Admin<T>>::put(new);
			// Do not pay a fee
			Ok(Pays::No.into())
		}

		/// Approve a RewardPool proposal, must be called from admin
		#[pallet::weight(<T as pallet::Config>::WeightInfo::approve_reward_pool())]
		pub fn approve_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender) == Self::admin(), Error::<T>::RequireAdmin);

			RewardPools::<T>::try_mutate(id, |pool| {
				let mut p = pool.take().ok_or(Error::<T>::NoSuchRewardPool)?;
				p.approved = true;
				*pool = Some(p);
				Self::deposit_event(Event::RewardPoolApproved { id });
				Ok(().into())
			})
		}

		/// Reject a RewardPool proposal, must be called from admin
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reject_reward_pool())]
		pub fn reject_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Some(sender) == Self::admin(), Error::<T>::RequireAdmin);

			let _ = RewardPools::<T>::try_mutate(id, |pool| -> DispatchResultWithPostInfo {
				let mut p = pool.take().ok_or(Error::<T>::NoSuchRewardPool)?;
				// a reward pool can't be rejected if it was approved earlier
				ensure!(!p.approved, Error::<T>::RewardPoolAlreadyApproved);
				// slash a portion from reserved balance
				let to_slash = T::SlashPercent::get() * p.total;
				let (_, unslashed) = T::Currency::slash_reserved(&p.owner, to_slash);
				// theoretically unslashed should be always 0, but just to play it safe
				// no error is thrown even if unslashed is non-zero.
				let actual_slashed = to_slash - unslashed;
				p.remain = p.remain.saturating_sub(actual_slashed);
				*pool = Some(p.clone());
				Self::deposit_event(Event::BalanceSlashed { who: p.owner, amount: actual_slashed });
				Self::deposit_event(Event::RewardPoolRejected { id });
				Ok(().into())
			})?; // important to propagate the mutation error to caller

			// has to be after mutation is done
			Self::unreserve_and_close_reward_pool(id)
		}

		/// Start a reward pool, can be called by admin or reward pool owner
		#[pallet::weight(<T as pallet::Config>::WeightInfo::start_reward_pool())]
		pub fn start_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			RewardPools::<T>::try_mutate(id, |pool| {
				let mut p = pool.take().ok_or(Error::<T>::NoSuchRewardPool)?;
				ensure!(
					Some(sender.clone()) == Self::admin() || sender == p.owner,
					Error::<T>::RequireAdminOrRewardPoolOwner
				);
				ensure!(p.approved, Error::<T>::RewardPoolUnapproved);
				p.started = true;
				*pool = Some(p);
				Self::deposit_event(Event::RewardPoolStarted { id });
				Ok(().into())
			})
		}

		/// Stop a reward pool, can be called by admin or reward pool owner
		#[pallet::weight(<T as pallet::Config>::WeightInfo::stop_reward_pool())]
		pub fn stop_reward_pool(origin: OriginFor<T>, id: T::PoolId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			RewardPools::<T>::try_mutate(id, |pool| {
				let mut p = pool.take().ok_or(Error::<T>::NoSuchRewardPool)?;
				ensure!(
					Some(sender.clone()) == Self::admin() || sender == p.owner,
					Error::<T>::RequireAdminOrRewardPoolOwner
				);
				ensure!(p.approved, Error::<T>::RewardPoolUnapproved);
				p.started = false;
				*pool = Some(p);
				Self::deposit_event(Event::RewardPoolStopped { id });
				Ok(().into())
			})
		}

		/// Close a reward pool, can be called by admin or reward pool owner
		///
		/// Note here `approved` state is not required, which gives the owner a
		/// chance to close it before the admin evaluates the proposal
		#[pallet::weight(<T as pallet::Config>::WeightInfo::close_reward_pool())]
		pub fn close_reward_pool(
			origin: OriginFor<T>,
			id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let pool = RewardPools::<T>::get(id).ok_or(Error::<T>::NoSuchRewardPool)?;
			ensure!(
				Some(sender.clone()) == Self::admin() || sender == pool.owner,
				Error::<T>::RequireAdminOrRewardPoolOwner
			);

			Self::unreserve_and_close_reward_pool(id)
		}

		/// Create a RewardPool proposal, can be called by any signed account
		#[pallet::weight(<T as pallet::Config>::WeightInfo::propose_reward_pool(T::MaximumNameLength::get()))]
		pub fn propose_reward_pool(
			origin: OriginFor<T>,
			name: Vec<u8>,
			total: BalanceOf<T>,
			start_at: T::BlockNumber,
			end_at: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			// a few sanity checks
			let sender = ensure_signed(origin)?;
			ensure!(total > 0u32.into(), Error::<T>::InvalidTotalBalance);
			ensure!(end_at >= start_at, Error::<T>::InvalidProposedBlock);

			let bounded_name: BoundedVec<u8, T::MaximumNameLength> =
				name.clone().try_into().expect("reward pool name is too long");

			// reserve the owner's balance
			T::Currency::reserve(&sender, total)?;
			let next_id: T::PoolId = Self::get_next_pool_id()?;
			// create the reward pool
			let new_reward_pool = RewardPool::<_, _, _, _, _> {
				id: next_id,
				name: bounded_name,
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
		/// TODO:
		/// `repatriate_reserved()` requires that the destination account is active
		/// otherwise `DeadAccount` error is returned. Is it OK in our case?
		#[pallet::weight(<T as pallet::Config>::WeightInfo::send_reward())]
		pub fn send_reward(
			origin: OriginFor<T>,
			id: T::PoolId,
			to: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			RewardPools::<T>::try_mutate(id, |pool| {
				let mut p = pool.take().ok_or(Error::<T>::NoSuchRewardPool)?;
				ensure!(sender == p.owner, Error::<T>::RequireRewardPoolOwner);
				ensure!(p.approved, Error::<T>::RewardPoolUnapproved);
				ensure!(p.started, Error::<T>::RewardPoolStopped);

				let now = <frame_system::Pallet<T>>::block_number();
				ensure!(now >= p.start_at, Error::<T>::RewardPoolRanTooEarly);
				ensure!(now <= p.end_at, Error::<T>::RewardPoolRanTooLate);

				// ensure the remaining amount of a pool >= amount to be sent
				ensure!(p.remain >= amount, Error::<T>::InsufficientRemain);
				// ensure the reserved balance of the pool owner >= amount of a pool
				// theoretically this should be always true, even when one account has multiple
				// reward pools and the balance is reserved/unreserved here and there.
				//
				// If it somehow fails, it implies somewhere the code logic is wrong or the the
				// reserved balance was (unexpectedly) unreserved
				//
				// we don't care so much about the comparison between reserved balance and
				// pool.remain, the worst case is pool.remain is higher than reserved balance when
				// unreserving, which is not so bad as up to 'reserved balance' will be unreserved
				// anyway.
				ensure!(
					T::Currency::reserved_balance(&p.owner) >= amount,
					Error::<T>::InsufficientReservedBalance
				);
				// do the transfer from the reserved balance
				// we shall make sure the correct amount is included in event even when
				// unmoved is non-zero (which should not happen)
				let unmoved =
					T::Currency::repatriate_reserved(&p.owner, &to, amount, BalanceStatus::Free)?;
				ensure!(unmoved == Zero::zero(), Error::<T>::UnexpectedUnMovedAmount);
				let actual_moved = amount - unmoved;
				p.remain = p.remain.saturating_sub(actual_moved);
				*pool = Some(p);
				Self::deposit_event(Event::RewardSent { to, amount: actual_moved });
				Ok(().into())
			})
		}
	}

	impl<T: Config> Pallet<T> {
		/// close the reward pool, remove it from the map
		/// and unreserve all the remaining token to the pool owner
		fn unreserve_and_close_reward_pool(id: T::PoolId) -> DispatchResultWithPostInfo {
			let pool = RewardPools::<T>::take(id).ok_or(Error::<T>::NoSuchRewardPool)?;
			// we don't care if reserved balance is less than pool.remain
			let _ = T::Currency::unreserve(&pool.owner, pool.remain);
			RewardPoolOwners::<T>::remove(pool.id);

			Self::deposit_event(Event::RewardPoolRemoved {
				id: pool.id,
				name: pool.name.into_inner(),
				owner: pool.owner,
			});

			Ok(().into())
		}

		pub fn get_sorted_pool_ids() -> Vec<T::PoolId> {
			let mut ids = RewardPools::<T>::iter_keys().collect::<Vec<T::PoolId>>();
			ids.sort();
			ids
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
			for (idx, id) in sorted_ids.iter().enumerate() {
				let expected_id: T::PoolId = (idx as u64).checked_add(1u64).unwrap().into();
				if id != &expected_id {
					return Ok(expected_id)
				}
			}

			Err(Error::<T>::NoVacantPoolId)
		}
	}
}
