//! # Pallet halving-mint
//!
//! This pallet mints the (native) token in a halving way.
//!
//! It will be parameterized with the total issuance count and halving interval (in blocks),
//! The minted token is deposited to the `beneficiary` account, which should be a privated
//! account derived from the PalletId(similar to treasury). There's a trait `OnTokenMinted`
//! to hook the callback into other pallet.
//!
//! The main parameters:
//! - total issuance
//! - halving interval
//! - beneficiary account
//! are defined as runtime constants. It implies that once onboarded, they can be changed
//! only by runtime upgrade. Thus it has a stronger guarantee in comparison to extrinsics.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use frame_support::traits::Currency;
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod traits;
pub use traits::OnTokenMinted;

pub type BalanceOf<T, I = ()> =
	<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// an on/off flag, used in both `MintState` and `OnTokenMintedState`
#[derive(
	PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, Encode, Decode, Debug, TypeInfo,
)]
pub enum State {
	#[default]
	#[codec(index = 0)]
	Stopped,
	#[codec(index = 1)]
	Running,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{ReservableCurrency, StorageVersion},
		PalletId,
	};
	use frame_system::pallet_prelude::{BlockNumberFor, *};
	use sp_runtime::{
		traits::{AccountIdConversion, One, Zero},
		Saturating,
	};

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The origin to control the minting configuration
		type ManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The total issuance of the (native) token
		#[pallet::constant]
		type TotalIssuance: Get<BalanceOf<Self, I>>;
		/// Halving internal in blocks, we force u32 type, BlockNumberFor<T> implements
		/// AtLeast32BitUnsigned so it's safe
		#[pallet::constant]
		type HalvingInterval: Get<u32>;
		/// The beneficiary PalletId, used for deriving its sovereign AccountId
		#[pallet::constant]
		type BeneficiaryId: Get<PalletId>;
		/// Hook for other pallets to deal with OnTokenMinted event
		type OnTokenMinted: OnTokenMinted<Self::AccountId, BalanceOf<Self, I>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		MintStateChanged { new_state: State },
		OnTokenMintedStateChanged { new_state: State },
		MintStarted { start_block: BlockNumberFor<T> },
		Minted { to: T::AccountId, amount: BalanceOf<T, I> },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		MintStateUnchanged,
		OnTokenMintedStateUnchanged,
		MintAlreadyStarted,
		MintNotStarted,
		StartBlockTooEarly,
		SkippedBlocksOverflow,
	}

	#[pallet::storage]
	#[pallet::getter(fn mint_state)]
	pub type MintState<T: Config<I>, I: 'static = ()> = StorageValue<_, State, ValueQuery>;

	/// If the `OnTokenMinted` callback is stopped or not
	#[pallet::storage]
	#[pallet::getter(fn on_token_minted_state)]
	pub type OnTokenMintedState<T: Config<I>, I: 'static = ()> = StorageValue<_, State, ValueQuery>;

	/// the block number from which the minting is started, `None` means minting
	/// is not started yet
	#[pallet::storage]
	#[pallet::getter(fn start_block)]
	pub type StartBlock<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BlockNumberFor<T>, OptionQuery>;

	/// Number of skipped blocks being counted when `MintState` is `Stopped`
	#[pallet::storage]
	#[pallet::getter(fn skipped_blocks)]
	pub type SkippedBlocks<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub mint_state: State,
		pub on_token_minted_state: State,
		pub start_block: Option<BlockNumberFor<T>>,
		#[serde(skip)]
		pub phantom: PhantomData<I>,
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self {
				mint_state: State::Stopped,
				on_token_minted_state: State::Stopped,
				start_block: None,
				phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {
			MintState::<T, I>::put(self.mint_state);
			OnTokenMintedState::<T, I>::put(self.on_token_minted_state);
			if let Some(n) = self.start_block {
				StartBlock::<T, I>::put(n);
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		fn on_initialize(now: BlockNumberFor<T>) -> Weight {
			let mut weight = Weight::zero();
			if let Some(start_block) = Self::start_block() {
				if Self::mint_state() == State::Running {
					let skipped_blocks = Self::skipped_blocks();
					// 3 reads: `mint_state`, `start_block`, `skipped_blocks`
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(3, 0));

					if now < start_block.saturating_add(skipped_blocks) {
						return weight;
					}

					let halving_interval = T::HalvingInterval::get();

					// calculate the amount of initially minted tokens before first halving
					let mut minted = T::TotalIssuance::get() / (halving_interval * 2).into();
					// halving round index
					let halving_round = (now - start_block.saturating_add(skipped_blocks))
						/ halving_interval.into();
					// beneficiary account
					let to = Self::beneficiary_account();

					// 2 reads: `total_issuance`, `halving_interval`
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(2, 0));

					// if we want to use bit shift, we need to:
					//   1. know the overlfow limit similar to what bitcoin has: `if (halvings >=
					//      64) return 0;` so 127 for u128
					//   2. coerce the `halving_round` to u32
					// but both `halving_round` and `minted` are associated types that need to be
					// defined during runtime binding thus plain division is used
					let mut i = BlockNumberFor::<T>::zero();
					while i < halving_round {
						minted /= 2u32.into();
						i += BlockNumberFor::<T>::one();
					}

					// theoreticlaly we can deal with the minted tokens directly in the trait impl
					// pallet, without depositing to an account first.
					// but the purpose of having the extra logic is to make sure the tokens are
					// minted to the beneficiary account, regardless of what happens callback. Even
					// if the callback errors out, it's guaranteed that the tokens are
					// already minted (and stored on an account), which resonates with the "fair
					// launch" concept.
					//
					// Also imagine there's no callback impl, in this case the tokens will still be
					// minted and accumulated.
					let _ = T::Currency::deposit_creating(&to, minted);
					Self::deposit_event(Event::Minted { to: to.clone(), amount: minted });
					if Self::on_token_minted_state() == State::Running {
						weight = weight.saturating_add(T::OnTokenMinted::token_minted(to, minted));
					}
					// 1 read: `on_token_minted_state`
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 0));
				} else {
					// we should have minted tokens but it's forcibly stopped
					let skipped_blocks =
						Self::skipped_blocks().saturating_add(BlockNumberFor::<T>::one());
					SkippedBlocks::<T, I>::put(skipped_blocks);
					// 1 read, 1 write: `SkippedBlocks`
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				}
			}
			weight
		}
	}

	// TODO: benchmarking and WeightInfo
	// IMO it's not **that** bad to use constant weight for extrinsics now as they are simple calls
	// and should only be called once or very few times.
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Set the state of the minting, it essentially "pause" and "resume" the minting process.
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn set_mint_state(origin: OriginFor<T>, state: State) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;
			ensure!(StartBlock::<T, I>::get().is_some(), Error::<T, I>::MintNotStarted);
			ensure!(state != Self::mint_state(), Error::<T, I>::MintStateUnchanged);
			MintState::<T, I>::put(state);
			Self::deposit_event(Event::MintStateChanged { new_state: state });
			Ok(Pays::No.into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn set_on_token_minted_state(
			origin: OriginFor<T>,
			state: State,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;
			ensure!(StartBlock::<T, I>::get().is_some(), Error::<T, I>::MintNotStarted);
			ensure!(
				state != Self::on_token_minted_state(),
				Error::<T, I>::OnTokenMintedStateUnchanged
			);
			OnTokenMintedState::<T, I>::put(state);
			Self::deposit_event(Event::OnTokenMintedStateChanged { new_state: state });
			Ok(Pays::No.into())
		}

		/// Start mint from next block, this is the earliest block the next minting can happen,
		/// as we already missed the intialization of current block and we don't do retroactive
		/// minting
		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn start_mint_from_next_block(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Self::start_mint_from_block(
				origin,
				frame_system::Pallet::<T>::block_number() + BlockNumberFor::<T>::one(),
			)
		}

		/// Start mint from a given block that is larger than the current block number
		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn start_mint_from_block(
			origin: OriginFor<T>,
			start_block: BlockNumberFor<T>,
		) -> DispatchResultWithPostInfo {
			T::ManagerOrigin::ensure_origin(origin)?;
			ensure!(StartBlock::<T, I>::get().is_none(), Error::<T, I>::MintAlreadyStarted);
			ensure!(
				start_block > frame_system::Pallet::<T>::block_number(),
				Error::<T, I>::StartBlockTooEarly
			);
			MintState::<T, I>::put(State::Running);
			OnTokenMintedState::<T, I>::put(State::Running);
			StartBlock::<T, I>::put(start_block);
			Self::deposit_event(Event::MintStarted { start_block });
			Ok(Pays::No.into())
		}
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		pub fn beneficiary_account() -> T::AccountId {
			T::BeneficiaryId::get().into_account_truncating()
		}
	}
}
