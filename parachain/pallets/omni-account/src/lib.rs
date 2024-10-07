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

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct IDGraphMember {
	pub id: MemberIdentity,
	pub hash: H256,
}

pub trait AccountIdConverter<T: Config> {
	fn convert(identity: &Identity) -> Option<T::AccountId>;
}

pub trait IDGraphHash {
	fn graph_hash(&self) -> H256;
}

impl<T> IDGraphHash for BoundedVec<IDGraphMember, T> {
	fn graph_hash(&self) -> H256 {
		let id_graph_members_hashes: Vec<H256> = self.iter().map(|member| member.hash).collect();
		H256::from(blake2_256(&id_graph_members_hashes.encode()))
	}
}

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
	// dispatched by a given number of members of the OmniAccount IDGraph from a given total
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
		/// The maximum number of identities an id graph can have.
		#[pallet::constant]
		type MaxIDGraphLength: Get<u32>;
		/// AccountId converter
		type AccountIdConverter: AccountIdConverter<Self>;
		/// The origin that represents the customised OmniAccount type
		type OmniAccountOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Self::AccountId,
		>;
	}

	pub type IDGraph<T> = BoundedVec<IDGraphMember, <T as Config>::MaxIDGraphLength>;

	#[pallet::origin]
	pub type Origin<T> = RawOrigin<<T as frame_system::Config>::AccountId>;

	#[pallet::storage]
	pub type LinkedIdentityHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = IDGraph<T>>;

	#[pallet::storage]
	#[pallet::getter(fn id_graph_hashes)]
	pub type IDGraphHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = H256>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Identity linked
		IdentityLinked { who: T::AccountId, identity: H256 },
		/// Identity remove
		IdentityRemoved { who: T::AccountId, identity_hashes: Vec<H256> },
		/// Identity made public
		IdentityMadePublic { who: T::AccountId, identity_hash: H256 },
		/// Some call is dispatched as omni-account origin
		DispatchedAsOmniAccount { who: T::AccountId, result: DispatchResult },
		/// Some call is dispatched as signed origin
		DispatchedAsSigned { who: T::AccountId, result: DispatchResult },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Identity is already linked
		IdentityAlreadyLinked,
		/// IDGraph len limit reached
		IDGraphLenLimitReached,
		/// Identity not found
		IdentityNotFound,
		/// Invalid identity
		InvalidIdentity,
		/// IDGraph not found
		UnknownIDGraph,
		/// Identity is private
		IdentityIsPrivate,
		/// Identities empty
		IdentitiesEmpty,
		/// IDGraph hash does not match
		IDGraphHashMismatch,
		/// Missing IDGraph hash
		IDGraphHashMissing,
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
				LinkedIdentityHashes::<T>::get(account_hash).ok_or(Error::<T>::IdentityNotFound)?;
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
				LinkedIdentityHashes::<T>::get(account_hash).ok_or(Error::<T>::IdentityNotFound)?;
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
		pub fn link_identity(
			origin: OriginFor<T>,
			who: Identity,
			member_account: IDGraphMember,
			maybe_id_graph_hash: Option<H256>,
		) -> DispatchResult {
			// We can't use `T::OmniAccountOrigin` here as the ownership of member account needs to
			// be firstly validated by the TEE-worker before dispatching the extrinsic
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				!LinkedIdentityHashes::<T>::contains_key(member_account.hash),
				Error::<T>::IdentityAlreadyLinked
			);
			let who_account_id = match T::AccountIdConverter::convert(&who) {
				Some(account_id) => account_id,
				None => return Err(Error::<T>::InvalidIdentity.into()),
			};
			let identity_hash = member_account.hash;
			let mut id_graph = Self::get_or_create_id_graph(
				who.clone(),
				who_account_id.clone(),
				maybe_id_graph_hash,
			)?;
			id_graph
				.try_push(member_account)
				.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;

			LinkedIdentityHashes::<T>::insert(identity_hash, who_account_id.clone());
			IDGraphHashes::<T>::insert(who_account_id.clone(), id_graph.graph_hash());
			IDGraphs::<T>::insert(who_account_id.clone(), id_graph);

			Self::deposit_event(Event::IdentityLinked {
				who: who_account_id,
				identity: identity_hash,
			});

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_identities(
			origin: OriginFor<T>,
			identity_hashes: Vec<H256>,
		) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(!identity_hashes.is_empty(), Error::<T>::IdentitiesEmpty);

			let mut id_graph_members =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::UnknownIDGraph)?;

			id_graph_members.retain(|member| {
				if identity_hashes.contains(&member.hash) {
					LinkedIdentityHashes::<T>::remove(member.hash);
					false
				} else {
					true
				}
			});

			if id_graph_members.is_empty() {
				IDGraphs::<T>::remove(&who);
			} else {
				IDGraphs::<T>::insert(who.clone(), id_graph_members);
			}

			Self::deposit_event(Event::IdentityRemoved { who, identity_hashes });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn make_identity_public(
			origin: OriginFor<T>,
			identity_hash: H256,
			public_identity: MemberIdentity,
		) -> DispatchResult {
			let who = T::OmniAccountOrigin::ensure_origin(origin)?;
			ensure!(public_identity.is_public(), Error::<T>::IdentityIsPrivate);

			let mut id_graph_members =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::UnknownIDGraph)?;
			let id_graph_link = id_graph_members
				.iter_mut()
				.find(|member| member.hash == identity_hash)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.id = public_identity;

			IDGraphs::<T>::insert(who.clone(), id_graph_members);

			Self::deposit_event(Event::IdentityMadePublic { who, identity_hash });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_or_create_id_graph(
			who: Identity,
			who_account_id: T::AccountId,
			maybe_id_graph_hash: Option<H256>,
		) -> Result<IDGraph<T>, Error<T>> {
			match IDGraphs::<T>::get(&who_account_id) {
				Some(id_graph_members) => {
					Self::verify_id_graph_hash(&who_account_id, maybe_id_graph_hash)?;
					Ok(id_graph_members)
				},
				None => Self::create_id_graph(who, who_account_id),
			}
		}

		fn verify_id_graph_hash(
			who: &T::AccountId,
			maybe_id_graph_hash: Option<H256>,
		) -> Result<(), Error<T>> {
			let current_id_graph_hash =
				IDGraphHashes::<T>::get(who).ok_or(Error::<T>::IDGraphHashMissing)?;
			match maybe_id_graph_hash {
				Some(id_graph_hash) => {
					ensure!(
						current_id_graph_hash == id_graph_hash,
						Error::<T>::IDGraphHashMismatch
					);
				},
				None => return Err(Error::<T>::IDGraphHashMissing),
			}

			Ok(())
		}

		fn create_id_graph(
			owner_identity: Identity,
			owner_account_id: T::AccountId,
		) -> Result<IDGraph<T>, Error<T>> {
			let owner_identity_hash =
				owner_identity.hash().map_err(|_| Error::<T>::InvalidIdentity)?;
			if LinkedIdentityHashes::<T>::contains_key(owner_identity_hash) {
				return Err(Error::<T>::IdentityAlreadyLinked);
			}
			let mut id_graph_members: IDGraph<T> = BoundedVec::new();
			id_graph_members
				.try_push(IDGraphMember {
					id: MemberIdentity::from(owner_identity.clone()),
					hash: owner_identity_hash,
				})
				.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
			LinkedIdentityHashes::<T>::insert(owner_identity_hash, owner_account_id.clone());
			IDGraphs::<T>::insert(owner_account_id.clone(), id_graph_members.clone());

			Ok(id_graph_members)
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
