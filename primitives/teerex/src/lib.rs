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
use ias_verify::SgxBuildMode;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;

#[derive(Encode, Decode, Default, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Enclave<PubKey, Url> {
	pub pubkey: PubKey, // FIXME: this is redundant information
	pub mr_enclave: [u8; 32],
	// Todo: make timestamp: Moment
	pub timestamp: u64, // unix epoch in milliseconds
	pub url: Url,       // utf8 encoded url
	pub sgx_mode: SgxBuildMode,
}

impl<PubKey, Url> Enclave<PubKey, Url> {
	pub fn new(
		pubkey: PubKey,
		mr_enclave: [u8; 32],
		timestamp: u64,
		url: Url,
		sgx_build_mode: SgxBuildMode,
	) -> Self {
		Enclave { pubkey, mr_enclave, timestamp, url, sgx_mode: sgx_build_mode }
	}
}

pub type ShardIdentifier = H256;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Request {
	pub shard: ShardIdentifier,
	pub cyphertext: Vec<u8>,
}
