/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use itp_sgx_crypto::{
	ed25519_derivation::DeriveEd25519, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt,
};
use sgx_crypto::rsa::Rsa3072KeyPair;
use sp_core::ed25519::Pair as Ed25519Pair;
use std::{format, vec::Vec};

#[cfg(all(not(feature = "sgx"), feature = "std"))]
use lc_rsa_wrapper::{RsaWrapperCreate, RsaWrapperDecrypt, RsaWrapperEncrypt};

#[derive(Clone)]
pub struct ShieldingCryptoMock {
	key: Rsa3072KeyPair,
	// this flag forces to use dummy(identity) encryption and decryption, otherwise
	// `test_threadpool_behaviour` won't pass, the root cause is unclear, but I suspect that
	// it's related to multi-threaded key creation and encryption - the `encrypt` call never
	// returns on another thread (somehow).
	//
	// since this struct is used in `std` only and `lc_rsa_wrapper` is a workaround anyway,
	// I consider it fine for now
	use_dummy_enc_dec: bool,
}

impl Default for ShieldingCryptoMock {
	fn default() -> Self {
		Self::new(false)
	}
}

impl ShieldingCryptoMock {
	pub fn new(use_dummy_enc_dec: bool) -> Self {
		ShieldingCryptoMock {
			use_dummy_enc_dec,
			#[cfg(all(not(feature = "std"), feature = "sgx"))]
			key: Rsa3072KeyPair::create().expect("default RSA3072 key for shielding key mock"),
			#[cfg(all(not(feature = "sgx"), feature = "std"))]
			key: Rsa3072KeyPair::create_with_rsa_wrapper()
				.expect("default RSA3072 key for shielding key mock"),
		}
	}
}

impl ShieldingCryptoEncrypt for ShieldingCryptoMock {
	type Error = itp_sgx_crypto::Error;

	fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
		if self.use_dummy_enc_dec {
			return Ok(data.to_vec())
		}

		#[cfg(all(not(feature = "std"), feature = "sgx"))]
		return self
			.key
			.encrypt(data)
			.map_err(|e| itp_sgx_crypto::Error::Other(format!("encrypt error: {:?}", e).into()));

		#[cfg(all(not(feature = "sgx"), feature = "std"))]
		self.key
			.public_key()
			.encrypt_with_rsa_wrapper(data)
			.map_err(|e| itp_sgx_crypto::Error::Other(format!("encrypt error: {:?}", e).into()))
	}
}

impl ShieldingCryptoDecrypt for ShieldingCryptoMock {
	type Error = itp_sgx_crypto::Error;

	fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
		if self.use_dummy_enc_dec {
			return Ok(data.to_vec())
		}

		#[cfg(all(not(feature = "std"), feature = "sgx"))]
		return self
			.key
			.decrypt(data)
			.map_err(|e| itp_sgx_crypto::Error::Other(format!("decrypt error: {:?}", e).into()));

		#[cfg(all(not(feature = "sgx"), feature = "std"))]
		self.key
			.private_key()
			.decrypt_with_rsa_wrapper(data)
			.map_err(|e| itp_sgx_crypto::Error::Other(format!("decrypt error: {:?}", e).into()))
	}
}

impl DeriveEd25519 for ShieldingCryptoMock {
	fn derive_ed25519(&self) -> Result<Ed25519Pair, itp_sgx_crypto::error::Error> {
		self.key.derive_ed25519()
	}
}
