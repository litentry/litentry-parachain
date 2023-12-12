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

use crate::indirect_calls::litentry::args_executor::ArgsExecutor;
use codec::{Decode, Encode};
use ita_stf::{TrustedCall, TrustedCallSigned};
use itc_parentchain_indirect_calls_executor::{
	error::{Error, IMPError, Result},
	IndirectDispatch,
};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_types::{ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{ErrorDetail, Identity};
use log::debug;
use sp_core::crypto::AccountId32;
use sp_runtime::{
	traits::{AccountIdLookup, StaticLookup},
	MultiAddress,
};
use std::vec::Vec;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct DeactivateIdentityArgs {
	shard: ShardIdentifier,
	encrypted_identity: Vec<u8>,
}

impl ArgsExecutor for DeactivateIdentityArgs {
	fn error(&self) -> Error {
		Error::IMPHandlingError(IMPError::DeactivateIdentityFailed(ErrorDetail::ImportError))
	}

	fn name() -> &'static str {
		"DeactivateIdentity"
	}

	fn shard(&self) -> ShardIdentifier {
		self.shard
	}

	fn prepare_trusted_call<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<TrustedCall> {
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;
		let account = AccountIdLookup::lookup(address).unwrap();
		debug!(
			"execute indirect call: DeactivateIdentity, who: {:?}, identity: {:?}",
			account_id_to_string(&account),
			identity
		);

		let enclave_account_id = executor.get_enclave_account().unwrap();
		Ok(TrustedCall::deactivate_identity(
			enclave_account_id.into(),
			account.into(),
			identity,
			None,
			hash,
		))
	}
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for DeactivateIdentityArgs
{
	type Args = (Option<MultiAddress<AccountId32, ()>>, H256);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		self.execute(executor, args.0, args.1)
	}
}
