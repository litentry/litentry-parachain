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
#[cfg(feature = "sgx")]
pub use sgx::*;

use crate::error::{Error, Result};
use k256::{
	ecdsa::{SigningKey, VerifyingKey},
	elliptic_curve::group::GroupEncoding,
	PublicKey,
};
use std::string::ToString;

/// File name of the sealed seed file.
pub const SEALED_SIGNER_SEED_FILE: &str = "ecdsa_key_sealed.bin";

#[derive(Clone, PartialEq, Eq)]
pub struct Pair {
	pub public: PublicKey,
	private: SigningKey,
}

impl Pair {
	pub fn new(private: SigningKey) -> Self {
		let public = PublicKey::from(VerifyingKey::from(&private));
		Self { private, public }
	}

	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		let private_key = SigningKey::from_bytes(bytes.into())
			.map_err(|e| Error::Other(format!("{:?}", e).into()))?;
		Ok(Self::new(private_key))
	}

	pub fn public_bytes(&self) -> [u8; 33] {
		self.public.as_affine().to_bytes().as_slice().try_into().unwrap()
	}

	pub fn private_bytes(&self) -> [u8; 32] {
		self.private.to_bytes().as_slice().try_into().unwrap()
	}

	// sign the prehashed message
	pub fn sign_prehash_recoverable(&self, payload: &[u8]) -> Result<[u8; 65]> {
		let (signature, rid) = self
			.private
			.sign_prehash_recoverable(payload)
			.map_err(|e| Error::Other(e.to_string().into()))?;
		let mut bytes = [0u8; 65];
		bytes[..64].copy_from_slice(signature.to_vec().as_slice());
		bytes[64] = rid.to_byte();
		Ok(bytes)
	}
}

#[cfg(feature = "sgx")]
pub mod sgx {
	use super::SEALED_SIGNER_SEED_FILE;
	use crate::{
		ecdsa::Pair,
		error::{Error, Result},
		key_repository::KeyRepository,
		std::string::ToString,
	};
	use itp_sgx_io::{seal, unseal, SealedIO};
	use k256::ecdsa::SigningKey;
	use log::*;
	use sgx_rand::{Rng, StdRng};
	use std::{path::PathBuf, string::String};

	/// Creates a repository for ecdsa keypair and initializes
	/// a fresh private key if it doesn't exist at `path`.
	pub fn create_ecdsa_repository(
		path: PathBuf,
		key_file_prefix: &str,
	) -> Result<KeyRepository<Pair, Seal>> {
		let seal = Seal::new(path, key_file_prefix.to_string());
		Ok(KeyRepository::new(seal.init()?, seal.into()))
	}

	#[derive(Clone, Debug)]
	pub struct Seal {
		base_path: PathBuf,
		key_file_prefix: String,
	}

	impl Seal {
		pub fn new(base_path: PathBuf, key_file_prefix: String) -> Self {
			Self { base_path, key_file_prefix }
		}

		pub fn path(&self) -> PathBuf {
			self.base_path
				.join(self.key_file_prefix.clone() + "_" + SEALED_SIGNER_SEED_FILE)
		}

		fn unseal_pair(&self) -> Result<Pair> {
			self.unseal()
		}

		pub fn exists(&self) -> bool {
			self.path().exists()
		}

		pub fn init(&self) -> Result<Pair> {
			if !self.exists() {
				info!("Keyfile not found, creating new! {}", self.path().display());
				let mut seed = [0u8; 32];
				let mut rand = StdRng::new()?;
				rand.fill_bytes(&mut seed);
				seal(&seed, self.path())?;
			}
			self.unseal_pair()
		}
	}

	impl SealedIO for Seal {
		type Error = Error;
		type Unsealed = Pair;

		fn unseal(&self) -> Result<Self::Unsealed> {
			let raw = unseal(self.path())?;
			let secret = SigningKey::from_slice(&raw)
				.map_err(|e| Error::Other(format!("{:?}", e).into()))?;
			Ok(Pair::new(secret))
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			let raw = unsealed.private.to_bytes();
			seal(&raw, self.path()).map_err(|e| e.into())
		}
	}
}

#[cfg(feature = "test")]
pub mod sgx_tests {
	use crate::{
		create_ecdsa_repository, key_repository::AccessKey, std::string::ToString, Pair, Seal,
	};
	use itp_sgx_temp_dir::TempDir;
	use k256::ecdsa::VerifyingKey;
	use sgx_tstd::path::PathBuf;

	pub fn ecdsa_creating_repository_with_same_path_and_prefix_results_in_same_key() {
		//given
		let key_file_prefix = "test";
		fn get_key_from_repo(path: PathBuf, prefix: &str) -> Pair {
			create_ecdsa_repository(path, prefix).unwrap().retrieve_key().unwrap()
		}
		let temp_dir = TempDir::with_prefix(
			"creating_repository_with_same_path_and_prefix_results_in_same_key",
		)
		.unwrap();
		let temp_path = temp_dir.path().to_path_buf();

		//when
		let first_key = get_key_from_repo(temp_path.clone(), key_file_prefix);
		let second_key = get_key_from_repo(temp_path.clone(), key_file_prefix);

		//then
		assert_eq!(first_key.public, second_key.public);
	}

	pub fn ecdsa_seal_init_should_create_new_key_if_not_present() {
		//given
		let temp_dir =
			TempDir::with_prefix("ecdsa_seal_init_should_create_new_key_if_not_present").unwrap();
		let seal = Seal::new(temp_dir.path().to_path_buf(), "test".to_string());
		assert!(!seal.exists());

		//when
		seal.init().unwrap();

		//then
		assert!(seal.exists());
	}

	pub fn ecdsa_seal_init_should_not_change_key_if_exists() {
		//given
		let temp_dir =
			TempDir::with_prefix("ecdsa_seal_init_should_not_change_key_if_exists").unwrap();
		let seal = Seal::new(temp_dir.path().to_path_buf(), "test".to_string());
		let pair = seal.init().unwrap();

		//when
		let new_pair = seal.init().unwrap();

		//then
		assert_eq!(pair.public, new_pair.public);
	}

	pub fn ecdsa_sign_should_produce_valid_signature() {
		//given
		let temp_dir = TempDir::with_prefix("ecdsa_sign_should_produce_valid_signature").unwrap();
		let seal = Seal::new(temp_dir.path().to_path_buf(), "test".to_string());
		let pair = seal.init().unwrap();
		let message = [1u8; 32];

		//when
		let (signature, rid) = &pair.private.sign_prehash_recoverable(&message).unwrap();

		//then
		let verifying_key = VerifyingKey::recover_from_prehash(&message, signature, *rid).unwrap();
		assert_eq!(verifying_key, VerifyingKey::from(&pair.private));
	}
}
