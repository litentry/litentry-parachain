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

#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::DispatchError;

pub type InfoHash = H256;
pub type CuratorIndex = u128;
pub type PoolProposalIndex = u128;

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolSetting<BlockNumber, Balance> {
	// The start time of staking pool
	pub start_time: BlockNumber,
	// How many epoch will staking pool last, n > 0, valid epoch index :[0..n)
	pub epoch: u128,
	// How many blocks each epoch consist
	pub epoch_range: BlockNumber,
	// The number of block regarding setup for purchasing hardware which deliver no non-native
	// token reward
	pub setup_time: BlockNumber,
	// Max staked amount of pool
	pub pool_cap: Balance,
	// Minimum amount of token required for pool starting
	pub minimum_cap: Balance,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct PoolMetadata<BoundedString> {
	/// The user friendly name of this staking pool. Limited in length by `PoolStringLimit`.
	pub name: BoundedString,
	/// The short description for this staking pool. Limited in length by `PoolStringLimit`.
	pub description: BoundedString,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode, Debug, TypeInfo)]
pub enum CandidateStatus {
	/// Initial status of legal file
	#[default]
	#[codec(index = 0)]
	Unverified,
	/// Checked and authorized status of legal file
	#[codec(index = 1)]
	Verified,
	/// Legal file suspicious and banned
	#[codec(index = 2)]
	Banned,
}
