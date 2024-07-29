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
#![allow(clippy::type_complexity)]

use frame_support::{
	migration::{put_storage_value, storage_key_iter},
	pallet_prelude::*,
	traits::{Currency, Get, OnRuntimeUpgrade},
	Blake2_128Concat, Twox64Concat,
};
use sp_runtime::{traits::Hash, Saturating};
use sp_std::{
	convert::{From, TryInto},
	marker::PhantomData,
	vec::Vec,
};
use sp_core_hashing::twox_64;

pub const DECIMAL_CONVERTOR: u32 = 1_000_000;

#[cfg(feature = "try-runtime")]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "try-runtime")]
use sp_std::collections::btree_map::BTreeMap;
use storage::migration::get_storage_value;

use pallet_identity::{RegistrarInfo, Registration};
type BalanceOf<T> = <<T as pallet_identity::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

// Replace Parachain Staking Storage for Decimal Change from 12 to 18
pub struct ReplacePalletIdentityStorage<T>(PhantomData<T>);

impl<T> ReplacePalletIdentityStorage<T>
where
    T: pallet_identity::Config
{
	// pallet_identity
	pub fn replace_identityof_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Running migration to ParachainIdentity IdentityOf"
		);
		let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"IdentityOf";
		let mut weight: Weight = frame_support::weights::Weight::zero();

        for (account, mut registration) in storage_key_iter::<
            T::AccountId,
            Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields>,
            Twox64Concat,
        >(pallet_prefix, storage_item_prefix).drain() {
            registration.deposit = registration.deposit.saturating_mul(DECIMAL_CONVERTOR.into());

            put_storage_value::<Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields>>(
				pallet_prefix,
				storage_item_prefix,
				&twox_64(&account.encode()),
				registration,
			);

			weight += T::DbWeight::get().reads_writes(1, 1);
        }

        weight
	}

    pub fn replace_subsof_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Running migration to ParachainIdentity SubsOf"
		);
		let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"SubsOf";
        let mut weight: Weight = frame_support::weights::Weight::zero();

        for (account, (balance, sub_accounts)) in storage_key_iter::<
            T::AccountId,
            (BalanceOf<T>, BoundedVec<T::AccountId, T::MaxSubAccounts>),
            Twox64Concat,
        >(pallet_prefix, storage_item_prefix).drain() {
            let new_balance = balance.saturating_mul(DECIMAL_CONVERTOR.into());

            put_storage_value::<(BalanceOf<T>, BoundedVec<T::AccountId, T::MaxSubAccounts>)>(
				pallet_prefix,
				storage_item_prefix,
				&twox_64(&account.encode()),
				(new_balance, sub_accounts),
			);

			weight += T::DbWeight::get().reads_writes(1, 1);
        }

        weight
	}

    pub fn replace_registrars_storage() -> frame_support::weights::Weight {
		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Running migration to ParachainIdentity Registrars"
		);
		let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"Registrars";

		let mut stored_data = match get_storage_value::<
			BoundedVec<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>, T::MaxRegistrars>,
		>(pallet_prefix, storage_item_prefix, b"").expect("Failed to retrieve ParachainIdentity Registrars storage");

		for registrar_option in &mut stored_data {
			if let Some(registrar) = registrar_option {
				let new_fee = registrar.fee.saturating_mul(DECIMAL_CONVERTOR.into());
				registrar.fee = new_fee;
			}
		}

        put_storage_value(
            pallet_prefix,
            storage_item_prefix,
            b"",
            &stored_data,
        );

		let weight = T::DbWeight::get();
		weight.reads(1) + weight.writes(1)
	}

}

