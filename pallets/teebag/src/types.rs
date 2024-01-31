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

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{RuntimeDebug, H256};
use sp_std::prelude::*;

pub type MrSigner = [u8; 32];
pub type MrEnclave = [u8; 32];
pub type Fmspc = [u8; 6];
pub type Cpusvn = [u8; 16];
pub type Pcesvn = u16;
pub type ShardIdentifier = H256;
pub type EnclaveFingerprint = H256;
pub type SidechainBlockNumber = u64;

#[derive(Encode, Decode, Default, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum DcapProvider {
	#[default]
	MAA,
	Intel,
	Local,
	Integritee,
}

#[derive(Encode, Decode, Clone, Copy, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum AttestationType {
	#[default]
	Ignore,
	Ias,
	Dcap(DcapProvider),
}

#[derive(Encode, Decode, Clone, Copy, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum WorkerType {
	#[default]
	Identity,
	BitAcross,
}

impl WorkerType {
	pub fn is_sidechain(&self) -> bool {
		self == &Self::Identity
	}
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
	#[default]
	#[codec(index = 0)]
	Production,
	#[codec(index = 1)]
	Debug,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Enclave {
	pub worker_type: WorkerType,
	pub mrenclave: MrEnclave,
	pub last_updated: u64,                 // unix epoch in milliseconds
	pub url: Vec<u8>,                      // utf8 encoded url
	pub shielding_pubkey: Option<Vec<u8>>, // JSON serialised enclave shielding pub key
	pub vc_pubkey: Option<Vec<u8>>,
	pub sgx_build_mode: SgxBuildMode,
	pub attestation_type: AttestationType,
}

impl Enclave {
	pub fn new(
		worker_type: WorkerType,
		url: Vec<u8>,
		shielding_pubkey: Option<Vec<u8>>,
		vc_pubkey: Option<Vec<u8>>,
		attestation_type: AttestationType,
	) -> Self {
		Enclave {
			worker_type,
			url,
			shielding_pubkey,
			vc_pubkey,
			attestation_type,
			..Default::default()
		}
	}

	pub fn new_full(
		worker_type: WorkerType,
		mrenclave: MrEnclave,
		last_updated: u64,
		url: Vec<u8>,
		shielding_pubkey: Option<Vec<u8>>,
		vc_pubkey: Option<Vec<u8>>,
		sgx_build_mode: SgxBuildMode,
		attestation_type: AttestationType,
	) -> Self {
		Enclave {
			worker_type,
			mrenclave,
			last_updated,
			url,
			shielding_pubkey,
			vc_pubkey,
			sgx_build_mode,
			attestation_type,
		}
	}
}

// use the name `RsaRequest` to differentiate from `AesRequest` (see aes_request.rs in
// tee-worker) `Rsa` implies that the payload is RSA-encrypted (using enclave's shielding key)
#[macro_export]
macro_rules! decl_rsa_request {
	($($t:meta),*) => {
		#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, $($t),*)]
		pub struct RsaRequest {
			pub shard: ShardIdentifier,
			pub payload: Vec<u8>,
		}
		impl RsaRequest {
			pub fn new(shard: ShardIdentifier, payload: Vec<u8>) -> Self {
				Self { shard, payload }
			}
		}
	};
}

decl_rsa_request!(TypeInfo, RuntimeDebug);
