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

#[cfg(test)]
pub mod orml_xcm_test {

	// use polkadot_runtime_parachains::Origin;

	use frame_support::{
		parameter_types,
		dispatch::RawOrigin,
		traits::{EnsureOrigin, OriginTrait},
		*,
	};

	use sp_std::marker::PhantomData;
	use xcm::latest::prelude::*;
	use xcm_executor::traits::Convert;

	// use xcm_builder::SignedToAccountId32;

	use primitives::*;
	use sp_std::prelude::*;

	use crate::EnsureRootOrTwoThirdsCouncil;
	parameter_types! {
		pub const RelayNetwork: NetworkId = NetworkId::Any;
	}
	
	// type  LocalOriginToLocation = SignedToAccountId32::<Origin, AccountId, RelayNetwork>;

	struct Filterensureorigin<Origin, Conversion, SpecialGroup>(
		PhantomData<(Origin, Conversion, SpecialGroup)>,
	);
	impl<
			Origin: OriginTrait + Clone,
			Conversion: Convert<Origin, MultiLocation>,
			SpecialGroup: EnsureOrigin<Origin>,
		> EnsureOrigin<Origin> for Filterensureorigin<Origin, Conversion, SpecialGroup>
	where
		Origin::PalletsOrigin: PartialEq,
	{
		type Success = MultiLocation;
		fn try_origin(o: Origin) -> Result<Self::Success, Origin> {
			// root can send the cross chain message

			let o = match SpecialGroup::try_origin(o) {
				Ok(_) => return Ok(Here.into()),
				Err(o) => o,
			};

			let o = match Conversion::convert(o) {
				Ok(location) => return Ok(location),
				Err(o) => o,
			};

			if o.caller() == Origin::root().caller() {
				Ok(Here.into())
			} else {
				Err(o)
			}
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn successful_origin() -> Origin {
			Origin::root()
		}
	}

	#[cfg(test)]
	pub fn orml_xcm_works<
		Origin: OriginTrait + From<RawOrigin<AccountId>> + Clone,
		LocalOriginToLocation: Convert<Origin, MultiLocation>,
	>() {
		
		let test_origin = frame_system::RawOrigin::Root;
		let res_account = Filterensureorigin::<Origin, LocalOriginToLocation, EnsureRootOrTwoThirdsCouncil>::try_origin(test_origin).unwrap();
	
		assert_eq!(res_account,Here);
	}
}
