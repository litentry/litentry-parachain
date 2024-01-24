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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use crate::{
	error::{Error, Result},
	MrEnclave, ScheduledEnclave, ScheduledEnclaveUpdater, SidechainBlockNumber,
	GLOBAL_SCHEDULED_ENCLAVE,
};

#[cfg(feature = "sgx")]
mod sgx {
	use crate::{
		error::{Error, Result},
		ScheduledEnclaveMap,
	};
	pub use codec::{Decode, Encode};
	pub use itp_settings::files::SCHEDULED_ENCLAVE_FILE;
	pub use itp_sgx_io::{seal, unseal, SealedIO};
	pub use log::*;
	pub use std::{boxed::Box, fs, path::PathBuf, sgxfs::SgxFile, sync::Arc};

	#[derive(Clone, Debug)]
	pub struct ScheduledEnclaveSeal {
		base_path: PathBuf,
	}

	impl ScheduledEnclaveSeal {
		pub fn new(base_path: PathBuf) -> Self {
			Self { base_path }
		}

		pub fn path(&self) -> PathBuf {
			self.base_path.join(SCHEDULED_ENCLAVE_FILE)
		}
	}

	impl SealedIO for ScheduledEnclaveSeal {
		type Error = Error;
		type Unsealed = ScheduledEnclaveMap;

		fn unseal(&self) -> Result<Self::Unsealed> {
			Ok(unseal(self.path()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<()> {
			info!("Seal scheduled enclave to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| seal(bytes, self.path()))?)
		}
	}
}

#[cfg(feature = "sgx")]
use sgx::*;

// TODO: unit-test
impl ScheduledEnclaveUpdater for ScheduledEnclave {
	#[cfg(feature = "std")]
	fn init(&self, _mrenclave: MrEnclave) -> Result<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn update(&self, _sbn: SidechainBlockNumber, _mrenclave: MrEnclave) -> Result<()> {
		Ok(())
	}

	#[cfg(feature = "std")]
	fn remove(&self, _sbn: SidechainBlockNumber) -> Result<()> {
		Ok(())
	}

	// if `SCHEDULED_ENCLAVE_FILE` exists, unseal and init from it
	// otherwise create a new instance and seal to static file
	#[cfg(feature = "sgx")]
	fn init(&self, mrenclave: MrEnclave) -> Result<()> {
		let _ = self.set_current_mrenclave(mrenclave)?;
		let _ = self.set_block_production_paused(false)?;
		let enclave_seal = ScheduledEnclaveSeal::new(self.seal_path.clone());
		if SgxFile::open(SCHEDULED_ENCLAVE_FILE).is_err() {
			info!(
				"[Enclave] ScheduledEnclave file not found, creating new! {}",
				SCHEDULED_ENCLAVE_FILE
			);
			let mut registry =
				GLOBAL_SCHEDULED_ENCLAVE.registry.write().map_err(|_| Error::PoisonLock)?;
			registry.clear();
			registry.insert(0, mrenclave);
			enclave_seal.seal(&*registry)
		} else {
			let m = enclave_seal.unseal()?;
			info!("[Enclave] ScheduledEnclave unsealed from file: {:?}", m);
			let mut registry =
				GLOBAL_SCHEDULED_ENCLAVE.registry.write().map_err(|_| Error::PoisonLock)?;
			*registry = m;
			Ok(())
		}
	}

	#[cfg(feature = "sgx")]
	fn update(&self, sbn: SidechainBlockNumber, mrenclave: MrEnclave) -> Result<()> {
		let mut registry =
			GLOBAL_SCHEDULED_ENCLAVE.registry.write().map_err(|_| Error::PoisonLock)?;
		registry.insert(sbn, mrenclave);
		ScheduledEnclaveSeal::new(self.seal_path.clone()).seal(&*registry)
	}

	#[cfg(feature = "sgx")]
	fn remove(&self, sbn: SidechainBlockNumber) -> Result<()> {
		let mut registry =
			GLOBAL_SCHEDULED_ENCLAVE.registry.write().map_err(|_| Error::PoisonLock)?;
		let old_value = registry.remove(&sbn);
		if old_value.is_some() {
			return ScheduledEnclaveSeal::new(self.seal_path.clone()).seal(&*registry)
		}
		Ok(())
	}

	fn get_current_mrenclave(&self) -> Result<MrEnclave> {
		self.current_mrenclave.read().map_err(|_| Error::PoisonLock).map(|l| *l)
	}

	fn set_current_mrenclave(&self, mrenclave: MrEnclave) -> Result<()> {
		let mut m = self.current_mrenclave.write().map_err(|_| Error::PoisonLock)?;
		*m = mrenclave;
		Ok(())
	}

	fn get_expected_mrenclave(&self, sbn: SidechainBlockNumber) -> Result<MrEnclave> {
		let registry = GLOBAL_SCHEDULED_ENCLAVE.registry.read().map_err(|_| Error::PoisonLock)?;
		let r = registry
			.iter()
			.filter(|(k, _)| **k <= sbn)
			.max_by_key(|(k, _)| **k)
			.ok_or(Error::EmptyRegistry)?;
		Ok(*r.1)
	}

	fn get_previous_mrenclave(&self, sbn: SidechainBlockNumber) -> Result<MrEnclave> {
		// TODO: optimise it
		let registry = GLOBAL_SCHEDULED_ENCLAVE.registry.read().map_err(|_| Error::PoisonLock)?;
		let r = registry
			.iter()
			.filter(|(k, _)| **k <= sbn)
			.max_by_key(|(k, _)| **k)
			.ok_or(Error::NoPreviousMRENCLAVE)?;
		let v = registry
			.iter()
			.filter(|(k, _)| **k < *r.0)
			.max_by_key(|(k, _)| **k)
			.ok_or(Error::NoPreviousMRENCLAVE)?;
		Ok(*v.1)
	}

	fn is_block_production_paused(&self) -> Result<bool> {
		self.block_production_paused.read().map_err(|_| Error::PoisonLock).map(|l| *l)
	}

	fn set_block_production_paused(&self, should_pause: bool) -> Result<()> {
		let mut p = self.block_production_paused.write().map_err(|_| Error::PoisonLock)?;
		*p = should_pause;
		Ok(())
	}
}
