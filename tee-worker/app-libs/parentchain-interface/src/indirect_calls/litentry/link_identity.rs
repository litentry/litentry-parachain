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

use crate::indirect_calls::litentry::args_executor::ArgsExecutor;
use codec::{Decode, Encode};
use ita_stf::{TrustedCall, TrustedCallSigned};
use itc_parentchain_indirect_calls_executor::{
	error::{Error, IMPError, Result},
	IndirectDispatch,
};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_types::{AccountId, ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{ErrorDetail, Identity, ValidationData, Web3Network};
use log::debug;
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;
use std::vec::Vec;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct LinkIdentityArgs {
	shard: ShardIdentifier,
	account: AccountId,
	encrypted_identity: Vec<u8>,
	encrypted_validation_data: Vec<u8>,
	encrypted_web3networks: Vec<u8>,
}

impl ArgsExecutor for LinkIdentityArgs {
	fn error(&self) -> Error {
		Error::IMPHandlingError(IMPError::LinkIdentityFailed(ErrorDetail::ImportError))
	}

	fn name() -> &'static str {
		"LinkIdentity"
	}

	fn shard(&self) -> ShardIdentifier {
		self.shard
	}

	fn prepare_trusted_call<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		_address: MultiAddress<AccountId, ()>,
		hash: H256,
	) -> Result<TrustedCall> {
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;
		let validation_data = ValidationData::decode(
			&mut executor.decrypt(&self.encrypted_validation_data)?.as_slice(),
		)?;
		let web3networks: Vec<Web3Network> =
			Decode::decode(&mut executor.decrypt(&self.encrypted_web3networks)?.as_slice())?;

		debug!(
			"indirect call LinkIdentity, who:{:?}, identity: {:?}, validation_data: {:?}",
			account_id_to_string(&self.account),
			identity,
			validation_data
		);

		let enclave_account_id = executor.get_enclave_account().unwrap();
		Ok(TrustedCall::link_identity(
			enclave_account_id.into(),
			self.account.clone().into(),
			identity,
			validation_data,
			web3networks,
			None,
			hash,
		))
	}
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for LinkIdentityArgs
{
	type Args = (Option<MultiAddress<AccountId32, ()>>, H256);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		self.execute(executor, args.0, args.1)
	}
}
