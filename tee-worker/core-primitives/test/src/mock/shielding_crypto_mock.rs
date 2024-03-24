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
use std::vec::Vec;

// TODO(Litentry):
// `Rsa3072KeyPair::create()` seems to be broken for `ucrypto` feature under v2.0 SDK, see
// https://github.com/apache/incubator-teaclave-sgx-sdk/issues/456

#[derive(Clone)]
pub struct ShieldingCryptoMock {
	key: Rsa3072KeyPair,
}

#[allow(clippy::derivable_impls)]
impl Default for ShieldingCryptoMock {
	fn default() -> Self {
		ShieldingCryptoMock { key: Rsa3072KeyPair::default() }
	}
}

impl ShieldingCryptoEncrypt for ShieldingCryptoMock {
	type Error = itp_sgx_crypto::Error;

	fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
		// self.key
		// 	.encrypt(data)
		// 	.map_err(|e| itp_sgx_crypto::Error::Other(format!("encrypt error: {:?}", e).into()))
		Ok(data.to_vec())
	}
}

impl ShieldingCryptoDecrypt for ShieldingCryptoMock {
	type Error = itp_sgx_crypto::Error;

	fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
		// self.key
		// 	.decrypt(data)
		// 	.map_err(|e| itp_sgx_crypto::Error::Other(format!("decrypt error: {:?}", e).into()))
		Ok(data.to_vec())
	}
}

impl DeriveEd25519 for ShieldingCryptoMock {
	fn derive_ed25519(&self) -> Result<Ed25519Pair, itp_sgx_crypto::error::Error> {
		self.key.derive_ed25519()
	}
}
