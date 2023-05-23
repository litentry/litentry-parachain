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
use crate::{error::Result, IndirectDispatch, IndirectExecutor, LitentryExecutor};
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
use litentry_primitives::{Identity, ParentchainBlockNumber};
use log::*;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct CreateIdentityArgs {
	shard: ShardIdentifier,
	account: AccountId,
	encrypted_identity: Vec<u8>,
	encrypted_metadata: Option<Vec<u8>>,
}

impl<Executor: IndirectExecutor+LitentryExecutor> IndirectDispatch<Executor> for CreateIdentityArgs {
	fn dispatch(&self, executor: &Executor) -> Result<()> {
		info!(
			"Found CreateIdentity extrinsic in block: Shard: {}\nAccount {:?}",
			bs58::encode(self.shard.encode()).into_string(),
			self.account
		);
		let shielding_key = executor.retrieve_key();
		// NOTE: Indirect Executor trait provides methods for encrypting and decrypting (part of refactor)
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
				let decrypted_metadata = executor.decrypt(&m)?;
				Some(MetadataOf::<Runtime>::decode(&mut decrypted_metadata.as_slice())?)
			},
		};
		// TODO: Till I find a way to pass information related to extrinsic, this will be commented
		// if extrinsic.signature.is_some() {
		let enclave_account_id = executor.get_enclave_account()?;
		// TODO: Need to get information about the block here
		let trusted_call = TrustedCall::create_identity(
			enclave_account_id,
			self.account.clone(),
			identity,
			metadata,
			0,
			H256::zero(),
		);
		let signed_trusted_call =
			executor.sign_call_with_self(&trusted_call, &self.shard)?;
		let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);
		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(self.shard, encrypted_trusted_call);
		// }
		Ok(())
	}
}