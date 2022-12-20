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

use derive_more::Display;

#[derive(Copy, Clone, Debug, Display)]
pub struct Ed25519Seal;

#[cfg(feature = "sgx")]
pub use sgx::*;

#[cfg(feature = "sgx")]
pub mod sgx {

	use super::*;
	use crate::error::{Error, Result};
	use codec::Encode;
	use itp_settings::files::SEALED_SIGNER_SEED_FILE;
	use itp_sgx_io::{seal, unseal, SealedIO, StaticSealedIO};
	use log::*;
	use sgx_rand::{Rng, StdRng};
	use sgx_tstd::vec::Vec;
	use sp_core::{crypto::Pair, ed25519};
	use std::{path::Path, sgxfs::SgxFile};

	impl StaticSealedIO for Ed25519Seal {
		type Error = Error;
		type Unsealed = ed25519::Pair;

		fn unseal_from_static_file() -> Result<ed25519::Pair> {
			let raw = unseal(SEALED_SIGNER_SEED_FILE)?;

			let key = ed25519::Pair::from_seed_slice(&raw)
				.map_err(|e| Error::Other(format!("{:?}", e).into()))?;

			Ok(key.into())
		}

		fn seal_to_static_file(unsealed: &Self::Unsealed) -> Result<()> {
			Ok(unsealed.seed().using_encoded(|bytes| seal(bytes, SEALED_SIGNER_SEED_FILE))?)
		}
	}

	impl SealedIO for Ed25519Seal {
		type Error = Error;
		type Unsealed = ed25519::Pair;

		fn unseal(&self) -> Result<Self::Unsealed> {
			Self::unseal_from_static_file()
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			Self::seal_to_static_file(unsealed)
		}
	}

	pub fn create_sealed_if_absent() -> Result<()> {
		if SgxFile::open(SEALED_SIGNER_SEED_FILE).is_err() {
			if Path::new(SEALED_SIGNER_SEED_FILE).exists() {
				panic!("[Enclave] Keyfile {} exists but can't be opened. has it been written by the same enclave?", SEALED_SIGNER_SEED_FILE);
			}
			info!("[Enclave] Keyfile not found, creating new! {}", SEALED_SIGNER_SEED_FILE);
			return create_sealed_seed()
		}
		Ok(())
	}

	pub fn create_sealed_seed() -> Result<()> {
		let mut seed = [0u8; 32];
		let mut rand = StdRng::new()?;
		rand.fill_bytes(&mut seed);

		Ok(seal(&seed, SEALED_SIGNER_SEED_FILE)?)
	}

	pub fn sign(payload: &Vec<u8>) -> Result<(Vec<u8>, Vec<u8>)> {
		let signer = Ed25519Seal::unseal_from_static_file()?;
		let signed_value = signer.sign(payload.as_slice());

		let mut public_key = [0_u8; 32];
		public_key.clone_from_slice(&signer.public().0);

		Ok((signed_value.0.to_vec(), public_key.to_vec()))
	}

	pub fn verify(sig: &ed25519::Signature, msg: &Vec<u8>) -> Result<bool> {
		let signer = Ed25519Seal::unseal_from_static_file()?;
		let ret = ed25519::Pair::verify(sig, msg, &ed25519::Public(signer.public().0));
		Ok(ret)
	}
}

#[cfg(feature = "test")]
pub mod tests {

	use super::*;
	use sp_core::ed25519::Signature;

	pub fn sign_verify_works() {
		let payload = "payload".as_bytes().to_vec();
		let (sig, _pubk) = sign(&payload).unwrap();
		let ret = verify(&Signature::from_slice(&sig).unwrap(), &payload).unwrap();

		assert_eq!(ret, true);
	}
}
