// Copyright 2020-2023 Litentry Technologies GmbH.
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

// This file contain the RPC response struct which will be encoded and
// passed back to the requester of trustedCall direct invocation (DI).
// They are mostly translated from the callback extrinsics in IMP.

use crate::AccountId;
use codec::{Decode, Encode};
use itp_types::H256;
use litentry_primitives::{AesOutput, Assertion};
use std::vec::Vec;
use itp_stf_interface::EncodeResult;

#[derive(Encode, Decode)]
pub enum TrustedCallResult {
	Empty,
	Streamed,
	SetUserShieldingKey(SetUserShieldingKeyResult),
	LinkIdentity(LinkIdentityResult),
	DeactivateIdentity(DeactivateIdentityResult),
	ActivateIdentity(ActivateIdentityResult),
	RequestVC(RequestVCResult),
}

impl EncodeResult for TrustedCallResult {
	fn get_encoded_result(self) -> Vec<u8> {
		match self {
			Self::Empty => Vec::default(),
			// true means that there are more results to come, see rpc_responder
			Self::Streamed => true.encode(),
			Self::SetUserShieldingKey(result) => result.encode(),
			Self::LinkIdentity(result)=> result.encode(),
			Self::DeactivateIdentity(result)=> result.encode(),
				Self::ActivateIdentity(result)=> result.encode(),
			Self::RequestVC(result) => result.encode(),
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyResult {
	pub account: AccountId,
	pub id_graph: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct LinkIdentityResult {
	pub account: AccountId,
	pub identity: AesOutput,
	pub id_graph: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct DeactivateIdentityResult {
	pub account: AccountId,
	pub identity: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct ActivateIdentityResult {
	pub account: AccountId,
	pub identity: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct RequestVCResult {
	pub account: AccountId,
	pub assertion: Assertion,
	pub vc_index: H256,
	pub vc_hash: H256,
	pub vc_payload: AesOutput,
}
