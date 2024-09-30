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

pub trait AccountIdConverter<T: Config> {
	fn convert(account_id: T::AccountId) -> Identity;
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
		/// AccountId converter
		type AccountIdConverter: AccountIdConverter<Self>;
	}
	pub type IDGraphLinks<T> = BoundedVec<(H256, MemberIdentity), <T as Config>::MaxIDGraphLength>;

	#[pallet::storage]
	pub type LinkedIdentityHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = ()>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = IDGraphLinks<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Identity linked
		IdentityLinked { who: T::AccountId, identity: H256 },
		/// Identity remove
		IdentityRemoved { who: T::AccountId, identity_hashes: Vec<H256> },
		/// Identity made public
		IdentityMadePublic { who: T::AccountId, identity_hash: H256 },
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
		/// Identity is private
		IdentityIsPrivate,
		/// Identities empty
		IdentitiesEmpty,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity_hash: H256,
			identity: MemberIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				!LinkedIdentityHashes::<T>::contains_key(identity_hash),
				Error::<T>::IdentityAlreadyLinked
			);
			let mut id_graph_links = match IDGraphs::<T>::get(&who) {
				Some(id_graph_links) => id_graph_links,
				None => {
					let prime_identity = T::AccountIdConverter::convert(who.clone());
					let prime_did =
						prime_identity.to_did().map_err(|_| Error::<T>::PrimeIdentityInvalid)?;
					let prime_identity_hash = H256::from(blake2_256(&prime_did.encode()));
					if LinkedIdentityHashes::<T>::contains_key(prime_identity_hash) {
						return Err(Error::<T>::IdentityAlreadyLinked.into());
					}
					let mut id_graph_links: IDGraphLinks<T> = BoundedVec::new();
					id_graph_links
						.try_push((prime_identity_hash, MemberIdentity::from(prime_identity)))
						.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
					LinkedIdentityHashes::<T>::insert(prime_identity_hash, ());
					IDGraphs::<T>::insert(who.clone(), id_graph_links.clone());
					id_graph_links
				},
			};
			id_graph_links
				.try_push((identity_hash, identity))
				.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
			LinkedIdentityHashes::<T>::insert(identity_hash, ());
			IDGraphs::<T>::insert(who.clone(), id_graph_links);
			Self::deposit_event(Event::IdentityLinked { who, identity: identity_hash });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_identities(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity_hashes: Vec<H256>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(!identity_hashes.is_empty(), Error::<T>::IdentitiesEmpty);

			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;

			id_graph_links.retain(|(id_hash, _)| !identity_hashes.contains(id_hash));

			if id_graph_links.is_empty() {
				IDGraphs::<T>::remove(&who);
			} else {
				IDGraphs::<T>::insert(who.clone(), id_graph_links);
			}

			for identity_hash in identity_hashes.iter() {
				LinkedIdentityHashes::<T>::remove(identity_hash);
			}

			Self::deposit_event(Event::IdentityRemoved { who, identity_hashes });

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn make_identity_public(
			origin: OriginFor<T>,
			who: T::AccountId,
			identity_hash: H256,
			public_identity: MemberIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(public_identity.is_public(), Error::<T>::IdentityIsPrivate);

			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;
			let id_graph_link = id_graph_links
				.iter_mut()
				.find(|(id_hash, _)| *id_hash == identity_hash)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.1 = public_identity;

			IDGraphs::<T>::insert(who.clone(), id_graph_links);

			Self::deposit_event(Event::IdentityMadePublic { who, identity_hash });

			Ok(())
		}
	}
}
