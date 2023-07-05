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
	command_utils::get_worker_api_direct,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli,
};
use ita_stf::{TrustedCall, TrustedOperation};
use itc_rpc_client::direct_client::DirectApi;
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::ParentchainBalance as Balance;
use log::*;
use sp_core::{crypto::Ss58Codec, Pair};
use std::boxed::Box;

#[derive(Parser)]
pub struct SetBalanceCommand {
	/// sender's AccountId in ss58check format
	account: String,

	/// amount to be transferred
	amount: Balance,
}

impl SetBalanceCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) {
		let who = get_pair_from_str(trusted_args, &self.account);
		let signer = get_pair_from_str(trusted_args, "//Alice");
		info!("account ss58 is {}", who.public().to_ss58check());

		println!("send trusted call set-balance({}, {})", who.public(), self.amount);

		let (mrenclave, shard) = get_identifiers(trusted_args);
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce = worker_api_direct.get_next_nonce(&shard, &(who.public().into())).unwrap();
		info!("nonce {:?} ", nonce);
		let top: TrustedOperation = TrustedCall::balance_set_balance(
			signer.public().into(),
			who.public().into(),
			self.amount,
			self.amount,
		)
		.sign(&KeyPair::Sr25519(Box::new(signer)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_args.direct);
		let _ = perform_trusted_operation(cli, trusted_args, &top);
	}
}
