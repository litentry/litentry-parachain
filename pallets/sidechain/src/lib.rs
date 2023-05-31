/*
Copyright 2021 Integritee AG and Supercomputing Systems AG

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_system::{self};
use pallet_teerex::Pallet as Teerex;
use sidechain_primitives::SidechainBlockConfirmation;
use sp_core::H256;
use sp_std::{prelude::*, str};
use teerex_primitives::ShardIdentifier;

pub use crate::weights::WeightInfo;

// Disambiguate associated types
pub type AccountId<T> = <T as frame_system::Config>::AccountId;
pub type ShardBlockNumber = (ShardIdentifier, u64);

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_teerex::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProposedSidechainBlock(T::AccountId, H256),
		FinalizedSidechainBlock(T::AccountId, H256),
	}

	// Enclave index of the worker that recently committed an update.
	#[pallet::storage]
	#[pallet::getter(fn worker_for_shard)]
	pub type WorkerForShard<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardIdentifier, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn latest_sidechain_block_confirmation)]
	pub type LatestSidechainBlockConfirmation<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardIdentifier, SidechainBlockConfirmation, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sidechain_block_finalization_candidate)]
	pub type SidechainBlockFinalizationCandidate<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardIdentifier, u64, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The integritee worker calls this function for every imported sidechain_block.
		#[pallet::call_index(0)]
		#[pallet::weight((<T as Config>::WeightInfo::confirm_imported_sidechain_block(), DispatchClass::Normal, Pays::Yes))]
		pub fn confirm_imported_sidechain_block(
			origin: OriginFor<T>,
			shard_id: ShardIdentifier,
			block_number: u64,
			next_finalization_candidate_block_number: u64,
			block_header_hash: H256,
		) -> DispatchResultWithPostInfo {
			let confirmation = SidechainBlockConfirmation { block_number, block_header_hash };

			let sender = ensure_signed(origin)?;
			Teerex::<T>::ensure_registered_enclave(&sender)?;
			let sender_index = Teerex::<T>::enclave_index(&sender);
			let sender_enclave = Teerex::<T>::enclave(sender_index)
				.ok_or(pallet_teerex::Error::<T>::EmptyEnclaveRegistry)?;
			ensure!(
				sender_enclave.mr_enclave.encode() == shard_id.encode(),
				pallet_teerex::Error::<T>::WrongMrenclaveForShard
			);

			// Simple logic for now: only accept blocks from first registered enclave.
			if sender_index != 1 {
				log::debug!(
					"Ignore block confirmation from registered enclave with index {:?}",
					sender_index
				);
				return Ok(().into())
			}

			let block_number = confirmation.block_number;
			let finalization_candidate_block_number =
				<SidechainBlockFinalizationCandidate<T>>::try_get(shard_id).unwrap_or(1);

			ensure!(
				block_number == finalization_candidate_block_number,
				<Error<T>>::ReceivedUnexpectedSidechainBlock
			);
			ensure!(
				next_finalization_candidate_block_number > finalization_candidate_block_number,
				<Error<T>>::InvalidNextFinalizationCandidateBlockNumber
			);

			<SidechainBlockFinalizationCandidate<T>>::insert(
				shard_id,
				next_finalization_candidate_block_number,
			);

			Self::finalize_block(shard_id, confirmation, &sender, sender_index);
			Ok(().into())
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// A proposed block is unexpected.
		ReceivedUnexpectedSidechainBlock,
		/// The value for the next finalization candidate is invalid.
		InvalidNextFinalizationCandidateBlockNumber,
	}
}

impl<T: Config> Pallet<T> {
	fn finalize_block(
		shard_id: ShardIdentifier,
		confirmation: SidechainBlockConfirmation,
		sender: &T::AccountId,
		sender_index: u64,
	) {
		<LatestSidechainBlockConfirmation<T>>::insert(shard_id, confirmation);
		<WorkerForShard<T>>::insert(shard_id, sender_index);
		let block_header_hash = confirmation.block_header_hash;
		log::debug!(
			"Imported sidechain block confirmed with shard {:?}, block header hash {:?}",
			shard_id,
			block_header_hash
		);
		Self::deposit_event(Event::FinalizedSidechainBlock(sender.clone(), block_header_hash));
	}
}

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(all(test, not(feature = "skip-ias-check")))]
mod tests;
pub mod weights;
