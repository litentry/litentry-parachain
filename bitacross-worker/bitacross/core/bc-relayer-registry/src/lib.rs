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

use litentry_primitives::Identity;
use log::error;
use std::{collections::BTreeMap, path::PathBuf};

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

pub type RelayerRegistryMap = BTreeMap<Identity, ()>;

#[derive(Default)]
pub struct RelayerRegistry {
	pub registry: RwLock<RelayerRegistryMap>,
	pub seal_path: PathBuf,
}

impl RelayerRegistry {
	pub fn new(base_dir: PathBuf) -> Self {
		RelayerRegistry { registry: Default::default(), seal_path: base_dir }
	}
}

pub type RegistryResult<T> = core::result::Result<T, RegistryError>;

#[cfg(feature = "sgx")]
use thiserror_sgx as thiserror;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
	#[error("poison lock")]
	PoisonLock,
	#[error("empty Relayer registry")]
	EmptyRegistry,
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
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
	use crate::{RegistryError as Error, RegistryResult as Result, RelayerRegistryMap};
	pub use codec::{Decode, Encode};
	pub use itp_settings::files::RELAYER_REGISTRY_FILE;
	pub use itp_sgx_io::{seal, unseal, SealedIO};
	pub use log::*;
	pub use std::{boxed::Box, fs, path::PathBuf, sgxfs::SgxFile, sync::Arc};

	#[derive(Clone, Debug)]
	pub struct RelayerRegistrySeal {
		base_path: PathBuf,
	}

	impl RelayerRegistrySeal {
		pub fn new(base_path: PathBuf) -> Self {
			Self { base_path }
		}

		pub fn path(&self) -> PathBuf {
			self.base_path.join(RELAYER_REGISTRY_FILE)
		}
	}

	impl SealedIO for RelayerRegistrySeal {
		type Error = Error;
		type Unsealed = RelayerRegistryMap;

		fn unseal(&self) -> Result<Self::Unsealed> {
			Ok(unseal(self.path()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			info!("Seal relayer registry to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| seal(bytes, self.path()))?)
		}
	}
}

#[cfg(feature = "sgx")]
use sgx::*;

pub trait RelayerRegistryUpdater {
	fn init(&self) -> RegistryResult<()>;
	fn update(&self, account: Identity) -> RegistryResult<()>;
	fn remove(&self, account: Identity) -> RegistryResult<()>;
}

pub trait RelayerRegistryLookup {
	fn contains_key(&self, account: Identity) -> bool;
}

impl RelayerRegistryUpdater for RelayerRegistry {
	#[cfg(feature = "std")]
	fn init(&self) -> RegistryResult<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn update(&self, account: Identity) -> RegistryResult<()> {
		let mut registry = self.registry.write().unwrap();
		registry.insert(account, ());
		Ok(())
	}

	#[cfg(feature = "std")]
	fn remove(&self, _account: Identity) -> RegistryResult<()> {
		Ok(())
	}

	// if `RELAYER_REGISTRY_FILE` exists, unseal and init from it
	// otherwise create a new instance and seal to static file
	#[cfg(feature = "sgx")]
	fn init(&self) -> RegistryResult<()> {
		let enclave_seal = RelayerRegistrySeal::new(self.seal_path.clone());
		if SgxFile::open(RELAYER_REGISTRY_FILE).is_err() {
			info!(
				"[Enclave] RelayerRegistry file not found, creating new! {}",
				RELAYER_REGISTRY_FILE
			);
			let registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			enclave_seal.seal(&*registry)
		} else {
			let m = enclave_seal.unseal()?;
			info!("[Enclave] RelayerRegistry unsealed from file: {:?}", m);
			let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
			*registry = m;
			Ok(())
		}
	}

	#[cfg(feature = "sgx")]
	fn update(&self, account: Identity) -> RegistryResult<()> {
		let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		registry.insert(account, ());
		RelayerRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
	}

	#[cfg(feature = "sgx")]
	fn remove(&self, account: Identity) -> RegistryResult<()> {
		let mut registry = self.registry.write().map_err(|_| RegistryError::PoisonLock)?;
		let old_value = registry.remove(&account);
		if old_value.is_some() {
			return RelayerRegistrySeal::new(self.seal_path.clone()).seal(&*registry)
		}
		Ok(())
	}
}

impl RelayerRegistryLookup for RelayerRegistry {
	#[cfg(feature = "std")]
	fn contains_key(&self, account: Identity) -> bool {
		let registry = self.registry.read().unwrap();
		registry.contains_key(&account)
	}

	#[cfg(feature = "sgx")]
	fn contains_key(&self, account: Identity) -> bool {
		// Using unwrap becaused poisoned locks are unrecoverable errors
		let registry = self.registry.read().unwrap();
		registry.contains_key(&account)
	}
}
