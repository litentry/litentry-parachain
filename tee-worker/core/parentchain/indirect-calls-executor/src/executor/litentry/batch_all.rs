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

use crate::{error::Error, executor::Executor, IndirectCallsExecutor};
use codec::Decode;
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
		pallet_utility::UTILCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata, Error as MetadataError,
	},
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{BatchAllFn, CreateIdentityFn, H256};

use litentry_primitives::ParentchainBlockNumber;

use super::create_identity::CreateIdentity;

pub(crate) struct BatchAll {
	pub(crate) block_number: ParentchainBlockNumber,
}

impl BatchAll {
	fn execute_internal<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
	>(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		extrinsic: ParentchainUncheckedExtrinsic<
			<Self as Executor<
				ShieldingKeyRepository,
				StfEnclaveSigner,
				TopPoolAuthor,
				NodeMetadataProvider,
			>>::Call,
		>,
	) -> Result<(), Error>
	where
		ShieldingKeyRepository: AccessKey,
		<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
			+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
		StfEnclaveSigner: StfEnclaveSigning,
		TopPoolAuthor: AuthorApi<H256, H256> + Send + Sync + 'static,
		NodeMetadataProvider: AccessNodeMetadata,
		NodeMetadataProvider::MetadataType:
			IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UTILCallIndexes,
	{
		let (_, calls) = extrinsic.function;

		for call in calls.iter() {
			let mut call = call.0.as_slice();

			if let Ok(call) = CreateIdentityFn::decode(&mut call) {
				let create_identity = CreateIdentity { block_number: self.block_number };
				if Executor::<
					ShieldingKeyRepository,
					StfEnclaveSigner,
					TopPoolAuthor,
					NodeMetadataProvider,
				>::is_target_call(
					&create_identity, &call, context.node_meta_data_provider.as_ref()
				) {
					let xt = ParentchainUncheckedExtrinsic {
						function: call,
						signature: extrinsic.signature.clone(),
					};
					create_identity.execute(context, xt)?;
				}
			}
		}
		Ok(())
	}
}

impl<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>
	Executor<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider> for BatchAll
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	StfEnclaveSigner: StfEnclaveSigning,
	TopPoolAuthor: AuthorApi<H256, H256> + Send + Sync + 'static,
	NodeMetadataProvider: AccessNodeMetadata,
	NodeMetadataProvider::MetadataType:
		IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UTILCallIndexes,
{
	type Call = BatchAllFn;

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(
		&self,
		metadata_type: &NodeMetadataProvider::MetadataType,
	) -> Result<[u8; 2], MetadataError> {
		metadata_type.batch_all_call_indexes()
	}

	fn execute(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<(), Error> {
		self.execute_internal(context, extrinsic)
			.map_err(|_| Error::BatchAllHandlingError)
	}
}
