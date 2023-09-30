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
	error::{Error, Result},
	IndirectExecutor,
};
use codec::Encode;
use ita_stf::{TrustedCall, TrustedOperation};
use itp_types::{ShardIdentifier, H256};
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;

pub trait ArgsExecutor {
	fn error(&self) -> Error;
	fn name() -> &'static str;
	fn shard(&self) -> ShardIdentifier;
	fn prepare_trusted_call<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<TrustedCall>;
	fn execute<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: Option<MultiAddress<AccountId32, ()>>,
		hash: H256,
	) -> Result<()> {
		if let Some(address) = address {
			if self.submit(executor, address, hash).is_err() {
				if let Err(internal_e) =
					executor.submit_trusted_call_from_error(self.shard(), None, &self.error(), hash)
				{
					log::warn!(
						"fail to handle internal errors in {}: {:?}",
						<Self as ArgsExecutor>::name(),
						internal_e
					);
				}
				return Err(self.error())
			}
		}
		Ok(())
	}

	fn submit<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<()> {
		let trusted_call = self.prepare_trusted_call(executor, address, hash)?;
		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &self.shard())?;
		let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(self.shard(), encrypted_trusted_call);
		Ok(())
	}
}
