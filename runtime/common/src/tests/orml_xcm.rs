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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>

use frame_support::{
	dispatch::RawOrigin,
	traits::{EnsureOrigin, OriginTrait},
	*,
};

use sp_runtime::traits::Dispatchable;

use crate::{
	tests::setup::{bob, ExtBuilder},
	BaseRuntimeRequirements, EnsureRootOrTwoThirdsCouncil, FilterEnsureOrigin,
};
use primitives::*;
use xcm::latest::prelude::*;
use xcm_executor::traits::Convert;

pub fn orml_xcm_root_works<
	Origin: OriginTrait
		+ From<RawOrigin<AccountId>>
		+ Clone
		+ std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
>()
where
	std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		Origin,
	>: std::convert::From<Origin>,
	Origin: std::convert::From<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
	>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		Origin,
	>: std::convert::From<Origin>,
{
	// test the RootOrTwoThirdsCouncil can send the xcm
	let test_root_origin = frame_system::RawOrigin::Root;
	let res_account = <FilterEnsureOrigin<
		Origin,
		LocalOriginToLocation,
		EnsureRootOrTwoThirdsCouncil,
	> as EnsureOrigin<Origin>>::try_origin(Origin::from(test_root_origin))
	.unwrap();

	assert_eq!(res_account, Here.into());
}

pub fn orml_xcm_signed_works<
	R: BaseRuntimeRequirements + frame_system::Config<Call = Call>,
	Origin: OriginTrait
		+ From<RawOrigin<AccountId>>
		+ Clone
		+ std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_balances::Call<R>>,
>()
where
	<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,

	std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		Origin,
	>: std::convert::From<Origin>,
	Origin: std::convert::From<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
	>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		Origin,
	>: std::convert::From<Origin>,
{
	ExtBuilder::<R>::default().build().execute_with(|| {
		let test_signed_origin = frame_system::RawOrigin::Signed(bob());
		let res_account = <FilterEnsureOrigin<
			Origin,
			LocalOriginToLocation,
			EnsureRootOrTwoThirdsCouncil,
		> as EnsureOrigin<Origin>>::try_origin(Origin::from(test_signed_origin))
		.unwrap();

		assert_ne!(res_account, Here.into());
	});
}

pub fn orml_xcm_two_thirds_councli_works<
	Origin: OriginTrait
		+ From<RawOrigin<AccountId>>
		+ Clone
		+ std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
	I,
>()
where
	std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		Origin,
	>: std::convert::From<Origin>,
	Origin: std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		> + std::convert::From<pallet_collective::RawOrigin<sp_runtime::AccountId32, I>>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		Origin,
	>: std::convert::From<Origin>,
{
	let tow_third_origin: Origin =
		pallet_collective::RawOrigin::<AccountId, I>::Members(2, 3).into();

	let res_account = <FilterEnsureOrigin<
		Origin,
		LocalOriginToLocation,
		EnsureRootOrTwoThirdsCouncil,
	> as EnsureOrigin<Origin>>::try_origin(tow_third_origin)
	.unwrap();

	assert_eq!(res_account, Here.into());
}

pub fn orml_xcm_one_four_councli_works<
	Origin: OriginTrait
		+ From<RawOrigin<AccountId>>
		+ Clone
		+ std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
	I,
>()
where
	std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		Origin,
	>: std::convert::From<Origin>,
	Origin: std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		> + std::convert::From<pallet_collective::RawOrigin<sp_runtime::AccountId32, I>>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		Origin,
	>: std::convert::From<Origin>,
{
	let one_four_origin: Origin =
		pallet_collective::RawOrigin::<AccountId, I>::Members(1, 4).into();

	let should_failed = <FilterEnsureOrigin<
		Origin,
		LocalOriginToLocation,
		EnsureRootOrTwoThirdsCouncil,
	> as EnsureOrigin<Origin>>::try_origin(one_four_origin).is_err();

	assert!(should_failed);

}

pub fn orml_xcm_half_councli_works<
	Origin: OriginTrait
		+ From<RawOrigin<AccountId>>
		+ Clone
		+ std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
	I,
>()
where
	std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
		Origin,
	>: std::convert::From<Origin>,
	Origin: std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		> + std::convert::From<pallet_collective::RawOrigin<sp_runtime::AccountId32, I>>,
	std::result::Result<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		Origin,
	>: std::convert::From<Origin>,
{
	let half_origin: Origin = pallet_collective::RawOrigin::<AccountId, I>::Members(1, 2).into();

	let should_failed = <FilterEnsureOrigin<
		Origin,
		LocalOriginToLocation,
		EnsureRootOrTwoThirdsCouncil,
	> as EnsureOrigin<Origin>>::try_origin(half_origin).is_err();

	assert!(should_failed);
}

pub fn orml_xcm_member_works<
	R: BaseRuntimeRequirements + frame_system::Config<Call = Call>,
	Origin: OriginTrait
	+ From<RawOrigin<AccountId>>
	+ Clone
	+ std::convert::From<
		pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
	> + std::fmt::Debug,
	LocalOriginToLocation: Convert<Origin, MultiLocation>,
	Call: Clone + Dispatchable<Origin = Origin> + From<pallet_balances::Call<R>>,
	I,
>()
	where
		<<R as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source:
		From<sp_runtime::AccountId32>,

		std::result::Result<frame_system::RawOrigin<sp_runtime::AccountId32>, Origin>:
		std::convert::From<Origin>,
		std::result::Result<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance1>,
			Origin,
		>: std::convert::From<Origin>,
		Origin: std::convert::From<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
		>+ std::convert::From<pallet_collective::RawOrigin<sp_runtime::AccountId32, I>>,
		std::result::Result<
			pallet_collective::RawOrigin<sp_runtime::AccountId32, pallet_balances::Instance2>,
			Origin,
		>: std::convert::From<Origin>,
{

	ExtBuilder::<R>::default().build().execute_with(|| {
		let member: Origin =
			pallet_collective::RawOrigin::<AccountId, I>::Member(bob()).into();

		let should_failed = <FilterEnsureOrigin<
			Origin,
			LocalOriginToLocation,
			EnsureRootOrTwoThirdsCouncil,
		> as EnsureOrigin<Origin>>::try_origin(member).is_err();

		assert!(should_failed);
	})
}