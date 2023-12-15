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

#[cfg(all(not(feature = "sgx"), feature = "std"))]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, MaxEncodedLen};
use itp_utils::hex::decode_hex;
use pallet_evm::{AddressMapping, HashedAddressMapping as GenericHashedAddressMapping};
use parentchain_primitives::{AccountId, Web3Network};
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, ecdsa, ed25519, sr25519, ByteArray, H160};
use sp_io::hashing::blake2_256;
use sp_runtime::{
	traits::{BlakeTwo256, ConstU32},
	BoundedVec,
};
use sp_std::vec::Vec;
use strum_macros::EnumIter;

pub type MaxStringLength = ConstU32<64>;
pub type IdentityString = BoundedVec<u8, MaxStringLength>;
pub type HashedAddressMapping = GenericHashedAddressMapping<BlakeTwo256>;

#[derive(Encode, Decode, Copy, Clone, Debug, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

impl<'a> TryFrom<&'a [u8]> for Address20 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address20, ()> {
		if x.len() == 20 {
			let mut data = [0; 20];
			data.copy_from_slice(x);
			Ok(Address20(data))
		} else {
			Err(())
		}
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

impl From<AccountId32> for Address32 {
	fn from(value: AccountId32) -> Self {
		let raw: [u8; 32] = value.as_slice().try_into().unwrap();
		Address32::from(raw)
	}
}

impl<'a> TryFrom<&'a [u8]> for Address32 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address32, ()> {
		if x.len() == 32 {
			let mut data = [0; 32];
			data.copy_from_slice(x);
			Ok(Address32(data))
		} else {
			Err(())
		}
	}
}

impl From<Address32> for AccountId32 {
	fn from(value: Address32) -> Self {
		let raw: [u8; 32] = *value.as_ref();
		AccountId32::from(raw)
	}
}

impl From<&Address32> for AccountId32 {
	fn from(value: &Address32) -> Self {
		(*value).into()
	}
}

impl From<sr25519::Public> for Address32 {
	fn from(k: sr25519::Public) -> Self {
		k.0.into()
	}
}

impl From<ed25519::Public> for Address32 {
	fn from(k: ed25519::Public) -> Self {
		k.0.into()
	}
}

// TODO: maybe use macros to reduce verbosity
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Address33([u8; 33]);
impl AsRef<[u8; 33]> for Address33 {
	fn as_ref(&self) -> &[u8; 33] {
		&self.0
	}
}

impl Default for Address33 {
	fn default() -> Self {
		Address33([0u8; 33])
	}
}

impl From<[u8; 33]> for Address33 {
	fn from(value: [u8; 33]) -> Self {
		Self(value)
	}
}

impl<'a> TryFrom<&'a [u8]> for Address33 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address33, ()> {
		if x.len() == 33 {
			let mut data = [0; 33];
			data.copy_from_slice(x);
			Ok(Address33(data))
		} else {
			Err(())
		}
	}
}

impl From<Address33> for ecdsa::Public {
	fn from(value: Address33) -> Self {
		let raw: [u8; 33] = *value.as_ref();
		ecdsa::Public::from_raw(raw)
	}
}

impl From<&Address33> for ecdsa::Public {
	fn from(value: &Address33) -> Self {
		(*value).into()
	}
}

impl From<ecdsa::Public> for Address33 {
	fn from(k: ecdsa::Public) -> Self {
		k.0.into()
	}
}

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter)]
pub enum Identity {
	// web2
	Twitter(IdentityString),
	Discord(IdentityString),
	Github(IdentityString),

	// web3
	Substrate(Address32),
	Evm(Address20),
	// bitcoin addresses are derived (one-way hash) from the pubkey
	// by using `Address33` as the Identity handle, it requires that pubkey
	// is retrievable by the wallet API when verifying the bitcoin account.
	// e.g. unisat-wallet: https://docs.unisat.io/dev/unisat-developer-service/unisat-wallet#getpublickey
	Bitcoin(Address33),
}

