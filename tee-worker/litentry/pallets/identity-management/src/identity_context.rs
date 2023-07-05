// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::{BlockNumberOf, Config, Web3Network};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum IdentityStatus {
	#[default]
	Active,
	Inactive,
}

// The context associated with the (litentry-account, did) pair
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct IdentityContext<T: Config> {
	// the sidechain block number at which the identity is linked
	pub link_block: BlockNumberOf<T>,
	// a list of web3 networks on which the identity should be used
	pub web3networks: BoundedVec<Web3Network, T::MaxWeb3NetworkLength>,
	// the identity status
	pub status: IdentityStatus,
}

impl<T: Config> IdentityContext<T> {
	pub fn new(
		link_block: BlockNumberOf<T>,
		web3networks: BoundedVec<Web3Network, T::MaxWeb3NetworkLength>,
	) -> Self {
		Self { link_block, web3networks, status: IdentityStatus::Active }
	}
}
