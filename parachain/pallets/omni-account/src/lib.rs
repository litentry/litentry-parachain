#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use core_primitives::{Identity, MemberIdentity};
pub use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_core::H256;
use sp_core_hashing::blake2_256;
use sp_std::vec::Vec;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MemberAccount {
	pub id: MemberIdentity,
	pub hash: H256,
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
		/// The event type of this pallet.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The origin which can manage the pallet.
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The maximum number of identities an id graph can have.
		#[pallet::constant]
		type MaxIDGraphLength: Get<u32>;
	}
	pub type IDGraphMembers<T> = BoundedVec<MemberAccount, <T as Config>::MaxIDGraphLength>;

	#[pallet::storage]
	pub type LinkedIdentityHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = ()>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = Identity, Value = IDGraphMembers<T>>;

	#[pallet::storage]
	#[pallet::getter(fn id_graph_hashes)]
	pub type IDGraphHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = Identity, Value = H256>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Identity linked
		IdentityLinked { who: Identity, identity: H256 },
		/// Identity remove
		IdentityRemoved { who: Identity, identity_hashes: Vec<H256> },
		/// Identity made public
		IdentityMadePublic { who: Identity, identity_hash: H256 },
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
		PrimeIdentityInvalid,
		/// Prime identity not found
		PrimeIdentityNotFound,
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
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity_hash: H256,
			identity: MemberIdentity,
			maybe_id_graph_hash: Option<H256>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				!LinkedIdentityHashes::<T>::contains_key(identity_hash),
				Error::<T>::IdentityAlreadyLinked
			);
			let mut id_graph_members = match IDGraphs::<T>::get(&who) {
				Some(id_graph_members) => {
					let current_id_graph_hash =
						IDGraphHashes::<T>::get(&who).ok_or(Error::<T>::PrimeIdentityNotFound)?;
					if let Some(id_graph_hash) = maybe_id_graph_hash {
						ensure!(
							current_id_graph_hash == id_graph_hash,
							Error::<T>::IDGraphHashMismatch
						);
					} else {
						return Err(Error::<T>::IDGraphHashMissing.into());
					}
					id_graph_members
				},
				None => {
					let who_hash = who.hash().map_err(|_| Error::<T>::PrimeIdentityInvalid)?;
					if LinkedIdentityHashes::<T>::contains_key(who_hash) {
						return Err(Error::<T>::IdentityAlreadyLinked.into());
					}
					let mut id_graph_members: IDGraphMembers<T> = BoundedVec::new();
					id_graph_members
						.try_push(MemberAccount {
							id: MemberIdentity::from(who.clone()),
							hash: who_hash,
						})
						.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
					LinkedIdentityHashes::<T>::insert(who_hash, ());
					IDGraphs::<T>::insert(who.clone(), id_graph_members.clone());
					id_graph_members
				},
			};
			id_graph_members
				.try_push(MemberAccount { id: identity, hash: identity_hash })
				.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
			LinkedIdentityHashes::<T>::insert(identity_hash, ());

			let id_graph_members_hashes: Vec<H256> =
				id_graph_members.iter().map(|member| member.hash).collect();
			let new_id_graph_hash = H256::from(blake2_256(&id_graph_members_hashes.encode()));
			IDGraphHashes::<T>::insert(who.clone(), new_id_graph_hash);

			IDGraphs::<T>::insert(who.clone(), id_graph_members);

			Self::deposit_event(Event::IdentityLinked { who, identity: identity_hash });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_identities(
			origin: OriginFor<T>,
			who: Identity,
			identity_hashes: Vec<H256>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(!identity_hashes.is_empty(), Error::<T>::IdentitiesEmpty);

			let mut id_graph_members =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::PrimeIdentityNotFound)?;

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

		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn make_identity_public(
			origin: OriginFor<T>,
			who: Identity,
			identity_hash: H256,
			public_identity: MemberIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(public_identity.is_public(), Error::<T>::IdentityIsPrivate);

			let mut id_graph_members =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::PrimeIdentityNotFound)?;
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
}
