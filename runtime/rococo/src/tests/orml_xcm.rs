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

use crate::{
	xcm_config::LocalOriginToLocation, CouncilInstance, Runtime, RuntimeCall, RuntimeOrigin,
};
use runtime_common::tests::orml_xcm;

#[test]
fn orml_xcm_root_works() {
	orml_xcm::orml_xcm_root_works::<Runtime, RuntimeOrigin, LocalOriginToLocation, RuntimeCall>();
}

#[test]
fn orml_xcm_signed_works() {
	orml_xcm::orml_xcm_signed_works::<Runtime, RuntimeOrigin, LocalOriginToLocation, RuntimeCall>();
}

#[test]
fn orml_xcm_two_thirds_council_works() {
	orml_xcm::orml_xcm_two_thirds_councli_works::<
		Runtime,
		RuntimeOrigin,
		LocalOriginToLocation,
		RuntimeCall,
		CouncilInstance,
	>();
}

#[test]
fn orml_xcm_one_four_council_works() {
	orml_xcm::orml_xcm_one_four_councli_works::<
		Runtime,
		RuntimeOrigin,
		LocalOriginToLocation,
		RuntimeCall,
		CouncilInstance,
	>();
}

#[test]
fn orml_xcm_half_council_works() {
	orml_xcm::orml_xcm_half_councli_works::<
		Runtime,
		RuntimeOrigin,
		LocalOriginToLocation,
		RuntimeCall,
		CouncilInstance,
	>();
}

#[test]
fn orml_xcm_member_works() {
	orml_xcm::orml_xcm_member_works::<
		Runtime,
		RuntimeOrigin,
		LocalOriginToLocation,
		RuntimeCall,
		CouncilInstance,
	>();
}
