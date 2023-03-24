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
	collections::HashMap,
	io::{Error, ErrorKind, Result as IOResult},
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
		pub scheduled_enclaves: HashMap<SidechainBlockNumber, ScheduledEnclaveInfo>,
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
		pub scheduled_enclaves: HashMap<SidechainBlockNumber, ScheduledEnclaveInfo>,
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
	fn add_scheduled_enclave(
		&mut self,
		scheduled_enclave: ScheduledEnclaveInfo,
	) -> IOResult<Option<ScheduledEnclaveInfo>>;

	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> IOResult<Option<ScheduledEnclaveInfo>>;

	/// give the current sidechain block number
	/// return the next scheduled mr_enclave info that will be used
	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo>;

	fn get_current_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo>;
}

impl ScheduledEnclaveHandle for ScheduledEnclaves {
	#[cfg(feature = "sgx")]
	fn from_static_file() -> IOResult<Self> {
		let raw = unseal(SCHEDULED_ENCLAVE_FILE).unwrap_or_default();
		if raw.is_empty() {
			return Ok(Self::default())
		}
		let s: Self = serde_json::from_slice(&raw).map_err(|e| Error::new(ErrorKind::Other, e))?;
		Ok(s)
	}

	#[cfg(feature = "sgx")]
	fn sync_to_static_file(&self) -> IOResult<()> {
		let s = serde_json::to_string(&self).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
		seal(s.as_bytes(), SCHEDULED_ENCLAVE_FILE)
	}

	fn add_scheduled_enclave(
		&mut self,
		scheduled_enclave: ScheduledEnclaveInfo,
	) -> IOResult<Option<ScheduledEnclaveInfo>> {
		let old_enclave = self
			.scheduled_enclaves
			.insert(scheduled_enclave.sidechain_block_number, scheduled_enclave);
		self.sync_to_static_file()?;
		Ok(old_enclave)
	}

	fn remove_scheduled_enclave(
		&mut self,
		sidechain_block_number: SidechainBlockNumber,
	) -> IOResult<Option<ScheduledEnclaveInfo>> {
		let enclave_info = self.scheduled_enclaves.remove(&sidechain_block_number);
		if enclave_info.is_some() {
			self.sync_to_static_file()?;
		}
		Ok(enclave_info)
	}

	fn get_next_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo> {
		self.scheduled_enclaves
			.iter()
			.filter_map(
				|(k, v)| {
					if k > &current_side_chain_number {
						Some(v.clone())
					} else {
						None
					}
				},
			)
			.min_by_key(|v| v.sidechain_block_number)
	}

	/// given current side chain number, returns  mrenclave info that currently should be used
	/// for example, if the scheduled mr enclaves like:
	/// | block_number | mr_enclave |
	/// | ---          | ---         |
	/// | 1            | mr_enclave_A |
	/// | 10           | mr_enclave_B |
	/// | 20           | mr_enclave_C |
	/// given current sidechain number 5, returns mr_enclave_A
	/// given current sidechain number 10, returns mr_enclave_B
	/// given current sidechain number 11, return mr_enclave_B
	/// given current sidechain number 22, return mr_enclave_C
	fn get_current_scheduled_enclave(
		&self,
		current_side_chain_number: SidechainBlockNumber,
	) -> Option<ScheduledEnclaveInfo> {
		self.scheduled_enclaves
			.iter()
			.filter_map(
				|(k, v)| {
					if k <= &current_side_chain_number {
						Some(v.clone())
					} else {
						None
					}
				},
			)
			.max_by_key(|v| v.sidechain_block_number)
	}
}
