//! Scheduled Mr Enclave

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "sgx")]
extern crate sgx_tstd as std;
#[cfg(feature = "sgx")]
use itp_settings::files::SCHEDULED_ENCLAVE_FILE;
#[cfg(feature = "sgx")]
use itp_sgx_io::{seal, unseal};
use itp_types::{MrEnclave, SidechainBlockNumber};
use litentry_primitives::ParentchainBlockNumber;
#[cfg(feature = "sgx")]
use serde::{Deserialize, Serialize};
use std::{
	io::{Error, ErrorKind, Result as IOResult},
	vec::Vec,
};

#[cfg(feature = "sgx")]
pub use sgx_env::*;
#[cfg(feature = "std")]
pub use std_env::*;

#[cfg(feature = "sgx")]
mod sgx_env {
	use super::*;

	#[derive(Clone, Debug, Serialize, Deserialize)]
	pub struct ScheduledEnclaveInfo {
		pub parachain_block_number: ParentchainBlockNumber,
		pub sidechain_block_number: SidechainBlockNumber,
		pub mr_enclave: MrEnclave,
	}

	#[derive(Clone, Debug, Default, Serialize, Deserialize)]
	pub struct ScheduledEnclaves {
		#[serde(default)]
		pub scheduled_enclaves: Vec<ScheduledEnclaveInfo>,
	}
}

#[cfg(feature = "std")]
mod std_env {
	use super::*;

	#[derive(Clone, Debug)]
	pub struct ScheduledEnclaveInfo {
		pub parachain_block_number: ParentchainBlockNumber,
		pub sidechain_block_number: SidechainBlockNumber,
		pub mr_enclave: MrEnclave,
	}

	#[derive(Clone, Debug, Default)]
	pub struct ScheduledEnclaves {
		pub scheduled_enclaves: Vec<ScheduledEnclaveInfo>,
	}
}

pub trait ScheduledEnclaveHandle {
	fn from_static_file() -> IOResult<Self>
	where
		Self: Sized,
	{
		Err(Error::new(ErrorKind::Other, "can't get from static file"))
	}
	fn sync_to_static_file(&self) -> IOResult<()> {
		Err(Error::new(ErrorKind::Other, "can't sync to static file"))
	}
	fn add_scheduled_enclave(&mut self, scheduled_enclave: ScheduledEnclaveInfo) -> IOResult<()>;
	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> IOResult<()>;
	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo>;
}

impl ScheduledEnclaveHandle for ScheduledEnclaves {
	#[cfg(feature = "sgx")]
	fn from_static_file() -> IOResult<Self> {
		let raw = unseal(SCHEDULED_ENCLAVE_FILE)?;
		let s: Self = serde_json::from_slice(&raw).map_err(|e| Error::new(ErrorKind::Other, e))?;
		Ok(s)
	}

	#[cfg(feature = "sgx")]
	fn sync_to_static_file(&self) -> IOResult<()> {
		let s = serde_json::to_string(&self).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
		seal(s.as_bytes(), SCHEDULED_ENCLAVE_FILE)
	}

	fn add_scheduled_enclave(&mut self, scheduled_enclave: ScheduledEnclaveInfo) -> IOResult<()> {
		self.scheduled_enclaves.push(scheduled_enclave);
		self.sync_to_static_file()?;
		Ok(())
	}

	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> IOResult<()> {
		self.scheduled_enclaves
			.retain(|info| info.sidechain_block_number != sidechain_block_number);
		self.sync_to_static_file()?;
		Ok(())
	}

	/// get the next scheduled enclave
	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo> {
		self.scheduled_enclaves
			.iter()
			.find(|enclave_info| enclave_info.sidechain_block_number > current_side_chain_number)
			.cloned()
	}
}
