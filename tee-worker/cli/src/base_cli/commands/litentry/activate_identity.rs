// Copyright 2020-2024 Trust Computing GmbH.
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

use super::IMP;
use crate::{
	command_utils::{get_chain_api, *},
	Cli, CliResult, CliResultOk,
};
use codec::{Decode, Encode};
use itc_rpc_client::direct_client::DirectApi;
use itp_stf_primitives::types::ShardIdentifier;
use lc_rsa_wrapper::RsaWrapperEncrypt;
use litentry_primitives::Identity;
use log::*;
use sp_core::sr25519 as sr25519_core;
use substrate_api_client::{ac_compose_macros::compose_extrinsic, SubmitAndWatch, XtStatus};

#[derive(Parser)]
pub struct ActivateIdentityCommand {
	/// AccountId in ss58check format
	account: String,
	/// Identity to be created, in did form
	did: String,
	/// Delegate signer for the account
	#[clap(short = 'd')]
	delegate: Option<String>,
}

impl ActivateIdentityCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let mut chain_api = get_chain_api(cli);

		let direct_api = get_worker_api_direct(cli);
		let mrenclave = direct_api.get_state_mrenclave().unwrap();
		let shard = ShardIdentifier::decode(&mut &mrenclave[..]).unwrap();
		let signer = self.get_signer();

		chain_api.set_signer(signer.into());

		let (_, encrypted_identity) = self.encrypt_identity(cli);

		let xt = compose_extrinsic!(chain_api, IMP, "activate_identity", shard, encrypted_identity);

		let tx_hash = chain_api.submit_and_watch_extrinsic_until(xt, XtStatus::Finalized).unwrap();
		println!("[+] ActivateIdentityCommand got finalized. Hash: {:?}", tx_hash);

		Ok(CliResultOk::None)
	}

	fn get_signer(&self) -> sr25519_core::Pair {
		let account = self.delegate.as_ref().unwrap_or(&self.account);
		get_pair_from_str(account).into()
	}

	fn encrypt_identity(&self, cli: &Cli) -> (Identity, Vec<u8>) {
		let identity = Identity::from_did(&self.did).unwrap();
		let tee_shielding_key = get_shielding_key(cli).unwrap();
		let encrypted_identity =
			tee_shielding_key.encrypt_with_rsa_wrapper(&identity.encode()).unwrap();
		(identity, encrypted_identity)
	}
}
