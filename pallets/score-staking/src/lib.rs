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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use frame_support::traits::Currency;
use sp_core::crypto::AccountId32;
use sp_runtime::{traits::CheckedSub, Perbill};
use sp_std::marker::PhantomData;

pub use pallet::*;

mod types;
pub use types::*;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

// Need this to convert hardcoded `AccountId32` to T::AccountId
pub trait AccountIdConvert<T: Config> {
	fn convert(account: AccountId32) -> T::AccountId;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use core_primitives::{Identity, YEARS};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Imbalance, ReservableCurrency, StorageVersion},
	};
	use frame_system::pallet_prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_parachain_staking::Config {
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type YearlyIssuance: Get<BalanceOf<Self>>;
		#[pallet::constant]
		type YearlyInflation: Get<Perbill>;
		#[pallet::constant]
		type DefaultRoundInterval: Get<u32>;
		/// The origin who manages this pallet
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The origin to feed scores
		type ScoreFeedOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// AccountId converter
		type AccountIdConvert: AccountIdConvert<Self>;
	}

	#[pallet::error]
	pub enum Error<T> {
		// the user account doesn't have an entry in parachain-staking
		UserNotStaked,
		// the user account has an entry but the total staked amount is (somehow) zero
		UserStakedAmountZero,
		// the user account doesn't exist in the registry
		UserNotExist,
		// convert `Identity` to substrate account failed
		ConvertIdentityFailed,
		// pool is not in running state
		PoolNotRun,
		// pool is already in running state
		PoolAlreadyRunning,
		// round index overflow
		RoundIndexOverflow,
		// the user claims more than what he has
		InsufficientBalance,
		// balance underflow
		BalanceUnderflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PoolStarted { start_block: BlockNumberFor<T> },
		PoolStopped {},
		ScoreFeederSet { new_score_feeder: Option<T::AccountId> },
		ScoreUpdated { who: Identity, new_score: Score },
		RewardClaimed { who: T::AccountId, amount: BalanceOf<T> },
	}

	#[pallet::storage]
	#[pallet::getter(fn score_feeder)]
	pub type ScoreFeeder<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn round)]
	pub type Round<T: Config> = StorageValue<_, RoundInfo<BlockNumberFor<T>>, ValueQuery>;

	#[pallet::type_value]
	pub fn DefaultRoundInterval<T: Config>() -> u32 {
		T::DefaultRoundInterval::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn round_interval)]
	pub type RoundInterval<T: Config> = StorageValue<_, u32, ValueQuery, DefaultRoundInterval<T>>;

	// use `Twox64Concat` and `T::AccountId` for faster and shorter storage
	// we might have tens of thousands of entries
	#[pallet::storage]
	#[pallet::getter(fn scores)]
	pub type Scores<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, ScorePayment<BalanceOf<T>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn state)]
	pub type State<T: Config> = StorageValue<_, PoolState, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub state: PoolState,
		marker: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { state: PoolState::Stopped, marker: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			State::<T>::put(self.state);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 0); // Self::state()

			if Self::state() == PoolState::Stopped {
				return weight
			}

			let mut r = Round::<T>::get();
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 0));

			let previous_duration = r.duration;
			if (now - r.start_block) < previous_duration.into() {
				// nothing to do there
				return weight
			}

			// We are about to start a new round
			// 1. update round info
			r.index = r.index.saturating_add(1);
			r.start_block = now;
			r.duration = RoundInterval::<T>::get();
			Round::<T>::put(r);
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

			// 2. calculate payout
			let total = (T::YearlyInflation::get() * T::YearlyIssuance::get() / YEARS.into()) *
				previous_duration.into();

			weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((2 * T::DbWeight::get().write, DispatchClass::Normal))]
		pub fn set_score_feeder(
			origin: OriginFor<T>,
			new_score_feeder: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::AdminOrigin::ensure_origin(origin)?;
			ScoreFeeder::<T>::put(new_score_feeder.clone());
			Self::deposit_event(Event::ScoreFeederSet { new_score_feeder: Some(new_score_feeder) });
			Ok(Pays::No.into())
		}

		/// Start (or restart) a currently stopped pool
		///
		/// It sets the RoundInfo.start_block to the current block number
		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn start_pool(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::AdminOrigin::ensure_origin(origin)?;
			ensure!(Self::state() == PoolState::Stopped, Error::<T>::PoolAlreadyRunning);
			State::<T>::put(PoolState::Running);
			let start_block = frame_system::Pallet::<T>::block_number();
			let mut r = Round::<T>::take();
			r.start_block = start_block;
			r.duration = Self::round_interval();
			Round::<T>::put(r);
			Self::deposit_event(Event::PoolStarted { start_block });
			Ok(Pays::No.into())
		}

		/// Stop a currently running pool, should be called as caution, as it will cause
		/// the current round to pause and eventually have a different duration than round
		/// interval
		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn stop_pool(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::AdminOrigin::ensure_origin(origin)?;
			ensure!(Self::state() == PoolState::Running, Error::<T>::PoolNotRun);
			State::<T>::put(PoolState::Stopped);
			// terminate the current round, advance the round index
			let mut r = Round::<T>::take();
			r.duration = (frame_system::Pallet::<T>::block_number() - r.start_block).into();
			r.index = r.index.checked_add(1).ok_or(Error::<T>::RoundIndexOverflow)?;
			Round::<T>::put(r);
			Self::deposit_event(Event::PoolStopped {});
			Ok(Pays::No.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn update_score(
			origin: OriginFor<T>,
			user: Identity,
			score: Score,
		) -> DispatchResultWithPostInfo {
			let account = T::AccountIdConvert::convert(
				user.to_account_id().ok_or(Error::<T>::ConvertIdentityFailed)?,
			);
			T::ScoreFeedOrigin::ensure_origin(origin)?;
			Scores::<T>::try_mutate(&account, |payment| {
				let state = pallet_parachain_staking::Pallet::<T>::delegator_state(&account)
					.ok_or(Error::<T>::UserNotStaked)?;
				ensure!(state.total > 0u32.into(), Error::<T>::UserStakedAmountZero);

				match payment {
					Some(s) => {
						s.score = score;
						*payment = Some(*s);
					},
					None => *payment = Some(ScorePayment { score, unpaid: 0u32.into() }),
				}
				Ok::<(), Error<T>>(())
			})?;
			Self::deposit_event(Event::ScoreUpdated { who: user, new_score: score });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn claim(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			Scores::<T>::try_mutate(&account, |payment| {
				let mut p = payment.take().ok_or(Error::<T>::UserNotExist)?;
				ensure!(amount <= p.unpaid, Error::<T>::InsufficientBalance);
				let rewarded =
					<T as pallet::Config>::Currency::deposit_into_existing(&account, amount)?
						.peek();
				p.unpaid = p.unpaid.checked_sub(&rewarded).ok_or(Error::<T>::BalanceUnderflow)?;
				*payment = Some(p);
				Self::deposit_event(Event::RewardClaimed {
					who: account.clone(),
					amount: rewarded,
				});
				Ok(().into())
			})
		}

		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn claim_all(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin.clone())?;
			let payment = Scores::<T>::get(&account).ok_or(Error::<T>::UserNotExist)?;
			Self::claim(origin, payment.unpaid)
		}
	}
}
