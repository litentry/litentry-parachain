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

use crate::{command_utils::get_chain_api, Cli};
use base58::ToBase58;
use codec::{Decode, Encode};
use itp_node_api::metadata::{
	pallet_sidechain::SIDECHAIN, pallet_teeracle::TEERACLE, pallet_teerex::TEEREX,
};
use itp_types::event::{
	PalletBalancesTrasnfer, PalletSidechainProposedSidechainBlock, PalletTeeracleAddedToWhitelist,
	PalletTeeracleExchangeRateDeleted, PalletTeeracleExchangeRateUpdated,
	PalletTeeracleRemovedFromWhitelist, PalletTeerexAddedEnclave, PalletTeerexForwarded,
	PalletTeerexProcessedParentchainBlock, PalletTeerexSetHeartbeatTimeout,
	PalletTeerexShieldFunds, PalletTeerexUnshieldedFunds,
};
use log::*;
use sp_application_crypto::Ss58Codec;
use std::{sync::mpsc::channel, vec::Vec};
use substrate_api_client::{utils::FromHexString, EventDetails, Events};

#[derive(Parser)]
pub struct ListenCommand {
	/// exit after given number of parentchain events
	#[clap(short, long = "exit-after")]
	events: Option<u32>,

	/// exit after given number of blocks
	#[clap(short, long = "await-blocks")]
	blocks: Option<u32>,
}

impl ListenCommand {
	pub(crate) fn run(&self, cli: &Cli) {
		println!("{:?} {:?}", self.events, self.blocks);
		let api = get_chain_api(cli);
		info!("Subscribing to events");
		let (events_in, events_out) = channel();
		#[allow(unused)]
		let mut count = 0u32;
		let mut blocks = 0u32;
		api.subscribe_events(events_in).unwrap();
		loop {
			if let Some(e) = self.events {
				if count >= e {
					return
				}
			};
			if let Some(b) = self.blocks {
				if blocks >= b {
					return
				}
			};

			let events_str = events_out.recv().unwrap();
			let event_bytes = Vec::from_hex(events_str).unwrap();
			let metadata = api.metadata.clone();
			blocks += 1;
			let events = Events::new(metadata, Default::default(), event_bytes);
			for maybe_event_details in events.iter() {
				let event_details = maybe_event_details.unwrap();
				print_event(count, &event_details);
			}
		}
	}
}

fn print_event(mut _count: u32, event: &EventDetails) {
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
			if let Ok(PalletBalancesTrasnfer { from, to, amount }) =
				PalletBalancesTrasnfer::decode(&mut bytes)
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
			_count += 1;
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
}
