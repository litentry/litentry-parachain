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

use aes::cipher::generic_array::GenericArray;
use k256::{
	elliptic_curve::group::GroupEncoding,
	schnorr::{SigningKey, VerifyingKey},
	AffinePoint, PublicKey, Secp256k1,
};

use std::{println, vec::Vec};

/// File name of the sealed seed file.
pub const SEALED_SIGNER_SEED_FILE: &str = "schnorr_key_sealed.bin";

#[derive(Clone)]
pub struct Pair {
	pub public: PublicKey,
	pub secret: SigningKey,
}

impl Pair {
	pub fn public_bytes(&self) -> Vec<u8> {
		self.public.as_affine().to_bytes().as_slice().to_vec()
	}
}

#[cfg(feature = "sgx")]
pub mod sgx {
	use super::SEALED_SIGNER_SEED_FILE;
	use crate::{
		error::{Error, Result},
		key_repository::KeyRepository,
		schnorr::Pair,
		std::string::ToString,
	};
	use itp_sgx_io::{seal, unseal, SealedIO};
	use k256::{
		schnorr::{signature::Signer, Signature, SigningKey, VerifyingKey},
		PublicKey,
	};
	use log::*;
	use sgx_rand::{Rng, StdRng};
	use std::{path::PathBuf, string::String};

	pub fn schnorr_sign(pair: Pair, payload: &[u8]) -> Result<[u8; 64]> {
		let signature: Signature =
			pair.secret.try_sign(payload).map_err(|e| Error::Other(e.to_string().into()))?;

		Ok(signature.to_bytes().into())
	}

	/// Creates a repository for schnorr keypair and initializes
	/// a fresh private key if it doesn't exist at `path`.
	pub fn create_schnorr_repository(
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
	}

	impl Seal {
		fn unseal_pair(&self) -> Result<Pair> {
			self.unseal()
		}

		fn exists(&self) -> bool {
			self.path().exists()
		}

		fn init(&self) -> Result<Pair> {
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
			let secret = SigningKey::from_bytes(&raw)
				.map_err(|e| Error::Other(format!("{:?}", e).into()))?;
			let public = PublicKey::from(secret.verifying_key().clone());
			Ok(Pair { public, secret })
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			let raw = unsealed.secret.to_bytes();
			seal(&raw, self.path()).map_err(|e| e.into())
		}
	}
}

#[cfg(feature = "test")]
pub mod sgx_tests {
	use super::sgx::*;
	use crate::{key_repository::AccessKey, schnorr::Pair, std::string::ToString, ToPubkey};
	use itp_sgx_temp_dir::TempDir;
	use k256::PublicKey;
	use std::path::{Path, PathBuf};

	#[test]
	pub fn creating_repository_with_same_path_and_prefix_results_in_same_key() {
		let key_file_prefix = "test";
		fn get_key_from_repo(path: PathBuf, prefix: &str) -> Pair {
			create_schnorr_repository(path, prefix).unwrap().retrieve_key().unwrap()
		}
		let temp_dir = TempDir::with_prefix(
			"creating_repository_with_same_path_and_prefix_results_in_same_key",
		)
		.unwrap();
		let temp_path = temp_dir.path().to_path_buf();
		assert_eq!(
			get_key_from_repo(temp_path.clone(), key_file_prefix),
			get_key_from_repo(temp_path.clone(), key_file_prefix)
		);
	}

	#[test]
	pub fn seal_init_should_create_new_key_if_not_present() {
		//given
		let temp_dir =
			TempDir::with_prefix("seal_init_should_create_new_key_if_not_present").unwrap();
		let seal = Seal::new(temp_dir.path().to_path_buf(), "test".to_string());
		assert!(!seal.exists());

		//when
		seal.init().unwrap();

		//then
		assert!(seal.exists());
	}

	#[test]
	pub fn seal_init_should_not_change_key_if_exists() {
		//given
		let temp_dir = TempDir::with_prefix("seal_init_should_not_change_key_if_exists").unwrap();
		let seal = Seal::new(temp_dir.path().to_path_buf(), "test".to_string());
		let pair = seal.init().unwrap();

		//when
		let new_pair = seal.init().unwrap();

		//then
		assert!(pair, new_pair);
	}
}
