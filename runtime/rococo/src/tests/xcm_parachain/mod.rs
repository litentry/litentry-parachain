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

use runtime_common::tests::{xcm_parachain, xcm_parachain::TestXCMRequirements};

use crate::{
	tests::{
		Call as RelayCall, Origin as RelayOrigin, ParaA, ParaB, Relay, RelayChainRuntime, TestNet,
	},
	xcm_config::{LocationToAccountId, UnitWeightCost},
	Call, Origin, Runtime,
};

pub mod relay_sproof_builder;

struct RococoTestXCMRequirements;

impl TestXCMRequirements for RococoTestXCMRequirements {
	type ParaOrigin = Origin;
	type ParaCall = Call;
	type ParaRuntime = Runtime;
	type ParaA = ParaA;
	type ParaB = ParaB;
	type Relay = Relay;
	type RelayOrigin = RelayOrigin;
	type RelayCall = RelayCall;
	type RelayRuntime = RelayChainRuntime;
	type UnitWeightCost = UnitWeightCost;
	type LocationToAccountId = LocationToAccountId;

	fn reset() {
		TestNet::reset()
	}
}

#[test]
fn test_xtokens_recognize_multilocation() {
	xcm_parachain::test_xtokens_recognize_multilocation::<RococoTestXCMRequirements>();
}

// If this test fail, at least some part of XCM fee rule changes
#[test]
fn test_xtokens_weight_parameter() {
	xcm_parachain::test_xtokens_weight_parameter::<RococoTestXCMRequirements>();
}

#[test]
fn test_pallet_xcm_recognize_multilocation() {
	xcm_parachain::test_pallet_xcm_recognize_multilocation::<RococoTestXCMRequirements>();
}

#[test]
fn test_methods_xtokens_expected_succeed() {
	xcm_parachain::test_methods_xtokens_expected_succeed::<RococoTestXCMRequirements>();
}

#[test]
fn test_methods_xtokens_expected_fail() {
	xcm_parachain::test_methods_xtokens_expected_fail::<RococoTestXCMRequirements>();
}

#[test]
fn test_methods_pallet_xcm_expected_succeed() {
	xcm_parachain::test_methods_pallet_xcm_expected_succeed::<RococoTestXCMRequirements>();
}

#[test]
fn test_methods_pallet_xcm_expected_fail() {
	xcm_parachain::test_methods_pallet_xcm_expected_fail::<RococoTestXCMRequirements>();
}

// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_between_sibling() {
	xcm_parachain::test_pallet_xcm_send_capacity_between_sibling::<RococoTestXCMRequirements>();
}

// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_without_transact() {
	xcm_parachain::test_pallet_xcm_send_capacity_without_transact::<RococoTestXCMRequirements>();
}

// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
#[test]
fn test_pallet_xcm_send_capacity_relay_manipulation() {
	xcm_parachain::test_pallet_xcm_send_capacity_relay_manipulation::<RococoTestXCMRequirements>();
}

// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
#[test]
fn test_pallet_xcm_send_capacity_parachain_manipulation() {
	xcm_parachain::test_pallet_xcm_send_capacity_parachain_manipulation::<RococoTestXCMRequirements>(
	);
}
