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

use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

use crate::assertion::network::{all_evm_web3networks, Web3Network};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum PlatformUserType {
	#[codec(index = 0)]
	KaratDaoUser,
	#[codec(index = 1)]
	MagicCraftStakingUser,
}

impl PlatformUserType {
	pub fn get_supported_networks(&self) -> Vec<Web3Network> {
		match self {
			Self::KaratDaoUser | Self::MagicCraftStakingUser => all_evm_web3networks(),
		}
	}
}
