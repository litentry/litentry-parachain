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

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SubstrateNetwork {
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EvmNetwork {
	Ethereum,
	BSC,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web3Network {
	Substrate(SubstrateNetwork),
	Evm(EvmNetwork),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web2Network {
	Twitter,
	Discord,
	Github,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IdentityWebType {
	Web2(Web2Network),
	Web3(Web3Network),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IdentityHandle {
	Address32([u8; 32]),
	Address20([u8; 20]),
	String(IdentityString),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Identity {
	pub web_type: IdentityWebType,
	pub handle: IdentityHandle,
}

impl Identity {
	#[cfg(any(feature = "std", feature = "sgx"))]
	pub fn flat(&self) -> Vec<u8> {
		let mut display = match &self.web_type {
			IdentityWebType::Web3(Web3Network::Substrate(sub)) =>
				format!("did:{:?}:web3:substrate:", sub)
					.to_ascii_lowercase()
					.as_bytes()
					.to_vec(),
			IdentityWebType::Web3(Web3Network::Evm(evm)) =>
				format!("did:{:?}:web3:evm:", evm).to_ascii_lowercase().as_bytes().to_vec(),
			IdentityWebType::Web2(web2) =>
				format!("did:{:?}:web2:_:", web2).to_ascii_lowercase().as_bytes().to_vec(),
		};
		let mut suffix: Vec<u8> = match &self.handle {
			IdentityHandle::String(inner) => inner.to_vec(),
			IdentityHandle::Address32(inner) =>
				format!("0x{}", HexDisplay::from(inner)).as_bytes().to_vec(),
			IdentityHandle::Address20(inner) =>
				format!("0x{}", HexDisplay::from(inner)).as_bytes().to_vec(),
		};
		display.append(&mut suffix);
		display
	}

	pub fn is_web2(&self) -> bool {
		match &self.web_type {
			IdentityWebType::Web2(_) => true,
			IdentityWebType::Web3(_) => false,
		}
	}

	pub fn is_web3(&self) -> bool {
		match &self.web_type {
			IdentityWebType::Web2(_) => false,
			IdentityWebType::Web3(_) => true,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		Identity, IdentityHandle, IdentityString, IdentityWebType, SubstrateNetwork, Web2Network,
		Web3Network,
	};
	use sp_core::Pair;
	use std::string;

	#[test]
	fn identity() {
		let sub_pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		// let eth_pair = sp_core::ed25519::Pair::from_string("//Alice", None).unwrap();
		let polkadot_identity = Identity {
			web_type: IdentityWebType::Web3(Web3Network::Substrate(SubstrateNetwork::Polkadot)),
			handle: IdentityHandle::Address32(sub_pair.public().0),
		};
		let twitter_identity = Identity {
			web_type: IdentityWebType::Web2(Web2Network::Twitter),
			handle: IdentityHandle::String(
				IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
			),
		};
		assert_eq!(
			"did:polkadot:web3:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			string::String::from_utf8(polkadot_identity.flat()).unwrap()
		);
		assert_eq!(
			"did:twitter:web2:_:litentry",
			string::String::from_utf8(twitter_identity.flat()).unwrap()
		);
	}
}
