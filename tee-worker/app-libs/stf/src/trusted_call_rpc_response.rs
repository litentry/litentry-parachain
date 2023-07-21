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
use litentry_primitives::AesOutput;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub(crate) struct SetUserShieldingKeyResponse {
	pub account: AccountId,
	pub id_graph: AesOutput,
	pub req_ext_hash: H256,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub(crate) struct LinkIdentityResponse {
	pub account: AccountId,
	pub identity: AesOutput,
	pub id_graph: AesOutput,
	pub req_ext_hash: H256,
}
