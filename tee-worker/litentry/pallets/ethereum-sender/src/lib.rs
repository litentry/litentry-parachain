// Copyright 2020-2023 Litentry Technologies GmbH.
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

// Ethereum Sender pallet run within TEE(enclave) -- aka IMT
// The pallet is integrated in SGX-runtime, the extrinsics are supposed
// to be called only by enclave
//
// This pallet is for signing/parsing raw ethereum transaction inside TEE
// TODO::
// This pallet is not ready yet


#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

mod contracts;

pub use pallet::*;

use frame_support::{pallet_prelude::*, traits::StorageVersion};
use frame_system::pallet_prelude::*;
use ethereum_tee::{AccountPrivateKey, TransactionMessageV2};

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use litentry_primitives::Address32;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// the manager origin for extrincis
		type ManageOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Transaction sent
		/// Should we leave records here?
		TransactionSent { signed_transaction: Vec<u8>, nonce: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unrecognized transaction
        TransactionFailed,
	}

    /// TODO:: This storage is not safe enough!!!
	/// Ethereum contract's administor private key
	#[pallet::storage]
	#[pallet::getter(fn ethereum_master_key)]
	pub type EthereumMasterKey<T: Config> = StorageValue<_, AccountPrivateKey, ValueQuery>;

	/// TODO:: This storage is not safe enough!!!
	/// Ethereum contract's administor nonce
	/// <chain_id, nonce>
	#[pallet::storage]
	#[pallet::getter(fn ethereum_master_nonce)]
	pub(crate) type EthereumMasterNonce<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u64,
		U256,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(15_000_000)]
		pub fn send_blockchain_vc(
			origin: OriginFor<T>,
			// TODO:: In the end, vc will be parsed into this form of transaction call someway,
			// So we temporary make it this type.
			vc_info: Bytes,
			chain_id: u64,
		) -> DispatchResult {
			T::ManageOrigin::ensure_origin(origin)?;
            /// TODO::Some vc generating code here
			let nonce = <EthereumMasterNonceM<T>>::get(chain_id)?;
			let txm = EIP1559TransactionMessage {
				chain_id: chain_id,
				nonce: nonce,
				max_priority_fee_per_gas: 1_500_000_000u64.into(),
				max_fee_per_gas: 20_000_000_000u64.into(),
				gas_limit: 31_524u64.into(),
				action: TransactionAction::Call(
					contracts::vc_ethereum_contract(),
				),
				value: 100_000_000_000_000_000u64.into(),
				input: vc_info,
				access_list: vec![],
			};
            let singed_raw_transaction = Vec::from(Bytes::from(
				<EthereumMasterKey<T>>::get().sign_transaction(TransactionMessageV2::EIP1559(txm))
					.ok_or(Error::<T>::TransactionFailed)?,
			));
			Self::deposit_event(Event::TransactionSent {
				signed_transaction: singed_raw_transaction,
				nonce: nonce,
			});

			<EthereumMasterNonceM<T>>::insert(chain_id, nonce + 1);
			Ok(())
		}
	}
}
