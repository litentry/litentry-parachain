/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/
//! Execute indirect calls, i.e. extrinsics extracted from parentchain blocks.
//!
//! The core struct of this crate is the [IndirectCallsExecutor] executor. It scans parentchain
//! blocks for relevant extrinsics, derives an indirect call for those and dispatches the
//! indirect call.

#![feature(trait_alias)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use thiserror_sgx as thiserror;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::sync::SgxRwLock as RwLock;

#[cfg(all(feature = "std", not(feature = "sgx")))]
use std::sync::RwLock;

pub mod error;
pub mod executor;

use crate::{
	error::{Error, Result},
	executor::{
		call_worker::CallWorker,
		litentry::{
			batch_all::BatchAll,
			create_identity::CreateIdentity,
			remove_identity::RemoveIdentity,
			request_vc::RequestVC,
			scheduled_enclave::{RemoveScheduledEnclave, UpdateScheduledEnclave},
			set_user_shielding_key::SetUserShieldingKey,
			verify_identity::VerifyIdentity,
		},
		shield_funds::ShieldFunds,
		DecorateExecutor,
	},
};
use binary_merkle_tree::merkle_root;
use codec::Encode;
use ita_stf::{TrustedCall, TrustedOperation};
use itp_node_api::metadata::{
	error::Result as MetadataResult, pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
	pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata,
	runtime_call::RuntimeCall,
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	AccountId, Assertion, CallIndex, OpaqueCall, ShardIdentifier, SupportedBatchCallMap,
	SupportedBatchCallParams, H256,
};
use litentry_primitives::ParentchainBlockNumber;
use log::*;
use sp_core::blake2_256;
use sp_runtime::traits::{Block as ParentchainBlockTrait, Header, Keccak256};
use std::{sync::Arc, vec, vec::Vec};

#[derive(Clone)]
pub enum ExecutionStatus<R> {
	Success(R),
	NextExecutor,
	Skip,
}

/// Trait to execute the indirect calls found in the extrinsics of a block.
pub trait ExecuteIndirectCalls {
	/// Scans blocks for extrinsics that ask the enclave to execute some actions.
	/// Executes indirect invocation calls, including shielding and unshielding calls.
	/// Returns a vector of OpaqueCalls that should be sent to parentchain.
	fn execute_indirect_calls_in_extrinsics<ParentchainBlock>(
		&self,
		block: &ParentchainBlock,
	) -> Result<Vec<OpaqueCall>>
	where
		ParentchainBlock: ParentchainBlockTrait<Hash = H256>;
}

pub struct IndirectCallsExecutor<R, S, T, N> {
	pub(crate) shielding_key_repo: Arc<R>,
	pub(crate) stf_enclave_signer: Arc<S>,
	pub(crate) top_pool_author: Arc<T>,
	pub(crate) node_metadata_provider: Arc<N>,
	pub(crate) supported_batch_call_map: Arc<RwLock<SupportedBatchCallMap>>,
}

