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
//! # Cryptocurrency teeracle pallet
//!
//! The teeracle pallet provides functionality for handling exchange rates of the coin (ex: TEER) to
//! different currencies
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The teeracle pallet provides functions for:
//!
//! - Setting exchange rates.
#![cfg_attr(not(feature = "std"), no_std)]
pub use crate::weights::WeightInfo;
pub use pallet::*;
pub use substrate_fixed::types::U32F32;
use teeracle_primitives::{DataSource, MAX_ORACLE_DATA_NAME_LEN};

const MAX_TRADING_PAIR_LEN: usize = 11;
const MAX_SOURCE_LEN: usize = 40;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, BoundedVec, WeakBoundedVec};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use teeracle_primitives::*;

	pub type OracleDataBlob<T> = BoundedVec<u8, <T as Config>::MaxOracleBlobLen>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_teerex::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		/// Max number of whitelisted oracle's releases allowed
		#[pallet::constant]
		type MaxWhitelistedReleases: Get<u32>;

		#[pallet::constant]
		type MaxOracleBlobLen: Get<u32>;
	}

	/// Exchange rates chain's cryptocurrency/currency (trading pair) from different sources
	#[pallet::storage]
	#[pallet::getter(fn exchange_rate)]
	pub(super) type ExchangeRates<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		TradingPairString,
		Blake2_128Concat,
		DataSource,
		ExchangeRate,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn oracle_data)]
	pub(super) type OracleData<T> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		OracleDataName,
		Blake2_128Concat,
		DataSource,
		OracleDataBlob<T>,
		ValueQuery,
	>;

	/// whitelist of trusted oracle's releases for different data sources
	#[pallet::storage]
	#[pallet::getter(fn whitelist)]
	pub(super) type Whitelists<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DataSource,
		WeakBoundedVec<[u8; 32], T::MaxWhitelistedReleases>,
		ValueQuery,
	>;

	// pub(super) type Whitelist<T: Config> =
	// 	StorageValue<_, WeakBoundedVec<[u8; 32], T::MaxWhitelistedReleases>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The exchange rate of trading pair was set/updated with value from source.
		/// \[data_source], [trading_pair], [new value\]
		ExchangeRateUpdated(DataSource, TradingPairString, Option<ExchangeRate>),
		ExchangeRateDeleted(DataSource, TradingPairString),
		OracleUpdated(OracleDataName, DataSource),
		AddedToWhitelist(DataSource, [u8; 32]),
		RemovedFromWhitelist(DataSource, [u8; 32]),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidCurrency,
		/// Too many MrEnclave in the whitelist.
		ReleaseWhitelistOverflow,
		ReleaseNotWhitelisted,
		ReleaseAlreadyWhitelisted,
		TradingPairStringTooLong,
		OracleDataNameStringTooLong,
		DataSourceStringTooLong,
		OracleBlobTooBig,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::add_to_whitelist())]
		pub fn add_to_whitelist(
			origin: OriginFor<T>,
			data_source: DataSource,
			mrenclave: [u8; 32],
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(data_source.len() <= MAX_SOURCE_LEN, Error::<T>::DataSourceStringTooLong);
			ensure!(
				!Self::is_whitelisted(&data_source, mrenclave),
				<Error<T>>::ReleaseAlreadyWhitelisted
			);
			<Whitelists<T>>::try_mutate(data_source.clone(), |mrenclave_vec| {
				mrenclave_vec.try_push(mrenclave)
			})
			.map_err(|_| Error::<T>::ReleaseWhitelistOverflow)?;
			Self::deposit_event(Event::AddedToWhitelist(data_source, mrenclave));
			Ok(())
		}
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::remove_from_whitelist())]
		pub fn remove_from_whitelist(
			origin: OriginFor<T>,
			data_source: DataSource,
			mrenclave: [u8; 32],
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(
				Self::is_whitelisted(&data_source, mrenclave),
				<Error<T>>::ReleaseNotWhitelisted
			);
			<Whitelists<T>>::mutate(&data_source, |mrenclave_vec| {
				mrenclave_vec.retain(|m| *m != mrenclave)
			});
			Self::deposit_event(Event::RemovedFromWhitelist(data_source, mrenclave));
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::update_oracle())]
		pub fn update_oracle(
			origin: OriginFor<T>,
			oracle_name: OracleDataName,
			data_source: DataSource,
			new_blob: OracleDataBlob<T>,
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			<pallet_teerex::Pallet<T>>::ensure_registered_enclave(&signer)?;
			let signer_index = <pallet_teerex::Pallet<T>>::enclave_index(signer);
			let signer_enclave = <pallet_teerex::Pallet<T>>::enclave(signer_index)
				.ok_or(pallet_teerex::Error::<T>::EmptyEnclaveRegistry)?;

			ensure!(
				Self::is_whitelisted(&data_source, signer_enclave.mr_enclave),
				<Error<T>>::ReleaseNotWhitelisted
			);
			ensure!(
				oracle_name.len() <= MAX_ORACLE_DATA_NAME_LEN,
				Error::<T>::OracleDataNameStringTooLong
			);
			ensure!(data_source.len() <= MAX_SOURCE_LEN, Error::<T>::DataSourceStringTooLong);
			ensure!(
				new_blob.len() as u32 <= T::MaxOracleBlobLen::get(),
				Error::<T>::OracleBlobTooBig
			);

			OracleData::<T>::insert(&oracle_name, &data_source, new_blob);
			Self::deposit_event(Event::<T>::OracleUpdated(oracle_name, data_source));
			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::update_exchange_rate())]
		pub fn update_exchange_rate(
			origin: OriginFor<T>,
			data_source: DataSource,
			trading_pair: TradingPairString,
			new_value: Option<ExchangeRate>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			<pallet_teerex::Pallet<T>>::ensure_registered_enclave(&sender)?;
			let sender_index = <pallet_teerex::Pallet<T>>::enclave_index(sender);
			let sender_enclave = <pallet_teerex::Pallet<T>>::enclave(sender_index)
				.ok_or(pallet_teerex::Error::<T>::EmptyEnclaveRegistry)?;
			// Todo: Never checks data source len
			ensure!(
				trading_pair.len() <= MAX_TRADING_PAIR_LEN,
				Error::<T>::TradingPairStringTooLong
			);
			ensure!(
				Self::is_whitelisted(&data_source, sender_enclave.mr_enclave),
				<Error<T>>::ReleaseNotWhitelisted
			);
			if new_value.is_none() || new_value == Some(U32F32::from_num(0)) {
				log::info!("Delete exchange rate : {:?}", new_value);
				ExchangeRates::<T>::mutate_exists(&trading_pair, &data_source, |rate| *rate = None);
				Self::deposit_event(Event::ExchangeRateDeleted(data_source, trading_pair));
			} else {
				log::info!("Update exchange rate : {:?}", new_value);
				ExchangeRates::<T>::mutate_exists(&trading_pair, &data_source, |rate| {
					*rate = new_value
				});
				Self::deposit_event(Event::ExchangeRateUpdated(
					data_source,
					trading_pair,
					new_value,
				));
			}
			Ok(().into())
		}
	}
}
impl<T: Config> Pallet<T> {
	fn is_whitelisted(data_source: &DataSource, mrenclave: [u8; 32]) -> bool {
		Self::whitelist(data_source).contains(&mrenclave)
	}
}

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
