/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use std::sync::RwLock;

use codec::{Decode, Encode};
use core::marker::PhantomData;
use itp_ocall_api::{EnclaveMetricsOCallApi};
use sgx_types::{SgxResult};
use std::{vec::Vec};

pub struct SidechainOCallApiMock<SignedSidechainBlockType> {
	fetch_from_peer_blocks: Option<Vec<SignedSidechainBlockType>>,
	number_of_fetch_calls: RwLock<usize>,
	_phantom: PhantomData<SignedSidechainBlockType>,
}

impl<SignedSidechainBlockType> SidechainOCallApiMock<SignedSidechainBlockType>
where
	SignedSidechainBlockType: Clone + Encode + Decode + Send + Sync,
{
	pub fn with_peer_fetch_blocks(mut self, blocks: Vec<SignedSidechainBlockType>) -> Self {
		self.fetch_from_peer_blocks = Some(blocks);
		self
	}

	pub fn number_of_fetch_calls(&self) -> usize {
		*self.number_of_fetch_calls.read().unwrap()
	}
}

impl<SignedSidechainBlockType> Default for SidechainOCallApiMock<SignedSidechainBlockType> {
	fn default() -> Self {
		SidechainOCallApiMock {
			fetch_from_peer_blocks: None,
			number_of_fetch_calls: RwLock::new(0),
			_phantom: Default::default(),
		}
	}
}

impl<SignedSidechainBlockType> Clone for SidechainOCallApiMock<SignedSidechainBlockType>
where
	SignedSidechainBlockType: Clone + Encode + Decode + Send + Sync,
{
	fn clone(&self) -> Self {
		SidechainOCallApiMock {
			fetch_from_peer_blocks: self.fetch_from_peer_blocks.clone(),
			number_of_fetch_calls: RwLock::new(*self.number_of_fetch_calls.read().unwrap()),
			_phantom: self._phantom,
		}
	}
}

impl<SignedSidechainBlockType> EnclaveMetricsOCallApi
	for SidechainOCallApiMock<SignedSidechainBlockType>
where
	SignedSidechainBlockType: Clone + Encode + Decode + Send + Sync,
{
	fn update_metric<Metric: Encode>(&self, _metric: Metric) -> SgxResult<()> {
		Ok(())
	}
}