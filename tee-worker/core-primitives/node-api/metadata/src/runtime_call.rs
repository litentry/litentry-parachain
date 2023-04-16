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

use crate::{error::Result, Error, NodeMetadata};

pub trait RuntimeCall {
	fn retrieve(&self) -> Result<u32>;
}

impl RuntimeCall for NodeMetadata {
	fn retrieve(&self) -> Result<u32> {
		if self.node_metadata.as_ref().is_none() {
			return Err(Error::MetadataNotSet)
		}
		let node_metadata = self.node_metadata.as_ref().unwrap();

		let runtime_call = node_metadata.types().types.iter().find(|ty| {
			let path = &ty.ty.path.segments;
			path.len() == 2 && path[1].as_str() == "RuntimeCall"
		});

		match runtime_call {
			Some(runtime_call) => Ok(runtime_call.id),
			None => Err(Error::NodeMetadata(substrate_api_client::MetadataError::CallNotFound(
				"RuntimeCall not found",
			))),
		}
	}
}
