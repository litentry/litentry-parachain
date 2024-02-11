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

// TODO: maybe use macros to simplify this
use crate::{error::Result, NodeMetadata};

const BITACROSS: &str = "Bitacross";

pub trait BitAcrossCallIndexes {
	fn add_relayer_call_indexes(&self) -> Result<[u8; 2]>;
	fn remove_relayer_call_indexes(&self) -> Result<[u8; 2]>;
}

impl BitAcrossCallIndexes for NodeMetadata {
	fn add_relayer_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(BITACROSS, "add_relayer")
	}

	fn remove_relayer_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(BITACROSS, "remove_relayer")
	}
}
