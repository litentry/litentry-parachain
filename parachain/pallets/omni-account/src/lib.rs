#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core_primitives::Identity;
	use frame_support::pallet_prelude::Identity as IdentityHasher;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The event type of this pallet.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The origin which can manage the pallet.
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type OpaqueLinkedIdentities<T: Config> =
		StorageMap<Hasher = IdentityHasher, Key = Vec<u8>, Value = ()>;

	#[pallet::storage]
	#[pallet::getter(fn opaque_id_graphs)]
	pub type OpaqueIDGraphs<T: Config> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = Identity,  // prime identity
		Value = Vec<u8>, // IDGraph
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// IDGraph created
		IDGraphCreated(Identity),
		/// IDGraph updated
		IDGraphUpdated(Identity),
		/// IDGraph removed
		IDGraphRemoved(Identity),
		/// LinkedIdentity added
		LinkedIdentityAdded(Vec<u8>),
		/// LinkedIdentity removed
		LinkedIdentityRemoved(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The IDGraph is not found
		IDGraphNotFound,
		/// The LinkedIdentity is already added
		LinkedIdentityAlreadyAdded,
		/// The LinkedIdentity is not found
		LinkedIdentityNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn insert_id_graph(
			origin: OriginFor<T>,
			prime_identity: Identity,
			id_graph: Vec<u8>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			let event = match OpaqueIDGraphs::<T>::contains_key(&prime_identity) {
				true => Event::IDGraphUpdated(prime_identity.clone()),
				false => Event::IDGraphCreated(prime_identity.clone()),
			};
			OpaqueIDGraphs::<T>::insert(prime_identity.clone(), id_graph);
			Self::deposit_event(event);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_id_graph(origin: OriginFor<T>, prime_identity: Identity) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				OpaqueIDGraphs::<T>::contains_key(&prime_identity),
				Error::<T>::IDGraphNotFound
			);
			OpaqueIDGraphs::<T>::remove(&prime_identity);
			Self::deposit_event(Event::IDGraphRemoved(prime_identity));

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn add_linked_identity(
			origin: OriginFor<T>,
			linked_identity: Vec<u8>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				!OpaqueLinkedIdentities::<T>::contains_key(&linked_identity),
				Error::<T>::LinkedIdentityAlreadyAdded
			);
			OpaqueLinkedIdentities::<T>::insert(linked_identity.clone(), ());
			Self::deposit_event(Event::LinkedIdentityAdded(linked_identity));

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal))]
		pub fn remove_linked_identity(
			origin: OriginFor<T>,
			linked_identity: Vec<u8>,
		) -> DispatchResult {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			ensure!(
				OpaqueLinkedIdentities::<T>::contains_key(&linked_identity),
				Error::<T>::LinkedIdentityNotFound
			);
			OpaqueLinkedIdentities::<T>::remove(&linked_identity);
			Self::deposit_event(Event::LinkedIdentityRemoved(linked_identity));

			Ok(())
		}
	}
}
