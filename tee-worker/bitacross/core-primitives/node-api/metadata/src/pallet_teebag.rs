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

use crate::{error::Result, NodeMetadata};

/// Pallet' name:
pub const TEEBAG: &str = "Teebag";

// we only list the extrinsics that we care
pub trait TeebagCallIndexes {
	fn force_add_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn force_remove_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]>;

	fn post_opaque_task_call_indexes(&self) -> Result<[u8; 2]>;

	fn parentchain_block_processed_call_indexes(&self) -> Result<[u8; 2]>;

	fn sidechain_block_imported_call_indexes(&self) -> Result<[u8; 2]>;
}

impl TeebagCallIndexes for NodeMetadata {
	fn force_add_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "force_add_authorized_enclave")
	}
	fn force_remove_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "force_remove_authorized_enclave")
	}
	fn register_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_enclave")
	}
	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "unregister_enclave")
	}
	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_quoting_enclave")
	}
	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_tcb_info")
	}
	fn post_opaque_task_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "post_opaque_task")
	}
	fn parentchain_block_processed_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "parentchain_block_processed")
	}
	fn sidechain_block_imported_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "sidechain_block_imported")
	}
}