impl<R, S, T, N> IndirectCallsExecutor<R, S, T, N>
where
	R: AccessKey,
	R::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	S: StfEnclaveSigning,
	T: AuthorApi<H256, H256> + Send + Sync + 'static,
	N: AccessNodeMetadata,
	N::MetadataType: TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
{
	pub fn new(
		shielding_key_repo: Arc<R>,
		stf_enclave_signer: Arc<S>,
		top_pool_author: Arc<T>,
		node_metadata_provider: Arc<N>,
	) -> Self {
		IndirectCallsExecutor {
			shielding_key_repo,
			stf_enclave_signer,
			top_pool_author,
			node_metadata_provider,
			supported_batch_call_map: Arc::new(RwLock::default()),
		}
	}

	// will be called after the node metadata for the enclave is set
	// see ffi::set_node_metadata
	// TODO: is there a better way to update it?
	pub fn update_supported_batch_call_map(&self) -> Result<()> {
		let mut m = self
			.supported_batch_call_map
			.write()
			.map_err(|_| Error::Other("Lock poisoning".into()))?;
		m.clear();

		// the intialised SupportedBatchCallParams are only placeholders
		#[allow(clippy::type_complexity)]
		let supported_call_indexes_fn: Vec<(
			fn(&N::MetadataType) -> MetadataResult<CallIndex>,
			SupportedBatchCallParams,
		)> = vec![
			(
				IMPCallIndexes::set_user_shielding_key_call_indexes,
				SupportedBatchCallParams::SetUserShieldingKey(Default::default()),
			),
			(
				IMPCallIndexes::create_identity_call_indexes,
				SupportedBatchCallParams::CreateIdentity((
					Default::default(),
					AccountId::new([0u8; 32]),
					Default::default(),
					None,
				)),
			),
			(
				IMPCallIndexes::remove_identity_call_indexes,
				SupportedBatchCallParams::RemoveIdentity(Default::default()),
			),
			(
				IMPCallIndexes::verify_identity_call_indexes,
				SupportedBatchCallParams::VerifyIdentity(Default::default()),
			),
			(
				VCMPCallIndexes::request_vc_call_indexes,
				SupportedBatchCallParams::RequestVC((Default::default(), Assertion::A1)),
			),
		];

		for f in supported_call_indexes_fn {
			let call = self.node_metadata_provider.get_from_metadata(|m| f.0(m));
			// ignore the errors
			if let Ok(Ok(c)) = call {
				m.insert(c, f.1);
			}
		}
		debug!("Supported batched call map: {:?}", *m);
		Ok(())
	}

	pub(crate) fn submit_trusted_call(
		&self,
		shard: ShardIdentifier,
		encrypted_trusted_call: Vec<u8>,
	) {
		if let Err(e) = futures::executor::block_on(
			self.top_pool_author.submit_top(encrypted_trusted_call, shard),
		) {
			error!("Error adding indirect trusted call to TOP pool: {:?}", e);
		}
	}

	pub(crate) fn submit_trusted_call_from_error(
		&self,
		shard: ShardIdentifier,
		account: Option<AccountId>,
		err: &Error,
		hash: H256,
	) -> Result<()> {
		let enclave_account = self.stf_enclave_signer.get_enclave_account()?;
		let shielding_key = self.shielding_key_repo.retrieve_key()?;
		let trusted_call = match err {
			error::Error::IMPHandlingError(e) =>
				TrustedCall::handle_imp_error(enclave_account, account, e.clone(), hash),
			error::Error::VCMPHandlingError(e) =>
				TrustedCall::handle_vcmp_error(enclave_account, account, e.clone(), hash),
			_ => return Err(Error::Other(("unsupported error").into())),
		};
		let signed_trusted_call =
			self.stf_enclave_signer.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = shielding_key.encrypt(&trusted_operation.encode())?;
		self.submit_trusted_call(shard, encrypted_trusted_call);
		Ok(())
	}

	/// Creates a processed_parentchain_block extrinsic for a given parentchain block hash and the merkle executed extrinsics.
	///
	/// Calculates the merkle root of the extrinsics. In case no extrinsics are supplied, the root will be a hash filled with zeros.
	fn create_processed_parentchain_block_call<ParentchainBlock>(
		&self,
		block_hash: H256,
		extrinsics: Vec<H256>,
		block_number: <<ParentchainBlock as ParentchainBlockTrait>::Header as Header>::Number,
	) -> Result<OpaqueCall>
	where
		ParentchainBlock: ParentchainBlockTrait<Hash = H256>,
	{
		let call = self.node_metadata_provider.get_from_metadata(|meta_data| {
			meta_data.confirm_processed_parentchain_block_call_indexes()
		})??;

		let root: H256 = merkle_root::<Keccak256, _>(extrinsics);
		Ok(OpaqueCall::from_tuple(&(call, block_hash, block_number, root)))
	}
}

