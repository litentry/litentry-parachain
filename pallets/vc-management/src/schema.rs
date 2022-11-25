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

// VC Schema 
// According to https://w3c-ccg.github.io/vc-json-schemas/, it defines JSON Schema for W3C Verifiable Credential.

use crate::{vc_context::Status, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use sp_std::vec::Vec;

type MaxStringLength = ConstU32<1024>;
pub type ContentString = BoundedVec<u8, MaxStringLength>;


#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VCSchema<T: Config> {
	// the schema id
	pub id: ContentString,
	// the schema author
	pub author: T::AccountId,
	// schema content
	pub content: ContentString,
	// status of the Schema
	pub status: Status,
}

impl<T: Config> VCSchema<T> {
	pub fn new(sid: Vec<u8>, author: T::AccountId, scontent: Vec<u8>) -> Self {
		let id: ContentString = sid.clone().try_into().expect("schema id is too long");
		let content: ContentString =
			scontent.clone().try_into().expect("schema content is too long");

		Self { id, author, content, status: Status::Active }
	}
}
