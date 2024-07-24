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

#![cfg(feature = "runtime-benchmarks")]
#![allow(clippy::type_complexity)]
#![allow(clippy::duplicated_attributes)]
#![allow(unused)]
#![allow(clippy::useless_vec)]
use super::*;
use crate::Pallet as bridge_transfer;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::{ensure, traits::SortedMembers, PalletId};
use frame_system::RawOrigin;
use hex_literal::hex;
use pallet_bridge::{EnsureOrigin, Get};
use sp_arithmetic::traits::Saturating;
use sp_runtime::traits::AccountIdConversion;
use sp_std::vec;

const MAXIMUM_ISSURANCE: u32 = 20_000;
const NATIVE_TOKEN_RESOURCE_ID: [u8; 32] =
	hex!("0000000000000000000000000000000a21dfe87028f214dd976be8479f5af001");
fn create_user<T: Config>(string: &'static str, n: u32, seed: u32) -> T::AccountId {
	let user: T::AccountId = account(string, n, seed);
	bridge_transfer::<T>::transfer(
		RawOrigin::Root.into(),
		user.clone(),
		(n * MAXIMUM_ISSURANCE).into(),
		NATIVE_TOKEN_RESOURCE_ID,
	);

	user
}

benchmarks! {
	transfer_assets{
		let sender:T::AccountId = create_user::<T>("sender",0u32,1u32);

		ensure!(T::TransferNativeMembers::contains(&sender),"add transfer_native_member failed");
		let dest_chain = 0;

		let r_id = NATIVE_TOKEN_RESOURCE_ID;

	}:_(RawOrigin::Signed(sender),50u32.into(),vec![0u8, 0u8, 0u8, 0u8],dest_chain,r_id)

	transfer{
		let r_id = NATIVE_TOKEN_RESOURCE_ID;

		let sender = PalletId(*b"litry/bg").into_account_truncating();

		let to_account:T::AccountId = create_user::<T>("to",1u32,2u32);

	}:_(RawOrigin::Signed(sender),to_account,50u32.into(), r_id)
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
