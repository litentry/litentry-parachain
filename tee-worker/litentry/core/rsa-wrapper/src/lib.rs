// Copyright 2020-2024 Trust Computing GmbH.
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

//! The new rust-SDK has some problems with RSA key creation/encryption under `ucrypto` (non-enclave) feature
//! See https://github.com/apache/incubator-teaclave-sgx-sdk/issues/456
//!
//! This simple crate uses the `rsa` crate to implement the RSA encryption in an alternative way.
//! It's a temporary workaround and can be ditched once the rust-sdk fixes the problem.
//!
//! Please note this crate is supposed to be used for non-enclave part only (e.g. CLI)

#[cfg(feature = "sgx")]
compile_error!("rsa-wrapper crate should not be used under sgx feature");

use num_traits::cast::FromPrimitive;
use rsa::{
	sha2::Sha256,
	traits::{PrivateKeyParts, PublicKeyParts},
	BigUint, Oaep, RsaPrivateKey, RsaPublicKey,
};
use sgx_crypto::rsa::{Rsa3072KeyPair, Rsa3072PrivateKey, Rsa3072PublicKey};
use sgx_types::{
	error::{SgxResult, SgxStatus},
	types::{Rsa3072Key, Rsa3072PubKey},
};

pub trait RsaWrapperCreate {
	type KeyType;
	fn create_with_rsa_wrapper() -> SgxResult<Self::KeyType>;
}

pub trait RsaWrapperEncrypt {
	fn encrypt_with_rsa_wrapper(&self, msg: &[u8]) -> SgxResult<Vec<u8>>;
}

pub trait RsaWrapperDecrypt {
	fn decrypt_with_rsa_wrapper(&self, ciphertext: &[u8]) -> SgxResult<Vec<u8>>;
}

impl RsaWrapperCreate for Rsa3072KeyPair {
	type KeyType = Rsa3072KeyPair;
	fn create_with_rsa_wrapper() -> SgxResult<Self> {
		let mut rng = rand::rngs::OsRng;
		let exp = BigUint::from_u64(65537).unwrap();
		let privkey =
			RsaPrivateKey::new_with_exp(&mut rng, 3072, &exp).map_err(|_| SgxStatus::Unexpected)?;
		let keypair = Rsa3072Key {
			modulus: privkey.n().to_bytes_le().try_into().unwrap(),
			e: [1, 0, 1, 0], // 65537
			d: privkey.d().to_bytes_le().try_into().unwrap(),
		};
		Ok(keypair.into())
	}
}

impl RsaWrapperEncrypt for Rsa3072PublicKey {
	fn encrypt_with_rsa_wrapper(&self, msg: &[u8]) -> SgxResult<Vec<u8>> {
		let pubkey: Rsa3072PubKey = self.public_key();
		let n = BigUint::from_bytes_le(pubkey.modulus.as_ref());
		let e = BigUint::from_bytes_le(pubkey.exponent.as_ref());
		let pubkey = RsaPublicKey::new(n, e).map_err(|_| SgxStatus::Unexpected)?;
		let mut rng = rand::rngs::OsRng;
		pubkey
			.encrypt(&mut rng, Oaep::new::<Sha256>(), msg)
			.map_err(|_| SgxStatus::Unexpected)
	}
}

impl RsaWrapperDecrypt for Rsa3072PrivateKey {
	fn decrypt_with_rsa_wrapper(&self, ciphertext: &[u8]) -> SgxResult<Vec<u8>> {
		let privkey = self.private_key();
		let n = BigUint::from_bytes_le(privkey.modulus.as_ref());
		let e = BigUint::from_bytes_le(privkey.e.as_ref());
		let d = BigUint::from_bytes_le(privkey.d.as_ref());
		let privkey = RsaPrivateKey::from_components(n, e, d, Default::default())
			.map_err(|_| SgxStatus::Unexpected)?;
		privkey
			.decrypt(Oaep::new::<Sha256>(), ciphertext)
			.map_err(|_| SgxStatus::Unexpected)
	}
}
