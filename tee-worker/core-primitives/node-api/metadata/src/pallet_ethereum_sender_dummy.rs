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

// TODO: maybe use macros to simplify this
use crate::{error::Result, NodeMetadata};

/// Pallet' name:
const ETHSENDER: &str = "EthereumSenderDummy";

pub trait EthereumSenderCallIndexes {
	fn send_blockchain_vc_call_indexes(&self) -> Result<[u8; 2]>;
	fn ethereum_sender_some_error_call_indexes(&self) -> Result<[u8; 2]>;
}

impl EthereumSenderCallIndexes for NodeMetadata {
	fn send_blockchain_vc_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(ETHSENDER, "send_blockchain_vc")
	}

	fn ethereum_sender_some_error_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(ETHSENDER, "some_error")
	}
}
