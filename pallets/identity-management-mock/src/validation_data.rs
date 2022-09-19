// Copyright 2020-2022 Litentry Technologies GmbH.
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

// The serialized validation data for verifying an identity
// we simply use parity codec to ser/de the data and transfer around
// (instead of serde_json)
//
// It's written in this IMP-mock crate just to give an idea how the
// `validation_data` should look like, the decoding and usage of it
// is done within TEE, where std(or sgx_tstd) can be enabled.
//
// The validation data itself is always sent along with DID, so the
// the struct doesn't include did.

use codec::{Decode, Encode};
use sp_runtime::MultiSignature;
use sp_std::vec::Vec;

// for now we use a simple link for web2 validation.
// When it's not enough, we'll have to extend it and/or
// write platform-specific struct
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct Web2ValidationData {
	pub link: Vec<u8>, // or String if under std
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct Web3ValidationData {
	pub message: Vec<u8>, // or String if under std
	pub signature: MultiSignature,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ValidationData {
	web2(Web2ValidationData),
	web3(Web3ValidationData),
}
