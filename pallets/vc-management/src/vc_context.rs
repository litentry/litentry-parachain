// Copyright 2020-2023 Trust Computing GmbH.
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

use codec::{Decode, Encode, MaxEncodedLen};
use core_primitives::{Assertion, Identity};
use scale_info::TypeInfo;
use sp_core::H256;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Status {
	#[codec(index = 0)]
	Active,
	#[codec(index = 1)]
	Disabled,
	// Revoked, // commented out for now, we can delete the VC entry when revoked
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct VCContext {
	// To be discussed: shall we make it public?
	// pros: easier for the user to disable/revoke VCs, we'll need the AccountId to verify
	//       the owner of VC. An alternative is to store such information within TEE.
	// cons: this information is then public, everyone knows e.g. ALICE owns VC ID 1234 + 4321
	// It's not bad though as it helps to verify the ownership of VC
	pub subject: Identity,
	// requested assertion type
	pub assertion: Assertion,
	// hash of the VC, computed via blake2_256
	pub hash: H256,
	// status of the VC
	pub status: Status,
}

impl VCContext {
	pub fn new(subject: Identity, assertion: Assertion, hash: H256) -> Self {
		Self { subject, assertion, hash, status: Status::Active }
	}
}
