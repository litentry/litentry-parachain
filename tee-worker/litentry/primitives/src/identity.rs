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

use core::fmt::{Debug, Formatter};
#[cfg(all(not(feature = "sgx"), feature = "std"))]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, Error, Input, MaxEncodedLen};
use itp_utils::if_production_or;
use pallet_evm::{AddressMapping, HashedAddressMapping as GenericHashedAddressMapping};
use parentchain_primitives::{AccountId, Web3Network};
use scale_info::{meta_type, Type, TypeDefSequence, TypeInfo};
use sp_core::{crypto::AccountId32, ed25519, sr25519, ByteArray, H160};
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentityString {
	#[cfg_attr(feature = "std", serde(flatten))]
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

impl Debug for Address20 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address20").finish(),
			f.debug_tuple("Address20").field(&self.0).finish()
		)
	}
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
	// web2
	Twitter(IdentityString),
	Discord(IdentityString),
	Github(IdentityString),

	// web3
	Substrate(Address32),
	Evm(Address20),
}

impl Identity {
	pub fn is_web2(&self) -> bool {
		matches!(self, Self::Twitter(..) | Self::Discord(..) | Self::Github(..))
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Self::Substrate(..) | Self::Evm(..))
	}

	pub fn is_substrate(&self) -> bool {
		matches!(self, Self::Substrate(..))
	}

	pub fn is_evm(&self) -> bool {
		matches!(self, Self::Evm(..))
	}

	// check if the given web3networks match the identity
	pub fn matches_web3networks(&self, networks: &Vec<Web3Network>) -> bool {
		(self.is_substrate() && !networks.is_empty() && networks.iter().all(|n| n.is_substrate()))
			|| (self.is_evm() && !networks.is_empty() && networks.iter().all(|n| n.is_evm()))
			|| (self.is_web2() && networks.is_empty())
	}

	/// Currently we only support mapping from Address32/Address20 to AccountId, not opposite.
	pub fn to_account_id(&self) -> Option<AccountId> {
		match self {
			Identity::Substrate(address) => {
				let mut data = [0u8; 32];
				data.copy_from_slice(address.as_ref());
				Some(AccountId32::from(data))
			},
			Identity::Evm(address) => {
				let substrate_version =
					HashedAddressMapping::into_account_id(H160::from_slice(address.as_ref()));
				Some(AccountId32::from(Into::<[u8; 32]>::into(substrate_version)))
			},
			_ => None,
		}
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
}
