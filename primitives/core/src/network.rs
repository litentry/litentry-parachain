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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use sp_std::vec::Vec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub const MAX_WEB3NETWORK_LEN: u32 = 128;
pub type BoundedWeb3Network = BoundedVec<Web3Network, ConstU32<MAX_WEB3NETWORK_LEN>>;

/// supported web3 networks
/// use a flattened style to avoid overly nested structure like:
/// `
/// 	Substrate(SubstrateNetwork),
/// 	Evm(EvmNetwork),
/// `
/// TODO: theoretically this should the the union of the supported networks of all data providers
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter)]
pub enum Web3Network {
	// substrate
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
	LitentryRococo,
	Khala,
	SubstrateTestnet, // when launched it with standalone (integritee-)node

	// evm
	Ethereum,
	Polygon,
	BSC,
}

impl Web3Network {
	pub fn is_substrate(&self) -> bool {
		matches!(
			self,
			Self::Polkadot |
				Self::Kusama | Self::Litentry |
				Self::Litmus | Self::LitentryRococo |
				Self::Khala | Self::SubstrateTestnet
		)
	}

	pub fn is_evm(&self) -> bool {
		matches!(self, Self::Ethereum | Self::Polygon | Self::BSC)
	}
}

pub fn get_all_web3networks() -> Vec<Web3Network> {
	Web3Network::iter().collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn web3network_length_ok() {
		let networks: Vec<Web3Network> = Web3Network::iter().collect();
		assert!(networks.len() < MAX_WEB3NETWORK_LEN.try_into().unwrap());
	}

	#[test]
	fn web3network_parsing_works() {
		let litentry_network = Web3Network::Litentry;
		assert!(litentry_network.is_substrate());

		let bsc_network = Web3Network::BSC;
		assert!(bsc_network.is_evm());
	}
}
