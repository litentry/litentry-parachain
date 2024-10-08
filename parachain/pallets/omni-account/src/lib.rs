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

pub use core_primitives::{Identity, MemberIdentity};
pub use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	traits::{IsSubType, UnfilteredDispatchable},
};
use frame_system::pallet_prelude::*;
use sp_core::H256;
use sp_core_hashing::blake2_256;
use sp_runtime::traits::Dispatchable;
use sp_std::boxed::Box;
use sp_std::vec::Vec;

pub type MemberCount = u32;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MemberAccount {
	pub id: MemberIdentity,
	pub hash: H256,
}

pub trait AccountIdConverter<T: Config> {
	fn convert(identity: &Identity) -> Option<T::AccountId>;
}

pub trait GetAccountStoreHash {
	fn hash(&self) -> H256;
}

impl<T> GetAccountStoreHash for BoundedVec<MemberAccount, T> {
	fn hash(&self) -> H256 {
		let hashes: Vec<H256> = self.iter().map(|member| member.hash).collect();
		hashes.using_encoded(blake2_256).into()
	}
}

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

		/// AccountId converter
		type AccountIdConverter: AccountIdConverter<Self>;

		/// The origin that represents the customised OmniAccount type
		type OmniAccountOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Self::AccountId,
		>;
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

	/// A map between OmniAccount and hash of its AccountStore
	#[pallet::storage]
	#[pallet::getter(fn account_store_hash)]
	pub type AccountStoreHash<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = H256>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some member account is added
		AccountAdded { who: T::AccountId, member_account_hash: H256 },
		/// Some member accounts are removed
		AccountRemoved { who: T::AccountId, member_account_hashes: Vec<H256> },
		/// Some member account is made public
		AccountMadePublic { who: T::AccountId, member_account_hash: H256 },
		/// Some call is dispatched as omni-account origin
		DispatchedAsOmniAccount { who: T::AccountId, result: DispatchResult },
		/// Some call is dispatched as signed origin
		DispatchedAsSigned { who: T::AccountId, result: DispatchResult },
	}

	#[pallet::error]
	pub enum Error<T> {
		AccountAlreadyAdded,
		AccountStoreLenLimitReached,
		AccountNotFound,
		InvalidAccount,
		UnknownAccountStore,
		AccountIsPrivate,
		EmptyAccount,
		AccountStoreHashMismatch,
		AccountStoreHashMissing,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// dispatch the `call` as RawOrigin::OmniAccount
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn dispatch_as_omni_account(
			origin: OriginFor<T>,
			account_hash: H256,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let omni_account =
				MemberAccountHash::<T>::get(account_hash).ok_or(Error::<T>::AccountNotFound)?;
			let result = call.dispatch(RawOrigin::OmniAccount(omni_account.clone()).into());
			Self::deposit_event(Event::DispatchedAsOmniAccount {
				who: omni_account,
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			Ok(())
		}

		// dispatch the `call` as the standard (frame_system) signed origin
		// TODO: what about other customised origin like collective?
		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn dispatch_as_signed(
			origin: OriginFor<T>,
			account_hash: H256,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let omni_account =
				MemberAccountHash::<T>::get(account_hash).ok_or(Error::<T>::AccountNotFound)?;
			let result =
				call.dispatch(frame_system::RawOrigin::Signed(omni_account.clone()).into());
			Self::deposit_event(Event::DispatchedAsSigned {
				who: omni_account,
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn add_account(
			origin: OriginFor<T>,
			who: Identity,
			member_account: MemberAccount,
			maybe_account_store_hash: Option<H256>,
		) -> DispatchResult {
			// We can't use `T::OmniAccountOrigin` here as the ownership of member account needs to
			// be firstly validated by the TEE-worker before dispatching the extrinsic
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				!MemberAccountHash::<T>::contains_key(member_account.hash),
				Error::<T>::AccountAlreadyAdded
			);
			let who_account_id = match T::AccountIdConverter::convert(&who) {
				Some(account_id) => account_id,
				None => return Err(Error::<T>::InvalidAccount.into()),
			};
			let hash = member_account.hash;
			let mut account_store = Self::get_or_create_account_store(
				who.clone(),
				who_account_id.clone(),
				maybe_account_store_hash,
			)?;
			account_store
				.try_push(member_account)
				.map_err(|_| Error::<T>::AccountStoreLenLimitReached)?;

			MemberAccountHash::<T>::insert(hash, who_account_id.clone());
			AccountStoreHash::<T>::insert(who_account_id.clone(), account_store.hash());
			AccountStore::<T>::insert(who_account_id.clone(), account_store);

			Self::deposit_event(Event::AccountAdded {
				who: who_account_id,
				member_account_hash: hash,
			});

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_accounts(
			origin: OriginFor<T>,
			member_account_hashes: Vec<H256>,
		) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(!member_account_hashes.is_empty(), Error::<T>::EmptyAccount);

			let mut member_accounts =
				AccountStore::<T>::get(&who).ok_or(Error::<T>::UnknownAccountStore)?;

			member_accounts.retain(|member| {
				if member_account_hashes.contains(&member.hash) {
					MemberAccountHash::<T>::remove(member.hash);
					false
				} else {
					true
				}
			});

			if member_accounts.is_empty() {
				AccountStore::<T>::remove(&who);
			} else {
				AccountStore::<T>::insert(who.clone(), member_accounts);
			}

			Self::deposit_event(Event::AccountRemoved { who, member_account_hashes });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn publicize_account(
			origin: OriginFor<T>,
			member_account_hash: H256,
			public_identity: MemberIdentity,
		) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(public_identity.is_public(), Error::<T>::AccountIsPrivate);

			let mut member_accounts =
				AccountStore::<T>::get(&who).ok_or(Error::<T>::UnknownAccountStore)?;
			let member_account = member_accounts
				.iter_mut()
				.find(|member| member.hash == member_account_hash)
				.ok_or(Error::<T>::AccountNotFound)?;
			member_account.id = public_identity;

			AccountStore::<T>::insert(who.clone(), member_accounts);

			Self::deposit_event(Event::AccountMadePublic { who, member_account_hash });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_or_create_account_store(
			who: Identity,
			who_account_id: T::AccountId,
			maybe_account_store_hash: Option<H256>,
		) -> Result<MemberAccounts<T>, Error<T>> {
			match AccountStore::<T>::get(&who_account_id) {
				Some(member_accounts) => {
					Self::verify_account_store_hash(&who_account_id, maybe_account_store_hash)?;
					Ok(member_accounts)
				},
				None => Self::create_account_store(who, who_account_id),
			}
		}

		fn verify_account_store_hash(
			who: &T::AccountId,
			maybe_account_store_hash: Option<H256>,
		) -> Result<(), Error<T>> {
			let current_account_store_hash =
				AccountStoreHash::<T>::get(who).ok_or(Error::<T>::AccountStoreHashMissing)?;
			match maybe_account_store_hash {
				Some(h) => {
					ensure!(current_account_store_hash == h, Error::<T>::AccountStoreHashMismatch);
				},
				None => return Err(Error::<T>::AccountStoreHashMissing),
			}

			Ok(())
		}

		fn create_account_store(
			owner_identity: Identity,
			owner_account_id: T::AccountId,
		) -> Result<MemberAccounts<T>, Error<T>> {
			let owner_identity_hash = owner_identity.hash();
			if MemberAccountHash::<T>::contains_key(owner_identity_hash) {
				return Err(Error::<T>::AccountAlreadyAdded);
			}
			let mut member_accounts: MemberAccounts<T> = BoundedVec::new();
			member_accounts
				.try_push(MemberAccount {
					id: MemberIdentity::from(owner_identity.clone()),
					hash: owner_identity_hash,
				})
				.map_err(|_| Error::<T>::AccountStoreLenLimitReached)?;
			MemberAccountHash::<T>::insert(owner_identity_hash, owner_account_id.clone());
			AccountStore::<T>::insert(owner_account_id.clone(), member_accounts.clone());

			Ok(member_accounts)
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