impl<R, S, T, N> ExecuteIndirectCalls for IndirectCallsExecutor<R, S, T, N>
where
	R: AccessKey,
	R::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	S: StfEnclaveSigning,
	T: AuthorApi<H256, H256> + Send + Sync + 'static,
	N: AccessNodeMetadata,
	N::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + UtilityCallIndexes + RuntimeCall,
{
	fn execute_indirect_calls_in_extrinsics<ParentchainBlock>(
		&self,
		block: &ParentchainBlock,
	) -> Result<Vec<OpaqueCall>>
	where
		ParentchainBlock: ParentchainBlockTrait<Hash = H256>,
	{
		let block_number = *block.header().number();
		let block_hash = block.hash();
		let mut calls = Vec::<OpaqueCall>::new();

		let parentchain_block_number: ParentchainBlockNumber =
			block_number.try_into().map_err(|_| Error::ConvertParentchainBlockNumber)?;

		debug!("Scanning block {:?} for relevant xt", block_number);
		let mut executed_calls = Vec::<H256>::new();

		// TODO: this logic might have better alternatives, see https://github.com/integritee-network/worker/issues/1156
		for xt_opaque in block.extrinsics().iter() {
			let encoded_xt_opaque = xt_opaque.encode();

			// Found ShieldFunds extrinsic in block.
			let shield_funds = ShieldFunds {};
			// Found CallWorker extrinsic in block.
			// No else-if here! Because the same opaque extrinsic can contain multiple Fns at once (this lead to intermittent M6 failures)
			let call_worker = CallWorker {};

			// litentry
			// Found SetUserShieldingKey extrinsic
			let set_user_shielding_key = SetUserShieldingKey {};
			// Found CreateIdentityFn extrinsic
			let create_identity = CreateIdentity { block_number: parentchain_block_number };
			// Found RemoveIdentityFn extrinsic
			let remove_identity = RemoveIdentity {};
			// Found VerifyIdentity extrinsic
			let verify_identity = VerifyIdentity { block_number: parentchain_block_number };
			// Found RequestVC extrinsic
			let request_vc = RequestVC { block_number: parentchain_block_number };
			// Found BatchAll extrinsic
			let batch_all = BatchAll { block_number: parentchain_block_number };
			// Found UpdateScheduledEnclave extrinisc
			let update_scheduled_enclave = UpdateScheduledEnclave {};
			// Found RemoveScheduledEnclave extrinisc
			let remove_scheduled_enclave = RemoveScheduledEnclave {};

			let executors: Vec<&dyn DecorateExecutor<R, S, T, N>> = vec![
				&shield_funds,
				&call_worker,
				&batch_all,
				&set_user_shielding_key,
				&create_identity,
				&remove_identity,
				&verify_identity,
				&request_vc,
				&update_scheduled_enclave,
				&remove_scheduled_enclave,
			];
			for executor in executors {
				match executor.decode_and_execute(self, &mut encoded_xt_opaque.as_slice()) {
					Ok(ExecutionStatus::Success(hash)) => {
						executed_calls.push(hash);
						break
					},
					Ok(ExecutionStatus::NextExecutor) => continue,
					Ok(ExecutionStatus::Skip) => break,
					Err(e) => {
						// the error should already be handled (reported) in the executor
						// we only log the error (again)
						// note it can be an error during error-handling (e.g. fail to create the trusted call)
						log::warn!("fail to execute indirect_call due to {:?} ", e);
						// We should keep the same error handling as the original function `handle_shield_funds_xt`.
						// `create_processed_parentchain_block_call` needs to be called in any case.
						break
					},
				}
			}
		}

		// Include a processed parentchain block confirmation for each block.
		let confirm_processed_parentchain_block_call = self
			.create_processed_parentchain_block_call::<ParentchainBlock>(
				block_hash,
				executed_calls,
				block_number,
			)?;
		calls.push(confirm_processed_parentchain_block_call);
		Ok(calls)
	}
}

pub(crate) fn hash_of<T: Encode>(xt: &T) -> H256 {
	blake2_256(&xt.encode()).into()
}

#[cfg(test)]
mod test {
	use super::*;
	use codec::{Decode, Encode};
	use ita_stf::TrustedOperation;
	use itc_parentchain_test::parentchain_block_builder::ParentchainBlockBuilder;
	use itp_node_api::{
		api_client::{
			ParentchainExtrinsicParams, ParentchainExtrinsicParamsBuilder,
			ParentchainUncheckedExtrinsic,
		},
		metadata::{metadata_mocks::NodeMetadataMock, provider::NodeMetadataRepository},
	};
	use itp_sgx_crypto::mocks::KeyRepositoryMock;
	use itp_stf_executor::mocks::StfEnclaveSignerMock;
	use itp_stf_primitives::types::AccountId;
	use itp_test::mock::shielding_crypto_mock::ShieldingCryptoMock;
	use itp_top_pool_author::mocks::AuthorApiMock;
	use itp_types::{
		extrinsics::fill_opaque_extrinsic_with_status, Block, CallWorkerFn, Request,
		ShardIdentifier, ShieldFundsFn,
	};
	use sp_core::{ed25519, Pair};
	use sp_runtime::{MultiSignature, OpaqueExtrinsic};
	use std::assert_matches::assert_matches;
	use substrate_api_client::{ExtrinsicParams, GenericAddress};

