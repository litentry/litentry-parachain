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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use core_primitives::{Identity, Intent, MemberAccount, OmniAccountConverter};
pub use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	traits::{IsSubType, UnfilteredDispatchable},
};
use frame_system::pallet_prelude::*;
use sp_core::H256;
use sp_runtime::traits::Dispatchable;
use sp_std::boxed::Box;
use sp_std::vec::Vec;

pub type MemberCount = u32;

// Customized origin for this pallet, to:
// 1. to decouple `TEECallOrigin` and extrinsic that should be sent from `OmniAccount` origin only
// 2. allow other pallets to specify ensure_origin using this origin
// 3. leave room for more delicate control over OmniAccount in the future (e.g. multisig-like control)
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub enum RawOrigin<AccountId> {
	// dispatched from OmniAccount T::AccountId
	OmniAccount(AccountId),
	// dispatched by a given number of members of the AccountStore from a given total
	OmniAccountMembers(AccountId, MemberCount, MemberCount),
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
	pub enum IntentExecutionResult {
		Success,
		Failure,
	}

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The runtime origin type
		type RuntimeOrigin: From<RawOrigin<Self::AccountId>>
			+ From<frame_system::RawOrigin<Self::AccountId>>;

		/// The overarching call type
		type RuntimeCall: Parameter
			+ Dispatchable<
				RuntimeOrigin = <Self as Config>::RuntimeOrigin,
				PostInfo = PostDispatchInfo,
			> + GetDispatchInfo
			+ From<frame_system::Call<Self>>
			+ UnfilteredDispatchable<RuntimeOrigin = <Self as Config>::RuntimeOrigin>
			+ IsSubType<Call<Self>>
			+ IsType<<Self as frame_system::Config>::RuntimeCall>;

		/// The event type of this pallet
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The origin that represents the off-chain worker
		type TEECallOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;

		/// The maximum number of accounts that an AccountGraph can have
		#[pallet::constant]
		type MaxAccountStoreLength: Get<MemberCount>;

		/// The origin that represents the customised OmniAccount type
		type OmniAccountOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Self::AccountId,
		>;

		/// Convert an `Identity` to OmniAccount type
		type OmniAccountConverter: OmniAccountConverter<OmniAccount = Self::AccountId>;

		type SetOmniExecutorOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
	}

	pub type MemberAccounts<T> = BoundedVec<MemberAccount, <T as Config>::MaxAccountStoreLength>;

	#[pallet::origin]
	pub type Origin<T> = RawOrigin<<T as frame_system::Config>::AccountId>;

	/// A map between OmniAccount and its MemberAccounts (a bounded vector of MemberAccount)
	#[pallet::storage]
	#[pallet::getter(fn account_store)]
	pub type AccountStore<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = MemberAccounts<T>>;

	/// A map between hash of MemberAccount and its belonging OmniAccount
	#[pallet::storage]
	pub type MemberAccountHash<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn omni_executor)]
	pub type OmniExecutor<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An account store is created
		AccountStoreCreated { who: T::AccountId },
		/// Some member account is added
		AccountAdded { who: T::AccountId, member_account_hash: H256 },
		/// Some member accounts are removed
		AccountRemoved { who: T::AccountId, member_account_hashes: Vec<H256> },
		/// Some member account is made public
		AccountMadePublic { who: T::AccountId, member_account_hash: H256 },
		/// An account store is updated
		AccountStoreUpdated { who: T::AccountId, account_store: MemberAccounts<T> },
		/// Some call is dispatched as omni-account origin
		DispatchedAsOmniAccount { who: T::AccountId, result: DispatchResult },
		/// Some call is dispatched as signed origin
		DispatchedAsSigned { who: T::AccountId, result: DispatchResult },
		/// Intent is requested
		IntentRequested { who: T::AccountId, intent: Intent },
		/// Intent is executed
		IntentExecuted { who: T::AccountId, intent: Intent, result: IntentExecutionResult },
		/// Omni executor is set
		OmniExecutorSet { omni_executor: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AccountAlreadyAdded,
		AccountStoreLenLimitReached,
		AccountNotFound,
		InvalidAccount,
		UnknownAccountStore,
		EmptyAccount,
		RequireOmniExecutor,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// dispatch the `call` as RawOrigin::OmniAccount
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn dispatch_as_omni_account(
			origin: OriginFor<T>,
			member_account_hash: H256,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let omni_account = MemberAccountHash::<T>::get(member_account_hash)
				.ok_or(Error::<T>::AccountNotFound)?;
			let result = call.dispatch(RawOrigin::OmniAccount(omni_account.clone()).into());
			Self::deposit_event(Event::DispatchedAsOmniAccount {
				who: omni_account,
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			Ok(Pays::No.into())
		}

		// dispatch the `call` as the standard (frame_system) signed origin
		// TODO: what about other customised origin like collective?
		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn dispatch_as_signed(
			origin: OriginFor<T>,
			member_account_hash: H256,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let omni_account = MemberAccountHash::<T>::get(member_account_hash)
				.ok_or(Error::<T>::AccountNotFound)?;
			let result: Result<
				PostDispatchInfo,
				sp_runtime::DispatchErrorWithPostInfo<PostDispatchInfo>,
			> = call.dispatch(frame_system::RawOrigin::Signed(omni_account.clone()).into());
			Self::deposit_event(Event::DispatchedAsSigned {
				who: omni_account,
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			Ok(Pays::No.into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn create_account_store(
			origin: OriginFor<T>,
			identity: Identity,
		) -> DispatchResultWithPostInfo {
			// initial creation request has to come from `TEECallOrigin`
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let _ = Self::do_create_account_store(identity)?;
			Ok(Pays::No.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn add_account(
			origin: OriginFor<T>,
			member_account: MemberAccount, // account to be added
		) -> DispatchResult {
			// mutation of AccountStore requires `OmniAccountOrigin`, same as "remove" and "publicize"
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(
				!MemberAccountHash::<T>::contains_key(member_account.hash()),
				Error::<T>::AccountAlreadyAdded
			);

			let mut member_accounts =
				AccountStore::<T>::get(&who).ok_or(Error::<T>::UnknownAccountStore)?;

			let hash = member_account.hash();
			member_accounts
				.try_push(member_account)
				.map_err(|_| Error::<T>::AccountStoreLenLimitReached)?;

			MemberAccountHash::<T>::insert(hash, who.clone());
			AccountStore::<T>::insert(who.clone(), member_accounts.clone());

			Self::deposit_event(Event::AccountAdded {
				who: who.clone(),
				member_account_hash: hash,
			});
			Self::deposit_event(Event::AccountStoreUpdated { who, account_store: member_accounts });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_accounts(
			origin: OriginFor<T>,
			member_account_hashes: Vec<H256>,
		) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(!member_account_hashes.is_empty(), Error::<T>::EmptyAccount);

			// TODO: shall we verify if MemberAccountHash's value is actually `who`?

			let mut member_accounts =
				AccountStore::<T>::get(&who).ok_or(Error::<T>::UnknownAccountStore)?;

			member_accounts.retain(|member| {
				if member_account_hashes.contains(&member.hash()) {
					MemberAccountHash::<T>::remove(member.hash());
					false
				} else {
					true
				}
			});

			if member_accounts.is_empty() {
				AccountStore::<T>::remove(&who);
			} else {
				AccountStore::<T>::insert(who.clone(), member_accounts.clone());
			}

			Self::deposit_event(Event::AccountRemoved { who: who.clone(), member_account_hashes });
			Self::deposit_event(Event::AccountStoreUpdated { who, account_store: member_accounts });

			Ok(())
		}

		/// make a member account public in the AccountStore
		/// we force `Identity` type to avoid misuse and additional check
		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn publicize_account(origin: OriginFor<T>, member_account: Identity) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;

			let hash = member_account.hash();
			let mut member_accounts =
				AccountStore::<T>::get(&who).ok_or(Error::<T>::UnknownAccountStore)?;
			let m = member_accounts
				.iter_mut()
				.find(|member| member.hash() == hash)
				.ok_or(Error::<T>::AccountNotFound)?;
			*m = member_account.into();

			AccountStore::<T>::insert(who.clone(), member_accounts.clone());

			Self::deposit_event(Event::AccountMadePublic {
				who: who.clone(),
				member_account_hash: hash,
			});
			Self::deposit_event(Event::AccountStoreUpdated { who, account_store: member_accounts });

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn request_intent(origin: OriginFor<T>, intent: Intent) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IntentRequested { who, intent });
			Ok(())
		}

		/// temporary extrinsic to upload the existing IDGraph from the worker onto chain
		#[pallet::call_index(7)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn update_account_store_by_one(
			origin: OriginFor<T>,
			who: Identity,
			member_account: MemberAccount,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin.clone())?;

			let who_account = T::OmniAccountConverter::convert(&who);

			let mut member_accounts = match AccountStore::<T>::get(&who_account) {
				Some(s) => s,
				None => Self::do_create_account_store(who)?,
			};

			if !member_accounts.contains(&member_account) {
				member_accounts
					.try_push(member_account.clone())
					.map_err(|_| Error::<T>::AccountStoreLenLimitReached)?;
			}

			MemberAccountHash::<T>::insert(member_account.hash(), who_account.clone());
			AccountStore::<T>::insert(who_account.clone(), member_accounts.clone());
			Self::deposit_event(Event::AccountStoreUpdated {
				who: who_account,
				account_store: member_accounts,
			});

			Ok(Pays::No.into())
		}

		#[pallet::call_index(8)]
		#[pallet::weight((195_000_000, DispatchClass::Normal,  Pays::No))]
		pub fn intent_executed(
			origin: OriginFor<T>,
			who: T::AccountId,
			intent: Intent,
			result: IntentExecutionResult,
		) -> DispatchResult {
			Self::ensure_omni_executor(origin)?;
			Self::deposit_event(Event::IntentExecuted { who, intent, result });
			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight((195_000_000, DispatchClass::Normal,  Pays::No))]
		pub fn set_omni_executor(
			origin: OriginFor<T>,
			new_omni_executor: T::AccountId,
		) -> DispatchResult {
			T::SetOmniExecutorOrigin::ensure_origin(origin)?;
			OmniExecutor::<T>::put(new_omni_executor.clone());
			Self::deposit_event(Event::OmniExecutorSet { omni_executor: new_omni_executor });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_create_account_store(identity: Identity) -> Result<MemberAccounts<T>, Error<T>> {
			let hash = identity.hash();
			let omni_account = T::OmniAccountConverter::convert(&identity);

			ensure!(!MemberAccountHash::<T>::contains_key(hash), Error::<T>::AccountAlreadyAdded);

			let mut member_accounts: MemberAccounts<T> = BoundedVec::new();
			member_accounts
				.try_push(identity.into())
				.map_err(|_| Error::<T>::AccountStoreLenLimitReached)?;

			MemberAccountHash::<T>::insert(hash, omni_account.clone());
			AccountStore::<T>::insert(omni_account.clone(), member_accounts.clone());

			Self::deposit_event(Event::AccountStoreCreated { who: omni_account.clone() });
			Self::deposit_event(Event::AccountStoreUpdated {
				who: omni_account,
				account_store: member_accounts.clone(),
			});

			Ok(member_accounts)
		}

		fn ensure_omni_executor(origin: OriginFor<T>) -> DispatchResult {
			ensure!(
				Some(ensure_signed(origin)?) == Self::omni_executor(),
				Error::<T>::RequireOmniExecutor
			);
			Ok(())
		}
	}
}

pub struct EnsureOmniAccount<AccountId>(PhantomData<AccountId>);
impl<O: Into<Result<RawOrigin<AccountId>, O>> + From<RawOrigin<AccountId>>, AccountId: Decode>
	EnsureOrigin<O> for EnsureOmniAccount<AccountId>
{
	type Success = AccountId;
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			RawOrigin::OmniAccount(id) => Ok(id),
			r => Err(O::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<O, ()> {
		let zero_account_id =
			AccountId::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
				.expect("infinite length input; no invalid inputs for type; qed");
		Ok(O::from(RawOrigin::OmniAccount(zero_account_id)))
	}
}
