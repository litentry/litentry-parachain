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

use crate::{
	test::{
		fixtures::types::ShardIdentifierFor,
		mocks::proposer_mock::{DefaultProposerMock, OutdatedBlockProposerMock},
	},
	ConsensusError,
};
use itp_types::{Block as ParentchainBlock, Header};
use its_consensus_common::Environment;
use its_primitives::types::block::SignedBlock as SignedSidechainBlock;

/// Mock proposer environment.
pub struct EnvironmentMock;

impl Environment<ParentchainBlock, SignedSidechainBlock> for EnvironmentMock {
	type Proposer = DefaultProposerMock;
	type Error = ConsensusError;

	fn init(
		&mut self,
		header: Header,
		_: ShardIdentifierFor<SignedSidechainBlock>,
	) -> Result<Self::Proposer, Self::Error> {
		Ok(DefaultProposerMock { parentchain_header: header })
	}
}

pub struct OutdatedBlockEnvironmentMock;

impl Environment<ParentchainBlock, SignedSidechainBlock> for OutdatedBlockEnvironmentMock {
	type Proposer = OutdatedBlockProposerMock;
	type Error = ConsensusError;

	fn init(
		&mut self,
		header: Header,
		_: ShardIdentifierFor<SignedSidechainBlock>,
	) -> Result<Self::Proposer, Self::Error> {
		Ok(OutdatedBlockProposerMock { parentchain_header: header })
	}
}
