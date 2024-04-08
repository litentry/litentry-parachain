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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use sp_core::H160;

pub use pallet::*;

mod custodial_type;
pub use custodial_type::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn mimic_btc_to_eth_storage)]
	pub type MimicBtcToEthStorage<T: Config> =
		StorageDoubleMap<_, Blake2_256, PubKey, Blake2_256, u64, BtcToEth>;

	#[pallet::storage]
	#[pallet::getter(fn mimic_eth_to_btc_storage)]
	pub type MimicEthToBtcStorage<T: Config> =
		StorageDoubleMap<_, Blake2_256, H160, Blake2_256, u64, EthToBtc>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BtcToEthSaved { btc: PubKey, tx_index: u64 },
		EthToBtcSaved { eth: H160, tx_index: u64 },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the admin account
		///
		/// Weights should be 2 DB writes: 1 for mode and 1 for event
		#[pallet::call_index(0)]
		#[pallet::weight({195_000_000})]
		pub fn write_btc_to_eth(
			_origin: OriginFor<T>,
			btc_sender: PubKey,
			data: BtcToEth,
		) -> DispatchResult {
			Self::deposit_event(Event::BtcToEthSaved { btc: btc_sender, tx_index: data.tx_index });
			<MimicBtcToEthStorage<T>>::insert(btc_sender, data.tx_index, data);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({195_000_000})]
		pub fn write_eth_to_btc(
			_origin: OriginFor<T>,
			eth_sender: H160,
			data: EthToBtc,
		) -> DispatchResult {
			Self::deposit_event(Event::EthToBtcSaved { eth: eth_sender, tx_index: data.tx_index });
			<MimicEthToBtcStorage<T>>::insert(eth_sender, data.tx_index, data);
			Ok(())
		}
	}
}
