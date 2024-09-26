#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub(crate) mod identity_context;

pub use core_primitives::{assertion::network::Web3Network, Identity, OmniAccountIdentity};
pub use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use identity_context::*;
use sp_core::{blake2_256, H256};
use sp_std::vec::Vec;

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
	pub type IDGraphLinks<T> =
		BoundedVec<(OmniAccountIdentity, IdentityContext<T>), <T as Config>::MaxIDGraphLength>;

	#[pallet::storage]
	pub type LinkedIdentities<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = H256, Value = ()>;

	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> =
		StorageMap<Hasher = Blake2_128Concat, Key = Identity, Value = IDGraphLinks<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Identity linked
		IdentityLinked { who: Identity, identity: H256 },
		/// Identity deactivate_identity
		IdentityDeactivated { who: Identity, identity: H256 },
		/// Identity activate_identity
		IdentityActivated { who: Identity, identity: H256 },
		/// Web3 networks updated
		Web3NetworksUpdated { identity: H256, web3networks: Vec<Web3Network> },
		/// Identity remove
		IdentityRemoved { who: Identity, identities: Vec<OmniAccountIdentity> },
		/// Identity made public
		IdentityMadePublic { who: Identity, identity: H256 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The Identity is already linked
		IdentityAlreadyLinked,
		/// IDGraph len limit reached
		IDGraphLenLimitReached,
		/// Identity not found
		IdentityNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn link_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: OmniAccountIdentity,
			web3networks: Vec<Web3Network>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let identity_hash = H256::from(blake2_256(&identity.encode()));
			ensure!(
				!LinkedIdentities::<T>::contains_key(identity_hash),
				Error::<T>::IdentityAlreadyLinked
			);
			let mut id_graph_links = match IDGraphs::<T>::get(&who) {
				Some(id_graph_links) => id_graph_links,
				None => {
					let prime_identity_hash =
						H256::from(blake2_256(&OmniAccountIdentity::from(who.clone()).encode()));
					if LinkedIdentities::<T>::contains_key(prime_identity_hash) {
						return Err(Error::<T>::IdentityAlreadyLinked.into());
					}
					let context = IdentityContext::<T>::new(
						<frame_system::Pallet<T>>::block_number(),
						who.default_web3networks(),
					);
					let mut id_graph_links: IDGraphLinks<T> = BoundedVec::new();
					id_graph_links
						.try_push((who.clone().into(), context))
						.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
					LinkedIdentities::<T>::insert(prime_identity_hash, ());
					IDGraphs::<T>::insert(who.clone(), id_graph_links.clone());
					id_graph_links
				},
			};
			let context =
				IdentityContext::<T>::new(<frame_system::Pallet<T>>::block_number(), web3networks);
			id_graph_links
				.try_push((identity, context))
				.map_err(|_| Error::<T>::IDGraphLenLimitReached)?;
			LinkedIdentities::<T>::insert(identity_hash, ());
			IDGraphs::<T>::insert(who.clone(), id_graph_links);
			Self::deposit_event(Event::IdentityLinked { who, identity: identity_hash });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn deactivate_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: OmniAccountIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;
			let id_graph_link = id_graph_links
				.iter_mut()
				.find(|(id, _)| id == &identity)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.1.deactivate();

			IDGraphs::<T>::insert(who.clone(), id_graph_links);

			let identity_hash = H256::from(blake2_256(&identity.encode()));

			Self::deposit_event(Event::IdentityDeactivated { who, identity: identity_hash });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn activate_identity(
			origin: OriginFor<T>,
			who: Identity,
			identity: OmniAccountIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;
			let id_graph_link = id_graph_links
				.iter_mut()
				.find(|(id, _)| id == &identity)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.1.activate();

			IDGraphs::<T>::insert(who.clone(), id_graph_links);

			let identity_hash = H256::from(blake2_256(&identity.encode()));

			Self::deposit_event(Event::IdentityActivated { who, identity: identity_hash });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn set_identity_networks(
			origin: OriginFor<T>,
			who: Identity,
			identity: OmniAccountIdentity,
			web3networks: Vec<Web3Network>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;
			let id_graph_link = id_graph_links
				.iter_mut()
				.find(|(id, _)| id == &identity)
				.ok_or(Error::<T>::IdentityNotFound)?;
			id_graph_link.1.set_web3networks(web3networks.clone());

			IDGraphs::<T>::insert(who.clone(), id_graph_links);

			let identity_hash = H256::from(blake2_256(&identity.encode()));

			Self::deposit_event(Event::Web3NetworksUpdated {
				identity: identity_hash,
				web3networks,
			});

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_identity(
			origin: OriginFor<T>,
			who: Identity,
			identities: Vec<OmniAccountIdentity>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;

			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;

			if !identities.is_empty() {
				id_graph_links.retain(|(id, _)| {
					if identities.contains(id) {
						let identity_hash = H256::from(blake2_256(&id.encode()));
						LinkedIdentities::<T>::remove(identity_hash);
						false
					} else {
						true
					}
				});
				IDGraphs::<T>::insert(who.clone(), id_graph_links);
			} else {
				id_graph_links.iter().for_each(|(id, _)| {
					let identity_hash = H256::from(blake2_256(&id.encode()));
					LinkedIdentities::<T>::remove(identity_hash);
				});
				IDGraphs::<T>::remove(&who);
			}

			Self::deposit_event(Event::IdentityRemoved { who, identities });

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn make_identity_public(
			origin: OriginFor<T>,
			who: Identity,
			private_identity: OmniAccountIdentity,
			public_identity: OmniAccountIdentity,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let mut id_graph_links =
				IDGraphs::<T>::get(&who).ok_or(Error::<T>::IdentityNotFound)?;
			let id_graph_link = id_graph_links
				.iter_mut()
				.find(|(id, _)| id == &private_identity)
				.ok_or(Error::<T>::IdentityNotFound)?;

			let public_identity_hash = H256::from(blake2_256(&public_identity.encode()));
			id_graph_link.0 = public_identity;
			IDGraphs::<T>::insert(who.clone(), id_graph_links);
			let private_identity_hash = H256::from(blake2_256(&private_identity.encode()));
			LinkedIdentities::<T>::remove(private_identity_hash);
			LinkedIdentities::<T>::insert(public_identity_hash, ());

			Self::deposit_event(Event::IdentityMadePublic { who, identity: public_identity_hash });

			Ok(())
		}
	}
}
