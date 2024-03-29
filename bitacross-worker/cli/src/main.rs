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

use bitacross_cli::{commands, Cli};
use clap::Parser;

fn main() {
	env_logger::builder()
		.format_timestamp(Some(env_logger::TimestampPrecision::Millis))
		.init();

	let cli = Cli::parse();

	commands::match_command(&cli).unwrap();
}
