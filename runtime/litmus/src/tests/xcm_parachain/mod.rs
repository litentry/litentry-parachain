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

// use super::setup::*;
use crate::{
	tests::{
		Call as RelayCall, Origin as RelayOrigin, ParaA, ParaB, Relay, RelayChainRuntime, TestNet,
	},
	xcm_config::{LocationToAccountId, UnitWeightCost},
	Call, Origin, Runtime,
};
pub mod relay_sproof_builder;

// pub const RELAY_UNIT: u128 = 1;

#[test]
fn test_xtokens_recognize_multilocation() {
	runtime_common::tests::xcm_parachain::test_xtokens_recognize_multilocation::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// If this test fail, at least some part of XCM fee rule changes
#[test]
fn test_xtokens_weight_parameter() {
	runtime_common::tests::xcm_parachain::test_xtokens_weight_parameter::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

#[test]
fn test_pallet_xcm_recognize_multilocation() {
	runtime_common::tests::xcm_parachain::test_pallet_xcm_recognize_multilocation::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

#[test]
fn test_methods_xtokens_expected_succeed() {
	runtime_common::tests::xcm_parachain::test_methods_xtokens_expected_succeed::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

#[test]
fn test_methods_xtokens_expected_fail() {
	runtime_common::tests::xcm_parachain::test_methods_xtokens_expected_fail::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

#[test]
fn test_methods_pallet_xcm_expected_succeed() {
	runtime_common::tests::xcm_parachain::test_methods_pallet_xcm_expected_succeed::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

#[test]
fn test_methods_pallet_xcm_expected_fail() {
	runtime_common::tests::xcm_parachain::test_methods_pallet_xcm_expected_fail::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// Send Xcm by root/individual on sibling to maniplulate XCM parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_between_sibling() {
	runtime_common::tests::xcm_parachain::test_pallet_xcm_send_capacity_between_sibling::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// Send Xcm by root/individual on relay to maniplulate xcm parachain soverign accounts
#[test]
fn test_pallet_xcm_send_capacity_without_transact() {
	runtime_common::tests::xcm_parachain::test_pallet_xcm_send_capacity_without_transact::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		RelayChainRuntime,
		RelayOrigin,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// Relay root manipulate its own sovereign account on Parachain A by Xcm::Transact (Flawed)
#[test]
fn test_pallet_xcm_send_capacity_relay_manipulation() {
	runtime_common::tests::xcm_parachain::test_pallet_xcm_send_capacity_relay_manipulation::<
		_,
		Origin,
		Call,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		RelayChainRuntime,
		RelayOrigin,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// Parachain root manipulate its own sovereign account on Relay by Xcm::Transact succeed
#[test]
fn test_pallet_xcm_send_capacity_parachain_manipulation() {
	runtime_common::tests::xcm_parachain::test_pallet_xcm_send_capacity_parachain_manipulation::<
		_,
		Origin,
		Runtime,
		ParaA,
		ParaB,
		Relay,
		RelayChainRuntime,
		RelayOrigin,
		RelayCall,
		UnitWeightCost,
		LocationToAccountId,
	>(|| TestNet::reset());
}

// TODO::figure out the other OriginKind scenario
