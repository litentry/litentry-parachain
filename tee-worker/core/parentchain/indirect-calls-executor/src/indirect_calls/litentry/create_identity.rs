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
use ita_sgx_runtime::{pallet_imt::MetadataOf, Runtime};
use ita_stf::{TrustedCall, TrustedOperation};

use itp_stf_primitives::types::AccountId;

use itp_types::{ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::Identity;
use log::{debug, info};
use sp_core::crypto::AccountId32;
use sp_runtime::MultiAddress;
use sp_std::vec::Vec;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct CreateIdentityArgs {
	shard: ShardIdentifier,
	account: AccountId,
	encrypted_identity: Vec<u8>,
	encrypted_metadata: Option<Vec<u8>>,
}
impl CreateIdentityArgs {
	fn internal_dispatch<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: Option<MultiAddress<AccountId32, ()>>,
		block_number: u32,
		xt_hash: H256,
	) -> Result<()> {
		info!(
			"Found CreateIdentity extrinsic in block: Shard: {}\nAccount {:?}",
			bs58::encode(self.shard.encode()).into_string(),
			self.account
		);
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;
		debug!(
			"execute indirect call: CreateIdentity, who: {:?}, identity: {:?}",
			account_id_to_string(&self.account),
			identity
		);
		let metadata = match &self.encrypted_metadata {
			None => None,
			Some(m) => {
				let decrypted_metadata = executor.decrypt(m)?;
				Some(MetadataOf::<Runtime>::decode(&mut decrypted_metadata.as_slice())?)
			},
		};
		if address.is_some() {
			let enclave_account_id = executor.get_enclave_account()?;
			let trusted_call = TrustedCall::create_identity(
				enclave_account_id,
				self.account.clone(),
				identity,
				metadata,
				block_number,
				xt_hash,
			);
			let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &self.shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);
			let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
			executor.submit_trusted_call(self.shard, encrypted_trusted_call);
		}
		Ok(())
	}
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for CreateIdentityArgs {
	type Args = (Option<MultiAddress<AccountId32, ()>>, u32, H256);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		let (address, block_number, xt_hash) = args;
		let e = Error::IMPHandlingError(IMPError::CreateIdentityFailed(ErrorDetail::ImportError));
		if self.internal_dispatch(executor, address, block_number, xt_hash).is_err() {
			if let Err(internal_e) = executor.submit_trusted_call_from_error(
				self.shard,
				Some(self.account.clone()),
				&e,
				xt_hash,
			) {
				log::warn!("fail to handle internal errors in create_identity: {:?}", internal_e);
			}
			return Err(e)
		}
		Ok(())
	}
}
