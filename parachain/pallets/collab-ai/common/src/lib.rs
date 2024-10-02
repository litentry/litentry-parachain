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
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{Get, RuntimeDebug, H256};
use sp_runtime::BoundedVec;
use sp_std::marker::PhantomData;

use frame_support::pallet_prelude::EnsureOrigin;
use frame_system::RawOrigin;

pub type InfoHash = H256;
pub type CuratorIndex = u128;
pub type GuardianIndex = u128;
pub type PoolProposalIndex = u128;
pub type InvestingPoolIndex = u128;

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

#[derive(
	PartialEq,
	Eq,
	Clone,
	Copy,
	Default,
	Encode,
	Decode,
	Debug,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, TypeInfo)]
pub enum GuardianVote {
	/// Does not care if this guardian get selected
	/// Please be aware Neutral will increase participate percentage
	/// which will increase the winning rate of guardian selection
	/// given a large amount of guardian competitor
	#[default]
	#[codec(index = 0)]
	Neutral,
	/// Want this guardian no matter which pool proposal
	#[codec(index = 1)]
	Aye,
	/// Against this guardian no matter which pool proposal
	#[codec(index = 2)]
	Nay,
	/// Support this guardian for only specific pool proposal
	/// And neutral for other pool proposal
	#[codec(index = 3)]
	Specific(PoolProposalIndex),
}

/// Some sort of check on the account is from some group.
pub trait CuratorQuery<AccountId> {
	/// All curator but banned ones
	fn is_curator(account: AccountId) -> bool;

	/// Only verified one
	fn is_verified_curator(account: AccountId) -> bool;
}

pub struct EnsureSignedAndCurator<AccountId, EC>(PhantomData<(AccountId, EC)>);
impl<
		O: Into<Result<RawOrigin<AccountId>, O>> + From<RawOrigin<AccountId>>,
		AccountId: Decode + Clone,
		EC,
	> EnsureOrigin<O> for EnsureSignedAndCurator<AccountId, EC>
where
	EC: CuratorQuery<AccountId>,
{
	type Success = AccountId;
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			RawOrigin::Signed(who) => {
				if EC::is_curator(who.clone()) {
					Ok(who)
				} else {
					Err(O::from(RawOrigin::Signed(who)))
				}
			},
			r => Err(O::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<O, ()> {
		// No promised successful_origin
		Err(())
	}
}

pub const INVESTING_POOL_INDEX_SHIFTER: u128 = 1_000_000_000_000_000;
pub const INVESTING_POOL_START_MONTH_SHIFTER: u128 = 1_000;
pub const INVESTING_POOL_END_MONTH_SHIFTER: u128 = 1;

// pub struct InvestingPoolAssetIdGenerator<AssetId>(PhantomData<AssetId>);
// impl<AssetId: From<u128>> InvestingPoolAssetIdGenerator<AssetId> {
// 	/// Create a series of new asset id based on pool index and reward epoch
// 	/// Return None if impossible to generate. e.g. overflow
// 	pub fn get_pool_token(pool_index: InvestingPoolIndex, epoch: u128) -> Option<Vec<AssetId>> {
// 		let pool_index_prefix = pool_index.checked_mul(INVESTING_POOL_INDEX_SHIFTER)?;

// 		let mut vec: Vec<AssetId> = Vec::new();
// 		for n in 0..(epoch + 1) {
// 			// vec.push(pool_index_prefix + )
// 		}
// 		None
// 	}
// }

/// Some sort of check on the account is from some group.
pub trait GuardianQuery<AccountId, S: Get<u32>> {
	/// All guardian but banned ones
	fn is_guardian(account: AccountId) -> bool;

	/// Only verified one
	fn is_verified_guardian(account: AccountId) -> bool;

	/// Get vote
	fn get_vote(voter: AccountId, guardian: AccountId) -> Option<GuardianVote>;
}
