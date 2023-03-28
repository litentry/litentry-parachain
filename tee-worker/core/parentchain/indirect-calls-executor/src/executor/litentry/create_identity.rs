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
	executor::Executor,
	hash_of, IndirectCallsExecutor,
};
use codec::{Decode, Encode};
use ita_sgx_runtime::{pallet_imt::MetadataOf, Runtime};
use ita_stf::{TrustedCall, TrustedOperation};
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

pub(crate) struct CreateIdentity {
	pub(crate) block_number: ParentchainBlockNumber,
}

impl CreateIdentity {
	fn execute_internal<R, S, T, N>(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: &ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
	) -> Result<()>
	where
		R: AccessKey,
		R::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
			+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
		S: StfEnclaveSigning,
		T: AuthorApi<H256, H256> + Send + Sync + 'static,
		N: AccessNodeMetadata,
		N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
	{
		let (_, (shard, account, encrypted_identity, encrypted_metadata)) = &extrinsic.function;
		let shielding_key = context.shielding_key_repo.retrieve_key()?;
		let identity: Identity =
			Identity::decode(&mut shielding_key.decrypt(encrypted_identity)?.as_slice())?;
		debug!(
			"execute indirect call: CreateIdentity, who: {:?}, identity: {:?}",
			account_id_to_string(account),
			identity
		);

		let metadata = match encrypted_metadata {
			None => None,
			Some(m) => {
				let decrypted_metadata = shielding_key.decrypt(m)?;
				Some(MetadataOf::<Runtime>::decode(&mut decrypted_metadata.as_slice())?)
			},
		};

		if extrinsic.signature.is_some() {
			let enclave_account_id = context.stf_enclave_signer.get_enclave_account()?;
			let trusted_call = TrustedCall::create_identity_runtime(
				enclave_account_id,
				account.clone(),
				identity,
				metadata,
				self.block_number,
				hash_of(extrinsic),
			);
			let signed_trusted_call =
				context.stf_enclave_signer.sign_call_with_self(&trusted_call, shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

			let encrypted_trusted_call = shielding_key.encrypt(&trusted_operation.encode())?;
			context.submit_trusted_call(*shard, encrypted_trusted_call);
		}
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for CreateIdentity
where
	R: AccessKey,
	R::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	S: StfEnclaveSigning,
	T: AuthorApi<H256, H256> + Send + Sync + 'static,
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
{
	type Call = CreateIdentityFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]> {
		metadata_type.create_identity_call_indexes().map_err(|e| e.into())
	}

	fn execute(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		// the account is user that requested to create the identity, not necessarily the extrinsic sender
		// there's case where a delegatee could send this extrinsic on behalf of the user
		let (_, (shard, account, _, _)) = &extrinsic.function;
		let e = Error::IMPHandlingError(IMPError::CreateIdentityFailed(ErrorDetail::ImportError));
		if self.execute_internal(context, &extrinsic).is_err() {
			// try to handle the error internally, if we get another error, log it and return the
			// original error
			if let Err(internal_e) = context.submit_trusted_call_from_error(
				*shard,
				Some(account.clone()),
				&e,
				hash_of(&extrinsic),
			) {
				warn!("fail to handle internal errors in create_identity: {:?}", internal_e);
			}
			return Err(e)
		}
		Ok(())
	}
}
