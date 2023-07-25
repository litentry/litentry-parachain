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

use crate::{
	error::{Error, ErrorDetail, IMPError, Result},
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};

use ita_stf::{TrustedCall, TrustedOperation};

use itp_types::{ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::Identity;
use log::debug;
use sp_runtime::traits::{AccountIdLookup, StaticLookup};
use std::vec::Vec;
use substrate_api_client::GenericAddress;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct ActivateIdentityArgs {
	shard: ShardIdentifier,
	encrypted_identity: Vec<u8>,
}

impl ActivateIdentityArgs {
	fn internal_dispatch<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: Option<GenericAddress>,
		hash: H256,
	) -> Result<()> {
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;

		if let Some(address) = address {
			let account = AccountIdLookup::lookup(address)?;
			debug!(
				"execute indirect call: ActivateIdentity, who: {:?}, identity: {:?}",
				account_id_to_string(&account),
				identity
			);

			let enclave_account_id = executor.get_enclave_account()?;
			let trusted_call = TrustedCall::activate_identity(
				enclave_account_id.into(),
				account.into(),
				identity,
				hash,
			);
			let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &self.shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

			let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
			executor.submit_trusted_call(self.shard, encrypted_trusted_call);
		}
		Ok(())
	}
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for ActivateIdentityArgs {
	type Args = (Option<GenericAddress>, H256);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		let (address, hash) = args;
		let e = Error::IMPHandlingError(IMPError::ActivateIdentityFailed(ErrorDetail::ImportError));
		if self.internal_dispatch(executor, address, hash).is_err() {
			if let Err(internal_e) =
				executor.submit_trusted_call_from_error(self.shard, None, &e, hash)
			{
				log::warn!("fail to handle internal errors in activate_identity: {:?}", internal_e);
			}
			return Err(e)
		}
		Ok(())
	}
}
