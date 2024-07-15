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

use log::error;
use std::{collections::BTreeMap, error::Error, path::PathBuf, string::String, vec::Vec};

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

pub type EnclaveRegistryMap = BTreeMap<Address32, String>;

#[derive(Default)]
pub struct EnclaveRegistry {
	pub registry: RwLock<EnclaveRegistryMap>,
	pub seal_path: PathBuf,
}

impl EnclaveRegistry {
	pub fn new(base_dir: PathBuf) -> Self {
		EnclaveRegistry { registry: Default::default(), seal_path: base_dir }
	}
}

pub type RegistryResult<T> = Result<T, RegistryError>;

use litentry_primitives::Address32;
#[cfg(feature = "sgx")]
use thiserror_sgx as thiserror;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
	#[error("poison lock")]
	PoisonLock,
	#[error("empty Enclave registry")]
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
	use crate::{EnclaveRegistryMap, RegistryError as Error, RegistryResult as Result};
	pub use codec::{Decode, Encode};
	pub use itp_settings::files::ENCLAVE_REGISTRY_FILE;
	pub use itp_sgx_io::{seal, unseal, SealedIO};
	pub use log::*;
	pub use std::{boxed::Box, fs, path::PathBuf, sgxfs::SgxFile, sync::Arc};

	#[derive(Clone, Debug)]
	pub struct EnclaveRegistrySeal {
		base_path: PathBuf,
	}

	impl EnclaveRegistrySeal {
		pub fn new(base_path: PathBuf) -> Self {
			Self { base_path }
		}

		pub fn path(&self) -> PathBuf {
			self.base_path.join(ENCLAVE_REGISTRY_FILE)
		}
	}

	impl SealedIO for EnclaveRegistrySeal {
		type Error = Error;
		type Unsealed = EnclaveRegistryMap;

		fn unseal(&self) -> Result<Self::Unsealed> {
			Ok(unseal(self.path()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			info!("Seal enclave registry to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| seal(bytes, self.path()))?)
		}
	}
}

#[cfg(feature = "sgx")]
use sgx::*;

pub trait EnclaveRegistrySealer {
	fn seal(&self, state: EnclaveRegistryMap) -> RegistryResult<()>;
	fn unseal(&self) -> RegistryResult<EnclaveRegistryMap>;
}

pub trait EnclaveRegistryUpdater {
	fn init(&self) -> RegistryResult<()>;
	fn update(&self, account: Address32, worker_url: String) -> RegistryResult<()>;
	fn remove(&self, account: Address32) -> RegistryResult<()>;
}

pub trait EnclaveRegistryLookup {
	fn contains_key(&self, account: &Address32) -> bool;
	fn get_all(&self) -> Vec<(Address32, String)>;
	fn get_worker_url(&self, account: &Address32) -> Option<String>;
}

impl EnclaveRegistrySealer for EnclaveRegistry {
	#[cfg(feature = "std")]
	fn seal(&self, _state: EnclaveRegistryMap) -> RegistryResult<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn unseal(&self) -> RegistryResult<EnclaveRegistryMap> {
		Ok(Default::default())
	}

	#[cfg(feature = "sgx")]
	fn seal(&self, mut state: EnclaveRegistryMap) -> RegistryResult<()> {
		let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		while let Some((key, val)) = state.pop_first() {
			registry.insert(key, val);
		}

		let enclave_seal = EnclaveRegistrySeal::new(self.seal_path.clone());
		enclave_seal.seal(&registry)
	}

	#[cfg(feature = "sgx")]
	fn unseal(&self) -> RegistryResult<EnclaveRegistryMap> {
		let enclave_seal = EnclaveRegistrySeal::new(self.seal_path.clone());
		enclave_seal.unseal()
	}
}

impl EnclaveRegistryUpdater for EnclaveRegistry {
	#[cfg(feature = "std")]
	fn init(&self) -> RegistryResult<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn update(&self, account: Address32, worker_url: String) -> RegistryResult<()> {
		let mut registry = self.registry.write().unwrap();
		registry.insert(account, worker_url);
		Ok(())
	}

	#[cfg(feature = "std")]
	fn remove(&self, _account: Address32) -> RegistryResult<()> {
		Ok(())
	}

	// if `ENCLAVE_REGISTRY_FILE` exists, unseal and init from it
	// otherwise create a new instance and seal to static file
	#[cfg(feature = "sgx")]
	fn init(&self) -> RegistryResult<()> {
		let enclave_seal = EnclaveRegistrySeal::new(self.seal_path.clone());
		if SgxFile::open(ENCLAVE_REGISTRY_FILE).is_err() {
			info!(
				"[Enclave] EnclaveRegistry file not found, creating new! {}",
				ENCLAVE_REGISTRY_FILE
			);
			let registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			enclave_seal.seal(&*registry)
		} else {
			let m = enclave_seal.unseal()?;
			info!("[Enclave] EnclaveRegistry unsealed from file: {:?}", m);
			let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			*registry = m;
			Ok(())
		}
	}

	#[cfg(feature = "sgx")]
	fn update(&self, account: Address32, worker_url: String) -> RegistryResult<()> {
		let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		registry.insert(account, worker_url);
		EnclaveRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
	}

	#[cfg(feature = "sgx")]
	fn remove(&self, account: Address32) -> RegistryResult<()> {
		let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		let old_value = registry.remove(&account);
		if old_value.is_some() {
			return EnclaveRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
		}
		Ok(())
	}
}

impl EnclaveRegistryLookup for EnclaveRegistry {
	#[cfg(feature = "std")]
	fn contains_key(&self, account: &Address32) -> bool {
		let registry = self.registry.read().unwrap();
		registry.contains_key(account)
	}

	#[cfg(feature = "std")]
	fn get_all(&self) -> Vec<(Address32, String)> {
		let registry = self.registry.read().unwrap();
		registry.iter().map(|(k, v)| (*k, v.clone())).collect()
	}

	#[cfg(feature = "std")]
	fn get_worker_url(&self, account: &Address32) -> Option<String> {
		let registry = self.registry.read().unwrap();
		registry.get(account).cloned()
	}

	#[cfg(feature = "sgx")]
	fn contains_key(&self, account: &Address32) -> bool {
		// Using unwrap becaused poisoned locks are unrecoverable errors
		let registry = self.registry.read().unwrap();
		registry.contains_key(account)
	}

	#[cfg(feature = "sgx")]
	fn get_all(&self) -> Vec<(Address32, String)> {
		// Using unwrap becaused poisoned locks are unrecoverable errors
		let registry = self.registry.read().unwrap();
		registry.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
	}

	#[cfg(feature = "sgx")]
	fn get_worker_url(&self, account: &Address32) -> Option<String> {
		// Using unwrap becaused poisoned locks are unrecoverable errors
		let registry = self.registry.read().unwrap();
		registry.get(account).cloned()
	}
}
