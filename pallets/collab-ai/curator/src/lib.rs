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

#[frame_support::pallet]
pub mod pallet {
	use super::{DispatchResult, *};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

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

		/// The period between a proposal being approved and enacted.
		///
		/// It should generally be a little more than the unstake period to ensure that
		/// voting stakers have an opportunity to remove themselves from the system in the case
		/// where they are on the losing side of a vote.
		#[pallet::constant]
		type EnactmentPeriod: Get<BlockNumberFor<Self>>;

		/// How often (in blocks) new public referenda are launched.
		#[pallet::constant]
		type LaunchPeriod: Get<BlockNumberFor<Self>>;

		/// How often (in blocks) to check for new votes.
		#[pallet::constant]
		type VotingPeriod: Get<BlockNumberFor<Self>>;

		/// The minimum period of vote locking.
		///
		/// It should be no shorter than enactment period to ensure that in the case of an approval,
		/// those successful voters are locked into the consequences that their votes entail.
		#[pallet::constant]
		type VoteLockingPeriod: Get<BlockNumberFor<Self>>;

		/// The minimum amount to be used as a deposit for a public referendum proposal.
		#[pallet::constant]
		type MinimumDeposit: Get<BalanceOf<Self>>;

		/// Indicator for whether an emergency origin is even allowed to happen. Some chains may
		/// want to set this permanently to `false`, others may want to condition it on things such
		/// as an upgrade having happened recently.
		#[pallet::constant]
		type InstantAllowed: Get<bool>;

		/// Minimum voting period allowed for a fast-track referendum.
		#[pallet::constant]
		type FastTrackVotingPeriod: Get<BlockNumberFor<Self>>;

		/// Period in blocks where an external proposal may not be re-submitted after being vetoed.
		#[pallet::constant]
		type CooloffPeriod: Get<BlockNumberFor<Self>>;

		/// The maximum number of public proposals that can exist at any time.
		#[pallet::constant]
		type MaxCurators: Get<u32>;

		/// The maximum number of deposits a public proposal may have at any time.
		#[pallet::constant]
		type MaxDeposits: Get<u32>;

		/// The maximum number of items which can be blacklisted.
		#[pallet::constant]
		type MaxBlacklisted: Get<u32>;

		/// Origin from which the next tabled referendum may be forced. This is a normal
		/// "super-majority-required" referendum.
		type ExternalOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which the next tabled referendum may be forced; this allows for the tabling
		/// of a majority-carries referendum.
		type ExternalMajorityOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which the next tabled referendum may be forced; this allows for the tabling
		/// of a negative-turnout-bias (default-carries) referendum.
		type ExternalDefaultOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which the new proposal can be made.
		///
		/// The success variant is the account id of the depositor.
		type SubmitOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// Origin from which the next majority-carries (or more permissive) referendum may be
		/// tabled to vote according to the `FastTrackVotingPeriod` asynchronously in a similar
		/// manner to the emergency origin. It retains its threshold method.
		type FastTrackOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which the next majority-carries (or more permissive) referendum may be
		/// tabled to vote immediately and asynchronously in a similar manner to the emergency
		/// origin. It retains its threshold method.
		type InstantOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which any referendum may be cancelled in an emergency.
		type CancellationOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which proposals may be blacklisted.
		type BlacklistOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which a proposal may be cancelled and its backers slashed.
		type CancelProposalOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin for anyone able to veto proposals.
		type VetoOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// Overarching type of all pallets origins.
		type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;

