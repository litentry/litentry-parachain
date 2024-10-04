#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use core_primitives::{IDGraphHash, IDGraphMember, Identity, MemberIdentity};
pub use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_core::H256;
use sp_std::vec::Vec;

pub trait AccountIdConverter<T: Config> {
	fn convert(identity: &Identity) -> Option<T::AccountId>;
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
	pub type IDGraph<T> = BoundedVec<IDGraphMember, <T as Config>::MaxIDGraphLength>;

	#[pallet::storage]
	pub type LinkedIdentityHashes<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = ()>;

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
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: Identity,
			member_account: IDGraphMember,
			maybe_id_graph_hash: Option<H256>,
		) -> DispatchResult {
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

			LinkedIdentityHashes::<T>::insert(identity_hash, ());
			IDGraphHashes::<T>::insert(who_account_id.clone(), id_graph.hash());
			IDGraphs::<T>::insert(who_account_id.clone(), id_graph);

			Self::deposit_event(Event::IdentityLinked {
				who: who_account_id,
				identity: identity_hash,
			});

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

			let who_account_id = match T::AccountIdConverter::convert(&who) {
				Some(account_id) => account_id,
				None => return Err(Error::<T>::InvalidIdentity.into()),
			};

			let mut id_graph_members =
				IDGraphs::<T>::get(&who_account_id).ok_or(Error::<T>::UnknownIDGraph)?;

			id_graph_members.retain(|member| {
				if identity_hashes.contains(&member.hash) {
					LinkedIdentityHashes::<T>::remove(member.hash);
					false
				} else {
					true
				}
			});

			if id_graph_members.is_empty() {
				IDGraphs::<T>::remove(&who_account_id);
			} else {
				IDGraphs::<T>::insert(who_account_id.clone(), id_graph_members);
			}

			Self::deposit_event(Event::IdentityRemoved { who: who_account_id, identity_hashes });

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

			let who_account_id = match T::AccountIdConverter::convert(&who) {
				Some(account_id) => account_id,
				None => return Err(Error::<T>::InvalidIdentity.into()),
			};

			let mut id_graph_members =
				IDGraphs::<T>::get(&who_account_id).ok_or(Error::<T>::UnknownIDGraph)?;
			let id_graph_link = id_graph_members
				.iter_mut()
				.find(|member| member.hash == identity_hash)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.id = public_identity;

			IDGraphs::<T>::insert(who_account_id.clone(), id_graph_members);

			Self::deposit_event(Event::IdentityMadePublic { who: who_account_id, identity_hash });

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
			LinkedIdentityHashes::<T>::insert(owner_identity_hash, ());
			IDGraphs::<T>::insert(owner_account_id.clone(), id_graph_members.clone());

			Ok(id_graph_members)
		}
	}
}
