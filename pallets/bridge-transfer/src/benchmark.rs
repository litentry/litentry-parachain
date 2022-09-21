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

//! bridge-transfer benchmark file

#![cfg(feature = "runtime-benchmarks")]
#![allow(clippy::type_complexity)]
#[allow(unused)]
use super::*;
use bridge::BalanceOf as balance;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::{ensure, traits::{Currency,SortedMembers}};
use frame_system::RawOrigin;
use pallet_bridge::EnsureOrigin;

const MAXIMUM_ISSURANCE: u32 = 20_000;

fn create_user<T: Config>(string: &'static str, n: u32, seed: u32) -> T::AccountId {
	let user = account(string, n, seed);

	let total = MAXIMUM_ISSURANCE.into();
	T::Currency::make_free_balance_be(&user, total);
	T::Currency::issue(total);
	user
}

benchmarks! {
	transfer_native{
		let sender:T::AccountId = create_user::<T>("sender",0u32,1u32);

		T::TransferNativeMembers::add(&sender);
		ensure!(T::TransferNativeMembers::contains(&sender),"add transfernativemember failed");

		let dest_chain = 0;

		pallet_bridge::Pallet::<T>::update_fee(
			RawOrigin::Root.into(),
			dest_chain,
			10u32.into(),
		)?;

		pallet_bridge::Pallet::<T>::whitelist_chain(
			RawOrigin::Root.into(),
			dest_chain,
		)?;

	}:_(RawOrigin::Signed(sender),50u32.into(),vec![0u8, 0u8, 0u8, 0u8],dest_chain)

	transfer{

		// let sender = bridge::account_id();
		// let bridge_id:T::AccountId = create_user::<T>("bridge",0u32,1u32);
		let origin = T::BridgeOrigin::successful_origin();

		let to_account:T::AccountId = create_user::<T>("to",1u32,2u32);

		let resource_id :bridge::ResourceId= [0u8;32];
	}:_<T::Origin>(origin,to_account,50u32.into(),resource_id)

	set_maximum_issuance{
		let origin = T::SetMaximumIssuanceOrigin::successful_origin();
		let maximum_issuance:balance<T> = 2u32.into();
	}:_<T::Origin>(origin,maximum_issuance)
	verify{
		assert_eq!(MaximumIssuance::<T>::get(),maximum_issuance);
	}

	set_external_balances{
		let external_balances:balance<T> = (MAXIMUM_ISSURANCE / 2).into();
	}:_(RawOrigin::Root,external_balances)
	verify{
		assert_eq!(<ExternalBalances<T>>::get(),external_balances);
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
