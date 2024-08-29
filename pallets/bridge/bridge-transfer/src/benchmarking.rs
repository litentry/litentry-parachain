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

//! bridge-transfer benchmark file

use super::{Pallet as BridgeTransfer, *};
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use hex_literal::hex;
use pallet_bridge_common::BridgeHandler;
use pallet_chain_bridge::{EnsureOrigin, Pallet as ChainBridge};
use sp_std::vec;

const RID: [u8; 32] = hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");

fn setup<T: Config>() {
	let dest_chain = 0;
	if !<pallet_chain_bridge::Pallet<T>>::chain_whitelisted(dest_chain) {
		<pallet_chain_bridge::Pallet<T>>::whitelist_chain(RawOrigin::Root.into(), dest_chain)
			.unwrap();
	}
	T::BridgeHandler::setup_native_asset_info(RID, 0u32.into()).unwrap();
}

fn create_funded_user<T: Config + pallet_assets::Config>(
	string: &'static str,
	n: u32,
	extra: u32,
) -> T::AccountId {
	const SEED: u32 = 0;
	let user = account(string, n, SEED);
	let total = <T as pallet_assets::Config>::Currency::minimum_balance() + extra.into();
	<T as pallet_assets::Config>::Currency::make_free_balance_be(&user, total);
	let _ = <T as pallet_assets::Config>::Currency::issue(total);
	user
}

fn do_transfer<T: Config>(to: T::AccountId, amount: u32) {
	BridgeTransfer::<T>::transfer(
		T::BridgeOrigin::try_successful_origin().unwrap(),
		to,
		amount.into(),
		RID,
	)
	.expect("BridgeTransfer::transfer failed");
}

#[allow(clippy::multiple_bound_locations)]
#[benchmarks(
    where <T as frame_system::Config>::Hash: From<[u8; 32]>,
		  T: pallet_assets::Config,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn transfer_assets() {
		setup::<T>();
		let sender = create_funded_user::<T>("sender", 1, 100);
		do_transfer::<T>(sender.clone(), 1);

		#[extrinsic_call]
		_(RawOrigin::Signed(sender), 5u32.into(), vec![0u8, 0u8, 0u8, 0u8], 0, RID);
	}

	#[benchmark]
	fn transfer() {
		setup::<T>();
		let to = create_funded_user::<T>("to", 1, 100);
		let o = T::BridgeOrigin::try_successful_origin().unwrap();

		#[extrinsic_call]
		_(o as T::RuntimeOrigin, to, 1u32.into(), RID);
	}

	impl_benchmark_test_suite!(BridgeTransfer, crate::mock::new_test_ext(), crate::mock::Test);
}