impl Identity {
	pub fn is_web2(&self) -> bool {
		matches!(self, Self::Twitter(..) | Self::Discord(..) | Self::Github(..))
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Self::Substrate(..) | Self::Evm(..) | Self::Bitcoin(..))
	}

	pub fn is_substrate(&self) -> bool {
		matches!(self, Self::Substrate(..))
	}

	pub fn is_evm(&self) -> bool {
		matches!(self, Self::Evm(..))
	}

	pub fn is_bitcoin(&self) -> bool {
		matches!(self, Self::Bitcoin(..))
	}

	// check if the given web3networks match the identity
	pub fn matches_web3networks(&self, networks: &Vec<Web3Network>) -> bool {
		(self.is_substrate() && !networks.is_empty() && networks.iter().all(|n| n.is_substrate()))
			|| (self.is_evm() && !networks.is_empty() && networks.iter().all(|n| n.is_evm()))
			|| (self.is_bitcoin()
				&& !networks.is_empty()
				&& networks.iter().all(|n| n.is_bitcoin()))
			|| (self.is_web2() && networks.is_empty())
	}

	/// Currently we only support mapping from Address32/Address20 to AccountId, not opposite.
	pub fn to_account_id(&self) -> Option<AccountId> {
		match self {
			Identity::Substrate(address) => Some(address.into()),
			Identity::Evm(address) =>
				Some(HashedAddressMapping::into_account_id(H160::from_slice(address.as_ref()))),
			Identity::Bitcoin(address) => Some(blake2_256(address.as_ref()).into()),
			_ => None,
		}
	}

	#[cfg(any(feature = "std", feature = "sgx"))]
	pub fn from_did(s: &str) -> Result<Self, std::boxed::Box<dyn std::error::Error + 'static>> {
		let did_prefix = std::string::String::from("did:litentry:");
		if s.starts_with(&did_prefix) {
			let did_suffix = &s[did_prefix.len()..];
			let v: Vec<&str> = did_suffix.split(':').collect();
			if v.len() == 2 {
				if v[0] == "substrate" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address32 conversion error")?;
					return Ok(Identity::Substrate(handle))
				} else if v[0] == "evm" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address20 conversion error")?;
					return Ok(Identity::Evm(handle))
				} else if v[0] == "bitcoin" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address33 conversion error")?;
					return Ok(Identity::Bitcoin(handle))
				} else if v[0] == "github" {
					return Ok(Identity::Github(
						v[1].as_bytes()
							.to_vec()
							.try_into()
							.map_err(|_| "github conversion error")?,
					))
				} else if v[0] == "discord" {
					return Ok(Identity::Discord(
						v[1].as_bytes()
							.to_vec()
							.try_into()
							.map_err(|_| "discord conversion error")?,
					))
				} else if v[0] == "twitter" {
					return Ok(Identity::Twitter(
						v[1].as_bytes()
							.to_vec()
							.try_into()
							.map_err(|_| "twitter conversion error")?,
					))
				} else {
					return Err("Unknown did type".into())
				}
			} else {
				return Err("Wrong did suffix".into())
			}
		}

		Err("Wrong did prefix".into())
	}
}

impl From<ed25519::Public> for Identity {
	fn from(value: ed25519::Public) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<sr25519::Public> for Identity {
	fn from(value: sr25519::Public) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<AccountId32> for Identity {
	fn from(value: AccountId32) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<Address32> for Identity {
	fn from(value: Address32) -> Self {
		Identity::Substrate(value)
	}
}

impl From<Address20> for Identity {
	fn from(value: Address20) -> Self {
		Identity::Evm(value)
	}
}

impl From<Address33> for Identity {
	fn from(value: Address33) -> Self {
		Identity::Bitcoin(value)
	}
}

impl From<[u8; 32]> for Identity {
	fn from(value: [u8; 32]) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<[u8; 20]> for Identity {
	fn from(value: [u8; 20]) -> Self {
		Identity::Evm(value.into())
	}
}

impl From<[u8; 33]> for Identity {
	fn from(value: [u8; 33]) -> Self {
		Identity::Bitcoin(value.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_std::vec;
	use strum::IntoEnumIterator;

	#[test]
	fn is_web2_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_web2(),
				match identity {
					Identity::Twitter(..) => true,
					Identity::Discord(..) => true,
					Identity::Github(..) => true,
					Identity::Substrate(..) => false,
					Identity::Evm(..) => false,
					Identity::Bitcoin(..) => false,
				}
			)
		})
	}

	#[test]
	fn is_web3_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_web3(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => true,
					Identity::Evm(..) => true,
					Identity::Bitcoin(..) => true,
				}
			)
		})
	}

	#[test]
	fn is_substrate_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_substrate(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => true,
					Identity::Evm(..) => false,
					Identity::Bitcoin(..) => false,
				}
			)
		})
	}

	#[test]
	fn is_evm_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_evm(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => false,
					Identity::Evm(..) => true,
					Identity::Bitcoin(..) => false,
				}
			)
		})
	}

	#[test]
	fn is_bitcoin_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_bitcoin(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => false,
					Identity::Evm(..) => false,
					Identity::Bitcoin(..) => true,
				}
			)
		})
	}

	#[test]
	fn matches_web3networks_works() {
		// web2 identity
		let mut id = Identity::Twitter("alice".as_bytes().to_vec().try_into().unwrap());
		let mut networks: Vec<Web3Network> = vec![];
		assert!(id.matches_web3networks(&networks));
		networks = vec![Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));

		// substrate identity
		id = Identity::Substrate(Default::default());
		networks = vec![];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::Bsc, Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::Litentry, Web3Network::Kusama];
		assert!(id.matches_web3networks(&networks));

		// evm identity
		id = Identity::Evm(Default::default());
		networks = vec![];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::Bsc, Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::Bsc, Web3Network::Ethereum];
		assert!(id.matches_web3networks(&networks));
	}
}
