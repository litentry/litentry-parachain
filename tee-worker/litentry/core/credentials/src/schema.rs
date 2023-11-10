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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use codec::{Decode, Encode, MaxEncodedLen};
use litentry_primitives::{SchemaContentString, SchemaIdString};
use scale_info::TypeInfo;

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Status {
	#[codec(index = 0)]
	Active,
	#[codec(index = 1)]
	Disabled,
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Schema {
	// the schema id
	pub id: SchemaIdString,
	// schema content
	pub content: SchemaContentString,
	// status of the Schema
	pub status: Status,
}

impl Schema {
	pub fn new(id: SchemaIdString, content: SchemaContentString) -> Self {
		Self { id, content, status: Status::Active }
	}
}
