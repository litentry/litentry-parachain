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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

mod aes;
mod ethereum_signature;
mod identity;
mod validation_data;

pub use aes::*;
pub use ethereum_signature::*;
pub use identity::*;
pub use validation_data::*;

use codec::{Decode, Encode, MaxEncodedLen};
use log::error;
pub use parentchain_primitives::{
	AccountId as ParentchainAccountId, AesOutput, Assertion, Balance as ParentchainBalance,
	BlockNumber as ParentchainBlockNumber, ErrorDetail, ErrorString, Hash as ParentchainHash,
	Header as ParentchainHeader, IMPError, Index as ParentchainIndex, IndexingNetworks,
	IntoErrorDetail, ParameterString, SchemaContentString, SchemaIdString,
	Signature as ParentchainSignature, SupportedNetwork, UserShieldingKeyNonceType,
	UserShieldingKeyType, VCMPError, ASSERTION_FROM_DATE, MAX_TAG_LEN, MINUTES, NONCE_LEN,
	USER_SHIELDING_KEY_LEN,
};
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, ecdsa, ed25519, sr25519, ByteArray, Hasher};
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::keccak_256};
use sp_runtime::traits::{BlakeTwo256, Verify};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum LitentryMultiAddress {
	Substrate(Address32),
	Evm(Address20),
}

impl From<LitentryMultiAddress> for AccountId32 {
	fn from(value: LitentryMultiAddress) -> Self {
		(&value).into()
	}
}

impl From<&LitentryMultiAddress> for AccountId32 {
	fn from(value: &LitentryMultiAddress) -> Self {
		match value {
			LitentryMultiAddress::Substrate(address) => {
				let mut data = [0u8; 32];
				data.copy_from_slice(address.as_ref());
				Self::from(data)
			},
			LitentryMultiAddress::Evm(address) => {
				let mut data = [0u8; 24];
				data[0..4].copy_from_slice(b"evm:");
				data[4..24].copy_from_slice(address.as_ref());
				let hash = BlakeTwo256::hash(&data);
				Self::from(Into::<[u8; 32]>::into(hash))
			},
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum LitentryMultiSignature {
	/// An Ed25519 signature.
	Ed25519(ed25519::Signature),
	/// An Sr25519 signature.
	Sr25519(sr25519::Signature),
	/// An ECDSA/SECP256k1 signature.
	Ecdsa(ecdsa::Signature),
	/// An ECDSA/keccak256 signature. An Ethereum signature. hash message with keccak256
	Ethereum(EthereumSignature),
}

impl LitentryMultiSignature {
	pub fn verify(&self, msg: &[u8], signer: &LitentryMultiAddress) -> bool {
		match signer {
			LitentryMultiAddress::Substrate(address) => self.verify_substrate(msg, address),
			LitentryMultiAddress::Evm(address) => self.verify_evm(msg, address),
		}
	}

	fn verify_substrate(&self, msg: &[u8], signer: &Address32) -> bool {
		match (self, signer) {
			(Self::Ed25519(ref sig), who) => match ed25519::Public::from_slice(who.as_ref()) {
				Ok(signer) => sig.verify(msg, &signer),
				Err(()) => false,
			},
			(Self::Sr25519(ref sig), who) => match sr25519::Public::from_slice(who.as_ref()) {
				Ok(signer) => sig.verify(msg, &signer),
				Err(()) => false,
			},
			(Self::Ecdsa(ref sig), who) => {
				let m = sp_io::hashing::blake2_256(msg);
				match sp_io::crypto::secp256k1_ecdsa_recover_compressed(sig.as_ref(), &m) {
					Ok(pubkey) =>
						&sp_io::hashing::blake2_256(pubkey.as_ref())
							== <dyn AsRef<[u8; 32]>>::as_ref(who),
					_ => false,
				}
			},
			_ => false,
		}
	}

	fn verify_evm(&self, msg: &[u8], signer: &Address20) -> bool {
		match (self, signer) {
			(Self::Ethereum(ref sig), who) => {
				let digest = keccak_256(msg);
				return match recover_evm_address(&digest, sig.as_ref()) {
					Ok(recovered_evm_address) => recovered_evm_address == who.as_ref().as_slice(),
					Err(_e) => {
						error!(
							"Could not verify evm signature msg: {:?}, signer {:?}",
							msg, signer
						);
						false
					},
				}
			},
			_ => false,
		}
	}
}

impl From<ed25519::Signature> for LitentryMultiSignature {
	fn from(x: ed25519::Signature) -> Self {
		Self::Ed25519(x)
	}
}

impl From<sr25519::Signature> for LitentryMultiSignature {
	fn from(x: sr25519::Signature) -> Self {
		Self::Sr25519(x)
	}
}

impl From<ecdsa::Signature> for LitentryMultiSignature {
	fn from(x: ecdsa::Signature) -> Self {
		Self::Ecdsa(x)
	}
}

pub fn recover_evm_address(
	msg: &[u8; 32],
	sig: &[u8; 65],
) -> core::result::Result<[u8; 20], sp_io::EcdsaVerifyError> {
	let pubkey = secp256k1_ecdsa_recover(sig, msg)?;
	let hashed_pk = keccak_256(&pubkey);

	let mut addr = [0u8; 20];
	addr[..20].copy_from_slice(&hashed_pk[12..32]);
	Ok(addr)
}
