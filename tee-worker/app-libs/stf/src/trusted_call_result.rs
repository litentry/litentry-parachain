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

use codec::{Decode, Encode};
use itp_stf_interface::StfExecutionResult;
use itp_types::H256;
use litentry_primitives::AesOutput;
use std::vec::Vec;

#[derive(Encode, Decode)]
pub enum TrustedCallResult {
	Empty,
	Streamed,
	SetUserShieldingKey(SetUserShieldingKeyResult),
	LinkIdentity(LinkIdentityResult),
	RequestVC(RequestVCResult),
}

impl StfExecutionResult for TrustedCallResult {
	fn get_encoded_result(self) -> Vec<u8> {
		match self {
			Self::Empty => Vec::default(),
			Self::Streamed => Vec::default(),
			Self::SetUserShieldingKey(result) => result.encode(),
			Self::LinkIdentity(result) => result.encode(),
			Self::RequestVC(result) => result.encode(),
		}
	}

	fn force_connection_wait(&self) -> bool {
		matches!(self, Self::Streamed)
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyResult {
	pub id_graph: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct LinkIdentityResult {
	pub id_graph: AesOutput,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct RequestVCResult {
	pub vc_index: H256,
	pub vc_hash: H256,
	pub vc_payload: AesOutput,
}
