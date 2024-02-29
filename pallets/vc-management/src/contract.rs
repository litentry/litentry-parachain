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

// VC Schema
// According to https://w3c-ccg.github.io/vc-json-schemas/, it defines JSON Schema for W3C Verifiable Credential.

use crate::{Config, ContractBytecode};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Contract<T: Config> {
	pub creator: T::AccountId,
	pub code: ContractBytecode,
}

impl<T: Config> Contract<T> {
	pub fn new(creator: T::AccountId, code: ContractBytecode) -> Self {
		Self { creator, code }
	}
}
