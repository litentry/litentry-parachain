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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use sp_std::{boxed::Box, fmt::Debug};

use lazy_static::lazy_static;
use log::error;
use std::{collections::BTreeMap, error::Error, path::PathBuf, sync::Arc, vec::Vec};

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

lazy_static! {
	/// Global instance of a SignerRegistry
	pub static ref GLOBAL_SIGNER_REGISTRY: Arc<SignerRegistry> = Default::default();
}

pub type PubKey = [u8; 33];

pub type SignerRegistryMap = BTreeMap<Address32, PubKey>;

#[derive(Default)]
pub struct SignerRegistry {
	pub registry: RwLock<SignerRegistryMap>,
	pub seal_path: PathBuf,
}

pub type RegistryResult<T> = Result<T, RegistryError>;

use litentry_primitives::Address32;
#[cfg(feature = "sgx")]
use thiserror_sgx as thiserror;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
	#[error("poison lock")]
	PoisonLock,
	#[error("empty Signer registry")]
	EmptyRegistry,
	#[error(transparent)]
	Other(#[from] Box<dyn Error + Sync + Send + 'static>),
}

impl From<std::io::Error> for RegistryError {
	fn from(e: std::io::Error) -> Self {
		Self::Other(e.into())
	}
}

impl From<codec::Error> for RegistryError {
	#[cfg(feature = "std")]
	fn from(e: codec::Error) -> Self {
		Self::Other(e.into())
	}

	#[cfg(feature = "sgx")]
	fn from(e: codec::Error) -> Self {
		Self::Other(std::format!("{:?}", e).into())
	}
}

#[cfg(feature = "sgx")]
mod sgx {
	use crate::{RegistryError as Error, RegistryResult as Result, SignerRegistryMap};
	pub use codec::{Decode, Encode};
	pub use itp_settings::files::SIGNER_REGISTRY_FILE;
	pub use itp_sgx_io::{seal, unseal, SealedIO};
	pub use log::*;
	pub use std::{boxed::Box, fs, path::PathBuf, sgxfs::SgxFile, sync::Arc};

	#[derive(Clone, Debug)]
	pub struct SignerRegistrySeal {
		base_path: PathBuf,
	}

	impl SignerRegistrySeal {
		pub fn new(base_path: PathBuf) -> Self {
			Self { base_path }
		}

		pub fn path(&self) -> PathBuf {
			self.base_path.join(SIGNER_REGISTRY_FILE)
		}
	}

	impl SealedIO for SignerRegistrySeal {
		type Error = Error;
		type Unsealed = SignerRegistryMap;

		fn unseal(&self) -> Result<Self::Unsealed> {
			Ok(unseal(self.path()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			info!("Seal signer registry to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| seal(bytes, self.path()))?)
		}
	}
}

#[cfg(feature = "sgx")]
use sgx::*;

pub trait SignerRegistrySealer {
	fn seal(&self, state: SignerRegistryMap) -> RegistryResult<()>;
	fn unseal(&self) -> RegistryResult<SignerRegistryMap>;
}

pub trait SignerRegistryUpdater {
	fn init(&self) -> RegistryResult<()>;
	fn update(&self, account: Address32, key: PubKey) -> RegistryResult<()>;
	fn remove(&self, account: Address32) -> RegistryResult<()>;
}

pub trait SignerRegistryLookup {
	fn contains_key(&self, account: &Address32) -> bool;
	fn get_all(&self) -> Vec<(Address32, PubKey)>;
}

impl SignerRegistrySealer for SignerRegistry {
	#[cfg(feature = "std")]
	fn seal(&self, _state: SignerRegistryMap) -> RegistryResult<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn unseal(&self) -> RegistryResult<SignerRegistryMap> {
		Ok(Default::default())
	}

	#[cfg(feature = "sgx")]
	fn seal(&self, mut state: SignerRegistryMap) -> RegistryResult<()> {
		let mut registry =
			GLOBAL_SIGNER_REGISTRY.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		while let Some((key, val)) = state.pop_first() {
			registry.insert(key, val);
		}

		let signer_seal = SignerRegistrySeal::new(self.seal_path.clone());
		signer_seal.seal(&state)
	}

	#[cfg(feature = "sgx")]
	fn unseal(&self) -> RegistryResult<SignerRegistryMap> {
		let signer_seal = SignerRegistrySeal::new(self.seal_path.clone());
		signer_seal.unseal()
	}
}

impl SignerRegistryUpdater for SignerRegistry {
	#[cfg(feature = "std")]
	fn init(&self) -> RegistryResult<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn update(&self, account: Address32, key: PubKey) -> RegistryResult<()> {
		let mut registry = self.registry.write().unwrap();
		registry.insert(account, key);
		Ok(())
	}

	#[cfg(feature = "std")]
	fn remove(&self, _account: Address32) -> RegistryResult<()> {
		Ok(())
	}

	// if `SIGNER_REGISTRY_FILE` exists, unseal and init from it
	// otherwise create a new instance and seal to static file
	#[cfg(feature = "sgx")]
	fn init(&self) -> RegistryResult<()> {
		let enclave_seal = SignerRegistrySeal::new(self.seal_path.clone());
		if SgxFile::open(SIGNER_REGISTRY_FILE).is_err() {
			info!("[Signer] SignerRegistry file not found, creating new! {}", SIGNER_REGISTRY_FILE);
			let registry =
				GLOBAL_SIGNER_REGISTRY.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			enclave_seal.seal(&*registry)
		} else {
			let m = enclave_seal.unseal()?;
			info!("[Signer] SignerRegistry unsealed from file: {:?}", m);
			let mut registry =
				GLOBAL_SIGNER_REGISTRY.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			*registry = m;
			Ok(())
		}
	}

	#[cfg(feature = "sgx")]
	fn update(&self, account: Address32, key: PubKey) -> RegistryResult<()> {
		let mut registry =
			GLOBAL_SIGNER_REGISTRY.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		registry.insert(account, key);
		SignerRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
	}

	#[cfg(feature = "sgx")]
	fn remove(&self, account: Address32) -> RegistryResult<()> {
		let mut registry =
			GLOBAL_SIGNER_REGISTRY.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		let old_value = registry.remove(&account);
		if old_value.is_some() {
			return SignerRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
		}
		Ok(())
	}
}

impl SignerRegistryLookup for SignerRegistry {
	#[cfg(feature = "std")]
	fn contains_key(&self, account: &Address32) -> bool {
		let registry = self.registry.read().unwrap();
		registry.contains_key(account)
	}

	#[cfg(feature = "std")]
	fn get_all(&self) -> Vec<(Address32, PubKey)> {
		let registry = self.registry.read().unwrap();
		registry.iter().map(|(k, v)| (*k, *v)).collect()
	}

	#[cfg(feature = "sgx")]
	fn contains_key(&self, account: &Address32) -> bool {
		// Using unwrap because poisoned locks are unrecoverable errors
		let registry = GLOBAL_SIGNER_REGISTRY.registry.read().unwrap();
		registry.contains_key(account)
	}

	#[cfg(feature = "sgx")]
	fn get_all(&self) -> Vec<(Address32, PubKey)> {
		// Using unwrap because poisoned locks are unrecoverable errors
		let registry = GLOBAL_SIGNER_REGISTRY.registry.read().unwrap();
		registry.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
	}
}
