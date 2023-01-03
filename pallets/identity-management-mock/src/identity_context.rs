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

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebugNoBound;
use scale_info::TypeInfo;

use crate::{BlockNumberOf, Config, Metadata};

// The context associated with the (litentry-account, did) pair
// TODO: maybe we have better naming
#[derive(Clone, Eq, PartialEq, RuntimeDebugNoBound, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct IdentityContext<T: Config> {
	// the metadata
	pub metadata: Option<Metadata>,
	// the block number (of parent chain) where the creation was intially requested
	pub creation_request_block: Option<BlockNumberOf<T>>,
	// the block number (of parent chain) where the verification was intially requested
	pub verification_request_block: Option<BlockNumberOf<T>>,
	// if this did is verified
	pub is_verified: bool,
}

// rust imposes overly restrictive bounds sometimes, see
// https://github.com/rust-lang/rust/issues/99463
impl<T: Config> Default for IdentityContext<T> {
	fn default() -> Self {
		Self {
			metadata: None,
			creation_request_block: None,
			verification_request_block: None,
			is_verified: false,
		}
	}
}

impl<T: Config> IdentityContext<T> {
	pub fn new(
		creation_request_block: BlockNumberOf<T>,
		verification_request_block: BlockNumberOf<T>,
	) -> Self {
		Self {
			metadata: None,
			creation_request_block: Some(creation_request_block),
			verification_request_block: Some(verification_request_block),
			is_verified: false,
		}
	}
}
