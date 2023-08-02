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
use sp_std::vec::Vec;
pub use validation_data::*;

use codec::{Decode, Encode, MaxEncodedLen};
use log::error;
pub use parentchain_primitives::{
	all_evm_web3networks, all_substrate_web3networks, all_web3networks,
	AccountId as ParentchainAccountId, AesOutput, Assertion, Balance as ParentchainBalance,
	BlockNumber as ParentchainBlockNumber, BoundedWeb3Network, ErrorDetail, ErrorString,
	Hash as ParentchainHash, Header as ParentchainHeader, IMPError, Index as ParentchainIndex,
	IntoErrorDetail, ParameterString, SchemaContentString, SchemaIdString,
	Signature as ParentchainSignature, UserShieldingKeyNonceType, UserShieldingKeyType, VCMPError,
	Web3Network, ASSERTION_FROM_DATE, MAX_TAG_LEN, MINUTES, NONCE_LEN, USER_SHIELDING_KEY_LEN,
};
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, sr25519, ByteArray};
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::keccak_256};
use sp_runtime::traits::Verify;
use std::string::ToString;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

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
	pub fn verify(&self, msg: &[u8], signer: &Identity) -> bool {
		match signer {
			Identity::Substrate(address) =>
				self.verify_substrate(substrate_wrap(msg).as_slice(), address)
					|| self.verify_substrate(msg, address),
			Identity::Evm(address) =>
				self.verify_evm(evm_eip191_wrap(msg).as_slice(), address)
					|| self.verify_evm(msg, address),
			_ => false,
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

// see https://github.com/litentry/litentry-parachain/issues/1137
fn substrate_wrap(msg: &[u8]) -> Vec<u8> {
	["<Bytes>".as_bytes(), msg, "</Bytes>".as_bytes()].concat()
}

// see https://github.com/litentry/litentry-parachain/issues/1970
fn evm_eip191_wrap(msg: &[u8]) -> Vec<u8> {
	["\x19Ethereum Signed Message:\n".as_bytes(), msg.len().to_string().as_bytes(), msg].concat()
}

pub type IdentityNetworkTuple = (Identity, Vec<Web3Network>);
