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

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::Pair;

	#[test]
	fn identity_parsing_works() {
		let pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		let twitter_identity =
			Identity::Twitter(IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap());
		let substrate_identity = Identity::Substrate(pair.public().0.into());
		let evm_identity = Identity::Evm([2u8; 20].into());

		assert!(twitter_identity.is_web2());

		assert!(substrate_identity.is_web3());
		assert!(substrate_identity.is_substrate());

		assert!(evm_identity.is_web3());
		assert!(evm_identity.is_evm());
	}
}
