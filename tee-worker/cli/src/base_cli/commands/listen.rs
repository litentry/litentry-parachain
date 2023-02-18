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
use itp_node_api::metadata::event::print_event;
use log::*;
use std::{sync::mpsc::channel, vec::Vec};
use substrate_api_client::{utils::FromHexString, Events};

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
				count += print_event(&event_details);
			}
		}
	}
}
