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

use crate::{pallet_sidechain::SIDECHAIN, pallet_teeracle::TEERACLE, pallet_teerex::TEEREX};
use base58::ToBase58;
use codec::{Decode, Encode};
use itp_sgx_runtime_primitives::types::ShardIdentifier;
use itp_types::{AccountId, Balance};
use litentry_primitives::{ParentchainBlockNumber as BlockNumber, ParentchainHash as Hash};
use log::*;
use primitive_types::H256;
use sp_core::crypto::Ss58Codec;
use substrate_api_client::EventDetails;
use substrate_fixed::types::U32F32;

// ============ pallet_balances ============
#[derive(Decode, Debug)]
pub struct PalletBalancesTransfer {
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

// helper fn to pretty-print the event, returns the count of `ProcessedParentchainBlock` event
pub fn print_event(event: &EventDetails) -> u32 {
	let mut count = 0;
	let pallet_name = event.pallet_name();
	let event_name = event.event_metadata().event();
	debug!(
		"Decoded: phase = {:?}, pallet = {:?} event = {:?}",
		event.phase(),
		pallet_name,
		event_name
	);
	let mut bytes = event.field_bytes();
	match (pallet_name, event_name) {
		("Balances", "Transfer") => {
			if let Ok(PalletBalancesTransfer { from, to, amount }) =
				PalletBalancesTransfer::decode(&mut bytes)
			{
				info!("[+] Received Trasnfer event");
				debug!("    Transactor:  {:?}", from.to_ss58check());
				debug!("    Destination: {:?}", to.to_ss58check());
				debug!("    Value:       {:?}", amount);
			} else {
				warn!("Ignoring unsupported balances event");
			}
		},

		(TEEREX, "AddedEnclave") => {
			if let Ok(PalletTeerexAddedEnclave { sender, worker_url }) =
				PalletTeerexAddedEnclave::decode(&mut bytes)
			{
				info!("[+] Received AddedEnclave event");
				info!("    Sender (Worker):  {:?}", sender);
				info!("    Registered URL: {:?}", worker_url);
			} else {
				warn!("Ignoring unsupported AddedEnclave event");
			}
		},
		(TEEREX, "Forwarded") => {
			if let Ok(PalletTeerexForwarded(shard)) = PalletTeerexForwarded::decode(&mut bytes) {
				info!("[+] Received trusted call for shard {}", shard.encode().to_base58());
			} else {
				warn!("Ignoring unsupported trusted call for shard");
			}
		},
		(TEEREX, "ProcessedParentchainBlock") => {
			if let Ok(PalletTeerexProcessedParentchainBlock {
				sender,
				block_hash,
				block_number,
				merkle_root,
			}) = PalletTeerexProcessedParentchainBlock::decode(&mut bytes)
			{
				count += 1;
				info!("[+] Received ProcessedParentchainBlock event");
				debug!("    From:    {:?}", sender.to_ss58check());
				debug!("    Block Hash: {:?}", hex::encode(block_hash));
				debug!("    Merkle Root: {:?}", hex::encode(merkle_root));
				debug!("    Block Number: {:?}", block_number);
			} else {
				warn!("Ignoring unsupported ProcessedParentchainBlock event");
			}
		},
		(TEEREX, "ShieldFunds") => {
			if let Ok(PalletTeerexShieldFunds(incognito_account)) =
				PalletTeerexShieldFunds::decode(&mut bytes)
			{
				info!("[+] Received ShieldFunds event");
				debug!("    For:    {:?}", hex::encode(incognito_account));
			} else {
				warn!("Ignoring unsupported ShieldFunds event");
			}
		},
		(TEEREX, "UnshieldedFunds") => {
			if let Ok(PalletTeerexUnshieldedFunds(incognito_account)) =
				PalletTeerexUnshieldedFunds::decode(&mut bytes)
			{
				info!("[+] Received UnshieldedFunds event");
				debug!("    For:    {:?}", incognito_account.to_ss58check());
			} else {
				warn!("Ignoring unsupported UnshieldedFunds event");
			}
		},
		(TEEREX, "SetHeartbeatTimeout") => {
			if let Ok(PalletTeerexSetHeartbeatTimeout(timeout)) =
				PalletTeerexSetHeartbeatTimeout::decode(&mut bytes)
			{
				info!("[+] Received SetHeartbeatTimeout event");
				debug!("    For:    {:?}", timeout);
			} else {
				warn!("Ignoring unsupported SetHeartbeatTimeout event");
			}
		},
		(TEERACLE, "ExchangeRateUpdated") => {
			if let Ok(PalletTeeracleExchangeRateUpdated { data_source, currency, exchange_rate }) =
				PalletTeeracleExchangeRateUpdated::decode(&mut bytes)
			{
				info!("[+] Received ExchangeRateUpdated event");
				info!("    Data source:  {:?}", data_source);
				info!("    Currency:  {:?}", currency);
				info!("    Exchange rate: {:?}", exchange_rate);
			} else {
				warn!("Ignoring unsupported ExchangeRateUpdated event");
			}
		},
		(TEERACLE, "ExchangeRateDeleted") => {
			if let Ok(PalletTeeracleExchangeRateDeleted { data_source, currency }) =
				PalletTeeracleExchangeRateDeleted::decode(&mut bytes)
			{
				info!("[+] Received ExchangeRateDeleted event");
				info!("    Data source:  {:?}", data_source);
				info!("    Currency:  {:?}", currency);
			} else {
				warn!("Ignoring unsupported ExchangeRateDeleted event");
			}
		},
		(TEERACLE, "AddedToWhitelist") => {
			if let Ok(PalletTeeracleAddedToWhitelist { data_source, mrenclave }) =
				PalletTeeracleAddedToWhitelist::decode(&mut bytes)
			{
				info!("[+] Received AddedToWhitelist event");
				info!("    Data source:  {:?}", data_source);
				info!("    mrenclave:  {:?}", hex::encode(mrenclave));
			} else {
				warn!("Ignoring unsupported AddedToWhitelist event");
			}
		},
		(TEERACLE, "RemovedFromWhitelist") => {
			if let Ok(PalletTeeracleRemovedFromWhitelist { data_source, mrenclave }) =
				PalletTeeracleRemovedFromWhitelist::decode(&mut bytes)
			{
				info!("[+] Received RemovedFromWhitelist event");
				info!("    Data source:  {:?}", data_source);
				info!("    mrenclave:  {:?}", hex::encode(mrenclave));
			} else {
				warn!("Ignoring unsupported RemovedFromWhitelist event");
			}
		},
		(SIDECHAIN, "ProposedSidechainBlock") => {
			if let Ok(PalletSidechainProposedSidechainBlock { sender, payload }) =
				PalletSidechainProposedSidechainBlock::decode(&mut bytes)
			{
				info!("[+] Received ProposedSidechainBlock event");
				debug!("    From:    {:?}", sender);
				debug!("    Payload: {:?}", hex::encode(payload));
			} else {
				warn!("Ignoring unsupported ProposedSidechainBlock event");
			}
		},
		_ => {
			// 		trace!("Ignoring event {:?}", evr);
		},
	}
	count
}
