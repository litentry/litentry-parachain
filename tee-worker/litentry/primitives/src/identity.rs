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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(any(feature = "std", feature = "sgx"))]
use std::format;

#[cfg(all(not(feature = "sgx"), feature = "std"))]
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "std", feature = "sgx"))]
use sp_core::hexdisplay::HexDisplay;
#[cfg(any(feature = "std", feature = "sgx"))]
use std::vec::Vec;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

pub type MaxStringLength = ConstU32<64>;
pub type IdentityString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address20([u8; 20]);

impl AsRef<[u8; 20]> for Address20 {
	fn as_ref(&self) -> &[u8; 20] {
		&self.0
	}
}

impl From<[u8; 20]> for Address20 {
	fn from(value: [u8; 20]) -> Self {
		Self(value)
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address32([u8; 32]);
impl AsRef<[u8; 32]> for Address32 {
	fn as_ref(&self) -> &[u8; 32] {
		&self.0
	}
}

impl From<[u8; 32]> for Address32 {
	fn from(value: [u8; 32]) -> Self {
		Self(value)
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SubstrateNetwork {
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
	LitentryRococo,
	Khala,
	TestNet, // when we launch it with standalone (integritee-)node
}

impl SubstrateNetwork {
	/// get the ss58 prefix, see https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json
	pub fn ss58_prefix(&self) -> u16 {
		match self {
			Self::Polkadot => 0,
			Self::Kusama => 2,
			Self::Litentry => 31,
			Self::Litmus => 131,
			Self::LitentryRococo => 42,
			Self::Khala => 30,
			Self::TestNet => 13,
		}
	}

	pub fn from_ss58_prefix(prefix: u16) -> Self {
		match prefix {
			0 => Self::Polkadot,
			2 => Self::Kusama,
			31 => Self::Litentry,
			131 => Self::Litmus,
			42 => Self::LitentryRococo,
			30 => Self::Khala,
			_ => Self::TestNet,
		}
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EvmNetwork {
	Ethereum,
	BSC,
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web2Network {
	Twitter,
	Discord,
	Github,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
	Substrate { network: SubstrateNetwork, address: Address32 },
	Evm { network: EvmNetwork, address: Address20 },
	Web2 { network: Web2Network, address: IdentityString },
}

impl Identity {
	#[cfg(any(feature = "std", feature = "sgx"))]
	pub fn flat(&self) -> Vec<u8> {
		match &self {
			Identity::Substrate { network, address } => {
				let mut data = format!("did:{:?}:web3:substrate:", network)
					.to_ascii_lowercase()
					.as_bytes()
					.to_vec();
				let mut suffix =
					format!("0x{}", HexDisplay::from(address.as_ref())).as_bytes().to_vec();
				data.append(&mut suffix);
				data
			},
			Identity::Evm { network, address } => {
				let mut data =
					format!("did:{:?}:web3:evm:", network).to_ascii_lowercase().as_bytes().to_vec();
				let mut suffix =
					format!("0x{}", HexDisplay::from(address.as_ref())).as_bytes().to_vec();
				data.append(&mut suffix);
				data
			},
			Identity::Web2 { network, address } => {
				let mut data =
					format!("did:{:?}:web2:_:", network).to_ascii_lowercase().as_bytes().to_vec();
				let mut suffix = address.to_vec();
				data.append(&mut suffix);
				data
			},
		}
	}

	pub fn is_web2(&self) -> bool {
		matches!(self, Identity::Web2 { .. })
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Identity::Evm { .. } | Identity::Substrate { .. })
	}
}

#[cfg(test)]
mod tests {
	use crate::{Identity, IdentityString, SubstrateNetwork, Web2Network};
	use sp_core::Pair;

	#[test]
	fn identity() {
		let sub_pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		// let eth_pair = sp_core::ed25519::Pair::from_string("//Alice", None).unwrap();
		let polkadot_identity: Identity = Identity::Substrate {
			network: SubstrateNetwork::Polkadot,
			address: sub_pair.public().0.into(),
		};

		let twitter_identity: Identity = Identity::Web2 {
			network: Web2Network::Twitter,
			address: IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
		};

		assert_eq!(
			"did:polkadot:web3:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			String::from_utf8(polkadot_identity.flat()).unwrap()
		);
		assert_eq!(
			"did:twitter:web2:_:litentry",
			String::from_utf8(twitter_identity.flat()).unwrap()
		);
	}
}
