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

pub extern crate alloc;

use crate::{AccountId, Web3Network};
use alloc::{format, str, string::String};
use codec::{Decode, Encode, Error, Input, MaxEncodedLen};
use core::fmt::{Debug, Formatter};
use itp_utils::{
	hex::{decode_hex, hex_encode},
	if_production_or,
};
use pallet_evm::{AddressMapping, HashedAddressMapping as GenericHashedAddressMapping};
use scale_info::{meta_type, Type, TypeDefSequence, TypeInfo};
use sp_core::{
	crypto::{AccountId32, ByteArray},
	ecdsa, ed25519, sr25519, H160,
};
use sp_io::hashing::blake2_256;
use sp_runtime::{
	traits::{BlakeTwo256, ConstU32},
	BoundedVec,
};
use sp_std::vec::Vec;
use strum_macros::EnumIter;

pub type MaxStringLength = ConstU32<64>;
pub type IdentityInnerString = BoundedVec<u8, MaxStringLength>;

pub type HashedAddressMapping = GenericHashedAddressMapping<BlakeTwo256>;

impl Decode for IdentityString {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		let inner: BoundedVec<u8, MaxStringLength> = Decode::decode(input)?;
		Ok(IdentityString { inner })
	}
}

impl Encode for IdentityString {
	fn encode(&self) -> Vec<u8> {
		let mut res = Vec::new();
		self.inner.encode_to(&mut res);
		res
	}
}

#[derive(Eq, PartialEq, Clone, MaxEncodedLen, Default)]
pub struct IdentityString {
	pub inner: IdentityInnerString,
}

impl TypeInfo for IdentityString {
	type Identity = BoundedVec<u8, MaxStringLength>;

	fn type_info() -> Type {
		TypeDefSequence::new(meta_type::<u8>()).into()
	}
}

impl IdentityString {
	pub fn new(inner: Vec<u8>) -> Self {
		IdentityString { inner: BoundedVec::truncate_from(inner) }
	}

	pub fn inner_ref(&self) -> &[u8] {
		self.inner.as_ref()
	}
}

impl Debug for IdentityString {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_struct("IdentityString").finish(),
			f.debug_struct("IdentityString").field("inner", &self.inner).finish()
		)
	}
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

impl Debug for Address20 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address20").finish(),
			f.debug_tuple("Address20").field(&self.0).finish()
		)
	}
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

impl Debug for Address32 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address32").finish(),
			f.debug_tuple("Address32").field(&self.0).finish()
		)
	}
}

// TODO: maybe use macros to reduce verbosity
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

impl Debug for Address33 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address33").finish(),
			f.debug_tuple("Address33").field(&self.0).finish()
		)
	}
}

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter)]
pub enum Identity {
	// web2
	#[codec(index = 0)]
	Twitter(IdentityString),
	#[codec(index = 1)]
	Discord(IdentityString),
	#[codec(index = 2)]
	Github(IdentityString),

	// web3
	#[codec(index = 3)]
	Substrate(Address32),
	#[codec(index = 4)]
	Evm(Address20),
	// bitcoin addresses are derived (one-way hash) from the pubkey
	// by using `Address33` as the Identity handle, it requires that pubkey
	// is retrievable by the wallet API when verifying the bitcoin account.
	// e.g. unisat-wallet: https://docs.unisat.io/dev/unisat-developer-service/unisat-wallet#getpublickey
	#[codec(index = 5)]
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
		(self.is_substrate() && !networks.is_empty() && networks.iter().all(|n| n.is_substrate())) ||
			(self.is_evm() && !networks.is_empty() && networks.iter().all(|n| n.is_evm())) ||
			(self.is_bitcoin() &&
				!networks.is_empty() &&
				networks.iter().all(|n| n.is_bitcoin())) ||
			(self.is_web2() && networks.is_empty())
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

