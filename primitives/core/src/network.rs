// Copyright 2020-2023 Trust Computing GmbH.
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
use strum_macros::{EnumIter, IntoStaticStr};

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
	IntoStaticStr,
)]
pub enum Web3Network {
	// substrate
	#[codec(index = 0)]
	Polkadot,
	#[codec(index = 1)]
	Kusama,
	#[codec(index = 2)]
	Litentry,
	#[codec(index = 3)]
	Litmus,
	#[codec(index = 4)]
	LitentryRococo,
	#[codec(index = 5)]
	Khala,
	#[codec(index = 6)]
	SubstrateTestnet, // when launched it with standalone (integritee-)node

	// evm
	#[codec(index = 7)]
	Ethereum,
	#[codec(index = 8)]
	Bsc,

	// btc, see https://github.com/rust-bitcoin/rust-bitcoin/blob/9ea3e29d61569479b7b4618c8ae1992612f3d01a/bitcoin/src/address/mod.rs#L64-L75
	#[codec(index = 9)]
	BitcoinP2tr,
	#[codec(index = 10)]
	BitcoinP2pkh,
	#[codec(index = 11)]
	BitcoinP2sh,
	#[codec(index = 12)]
	BitcoinP2wpkh,
	#[codec(index = 13)]
	BitcoinP2wsh,
}

// mainly used in CLI
impl TryFrom<&str> for Web3Network {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Web3Network::iter()
			.find(|n| <Self as Into<&'static str>>::into(*n).to_lowercase() == value.to_lowercase())
			.ok_or(())
	}
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
		matches!(self, Self::Ethereum | Self::Bsc)
	}

	pub fn is_bitcoin(&self) -> bool {
		matches!(
			self,
			Self::BitcoinP2tr |
				Self::BitcoinP2pkh |
				Self::BitcoinP2sh |
				Self::BitcoinP2wpkh |
				Self::BitcoinP2wsh
		)
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

pub fn all_bitcoin_web3networks() -> Vec<Web3Network> {
	Web3Network::iter().filter(|n| n.is_bitcoin()).collect()
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
					Web3Network::BitcoinP2tr => false,
					Web3Network::BitcoinP2pkh => false,
					Web3Network::BitcoinP2sh => false,
					Web3Network::BitcoinP2wpkh => false,
					Web3Network::BitcoinP2wsh => false,
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
					Web3Network::BitcoinP2tr => false,
					Web3Network::BitcoinP2pkh => false,
					Web3Network::BitcoinP2sh => false,
					Web3Network::BitcoinP2wpkh => false,
					Web3Network::BitcoinP2wsh => false,
				}
			)
		})
	}

	#[test]
	fn is_bitcoin_works() {
		Web3Network::iter().for_each(|network| {
			assert_eq!(
				network.is_bitcoin(),
				match network {
					Web3Network::Polkadot => false,
					Web3Network::Kusama => false,
					Web3Network::Litentry => false,
					Web3Network::Litmus => false,
					Web3Network::LitentryRococo => false,
					Web3Network::Khala => false,
					Web3Network::SubstrateTestnet => false,
					Web3Network::Ethereum => false,
					Web3Network::Bsc => false,
					Web3Network::BitcoinP2tr => true,
					Web3Network::BitcoinP2pkh => true,
					Web3Network::BitcoinP2sh => true,
					Web3Network::BitcoinP2wpkh => true,
					Web3Network::BitcoinP2wsh => true,
				}
			)
		})
	}

	#[test]
	fn try_from_str_works() {
		let mut n: Result<Web3Network, ()> = "polkadot".try_into();
		assert_eq!(n.unwrap(), Web3Network::Polkadot);
		n = "poLkAdOt".try_into();
		assert_eq!(n.unwrap(), Web3Network::Polkadot);
		n = "NonExist".try_into();
		assert_eq!(n, Err(()))
	}
}
