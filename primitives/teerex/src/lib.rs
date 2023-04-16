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

//!Primitives for teerex
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
	Debug,
	Production,
}

impl Default for SgxBuildMode {
	fn default() -> Self {
		SgxBuildMode::Production
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Enclave<PubKey, Url> {
	pub pubkey: PubKey, // FIXME: this is redundant information
	pub mr_enclave: MrEnclave,
	// Todo: make timestamp: Moment
	pub timestamp: u64,                 // unix epoch in milliseconds
	pub url: Url,                       // utf8 encoded url
	pub shielding_key: Option<Vec<u8>>, // JSON serialised enclave shielding key
	pub vc_pubkey: Option<Vec<u8>>,
	pub sgx_mode: SgxBuildMode,
	pub sgx_metadata: SgxEnclaveMetadata,
}

impl<PubKey, Url> Enclave<PubKey, Url> {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		pubkey: PubKey,
		mr_enclave: MrEnclave,
		timestamp: u64,
		url: Url,
		shielding_key: Option<Vec<u8>>,
		vc_pubkey: Option<Vec<u8>>,
		sgx_build_mode: SgxBuildMode,
		sgx_metadata: SgxEnclaveMetadata,
	) -> Self {
		Enclave {
			pubkey,
			mr_enclave,
			timestamp,
			url,
			shielding_key,
			vc_pubkey,
			sgx_mode: sgx_build_mode,
			sgx_metadata,
		}
	}
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, sp_core::RuntimeDebug)]
pub struct SgxEnclaveMetadata {
	pub quote: Vec<u8>,
	pub quote_sig: Vec<u8>,
	pub quote_cert: Vec<u8>,
}

impl SgxEnclaveMetadata {
	pub fn new(quote: Vec<u8>, quote_sig: Vec<u8>, quote_cert: Vec<u8>) -> Self {
		SgxEnclaveMetadata { quote, quote_sig, quote_cert }
	}
}

/// The list of valid TCBs for an enclave.
#[derive(Encode, Decode, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct QeTcb {
	pub isvsvn: u16,
}

impl QeTcb {
	pub fn new(isvsvn: u16) -> Self {
		Self { isvsvn }
	}
}

/// This represents all the collateral data that we need to store on chain in order to verify
/// the quoting enclave validity of another enclave that wants to register itself on chain
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct QuotingEnclave {
	// Todo: make timestamp: Moment
	pub issue_date: u64, // unix epoch in milliseconds
	// Todo: make timestamp: Moment
	pub next_update: u64, // unix epoch in milliseconds
	pub miscselect: [u8; 4],
	pub miscselect_mask: [u8; 4],
	pub attributes: [u8; 16],
	pub attributes_mask: [u8; 16],
	pub mrsigner: MrSigner,
	pub isvprodid: u16,
	/// Contains only the TCB versions that are considered UpToDate
	pub tcb: Vec<QeTcb>,
}

impl QuotingEnclave {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		issue_date: u64,
		next_update: u64,
		miscselect: [u8; 4],
		miscselect_mask: [u8; 4],
		attributes: [u8; 16],
		attributes_mask: [u8; 16],
		mrsigner: MrSigner,
		isvprodid: u16,
		tcb: Vec<QeTcb>,
	) -> Self {
		Self {
			issue_date,
			next_update,
			miscselect,
			miscselect_mask,
			attributes,
			attributes_mask,
			mrsigner,
			isvprodid,
			tcb,
		}
	}

	pub fn attributes_flags_mask_as_u64(&self) -> u64 {
		let slice_as_array: [u8; 8] = self.attributes_mask[0..8].try_into().unwrap();
		u64::from_le_bytes(slice_as_array)
	}

	pub fn attributes_flags_as_u64(&self) -> u64 {
		let slice_as_array: [u8; 8] = self.attributes[0..8].try_into().unwrap();
		u64::from_le_bytes(slice_as_array)
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct TcbVersionStatus {
	pub cpusvn: Cpusvn,
	pub pcesvn: Pcesvn,
}

impl TcbVersionStatus {
	pub fn new(cpusvn: Cpusvn, pcesvn: Pcesvn) -> Self {
		Self { cpusvn, pcesvn }
	}

	pub fn verify_examinee(&self, examinee: &TcbVersionStatus) -> bool {
		for (v, r) in self.cpusvn.iter().zip(examinee.cpusvn.iter()) {
			if *v > *r {
				return false
			}
		}
		self.pcesvn <= examinee.pcesvn
	}
}

/// This represents all the collateral data that we need to store on chain in order to verify
/// the quoting enclave validity of another enclave that wants to register itself on chain
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct TcbInfoOnChain {
	// Todo: make timestamp: Moment
	pub issue_date: u64, // unix epoch in milliseconds
	// Todo: make timestamp: Moment
	pub next_update: u64, // unix epoch in milliseconds
	tcb_levels: Vec<TcbVersionStatus>,
}

impl TcbInfoOnChain {
	pub fn new(issue_date: u64, next_update: u64, tcb_levels: Vec<TcbVersionStatus>) -> Self {
		Self { issue_date, next_update, tcb_levels }
	}

	pub fn verify_examinee(&self, examinee: &TcbVersionStatus) -> bool {
		for tb in &self.tcb_levels {
			if tb.verify_examinee(examinee) {
				return true
			}
		}
		false
	}
}

pub type MrSigner = [u8; 32];
pub type MrEnclave = [u8; 32];
pub type Fmspc = [u8; 6];
pub type Cpusvn = [u8; 16];
pub type Pcesvn = u16;
pub type ShardIdentifier = H256;
pub type SidechainBlockNumber = u64;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Request {
	pub shard: ShardIdentifier,
	pub cyphertext: Vec<u8>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex_literal::hex;

	#[test]
	fn tcb_full_is_valid() {
		// The strings are the hex encodings of the 16-byte CPUSVN numbers
		let reference = TcbVersionStatus::new(hex!("11110204018007000000000000000000"), 7);
		assert!(reference.verify_examinee(&reference));
		assert!(reference
			.verify_examinee(&TcbVersionStatus::new(hex!("11110204018007000000000000000000"), 7)));
		assert!(reference
			.verify_examinee(&TcbVersionStatus::new(hex!("21110204018007000000000000000001"), 7)));
		assert!(!reference
			.verify_examinee(&TcbVersionStatus::new(hex!("10110204018007000000000000000000"), 6)));
		assert!(!reference
			.verify_examinee(&TcbVersionStatus::new(hex!("11110204018007000000000000000000"), 6)));
	}
}