#[cfg(feature = "try-runtime")]
impl<T> ReplacePalletIdentityStorage<T>
where
    T: pallet_identity::Config
{
	// pallet_identity
	pub fn pre_upgrade_identityof_storage() -> Result<Vec<u8>, &'static str> {

		let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"IdentityOf";
        let result: Vec<_> = storage_key_iter(pallet_prefix, storage_item_prefix).into_iter().map(|(account, registration)| {
            let mut new_registration: Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields> = registration;
            new_registration.deposit = registration.deposit.saturating_mul(DECIMAL_CONVERTOR.into());
            (account, new_registration)
        }).collect();

		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Finished performing IdentityOf pre upgrade checks"
		);

		Ok(result.encode())
	}
	pub fn post_upgrade_identityof_storage(state: Vec<u8>) -> Result<(), &'static str> {
        let expected_result =
            Vec::<(T::AccountId, Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields>)>::decode(&mut &state[..])
                .map_err(|_| "Failed to decode Bounties")?;

        let pallet_prefix: &[u8] = b"ParachainIdentity";
        let storage_item_prefix: &[u8] = b"IdentityOf";
        let actual_result: Vec<_> = storage_key_iter::<
            T::AccountId,
            Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields>,
            Twox64Concat,
        >(pallet_prefix, storage_item_prefix)
        .collect();

        for x in 0..actual_result.len() {
            assert_eq!(actual_result[x], expected_result[x]);
        }

        log::info!(
            target: "ReplacePalletIdentityStorage",
            "Finished performing IdentityOf post upgrade checks"
        );

        Ok(())


	}
	pub fn pre_upgrade_subsof_storage() -> Result<Vec<u8>, &'static str> {
        let pallet_prefix: &[u8] = b"ParachainIdentity";
        let storage_item_prefix: &[u8] = b"SubsOf";
        let result: Vec<_> = storage_key_iter(pallet_prefix, storage_item_prefix).into_iter().map(|(account, (balance, sub_accounts))| {
            let new_balance = balance.saturating_mul(DECIMAL_CONVERTOR.into());
            (account, (new_balance, sub_accounts))
        }).collect();

        log::info!(
            target: "ReplacePalletIdentityStorage",
            "Finished performing SubsOf pre upgrade checks"
        );

		Ok(result.encode())
	}
	pub fn post_upgrade_subsof_storage(state: Vec<u8>) -> Result<(), &'static str> {
        let expected_result =
            Vec::<(T::AccountId, (BalanceOf<T>, BoundedVec<T::AccountId, T::MaxSubAccounts>))>::decode(&mut &state[..])
                .map_err(|_| "Failed to decode Bounties")?;

        let pallet_prefix: &[u8] = b"ParachainIdentity";
        let storage_item_prefix: &[u8] = b"SubsOf";
        let actual_result: Vec<_> = storage_key_iter::<
            T::AccountId,
            (BalanceOf<T>, BoundedVec<T::AccountId, T::MaxSubAccounts>),
            Twox64Concat,
        >(pallet_prefix, storage_item_prefix)
        .collect();

        for x in 0..actual_result.len() {
            assert_eq!(actual_result[x], expected_result[x]);
        }

        log::info!(
            target: "ReplacePalletIdentityStorage",
            "Finished performing SubsOf post upgrade checks"
        );

		Ok(())
	}
	pub fn pre_upgrade_registrars_storage() -> Result<Vec<u8>, &'static str> {
		let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"Registrars";

		let mut stored_data = match get_storage_value::<
			BoundedVec<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>, T::MaxRegistrars>,
		>(pallet_prefix, storage_item_prefix, b"")
		{
			Some(data) => data,
			None => {
				panic!("Failed to retrieve ParachainIdentity Registrars storage");
			},
		};

		for registrar_option in &mut stored_data {
			if let Some(registrar) = registrar_option {
				registrar.fee = registrar.fee.saturating_mul(DECIMAL_CONVERTOR.into());
			}
		}

		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Finished performing Registrars pre upgrade checks"
		);
		Ok(stored_data.encode())
	}
	pub fn post_upgrade_registrars_storage(state: Vec<u8>) -> Result<(), &'static str> {
        let expected_result =
            BoundedVec::<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>, T::MaxRegistrars>::decode(&mut &state[..])
                .map_err(|_| "Failed to decode Bounties")?;

        let pallet_prefix: &[u8] = b"ParachainIdentity";
		let storage_item_prefix: &[u8] = b"Registrars";

		let mut actual_result = match get_storage_value::<
			BoundedVec<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>, T::MaxRegistrars>,
		>(pallet_prefix, storage_item_prefix, b"")
		{
			Some(data) => data,
			None => {
				panic!("Failed to retrieve ParachainIdentity Registrars storage");
			},
		};

		assert_eq!(expected_result.encode(), actual_result.encode());

		log::info!(
			target: "ReplacePalletIdentityStorage",
			"Finished performing Registrars post upgrade checks"
		);
		Ok(())
	}
}

impl<T> OnRuntimeUpgrade for ReplacePalletIdentityStorage<T>
where
	T: frame_system::Config + pallet_identity::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		// pallet_identity
		let identityof_vec = Self::pre_upgrade_identityof_storage()?;
		let subsof_vec = Self::pre_upgrade_subsof_storage()?;
		let registrars_vec = Self::pre_upgrade_registrars_storage()?;

		Ok((
            identityof_vec,
            subsof_vec,
            registrars_vec
        ).encode())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		let mut weight = frame_support::weights::Weight::from_parts(0, 0);
		// pallet_identity
		weight += Self::replace_identityof_storage();
		weight += Self::replace_subsof_storage();
		weight += Self::replace_registrars_storage();

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		let pre_vec: (Vec<u8>, Vec<u8>, Vec<u8>) =
			Decode::decode(&mut &state[..]).map_err(|_| "Failed to decode Tuple")?;
		// pallet_identity
		Self::post_upgrade_identityof_storage(pre_vec.0)?;
		Self::post_upgrade_subsof_storage(pre_vec.1)?;
		Self::post_upgrade_registrars_storage(pre_vec.2)?;

		Ok(())
	}
}
