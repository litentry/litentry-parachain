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
	error::{Error, ErrorDetail, IMPError},
};
use crate::{error::Result, IndirectDispatch, IndirectExecutor};
use codec::{Decode, Encode};
use ita_stf::{TrustedCall, TrustedOperation};
use itp_stf_primitives::types::AccountId;
use itp_types::{Balance, ShardIdentifier};
use log::{debug, info};
use std::vec::Vec;
use ita_sgx_runtime::{pallet_imt::MetadataOf, Runtime};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
		pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{CreateIdentityFn, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{Identity, ParentchainBlockNumber, ValidationData};
use log::*;
use substrate_api_client::GenericAddress;
use sp_runtime::traits::{AccountIdLookup, StaticLookup};

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct VerifyIdentityArgs {
	shard: ShardIdentifier,
	encrypted_identity: Vec<u8>,
	encrypted_validation_data: Vec<u8>,
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for VerifyIdentityArgs {
	type Args = (Option<GenericAddress>, H256, u32);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		let (address, hash, block) = args;

		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;
		let validation_data = ValidationData::decode(
			&mut executor.decrypt(&self.encrypted_validation_data)?.as_slice(),
		)?;

		if let Some(address) = address {
			let account = AccountIdLookup::lookup(address.clone())?;
			debug!(
				"indirect call VerifyIdentity, who:{:?}, identity: {:?}, validation_data: {:?}",
				account_id_to_string(&account),
				identity,
				validation_data
			);

			let enclave_account_id = executor.get_enclave_account()?;
			let trusted_call = TrustedCall::verify_identity_callback(
				enclave_account_id,
				account,
				identity,
				block,
				hash,
			);
			let signed_trusted_call =
				executor.sign_call_with_self(&trusted_call, &self.shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

			let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
			executor.submit_trusted_call(self.shard, encrypted_trusted_call);
		}
		Ok(())
	}
}