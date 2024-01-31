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

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum DcapProvider {
	#[default]
	MAA,
	Intel,
	Local,
	Integritee,
}

#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum RemoteAttestationType {
	#[default]
	Ignore,
	Ias,
	Dcap(DcapProvider),
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
	#[codec(index = 0)]
	Debug,
	#[default]
	#[codec(index = 1)]
	Production,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
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

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, RuntimeDebug)]
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
