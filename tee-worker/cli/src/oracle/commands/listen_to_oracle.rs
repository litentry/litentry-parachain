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
use codec::Decode;
use itp_node_api::{
	api_client::ParentchainApi,
	metadata::{event::PalletTeeracleOracleUpdated, pallet_teeracle::TEERACLE},
};
use itp_time_utils::{duration_now, remaining_time};
use log::{debug, warn};
use std::{sync::mpsc::channel, time::Duration};
use substrate_api_client::{Events, FromHexString};

/// Listen to exchange rate events.
#[derive(Debug, Clone, Parser)]
pub struct ListenToOracleEventsCmd {
	/// Listen for `duration` in seconds.
	duration: u64,
}

type EventCount = u32;

impl ListenToOracleEventsCmd {
	pub fn run(&self, cli: &Cli) {
		let api = get_chain_api(cli);
		let duration = Duration::from_secs(self.duration);
		let count = count_oracle_update_events(&api, duration);
		println!("Number of Oracle events received : ");
		println!("   EVENTS_COUNT: {}", count);
	}
}

fn count_oracle_update_events(api: &ParentchainApi, duration: Duration) -> EventCount {
	let stop = duration_now() + duration;

	//subscribe to events
	let (events_in, events_out) = channel();
	api.subscribe_events(events_in).unwrap();
	let mut count = 0;

	while remaining_time(stop).unwrap_or_default() > Duration::ZERO {
		let events_str = events_out.recv().unwrap();
		let event_bytes = Vec::from_hex(events_str).unwrap();
		let metadata = api.metadata.clone();
		let events = Events::new(metadata, Default::default(), event_bytes);
		for maybe_event_details in events.iter() {
			let event_details = maybe_event_details.unwrap();
			let pallet_name = event_details.pallet_name();
			let event_name = event_details.event_metadata().event();
			debug!(
				"Decoded: phase = {:?}, pallet = {:?} event = {:?}",
				event_details.phase(),
				pallet_name,
				event_name
			);
			if let (TEERACLE, "OracleUpdated") = (pallet_name, event_name) {
				let mut bytes = event_details.field_bytes();
				if let Ok(PalletTeeracleOracleUpdated { oracle_data_name, data_source }) =
					PalletTeeracleOracleUpdated::decode(&mut bytes)
				{
					count += 1;
					debug!("Received OracleUpdated event");
					println!(
						"OracleUpdated: ORACLE_NAME : {}, SRC : {}",
						oracle_data_name, data_source
					);
				} else {
					warn!("Ignoring unsupported OracleUpdated event");
				}
			}
		}
	}
	debug!("Received {} OracleUpdated event(s) in total", count);
	count
}
