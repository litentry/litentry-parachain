// Copyright 2020-2023 Trust Computing GmbH.
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

use crate::{
	trusted_cli::TrustedCli, trusted_command_utils::get_pair_from_str,
	trusted_operation::perform_trusted_operation, Cli, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{IDGraph, Runtime, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::Identity;

#[derive(Parser)]
pub struct IDGraphCommand {
	// did format - will be converted to `Identity`
	// not using e.g. ss58 format as we support both evm and substrate
	did: String,
}

impl IDGraphCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice");
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();

		let top: TrustedOperation =
			TrustedGetter::id_graph(id).sign(&KeyPair::Sr25519(Box::new(alice))).into();
		let idgraph = perform_trusted_operation(cli, trusted_cli, &top)
			.map(|v| IDGraph::<Runtime>::decode(&mut v.unwrap().as_slice()).ok());
		println!("{:#?}", idgraph.unwrap().unwrap());

		Ok(CliResultOk::None)
	}
}
