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
	error::{Error, ErrorDetail, Result, VCMPError},
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};

use ita_stf::TrustedCall;

use itp_types::{ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;

use log::debug;
use parachain_core_primitives::Assertion;
use sp_runtime::traits::{AccountIdLookup, StaticLookup};

use crate::indirect_calls::litentry::args_executor::ArgsExecutor;
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RequestVCArgs {
	shard: ShardIdentifier,
	assertion: Assertion,
}

impl ArgsExecutor for RequestVCArgs {
	fn error(&self) -> Error {
		Error::VCMPHandlingError(VCMPError::RequestVCFailed(
			self.assertion.clone(),
			ErrorDetail::ImportError,
		))
	}

	fn name() -> &'static str {
		"RequestVC"
	}

	fn shard(&self) -> ShardIdentifier {
		self.shard
	}

	fn prepare_trusted_call<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: MultiAddress<AccountId32, ()>,
		hash: H256,
	) -> Result<TrustedCall> {
		let account = AccountIdLookup::lookup(address).unwrap();
		debug!(
			"indirect call Requested VC, who:{:?}, assertion: {:?}",
			account_id_to_string(&account),
			self.assertion
		);
		let enclave_account_id = executor.get_enclave_account().unwrap();
		Ok(TrustedCall::request_vc(
			enclave_account_id.into(),
			account.into(),
			self.assertion.clone(),
			hash,
		))
	}
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for RequestVCArgs {
	type Args = (Option<MultiAddress<AccountId32, ()>>, H256);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		self.execute(executor, args.0, args.1)
	}
}
