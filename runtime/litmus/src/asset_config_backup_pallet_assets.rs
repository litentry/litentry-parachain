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

use super::xcm_config::CurrencyId;
use super::{
	AccountId, Balance, Balances,
	CouncilCollective, Event, Runtime,
};

use frame_support::{
	parameter_types,
	traits::EnsureOneOf,
};

use frame_system::EnsureRoot;

pub type ForeignAssetInstance = pallet_assets::Instance1;

// TODO: implmentation needed
pub type AssetId = u128;
// For foreign assets, these parameters dont matter much
// as this will only be called by root with the forced arguments
// No deposit is substracted with those methods
// For local assets, they do matter. We use similar parameters
// to those in statemine (except for approval)
parameter_types! {
    // TODO figure out these six number setting properly
	pub const AssetDeposit: Balance = 100;
	pub const ApprovalDeposit: Balance = 0;
	pub const AssetsStringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 100;
	pub const MetadataDepositPerByte: Balance = 100;
    pub const AssetAccountDeposit: Balance = 100;
}

/// We allow root and Chain council to execute privileged asset operations.
pub type AssetsForceOrigin = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

// Foreign assets
impl pallet_assets::Config<ForeignAssetInstance> for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = AssetsForceOrigin;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type Freezer = ();
	type Extra = ();
	type AssetAccountDeposit = AssetAccountDeposit;
    // TODO: Weight file for pallet_assets
	type WeightInfo = ();
}