	pub fn from_did(s: &str) -> Result<Self, &'static str> {
		let did_prefix = String::from("did:litentry:");
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
					return Ok(Identity::Github(IdentityString::new(v[1].as_bytes().to_vec())))
				} else if v[0] == "discord" {
					return Ok(Identity::Discord(IdentityString::new(v[1].as_bytes().to_vec())))
				} else if v[0] == "twitter" {
					return Ok(Identity::Twitter(IdentityString::new(v[1].as_bytes().to_vec())))
				} else {
					return Err("Unknown did type".into())
				}
			} else {
				return Err("Wrong did suffix".into())
			}
		}

		Err("Wrong did prefix".into())
	}

	pub fn to_did(&self) -> Result<String, &'static str> {
		Ok(format!(
			"did:litentry:{}",
			match self {
				Identity::Evm(address) => format!("evm:{}", &hex_encode(address.as_ref())),
				Identity::Substrate(address) =>
					format!("substrate:{}", &hex_encode(address.as_ref())),
				Identity::Bitcoin(address) => format!("bitcoin:{}", &hex_encode(address.as_ref())),
				Identity::Twitter(handle) => format!(
					"twitter:{}",
					str::from_utf8(handle.inner_ref())
						.map_err(|_| "twitter handle conversion error")?
				),
				Identity::Discord(handle) => format!(
					"discord:{}",
					str::from_utf8(handle.inner_ref())
						.map_err(|_| "discord handle conversion error")?
				),
				Identity::Github(handle) => format!(
					"github:{}",
					str::from_utf8(handle.inner_ref())
						.map_err(|_| "github handle conversion error")?
				),
			}
		))
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
	use codec::DecodeAll;
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
		let mut id = Identity::Twitter(IdentityString::new("alice".as_bytes().to_vec()));
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

	#[test]
	fn test_encode_identity_string() {
		//it should be encoded to inner representation
		let identity_string = IdentityString::new("mock_user".as_bytes().to_vec());
		let inner: IdentityInnerString = BoundedVec::truncate_from("mock_user".as_bytes().to_vec());
		assert_eq!(inner.encode(), identity_string.encode())
	}

	#[test]
	fn test_decode_identity_string() {
		let decoded: Vec<u8> = vec![36, 109, 111, 99, 107, 95, 117, 115, 101, 114];
		let identity_string: IdentityString =
			IdentityString::decode_all(&mut decoded.as_slice()).unwrap();
		assert_eq!(identity_string, IdentityString::new("mock_user".as_bytes().to_vec()));
	}

	#[test]
	fn test_substrate_did() {
		let identity = Identity::Substrate([0; 32].into());
		let did_str = "did:litentry:substrate:0x0000000000000000000000000000000000000000000000000000000000000000";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}

	#[test]
	fn test_evm_did() {
		let identity = Identity::Evm([0; 20].into());
		let did_str = "did:litentry:evm:0x0000000000000000000000000000000000000000";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}

	#[test]
	fn test_bitcoin_did() {
		let identity = Identity::Bitcoin([0; 33].into());
		let did_str = "did:litentry:bitcoin:0x000000000000000000000000000000000000000000000000000000000000000000";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}

	#[test]
	fn test_discord_did() {
		let identity = Identity::Discord(IdentityString::new("discord_handle".as_bytes().to_vec()));
		let did_str = "did:litentry:discord:discord_handle";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}

	#[test]
	fn test_twitter_did() {
		let identity = Identity::Twitter(IdentityString::new("twitter_handle".as_bytes().to_vec()));
		let did_str = "did:litentry:twitter:twitter_handle";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}

	#[test]

	fn test_github_did() {
		let identity = Identity::Github(IdentityString::new("github_handle".as_bytes().to_vec()));
		let did_str = "did:litentry:github:github_handle";
		assert_eq!(identity.to_did().unwrap(), did_str);
		assert_eq!(Identity::from_did(did_str).unwrap(), identity);
	}
}
