// Copyright 2020-2023 Litentry Technologies GmbH.
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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::{
		pallet_prelude::*, traits::StorageVersion, transactional, PalletId, Parameter,
	};
	use frame_system::{
		pallet_prelude::*,
		{self as system},
	};
	use sp_std::prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Origin used to administer the pallet
		type GroupManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Group member added to set
		GroupMemberAdded(T::AccountId),
		/// Group member removed from set
		GroupMemberRemoved(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Group memeber already in set
		GroupMemberAlreadyExists,
		/// Provided accountId is not a Group member
		GroupMemberInvalid,
	}

	#[pallet::storage]
	#[pallet::getter(fn group_control_on)]
	pub type GroupControlOn<T: Config<I>, I: 'static = ()> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn group_members)]
	pub type GroupMembers<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Adds a new group member
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn add_group_member(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			ensure!(!Self::is_group_member(&v), Error::<T, I>::GroupMemberAlreadyExists);
			GroupMembers::<T, I>::insert(&v, true);
			Self::deposit_event(Event::GroupMemberAdded(v));
			Ok(())
		}

		/// Batch adding of new group members
		#[pallet::call_index(1)]
		#[pallet::weight(100_000)]
		#[transactional]
		pub fn batch_add_group_members(
			origin: OriginFor<T>,
			vs: Vec<T::AccountId>,
		) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			for v in vs {
				ensure!(!Self::is_group_member(&v), Error::<T, I>::GroupMemberAlreadyExists);
				GroupMembers::<T, I>::insert(&v, true);
				Self::deposit_event(Event::GroupMemberAdded(v));
			}
			Ok(())
		}

		/// Removes an existing group members
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn remove_group_member(origin: OriginFor<T>, v: T::AccountId) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			ensure!(Self::is_group_member(&v), Error::<T, I>::GroupMemberInvalid);
			GroupMembers::<T, I>::remove(&v);
			Self::deposit_event(Event::GroupMemberRemoved(v));
			Ok(())
		}

		/// Batch Removing existing group members
		#[pallet::call_index(3)]
		#[pallet::weight(100_000)]
		#[transactional]
		pub fn batch_remove_group_members(
			origin: OriginFor<T>,
			vs: Vec<T::AccountId>,
		) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			for v in vs {
				ensure!(Self::is_group_member(&v), Error::<T, I>::GroupMemberInvalid);
				GroupMembers::<T, I>::remove(&v);
				Self::deposit_event(Event::GroupMemberRemoved(v));
			}
			Ok(())
		}

		/// Swith GroupControlOn on
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn switch_group_control_on(origin: OriginFor<T>) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			<GroupControlOn<T, I>>::put(true);
			Ok(())
		}

		/// Swith GroupControlOn off
		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn switch_group_control_off(origin: OriginFor<T>) -> DispatchResult {
			T::GroupManagerOrigin::ensure_origin(origin)?;
			<GroupControlOn<T, I>>::put(false);
			Ok(())
		}
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Checks if who is a group member
		pub fn is_group_member(who: &T::AccountId) -> bool {
			Self::group_members(who)
		}
	}

	/// Simple ensure origin for the group account
	impl<T: Config<I>, I: 'static> EnsureOrigin<T::RuntimeOrigin> for Pallet<T, I> {
		type Success = T::AccountId;
		fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
			// If function off, then pass everything as long as signed
			if !Pallet::<T, I>::group_control_on() {
				o.into().and_then(|o| match o {
					system::RawOrigin::Signed(who) => Ok(who),
					r => Err(T::RuntimeOrigin::from(r)),
				})
			} else {
				o.into().and_then(|o| match o {
					system::RawOrigin::Signed(ref who) if Pallet::<T, I>::is_group_member(who) =>
						Ok(who.clone()),
					r => Err(T::RuntimeOrigin::from(r)),
				})
			}
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
			let who = frame_benchmarking::account::<T::AccountId>("successful_origin", 0, 0);
			GroupMembers::<T, I>::insert(&who, true);
			Ok(frame_system::RawOrigin::Signed(who).into())
		}
	}
}
