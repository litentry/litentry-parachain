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

use crate::{error::Result, NodeMetadata};

/// Pallet name:
const VCM: &str = "VCManagement";

pub trait VCMCallIndexes {
	fn vc_schema_issued_call_indexes(&self) -> Result<[u8; 2]>;
}

impl VCMCallIndexes for NodeMetadata {
	fn vc_schema_issued_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(VCM, "add_schema")
	}
}
