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
use frame_support::{
	traits::{Get, OnRuntimeUpgrade},
	Identity, StorageHasher, Twox128,
};
use pallet_preimage::RequestStatus;
use sp_std::marker::PhantomData;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

use crate::migration::clear_storage_prefix;
use frame_support::{
	migration::{put_storage_value, storage_key_iter},
	Twox64Concat,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_bounties::{Bounties, BountyIndex, BountyStatus};
use pallet_treasury::BalanceOf;
use parity_scale_codec::{Decode, Encode, EncodeLike};
use sp_runtime::Saturating;
use sp_std::collections::btree_map::BTreeMap;

use crate::migration::DECIMAL_CONVERTOR;

pub struct ReplacePreImageStorage<T>(PhantomData<T>);
impl<T> OnRuntimeUpgrade for ReplacePreImageStorage<T>
where
	T: pallet_preimage::Config + pallet_treasury::Config,
	BalanceOf<T>: From<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let pallet_prefix: &[u8] = b"Preimage";
		let storage_item_prefix: &[u8] = b"StatusFor";
		let stored_data: Vec<_> = storage_key_iter::<
			T::Hash,
			RequestStatus<T::AccountId, BalanceOf<T>>,
			Identity,
		>(pallet_prefix, storage_item_prefix)
		.collect();

		let result: Vec<_> = stored_data
			.into_iter()
			.map(|(hash, status)| {
				let mut new_status = match status {
					RequestStatus::Requested { deposit, count, len } => {
						if let Some((account, balance)) = deposit {
							RequestStatus::Requested {
								deposit: Some((
									account,
									balance.saturating_mul(DECIMAL_CONVERTOR.into()),
								)),
								count,
								len,
							}
						} else {
							RequestStatus::Requested { deposit, count, len }
						}
					},
					RequestStatus::Unrequested { deposit, len } => RequestStatus::Unrequested {
						deposit: (deposit.0, deposit.1.saturating_mul(DECIMAL_CONVERTOR.into())),
						len,
					},
				};

				(hash, new_status)
			})
			.collect();

		log::info!(
			target: "ReplacePreImageStorage",
			"Finished performing pre upgrade checks"
		);

		Ok(result.encode())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplacePreImageStorage",
			"running migration to Preimage StatusFor Storage Item"
		);
		let pallet_prefix: &[u8] = b"Preimage";
		let storage_item_prefix: &[u8] = b"StatusFor";
		let stored_data: Vec<_> = storage_key_iter::<
			T::Hash,
			RequestStatus<T::AccountId, BalanceOf<T>>,
			Identity,
		>(pallet_prefix, storage_item_prefix)
		.collect();

		let migrated_count = frame_support::weights::Weight::from_parts(
			0,
			stored_data
				.len()
				.try_into()
				.expect("There are between 0 and 2**64 mappings stored."),
		);

		// Now clear previos storage
		let _ = clear_storage_prefix(pallet_prefix, storage_item_prefix, &[], None, None);

		// Assert that old storage is empty
		assert!(storage_key_iter::<T::Hash, RequestStatus<T::AccountId, BalanceOf<T>>, Identity>(
			pallet_prefix,
			storage_item_prefix
		)
		.next()
		.is_none());

		for (hash, status) in stored_data {
			let mut new_status = match status {
				RequestStatus::Requested { deposit, count, len } => {
					if let Some((account, balance)) = deposit {
						RequestStatus::Requested {
							deposit: Some((
								account,
								balance.saturating_mul(DECIMAL_CONVERTOR.into()),
							)),
							count,
							len,
						}
					} else {
						RequestStatus::Requested { deposit, count, len }
					}
				},
				RequestStatus::Unrequested { deposit, len } => RequestStatus::Unrequested {
					deposit: (deposit.0, deposit.1.saturating_mul(DECIMAL_CONVERTOR.into())),
					len,
				},
			};

			// The storage item is using Identity so we don't need to do addtitional hashing and can
			// directly put into storage
			put_storage_value::<RequestStatus<T::AccountId, BalanceOf<T>>>(
				pallet_prefix,
				storage_item_prefix,
				hash.as_ref(),
				new_status,
			);
		}

		let weight = T::DbWeight::get();
		migrated_count.saturating_mul(weight.write + weight.read)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let expected_result =
			Vec::<(T::Hash, RequestStatus<T::AccountId, BalanceOf<T>>)>::decode(&mut &state[..])
				.map_err(|_| "Failed to decode Bounties")?;

		let pallet_prefix: &[u8] = b"Preimage";
		let storage_item_prefix: &[u8] = b"StatusFor";
		let actual_result: Vec<_> = storage_key_iter::<
			T::Hash,
			RequestStatus<T::AccountId, BalanceOf<T>>,
			Identity,
		>(pallet_prefix, storage_item_prefix)
		.collect();

		for x in 0..actual_result.len() {
			assert_eq!(actual_result[x], expected_result[x])
		}

		log::info!(
			target: "ReplacePreImageStorage",
			"Finished performing post upgrade checks"
		);

		Ok(())
	}
}
