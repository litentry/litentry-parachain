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
use sp_std::{hash::Hash, vec::Vec};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub const MAX_WEB3NETWORK_LEN: u32 = 128;
pub type BoundedWeb3Network = BoundedVec<Web3Network, ConstU32<MAX_WEB3NETWORK_LEN>>;

/// supported web3 networks
/// use a flattened style to avoid overly nested structure like:
/// {
///   Substrate(SubstrateNetwork),
///   Evm(EvmNetwork),
/// }
/// TODO: theoretically this should the the union of the supported networks of all data providers
#[derive(
	Encode,
	Decode,
	Copy,
	Clone,
	Debug,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	TypeInfo,
	MaxEncodedLen,
	EnumIter,
)]
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
	Bsc,
}

impl Web3Network {
	pub fn get_name(self) -> &'static str {
		match self {
			Web3Network::Polkadot => "Polkadot",
			Web3Network::Kusama => "Kusama",
			Web3Network::Litentry => "Litentry",
			Web3Network::Litmus => "Litmus",
			Web3Network::LitentryRococo => "LitentryRococo",
			Web3Network::Khala => "Khala",
			Web3Network::SubstrateTestnet => "SubstrateTestnet",
			Web3Network::Ethereum => "Ethereum",
			Web3Network::Bsc => "Bsc",
		}
	}

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
		matches!(self, Self::Ethereum | Self::Bsc)
	}
}

pub fn all_web3networks() -> Vec<Web3Network> {
	Web3Network::iter().collect()
}

pub fn all_substrate_web3networks() -> Vec<Web3Network> {
	Web3Network::iter().filter(|n| n.is_substrate()).collect()
}

pub fn all_evm_web3networks() -> Vec<Web3Network> {
	Web3Network::iter().filter(|n| n.is_evm()).collect()
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
	fn is_evm_works() {
		Web3Network::iter().for_each(|network| {
			assert_eq!(
				network.is_evm(),
				match network {
					Web3Network::Polkadot => false,
					Web3Network::Kusama => false,
					Web3Network::Litentry => false,
					Web3Network::Litmus => false,
					Web3Network::LitentryRococo => false,
					Web3Network::Khala => false,
					Web3Network::SubstrateTestnet => false,
					Web3Network::Ethereum => true,
					Web3Network::Bsc => true,
				}
			)
		})
	}

	#[test]
	fn is_substrate_works() {
		Web3Network::iter().for_each(|network| {
			assert_eq!(
				network.is_substrate(),
				match network {
					Web3Network::Polkadot => true,
					Web3Network::Kusama => true,
					Web3Network::Litentry => true,
					Web3Network::Litmus => true,
					Web3Network::LitentryRococo => true,
					Web3Network::Khala => true,
					Web3Network::SubstrateTestnet => true,
					Web3Network::Ethereum => false,
					Web3Network::Bsc => false,
				}
			)
		})
	}
}