		/// Handler for the unbalanced reduction when slashing a preimage deposit.
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
	}

	/// The number of (public) curator that have been made so far.
	#[pallet::storage]
	#[pallet::getter(fn public_curator_count)]
	pub type PublicCuratorCount<T> = StorageValue<_, CuratorIndex, ValueQuery>;

	/// The public Curator. The second item is current using curator legal file hash.
	#[pallet::storage]
	#[pallet::getter(fn public_curators)]
	pub type PublicCurators<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		(CuratorIndex, InfoHash),
		OptionQuery,
	>;

	// Storing all history curator info hash with updated time
	// We all record the user who update the hash in the first place
	#[pallet::storage]
	#[pallet::getter(fn curator_info_hash)]
	pub type CuratorInfoHash<T: Config> = StorageMap<
		_,
		Twox64Concat,
		InfoHash,
		(BlockNumberFor<T>, T::AccountId, CandidateStatus),
		OptionQuery,
	>;

	/// The next free referendum index, aka the number of referenda started so far.
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
		/// A public proposal has been tabled for referendum vote.
		Tabled { proposal_index: PropIndex, deposit: BalanceOf<T> },
		/// An external proposal has been tabled.
		ExternalTabled,
		/// A referendum has begun.
		Started { ref_index: ReferendumIndex, threshold: VoteThreshold },
		/// A proposal has been approved by referendum.
		Passed { ref_index: ReferendumIndex },
		/// A proposal has been rejected by referendum.
		NotPassed { ref_index: ReferendumIndex },
		/// A referendum has been cancelled.
		Cancelled { ref_index: ReferendumIndex },
		/// An account has delegated their vote to another account.
		Delegated { who: T::AccountId, target: T::AccountId },
		/// An account has cancelled a previous delegation operation.
		Undelegated { account: T::AccountId },
		/// An external proposal has been vetoed.
		Vetoed { who: T::AccountId, proposal_hash: H256, until: BlockNumberFor<T> },
		/// A proposal_hash has been blacklisted permanently.
		Blacklisted { proposal_hash: H256 },
		/// An account has voted in a referendum
		Voted { voter: T::AccountId, ref_index: ReferendumIndex, vote: AccountVote<BalanceOf<T>> },
		/// An account has secconded a proposal
		Seconded { seconder: T::AccountId, prop_index: PropIndex },
		/// A proposal got canceled.
		ProposalCanceled { prop_index: PropIndex },
		/// Metadata for a proposal or a referendum has been set.
		MetadataSet {
			/// Metadata owner.
			owner: MetadataOwner,
			/// Preimage hash.
			hash: PreimageHash,
		},
		/// Metadata for a proposal or a referendum has been cleared.
		MetadataCleared {
			/// Metadata owner.
			owner: MetadataOwner,
			/// Preimage hash.
			hash: PreimageHash,
		},
		/// Metadata has been transferred to new owner.
		MetadataTransferred {
			/// Previous metadata owner.
			prev_owner: MetadataOwner,
			/// New metadata owner.
			owner: MetadataOwner,
			/// Preimage hash.
			hash: PreimageHash,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Value too low
		ValueLow,
		/// Proposal does not exist
		ProposalMissing,
		/// Cannot cancel the same proposal twice
		AlreadyCanceled,
		/// Proposal already made
		DuplicateProposal,
		/// Proposal still blacklisted
		ProposalBlacklisted,
		/// Next external proposal not simple majority
		NotSimpleMajority,
		/// Invalid hash
		InvalidHash,
		/// No external proposal
		NoProposal,
		/// Identity may not veto a proposal twice
		AlreadyVetoed,
		/// Vote given for invalid referendum
		ReferendumInvalid,
		/// No proposals waiting
		NoneWaiting,
		/// The given account did not vote on the referendum.
		NotVoter,
		/// The actor has no permission to conduct the action.
		NoPermission,
		/// The account is already delegating.
		AlreadyDelegating,
		/// Too high a balance was provided that the account cannot afford.
		InsufficientFunds,
		/// The account is not currently delegating.
		NotDelegating,
		/// The account currently has votes attached to it and the operation cannot succeed until
		/// these are removed, either through `unvote` or `reap_vote`.
		VotesExist,
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
			info_hash: Option<InfoHash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure curator not existing yet


		}

		/// Updating a curator legal info
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn update_curator(
			origin: OriginFor<T>,
			info_hash: Option<InfoHash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure existing
		}



		/// Curator propose a staking pool
		///
		/// The dispatch origin of this call must be _Signed_ and the sender must
		/// have funds to cover the deposit.
		///
		/// - `proposal_hash`: The hash of the proposal preimage.
		/// - `value`: The amount of deposit (must be at least `MinimumDeposit`).
		///
		/// Emits `Proposed`.
		#[pallet::call_index(0)]
		#[pallet::weight(W{195_000_000})]
		pub fn propose_staking_pool(
			origin: OriginFor<T>,
			pool_setup: PoolSetting<BlockNumberFor<T>, BalanceOf<T>>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;





			ensure!(value >= T::MinimumDeposit::get(), Error::<T>::ValueLow);

			let index = Self::public_prop_count();
			let real_prop_count = PublicProps::<T>::decode_len().unwrap_or(0) as u32;
			let max_proposals = T::MaxProposals::get();
			ensure!(real_prop_count < max_proposals, Error::<T>::TooMany);
			let proposal_hash = proposal.hash();

			if let Some((until, _)) = <Blacklist<T>>::get(proposal_hash) {
				ensure!(
					<frame_system::Pallet<T>>::block_number() >= until,
					Error::<T>::ProposalBlacklisted,
				);
			}

			T::Currency::reserve(&who, value)?;

			let depositors = BoundedVec::<_, T::MaxDeposits>::truncate_from(vec![who.clone()]);
			DepositOf::<T>::insert(index, (depositors, value));

			PublicPropCount::<T>::put(index + 1);

			PublicProps::<T>::try_append((index, proposal, who))
				.map_err(|_| Error::<T>::TooMany)?;

			Self::deposit_event(Event::<T>::Proposed { proposal_index: index, deposit: value });
			Ok(())
		}

		/// Signals agreement with a particular proposal.
		///
		/// The dispatch origin of this call must be _Signed_ and the sender
		/// must have funds to cover the deposit, equal to the original deposit.
		///
		/// - `proposal`: The index of the proposal to second.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::second())]
		pub fn second(
			origin: OriginFor<T>,
			#[pallet::compact] proposal: PropIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let seconds = Self::len_of_deposit_of(proposal).ok_or(Error::<T>::ProposalMissing)?;
			ensure!(seconds < T::MaxDeposits::get(), Error::<T>::TooMany);
			let mut deposit = Self::deposit_of(proposal).ok_or(Error::<T>::ProposalMissing)?;
			T::Currency::reserve(&who, deposit.1)?;
			let ok = deposit.0.try_push(who.clone()).is_ok();
			debug_assert!(ok, "`seconds` is below static limit; `try_insert` should succeed; qed");
			<DepositOf<T>>::insert(proposal, deposit);
			Self::deposit_event(Event::<T>::Seconded { seconder: who, prop_index: proposal });
			Ok(())
		}
	}
}