	type TestShieldingKeyRepo = KeyRepositoryMock<ShieldingCryptoMock>;
	type TestStfEnclaveSigner = StfEnclaveSignerMock;
	type TestTopPoolAuthor = AuthorApiMock<H256, H256>;
	type TestNodeMetadataRepository = NodeMetadataRepository<NodeMetadataMock>;
	type TestIndirectCallExecutor = IndirectCallsExecutor<
		TestShieldingKeyRepo,
		TestStfEnclaveSigner,
		TestTopPoolAuthor,
		TestNodeMetadataRepository,
	>;

	type Seed = [u8; 32];
	const TEST_SEED: Seed = *b"12345678901234567890123456789012";

	#[test]
	fn indirect_call_can_be_added_to_pool_successfully() {
		let _ = env_logger::builder().is_test(true).try_init();

		let (indirect_calls_executor, top_pool_author, _) =
			test_fixtures([0u8; 32], NodeMetadataMock::new());

		let opaque_extrinsic =
			OpaqueExtrinsic::from_bytes(call_worker_unchecked_extrinsic().encode().as_slice())
				.unwrap();

		let parentchain_block = ParentchainBlockBuilder::default()
			.with_extrinsics(vec![
				fill_opaque_extrinsic_with_status(opaque_extrinsic, true).unwrap()
			])
			.build();

		indirect_calls_executor
			.execute_indirect_calls_in_extrinsics(&parentchain_block)
			.unwrap();

		assert_eq!(1, top_pool_author.pending_tops(shard_id()).unwrap().len());
	}

	#[test]
	fn failed_indirect_call_is_skipped() {
		let _ = env_logger::builder().is_test(true).try_init();

		let (indirect_calls_executor, top_pool_author, _) =
			test_fixtures([0u8; 32], NodeMetadataMock::new());

		let opaque_extrinsic =
			OpaqueExtrinsic::from_bytes(call_worker_unchecked_extrinsic().encode().as_slice())
				.unwrap();

		let parentchain_block = ParentchainBlockBuilder::default()
			.with_extrinsics(vec![
				fill_opaque_extrinsic_with_status(opaque_extrinsic, false).unwrap()
			])
			.build();

		indirect_calls_executor
			.execute_indirect_calls_in_extrinsics(&parentchain_block)
			.unwrap();

		assert_eq!(0, top_pool_author.pending_tops(shard_id()).unwrap().len());
	}

	#[test]
	fn shielding_call_can_be_added_to_pool_successfully() {
		let _ = env_logger::builder().is_test(true).try_init();

		let mr_enclave = [33u8; 32];
		let (indirect_calls_executor, top_pool_author, shielding_key_repo) =
			test_fixtures(mr_enclave.clone(), NodeMetadataMock::new());
		let shielding_key = shielding_key_repo.retrieve_key().unwrap();

		let opaque_extrinsic = OpaqueExtrinsic::from_bytes(
			shield_funds_unchecked_extrinsic(&shielding_key).encode().as_slice(),
		)
		.unwrap();

		let parentchain_block = ParentchainBlockBuilder::default()
			.with_extrinsics(vec![
				fill_opaque_extrinsic_with_status(opaque_extrinsic, true).unwrap()
			])
			.build();

		indirect_calls_executor
			.execute_indirect_calls_in_extrinsics(&parentchain_block)
			.unwrap();

		assert_eq!(1, top_pool_author.pending_tops(shard_id()).unwrap().len());
		let submitted_extrinsic =
			top_pool_author.pending_tops(shard_id()).unwrap().first().cloned().unwrap();
		let decrypted_extrinsic = shielding_key.decrypt(&submitted_extrinsic).unwrap();
		let decoded_operation =
			TrustedOperation::decode(&mut decrypted_extrinsic.as_slice()).unwrap();
		assert_matches!(decoded_operation, TrustedOperation::indirect_call(_));
		let trusted_call_signed = decoded_operation.to_call().unwrap();
		assert!(trusted_call_signed.verify_signature(&mr_enclave, &shard_id()));
	}

