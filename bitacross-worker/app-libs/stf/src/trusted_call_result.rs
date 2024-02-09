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

// This file contain the RPC response struct which will be encoded and
// passed back to the requester of trustedCall direct invocation (DI).
// They are mostly translated from the callback extrinsics in IMP.

use codec::{Decode, Encode};
use itp_stf_interface::StfExecutionResult;
use std::vec::Vec;

#[derive(Encode, Decode, Debug)]
pub enum TrustedCallResult {
	#[codec(index = 0)]
	Empty,
	#[codec(index = 1)]
	Streamed,
}

impl StfExecutionResult for TrustedCallResult {
	fn get_encoded_result(self) -> Vec<u8> {
		match self {
			Self::Empty => Vec::default(),
			Self::Streamed => Vec::default(),
		}
	}

	fn force_connection_wait(&self) -> bool {
		matches!(self, Self::Streamed)
	}
}
