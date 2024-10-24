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

use crate::listener::IntentEventId;
use executor_core::primitives::GetEventId;
use executor_core::sync_checkpoint_repository::Checkpoint;
use parity_scale_codec::{Decode, Encode};

/// Used to uniquely identify intent event on parentchain.
#[derive(Clone, Debug)]
pub struct EventId {
	pub block_num: u64,
	pub event_idx: u64,
}

impl EventId {
	pub fn new(block_num: u64, event_idx: u64) -> Self {
		Self { block_num, event_idx }
	}
}

/// Represents parentchain sync checkpoint.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct SyncCheckpoint {
	pub block_num: u64,
	pub event_idx: Option<u64>,
}

impl SyncCheckpoint {
	pub fn new(block_num: u64, event_idx: Option<u64>) -> Self {
		Self { block_num, event_idx }
	}

	pub fn from_event_id(event_id: &EventId) -> Self {
		Self::new(event_id.block_num, Some(event_id.event_idx))
	}

	pub fn from_block_num(block_num: u64) -> Self {
		Self::new(block_num, None)
	}

	pub fn just_block_num(&self) -> bool {
		self.event_idx.is_none()
	}
}

impl Checkpoint for SyncCheckpoint {
	fn just_block_num(&self) -> bool {
		self.event_idx.is_none()
	}

	fn get_block_num(&self) -> u64 {
		self.block_num
	}
}

impl From<u64> for SyncCheckpoint {
	fn from(block_num: u64) -> Self {
		Self::from_block_num(block_num)
	}
}

impl From<IntentEventId> for SyncCheckpoint {
	fn from(event_id: IntentEventId) -> Self {
		Self::from_event_id(&event_id)
	}
}

impl PartialOrd for SyncCheckpoint {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.block_num > other.block_num {
			Some(std::cmp::Ordering::Greater)
		} else if self.block_num < other.block_num {
			Some(std::cmp::Ordering::Less)
		} else if self.event_idx > other.event_idx {
			Some(std::cmp::Ordering::Greater)
		} else if self.event_idx < other.event_idx {
			Some(std::cmp::Ordering::Less)
		} else {
			Some(std::cmp::Ordering::Equal)
		}
	}
}

pub struct BlockEvent {
	pub id: EventId,
	pub pallet_name: String,
	pub variant_name: String,
	pub variant_index: u8,
	pub field_bytes: Vec<u8>,
}

impl BlockEvent {
	pub fn new(
		id: EventId,
		pallet_name: String,
		variant_name: String,
		variant_index: u8,
		field_bytes: Vec<u8>,
	) -> Self {
		Self { id, pallet_name, variant_name, variant_index, field_bytes }
	}
}

impl GetEventId<EventId> for BlockEvent {
	fn get_event_id(&self) -> EventId {
		self.id.clone()
	}
}
