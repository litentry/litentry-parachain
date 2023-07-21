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
	error::{Error, Result},
	executor::{
		litentry::{
			link_identity::LinkIdentity, deactivate_identity::DeactivateIdentity, request_vc::RequestVC,
			set_user_shielding_key::SetUserShieldingKey,
		},
		Executor,
	},
	IndirectCallsExecutor,
};
use codec::{Decode, Input};
use core::ops::Deref;
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
use itp_types::{
	extrinsics::ParentchainUncheckedExtrinsicWithStatus, BatchAllFn, BatchCall, CallIndex,
	LinkIdentityParams, DeactivateIdentityParams, RequestVCParams, SetUserShieldingKeyParams,
	SupportedBatchCallMap, SupportedBatchCallParams, H256,
};
use sp_std::{vec, vec::Vec};

pub(crate) struct BatchAll {}

const V4: u8 = 4;

// TODO: maybe the logic in `decode_batch_call` and `execute_internal` can be further improved
impl BatchAll {
	fn execute_internal<R, S, T, N>(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<<Self as Executor<R, S, T, N>>::Call>,
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
		let (_, calls) = extrinsic.function;

		// we don't need to check is_target_call() again - this is guaranteed when filling Vec<BatchCall>
		for call in calls {
			match call.params {
				SupportedBatchCallParams::SetUserShieldingKey(p) => {
					let set_user_shielding_key = SetUserShieldingKey {};
					let c = (call.index, p);
					let xt = ParentchainUncheckedExtrinsic {
						function: c,
						signature: extrinsic.signature.clone(),
					};
					set_user_shielding_key.execute(context, xt)?;
				},
				SupportedBatchCallParams::LinkIdentity(p) => {
					let link_identity = LinkIdentity {};
					let c = (call.index, p);
					let xt = ParentchainUncheckedExtrinsic {
						function: c,
						signature: extrinsic.signature.clone(),
					};
					link_identity.execute(context, xt)?;
				},
				SupportedBatchCallParams::DeactivateIdentity(p) => {
					let deactivate_identity = DeactivateIdentity {};
					let c = (call.index, p);
					let xt = ParentchainUncheckedExtrinsic {
						function: c,
						signature: extrinsic.signature.clone(),
					};
					deactivate_identity.execute(context, xt)?;
				},
				SupportedBatchCallParams::ActivateIdentity(p) => {
					let activate_identity = ActivateIdentity {};
					let c = (call.index, p);
					let xt = ParentchainUncheckedExtrinsic {
						function: c,
						signature: extrinsic.signature.clone(),
					};
					activate_identity.execute(context, xt)?;
				},
				SupportedBatchCallParams::RequestVC(p) => {
					let request_vc = RequestVC;
					let c = (call.index, p);
					let xt = ParentchainUncheckedExtrinsic {
						function: c,
						signature: extrinsic.signature.clone(),
					};
					request_vc.execute(context, xt)?;
				},
			};
		}
		Ok(())
	}
}

impl<R, S, T, N> Executor<R, S, T, N> for BatchAll
where
	R: AccessKey,
	R::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	S: StfEnclaveSigning,
	T: AuthorApi<H256, H256> + Send + Sync + 'static,
	N: AccessNodeMetadata,
	N::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes + UtilityCallIndexes,
{
	type Call = BatchAllFn;

	// Override the default impl because we need customised decoding of batched calls
	fn decode(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		input: &mut &[u8],
	) -> Result<itp_types::extrinsics::ParentchainUncheckedExtrinsicWithStatus<Self::Call>> {
		// We have two vector_prefixes:
		// - when encoding `OpaqueExtrinsicWithStatus`
		// - when encoding the innen `UncheckedExtrinsicV4`
		let _: Vec<()> = Decode::decode(input)?;
		let _: Vec<()> = Decode::decode(input)?;

		// mostly copied from
		// https://github.com/scs/substrate-api-client/blob/6516cd654435a68c883d56fcde09410e65f29a74/primitives/src/extrinsics.rs#L106-L125
		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != V4 {
			return Err(codec::Error::from("Invalid transaction version").into())
		}

		let supported_batch_call_map = context
			.supported_batch_call_map
			.read()
			.map_err(|_| Error::BatchAllHandlingError)?;
		let xt = ParentchainUncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(input)?) } else { None },
			function: decode_batch_call(input, supported_batch_call_map.deref())?,
		};

		let status: bool = Decode::decode(input)?;
		Ok(ParentchainUncheckedExtrinsicWithStatus { xt, status })
	}

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(&self, metadata_type: &N::MetadataType) -> Result<[u8; 2]> {
		metadata_type.batch_all_call_indexes().map_err(|e| e.into())
	}

	fn execute(
		&self,
		context: &IndirectCallsExecutor<R, S, T, N>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<()> {
		self.execute_internal(context, extrinsic)
			.map_err(|_| Error::BatchAllHandlingError)
	}
}

pub(crate) fn decode_batch_call(
	input: &mut &[u8],
	supported_batch_call_map: &SupportedBatchCallMap,
) -> Result<BatchAllFn> {
	let call_index: CallIndex = Decode::decode(input)?;
	let call_count: Vec<()> = Decode::decode(input)?;
	let mut calls: Vec<BatchCall> = vec![];

	for _i in 0..call_count.len() {
		let index: CallIndex = Decode::decode(input)?;
		let p = supported_batch_call_map.get(&index).ok_or(Error::BatchAllHandlingError)?;

		let params = match p {
			SupportedBatchCallParams::SetUserShieldingKey(..) => {
				let decoded_params = SetUserShieldingKeyParams::decode(input)?;
				SupportedBatchCallParams::SetUserShieldingKey(decoded_params)
			},
			SupportedBatchCallParams::LinkIdentity(..) => {
				let decoded_params = LinkIdentityParams::decode(input)?;
				SupportedBatchCallParams::LinkIdentity(decoded_params)
			},
			SupportedBatchCallParams::DeactivateIdentity(..) => {
				let decoded_params = DeactivateIdentityParams::decode(input)?;
				SupportedBatchCallParams::DeactivateIdentity(decoded_params)
			},
			SupportedBatchCallParams::RequestVC(..) => {
				let decoded_params = RequestVCParams::decode(input)?;
				SupportedBatchCallParams::RequestVC(decoded_params)
			},
		};
		calls.push(BatchCall { index, params });
	}
	Ok((call_index, calls))
}
