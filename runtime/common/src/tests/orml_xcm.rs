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

use crate::{EnsureRootOrTwoThirdsCouncil, FilterEnsureOrigin};
use primitives::*;
use xcm::latest::prelude::*;
use xcm_executor::traits::Convert;

pub fn orml_xcm_works<
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
{
	let test_origin = frame_system::RawOrigin::Root;
	let res_account = <FilterEnsureOrigin<
		Origin,
		LocalOriginToLocation,
		EnsureRootOrTwoThirdsCouncil,
	> as EnsureOrigin<Origin>>::try_origin(Origin::from(test_origin))
	.unwrap();

	assert_eq!(res_account, Here.into());
}
