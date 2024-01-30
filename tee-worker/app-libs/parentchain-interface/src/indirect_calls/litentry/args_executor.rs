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

use codec::Encode;
use ita_stf::{Getter, TrustedCall, TrustedCallSigned, TrustedOperation};
use itc_parentchain_indirect_calls_executor::error::{Error, Result};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_types::{ShardIdentifier, H256};
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;

pub trait ArgsExecutor {
	fn error(&self) -> Error;
	fn name() -> &'static str;
	fn shard(&self) -> ShardIdentifier;
	fn prepare_trusted_call<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<TrustedCall>;
	fn execute<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		address: Option<MultiAddress<AccountId32, ()>>,
		hash: H256,
	) -> Result<()> {
		if let Some(address) = address {
			if self.submit(executor, address, hash).is_err() {
				let enclave_account = executor.get_enclave_account()?;
				let trusted_call = match self.error() {
					Error::IMPHandlingError(e) =>
						TrustedCall::handle_imp_error(enclave_account.into(), None, e, hash),

					Error::VCMPHandlingError(e) =>
						TrustedCall::handle_vcmp_error(enclave_account.into(), None, e, hash),
					_ => return Err(Error::Other(("unsupported error").into())),
				};
				let signed_trusted_call =
					executor.sign_call_with_self(&trusted_call, &self.shard())?;
				let trusted_operation =
					TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(
						signed_trusted_call,
					);

				let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;

				executor.submit_trusted_call(self.shard(), encrypted_trusted_call);
				return Ok(())
			}
		}
		Ok(())
	}

	fn submit<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<()> {
		let trusted_call = self.prepare_trusted_call(executor, address, hash)?;
		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &self.shard())?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(self.shard(), encrypted_trusted_call);
		Ok(())
	}
}
