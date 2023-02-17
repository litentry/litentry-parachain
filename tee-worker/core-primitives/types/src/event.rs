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

use crate::{AccountId, Balance};
use codec::Decode;
use itp_sgx_runtime_primitives::types::ShardIdentifier;
use litentry_primitives::{ParentchainBlockNumber as BlockNumber, ParentchainHash as Hash};
use primitive_types::H256;
use substrate_fixed::types::U32F32;

// ============ pallet_balances ============
#[derive(Decode, Debug)]
pub struct PalletBalancesTrasnfer {
	pub from: AccountId,
	pub to: AccountId,
	pub amount: Balance,
}

// ============ pallet_teerex ============
#[derive(Decode, Debug)]
pub struct PalletTeerexAddedEnclave {
	pub sender: AccountId,
	pub worker_url: String,
}

#[derive(Decode, Debug)]
pub struct PalletTeerexForwarded(pub ShardIdentifier);

#[derive(Decode, Debug)]
pub struct PalletTeerexProcessedParentchainBlock {
	pub sender: AccountId,
	pub block_hash: Hash,
	pub merkle_root: Hash,
	pub block_number: BlockNumber,
}

#[derive(Decode, Debug)]
pub struct PalletTeerexShieldFunds(pub Vec<u8>);

#[derive(Decode, Debug)]
pub struct PalletTeerexUnshieldedFunds(pub AccountId);

#[derive(Decode, Debug)]
pub struct PalletTeerexSetHeartbeatTimeout(pub u64);

// ============ pallet_teeracle ============
#[derive(Decode, Debug)]
pub struct PalletTeeracleExchangeRateUpdated {
	pub data_source: String,
	pub currency: String,
	pub exchange_rate: U32F32,
}

#[derive(Decode, Debug)]
pub struct PalletTeeracleExchangeRateDeleted {
	pub data_source: String,
	pub currency: String,
}

#[derive(Decode, Debug)]
pub struct PalletTeeracleOracleUpdated {
	pub oracle_data_name: String,
	pub data_source: String,
}

#[derive(Decode, Debug)]
pub struct PalletTeeracleAddedToWhitelist {
	pub data_source: String,
	pub mrenclave: [u8; 32],
}

#[derive(Decode, Debug)]
pub struct PalletTeeracleRemovedFromWhitelist {
	pub data_source: String,
	pub mrenclave: [u8; 32],
}

// ============ pallet_sidechain ============
#[derive(Decode, Debug)]
pub struct PalletSidechainProposedSidechainBlock {
	pub sender: String,
	pub payload: H256,
}
