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

// Define the user shielding key constant and struct
// Put it in primitives as it will be used by multiple pallets/external crates

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
use sp_std::prelude::*;

use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type ShardIdentifier = sp_core::H256;
pub type MrenclaveType = [u8; 32];

// we use 256-bit AES-GCM as user shielding key
// TODO: use constants from `ring` crate, e.g. ring::aead::NONCE_LEN
pub const USER_SHIELDING_KEY_LEN: usize = 32;
pub use ring::aead::{MAX_TAG_LEN, NONCE_LEN};

pub type UserShieldingKeyType = [u8; USER_SHIELDING_KEY_LEN];
pub type UserShieldingKeyNonceType = [u8; NONCE_LEN];

// all-in-one struct containing the encrypted ciphertext with user's
// shielding key and other metadata that is required for decryption
//
// by default a postfix tag is used => last 16 bytes of ciphertext is MAC tag
#[derive(Debug, Default, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct AesOutput {
	pub ciphertext: Vec<u8>,
	pub aad: Vec<u8>,
	pub nonce: UserShieldingKeyNonceType, // IV
}
