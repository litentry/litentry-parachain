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

//! A pallet to serve as the interface for F/E for verifiable credentials (VC)
//! management. Similar to IMP pallet, the actual processing will be done within TEE.

// TODO: benchmark and weights: we need worst-case scenarios

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use primitives::{AesOutput, ShardIdentifier};
use sp_core::H256;
use sp_std::vec::Vec;

// fn types for handling inside tee-worker
pub type GenerateVCFn = ([u8; 2], ShardIdentifier, u32);

#[frame_support::pallet]
pub mod pallet {
	use super::{AesOutput, ShardIdentifier, Vec, H256};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// some extrinsics should only be called by origins from TEE
		type TEECallOrigin: EnsureOrigin<Self::Origin>;
	}

	// a map VC-ID -> hash of VC
	// current type: u64 -> H256
	#[pallet::storage]
	#[pallet::getter(fn vc_hashes)]
	pub type VCHashes<T> = StorageMap<_, Blake2_256, u64, H256>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO: do we need account as event parameter? This needs to be decided by F/E
		VCGenerationRequested { shard: ShardIdentifier, ruleset_id: u32 },
		// event that should be triggered by TEECallOrigin
		// a VC for an account is generated
		VCGenerated { account: AesOutput, vc: AesOutput },
		// a VC's hash is stored on parachain
		VCHashStored { id: u64, hash: H256 },
		// some error happened during processing in TEE, we use string-like
		// parameters for more "generic" error event reporting
		// TODO: maybe use concrete errors instead of events when we are more sure
		// see also the comment at the beginning
		SomeError { func: Vec<u8>, error: Vec<u8> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the VC ID already exists
		VCIdExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(195_000_000)]
		pub fn generate_vc(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			ruleset_id: u32,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::VCGenerationRequested { shard, ruleset_id });
			Ok(().into())
		}

		/// ---------------------------------------------------
		/// The following extrinsics are supposed to be called by TEE only
		/// ---------------------------------------------------
		#[pallet::weight(195_000_000)]
		pub fn vc_generated(
			origin: OriginFor<T>,
			account: AesOutput,
			vc: AesOutput,
			id: u64,
			hash: H256,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::VCGenerated { account, vc });
			ensure!(!VCHashes::<T>::contains_key(id), Error::<T>::VCIdExist);
			VCHashes::<T>::insert(id, hash);
			Self::deposit_event(Event::VCHashStored { id, hash });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn some_error(
			origin: OriginFor<T>,
			func: Vec<u8>,
			error: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::SomeError { func, error });
			Ok(Pays::No.into())
		}
	}
}
