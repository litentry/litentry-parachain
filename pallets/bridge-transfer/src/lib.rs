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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use frame_support::{
		pallet_prelude::*,
		traits::{fungible::Mutate, Currency, SortedMembers, StorageVersion},
		transactional,
	};
	use frame_system::pallet_prelude::*;
	use pallet_parachain_staking::IssuanceAdapter;
	use sp_runtime::traits::{BadOrigin, CheckedAdd, CheckedSub};
	use sp_std::vec::Vec;

	pub use pallet_bridge as bridge;

	type ResourceId = bridge::ResourceId;

	type BalanceOf<T> = <<T as bridge::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + bridge::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Specifies the origin check provided by the bridge for calls that can only be called by
		/// the bridge pallet
		type BridgeOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>;

		/// The priviledged accounts to call the transfer_native
		type TransferNativeMembers: SortedMembers<Self::AccountId>;

		/// The privileged origin to call update_maximum_issuance
		type SetMaximumIssuanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		#[pallet::constant]
		type NativeTokenResourceId: Get<ResourceId>;

		#[pallet::constant]
		type DefaultMaximumIssuance: Get<bridge::BalanceOf<Self>>;

		#[pallet::constant]
		// In parachain local decimal format
		type ExternalTotalIssuance: Get<bridge::BalanceOf<Self>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// MaximumIssuance was changed
		MaximumIssuanceChanged { old_value: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidCommand,
		InvalidResourceId,
		ReachMaximumSupply,
		OverFlow,
	}

	#[pallet::storage]
	#[pallet::getter(fn bridge_balances)]
	pub type BridgeBalances<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		bridge::ResourceId,
		Twox64Concat,
		T::AccountId,
		bridge::BalanceOf<T>,
	>;

	#[pallet::type_value]
	pub fn DefaultExternalBalances<T: Config>() -> bridge::BalanceOf<T> {
		T::ExternalTotalIssuance::get()
			.checked_sub(&<<T as bridge::Config>::Currency as Currency<
				<T as frame_system::Config>::AccountId,
			>>::total_issuance())
			.map_or_else(|| 0u32.into(), |v| v)
	}

	#[pallet::storage]
	#[pallet::getter(fn external_balances)]
	pub type ExternalBalances<T: Config> =
		StorageValue<_, bridge::BalanceOf<T>, ValueQuery, DefaultExternalBalances<T>>;

	#[pallet::storage]
	#[pallet::getter(fn maximum_issuance)]
	pub type MaximumIssuance<T: Config> =
		StorageValue<_, bridge::BalanceOf<T>, ValueQuery, T::DefaultMaximumIssuance>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfers some amount of the native token to some recipient on a (whitelisted)
		/// destination chain.
		#[pallet::weight(<T as Config>::WeightInfo::transfer_native())]
		#[transactional]
		pub fn transfer_native(
			origin: OriginFor<T>,
			amount: bridge::BalanceOf<T>,
			recipient: Vec<u8>,
			dest_id: bridge::BridgeChainId,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			ensure!(T::TransferNativeMembers::contains(&source), BadOrigin);
			let resource_id = T::NativeTokenResourceId::get();

			let external_balances =
				<ExternalBalances<T>>::get().checked_add(&amount).ok_or(Error::<T>::OverFlow)?;
			<ExternalBalances<T>>::put(external_balances);

			<bridge::Pallet<T>>::transfer_fungible(source, dest_id, resource_id, recipient, amount)
		}

		/// Executes a simple currency transfer using the bridge account as the source
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: bridge::BalanceOf<T>,
			rid: ResourceId,
		) -> DispatchResult {
			T::BridgeOrigin::ensure_origin(origin)?;

			let total_issuance = <T as bridge::Config>::Currency::total_issuance();
			let new_issuance = total_issuance.checked_add(&amount).ok_or(Error::<T>::OverFlow)?;
			if new_issuance > MaximumIssuance::<T>::get() {
				return Err(Error::<T>::ReachMaximumSupply.into())
			}
			if rid == T::NativeTokenResourceId::get() {
				let external_balances = <ExternalBalances<T>>::get()
					.checked_sub(&amount)
					.ok_or(Error::<T>::OverFlow)?;
				// ERC20 LIT mint
				<T as bridge::Config>::Currency::mint_into(&to, amount)?;
				<ExternalBalances<T>>::put(external_balances);
			} else {
				return Err(Error::<T>::InvalidResourceId.into())
			}
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_maximum_issuance())]
		pub fn set_maximum_issuance(
			origin: OriginFor<T>,
			maximum_issuance: bridge::BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::SetMaximumIssuanceOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::MaximumIssuanceChanged {
				old_value: MaximumIssuance::<T>::get(),
			});
			MaximumIssuance::<T>::set(maximum_issuance);
			Ok(Pays::No.into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::set_external_balances())]
		pub fn set_external_balances(
			origin: OriginFor<T>,
			external_balances: bridge::BalanceOf<T>,
		) -> DispatchResult {
			frame_system::ensure_root(origin)?;
			<ExternalBalances<T>>::put(external_balances);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {}

	impl<T: Config> IssuanceAdapter<BalanceOf<T>> for Pallet<T> {
		fn adapted_total_issuance() -> BalanceOf<T> {
			<ExternalBalances<T>>::get()
		}
	}
}
