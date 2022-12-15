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
use frame_support::{
	ensure,
	traits::{Currency, SortedMembers},
	PalletId,
};
use frame_system::RawOrigin;
use pallet_bridge::{EnsureOrigin, Get};
use sp_arithmetic::traits::Saturating;
use sp_runtime::traits::AccountIdConversion;
use sp_std::vec;

const MAXIMUM_ISSURANCE: u32 = 20_000;

fn create_user<T: Config>(string: &'static str, n: u32, seed: u32) -> T::AccountId {
	let user = account(string, n, seed);

	let default_balance = T::Currency::minimum_balance().saturating_mul(MAXIMUM_ISSURANCE.into());
	let _ = T::Currency::deposit_creating(&user, default_balance);
	user
}

benchmarks! {
	transfer_native{
		let sender:T::AccountId = create_user::<T>("sender",0u32,1u32);

		ensure!(T::TransferNativeMembers::contains(&sender),"add transfer_native_member failed");

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

		let sender = PalletId(*b"litry/bg").into_account_truncating();

		let default_balance =
		T::Currency::minimum_balance().saturating_mul(MAXIMUM_ISSURANCE.into());
		let _ = T::Currency::deposit_creating(&sender, default_balance);

		let to_account:T::AccountId = create_user::<T>("to",1u32,2u32);

		let resource_id :bridge::ResourceId= T::NativeTokenResourceId::get();

	}:_(RawOrigin::Signed(sender),to_account,50u32.into(),resource_id)

	set_maximum_issuance{
		let origin = T::SetMaximumIssuanceOrigin::successful_origin();
		let maximum_issuance:balance<T> = 2u32.into();
	}:_<T::RuntimeOrigin>(origin,maximum_issuance)
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