	#[test]
	fn ensure_empty_extrinsic_vec_triggers_zero_filled_merkle_root() {
		// given
		let dummy_metadata = NodeMetadataMock::new();
		let (indirect_calls_executor, _, _) = test_fixtures([38u8; 32], dummy_metadata.clone());

		let block_hash = H256::from([1; 32]);
		let extrinsics = Vec::new();
		let confirm_processed_parentchain_block_indexes =
			dummy_metadata.confirm_processed_parentchain_block_call_indexes().unwrap();
		let expected_call =
			(confirm_processed_parentchain_block_indexes, block_hash, 1, H256::default()).encode();

		// when
		let call = indirect_calls_executor
			.create_processed_parentchain_block_call::<Block>(block_hash, extrinsics, 1)
			.unwrap();

		// then
		assert_eq!(call.0, expected_call);
	}

	#[test]
	fn ensure_non_empty_extrinsic_vec_triggers_non_zero_merkle_root() {
		// given
		let dummy_metadata = NodeMetadataMock::new();
		let (indirect_calls_executor, _, _) = test_fixtures([39u8; 32], dummy_metadata.clone());

		let block_hash = H256::from([1; 32]);
		let extrinsics = vec![H256::from([4; 32]), H256::from([9; 32])];
		let confirm_processed_parentchain_block_indexes =
			dummy_metadata.confirm_processed_parentchain_block_call_indexes().unwrap();

		let zero_root_call =
			(confirm_processed_parentchain_block_indexes, block_hash, 1, H256::default()).encode();

		// when
		let call = indirect_calls_executor
			.create_processed_parentchain_block_call::<Block>(block_hash, extrinsics, 1)
			.unwrap();

		// then
		assert_ne!(call.0, zero_root_call);
	}

	fn shield_funds_unchecked_extrinsic(
		shielding_key: &ShieldingCryptoMock,
	) -> ParentchainUncheckedExtrinsic<ShieldFundsFn> {
		let target_account = shielding_key.encrypt(&AccountId::new([2u8; 32]).encode()).unwrap();
		let dummy_metadata = NodeMetadataMock::new();

		let shield_funds_indexes = dummy_metadata.shield_funds_call_indexes().unwrap();
		ParentchainUncheckedExtrinsic::<ShieldFundsFn>::new_signed(
			(shield_funds_indexes, target_account, 1000u128, shard_id()),
			GenericAddress::Address32([1u8; 32]),
			MultiSignature::Ed25519(default_signature()),
			default_extrinsic_params().signed_extra(),
		)
	}

	fn call_worker_unchecked_extrinsic() -> ParentchainUncheckedExtrinsic<CallWorkerFn> {
		let request = Request { shard: shard_id(), cyphertext: vec![1u8, 2u8] };
		let dummy_metadata = NodeMetadataMock::new();
		let call_worker_indexes = dummy_metadata.call_worker_call_indexes().unwrap();

		ParentchainUncheckedExtrinsic::<CallWorkerFn>::new_signed(
			(call_worker_indexes, request),
			GenericAddress::Address32([1u8; 32]),
			MultiSignature::Ed25519(default_signature()),
			default_extrinsic_params().signed_extra(),
		)
	}

	fn default_signature() -> ed25519::Signature {
		signer().sign(&[0u8])
	}

	fn signer() -> ed25519::Pair {
		ed25519::Pair::from_seed(&TEST_SEED)
	}

	fn shard_id() -> ShardIdentifier {
		ShardIdentifier::default()
	}

	fn default_extrinsic_params() -> ParentchainExtrinsicParams {
		ParentchainExtrinsicParams::new(
			0,
			0,
			0,
			H256::default(),
			ParentchainExtrinsicParamsBuilder::default(),
		)
	}
	fn test_fixtures(
		mr_enclave: [u8; 32],
		metadata: NodeMetadataMock,
	) -> (TestIndirectCallExecutor, Arc<TestTopPoolAuthor>, Arc<TestShieldingKeyRepo>) {
		let shielding_key_repo = Arc::new(TestShieldingKeyRepo::default());
		let stf_enclave_signer = Arc::new(TestStfEnclaveSigner::new(mr_enclave));
		let top_pool_author = Arc::new(TestTopPoolAuthor::default());
		let node_metadata_repo = Arc::new(NodeMetadataRepository::new(metadata));

		let executor = IndirectCallsExecutor::new(
			shielding_key_repo.clone(),
			stf_enclave_signer,
			top_pool_author.clone(),
			node_metadata_repo,
		);

		(executor, top_pool_author, shielding_key_repo)
	}
}
