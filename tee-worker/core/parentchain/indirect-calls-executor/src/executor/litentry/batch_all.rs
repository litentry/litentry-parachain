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

// use super::{create_identity::CreateIdentity, set_user_shielding_key::SetUserShieldingKey};
use crate::{
	error::{Error, Result},
	executor::{
		litentry::{
			create_identity::CreateIdentity, remove_identity::RemoveIdentity,
			set_user_shielding_key::SetUserShieldingKey, verify_identity::VerifyIdentity,
		},
		Executor,
	},
	IndirectCallsExecutor,
};
use codec::{Decode, Input};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes,
		pallet_teerex::TeerexCallIndexes,
		pallet_utility::UTILCallIndexes,
		pallet_vcmp::VCMPCallIndexes,
		provider::{AccessNodeMetadata, Error as metadataProviderError},
	},
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	extrinsics::ParentchainUncheckedExtrinsicWithStatus, BatchAllFn, CallIndex,
	CreateIdentityParameters, RemoveIdentityParameters, SetUserShieldingKeyParameters,
	SupportedCall, VerifyIdentityParameters, H256,
};
use litentry_primitives::ParentchainBlockNumber;
use sp_std::{vec, vec::Vec};

pub(crate) struct BatchAll {
	pub(crate) block_number: ParentchainBlockNumber,
}

const V4: u8 = 4;

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
	) -> Result<()>
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
		log::warn!("enter batch call, len:{}", calls.len());

		for call in calls.iter() {
			match call {
				itp_types::SupportedCall::SetUserShieldingKey(call_index, Some(params)) => {
					let set_user_shielding_key = SetUserShieldingKey {};
					let call = (*call_index, params.clone());
					if Executor::<
						ShieldingKeyRepository,
						StfEnclaveSigner,
						TopPoolAuthor,
						NodeMetadataProvider,
					>::is_target_call(
						&set_user_shielding_key, &call, context.node_meta_data_provider.as_ref()
					) {
						let xt = ParentchainUncheckedExtrinsic {
							function: call,
							signature: extrinsic.signature.clone(),
						};
						set_user_shielding_key.execute(context, xt)?;
					}
				},
				itp_types::SupportedCall::CreateIdentity(call_index, Some(params)) => {
					let create_identity = CreateIdentity { block_number: self.block_number };
					let call = (*call_index, params.clone());
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
				},
				itp_types::SupportedCall::VerifyIdentity(call_index, Some(params)) => {
					let verify_identity = VerifyIdentity { block_number: self.block_number };
					let call = (*call_index, params.clone());
					if Executor::<
						ShieldingKeyRepository,
						StfEnclaveSigner,
						TopPoolAuthor,
						NodeMetadataProvider,
					>::is_target_call(
						&verify_identity, &call, context.node_meta_data_provider.as_ref()
					) {
						let xt = ParentchainUncheckedExtrinsic {
							function: call,
							signature: extrinsic.signature.clone(),
						};
						verify_identity.execute(context, xt)?;
					}
				},
				itp_types::SupportedCall::RemoveIdentity(call_index, Some(params)) => {
					let remove_identity = RemoveIdentity {};
					let call = (*call_index, params.clone());
					if Executor::<
						ShieldingKeyRepository,
						StfEnclaveSigner,
						TopPoolAuthor,
						NodeMetadataProvider,
					>::is_target_call(
						&remove_identity, &call, context.node_meta_data_provider.as_ref()
					) {
						let xt = ParentchainUncheckedExtrinsic {
							function: call,
							signature: extrinsic.signature.clone(),
						};
						remove_identity.execute(context, xt)?;
					}
				},
				_ => {
					log::warn!("Unimplemented. Call: {:?}", call);
				},
			};
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

	fn decode(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		input: &mut &[u8],
	) -> Result<itp_types::extrinsics::ParentchainUncheckedExtrinsicWithStatus<Self::Call>> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		// OpaqueExtrinsicWithStatus & OpaqueExtrinsic together have two leyers encode with vec.
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != V4 {
			return Err(codec::Error::from("Invalid transaction version").into())
		}

		let actual_xt = ParentchainUncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(input)?) } else { None },
			function: decode_batch_call(input, context.node_meta_data_provider.as_ref())?,
		};

		let status: bool = Decode::decode(input)?;
		Ok(ParentchainUncheckedExtrinsicWithStatus { xt: actual_xt, status })
	}

	fn call_index(&self, call: &Self::Call) -> [u8; 2] {
		call.0
	}

	fn call_index_from_metadata(
		&self,
		metadata_type: &NodeMetadataProvider::MetadataType,
	) -> Result<[u8; 2]> {
		metadata_type.batch_all_call_indexes().map_err(|e| e.into())
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
	) -> Result<()> {
		self.execute_internal(context, extrinsic)
			.map_err(|_| Error::BatchAllHandlingError)
	}
}

pub(crate) fn decode_batch_call<NodeMetadataProvider>(
	input: &mut &[u8],
	metadata_repo: &NodeMetadataProvider,
) -> Result<BatchAllFn>
where
	NodeMetadataProvider: AccessNodeMetadata,
	NodeMetadataProvider::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + UTILCallIndexes,
{
	let call_index: [u8; 2] = Decode::decode(input)?;
	let vector: Vec<()> = Decode::decode(input)?;
	let mut supported_call: Vec<SupportedCall> = vec![];

	supported_call.push(metadata_repo.get_from_metadata(
		|m| -> core::result::Result<SupportedCall, metadataProviderError> {
			Ok(SupportedCall::SetUserShieldingKey(
				IMPCallIndexes::set_user_shielding_key_call_indexes(m)?,
				None,
			))
		},
	)??);

	supported_call.push(metadata_repo.get_from_metadata(
		|m| -> core::result::Result<SupportedCall, metadataProviderError> {
			Ok(SupportedCall::CreateIdentity(
				IMPCallIndexes::create_identity_call_indexes(m)?,
				None,
			))
		},
	)??);

	supported_call.push(metadata_repo.get_from_metadata(
		|m| -> core::result::Result<SupportedCall, metadataProviderError> {
			Ok(SupportedCall::RemoveIdentity(
				IMPCallIndexes::remove_identity_call_indexes(m)?,
				None,
			))
		},
	)??);

	supported_call.push(metadata_repo.get_from_metadata(
		|m| -> core::result::Result<SupportedCall, metadataProviderError> {
			Ok(SupportedCall::VerifyIdentity(
				IMPCallIndexes::verify_identity_call_indexes(m)?,
				None,
			))
		},
	)??);

	let mut actual_calls: Vec<SupportedCall> = vec![];
	for _i in 0..vector.len() {
		let call_index: CallIndex = Decode::decode(input)?;
		supported_call.iter_mut().for_each(|call| match call {
			SupportedCall::SetUserShieldingKey(expected_call_idnex, _params) =>
				if expected_call_idnex == &call_index {
					let actual_params = SetUserShieldingKeyParameters::decode(input).unwrap();
					actual_calls
						.push(SupportedCall::SetUserShieldingKey(call_index, Some(actual_params)));
				},
			SupportedCall::CreateIdentity(expected_call_idnex, _params) => {
				if expected_call_idnex == &call_index {
					let actual_params = CreateIdentityParameters::decode(input).unwrap();
					actual_calls
						.push(SupportedCall::CreateIdentity(call_index, Some(actual_params)));
				}
			},
			SupportedCall::RemoveIdentity(expected_call_idnex, _params) => {
				if expected_call_idnex == &call_index {
					let actual_params = RemoveIdentityParameters::decode(input).unwrap();
					actual_calls
						.push(SupportedCall::RemoveIdentity(call_index, Some(actual_params)));
				}
			},
			SupportedCall::VerifyIdentity(expected_call_idnex, _params) => {
				if expected_call_idnex == &call_index {
					let actual_params = VerifyIdentityParameters::decode(input).unwrap();
					actual_calls
						.push(SupportedCall::VerifyIdentity(call_index, Some(actual_params)));
				}
			},
		});
	}
	Ok((call_index, actual_calls))
}

#[cfg(test)]
mod tests {

	use super::{decode_batch_call, V4};
	use codec::{Decode, Encode, Input};
	use itp_node_api::{
		api_client::{ParentchainApi, ParentchainUncheckedExtrinsic, WsRpcClient},
		metadata::{provider::NodeMetadataRepository, NodeMetadata},
	};
	use itp_types::{extrinsics::fill_opaque_extrinsic_with_status, SupportedCall};
	use parachain_core_primitives::UncheckedExtrinsic;
	use sp_runtime::OpaqueExtrinsic;
	use substrate_api_client::Metadata;

	#[test]
	fn decoed_batch_call_extinsic() {
		let _ = env_logger::builder().is_test(true).try_init();

		let batch_all_extrinsic = "d10f8400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01e62a3e9892acaf937f429cdfb1335a0eaf64f1831786a75829fc614ab9d1b0753755c791831a7252e9948a92208e82e00c8c0a74b282dfc71047c8573b68968028030400090208360317c6e5c576916d2757f79af6c211367a46bf3b0de6d0e91ff8c1a288ac067be9d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d010620bb433f3cf2f6485b21e99cec3e403d80d0be66393f1f9a9a96f2f32106a8b9e35865a1dd51a4d8f88cfba755c382f5bb349246b9d1f5a648f261126103fd14c7f045900166556d06427f9e283baad84efbe67f0894c48adc7bab5227eb474c9d8b2849bf676a76671ff4f5c88df119a2a9b825fcc2d973d66507df528377d449cae1af43e51a70cc8481ea00a896029595a6df97f18db0dfde00907509775ad7707e00ea12b548e1bf4a945ca2d1f21446497b92fae008a3f3816001177c4c319ee7dc6e6f70220966d3ccf24f37123645939c9498366c6f0238c12ad67383594d9ac5bfb1ae9a26ef1c0910f9ecf15956f7ba5b98afdd32de3de132b009eaf387f5ca7195ba9ac1c3f1e3bd9f5d83a1d0c605cf9fe584557107c4c87fc5ba28dab766166c64f97d8c498d3989bdc322c3043b30e4b966f0109cb7720f372d4b5bc55c42fef48e04d472c1dd3b85fcd4ed0e996dc0d4372d26e185f8baafa71f4acd2400ed327c7470f81803e84c242429595dfa40254c3f4a3792d332d8a100360317c6e5c576916d2757f79af6c211367a46bf3b0de6d0e91ff8c1a288ac067be9d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0106302e9ce143c9c545f7cabeec6bbd051d0e3fe12f14c3fc728378ea791d30c37fc7aed91b8769828ad6dd9152e5fbcb7898e7ed720817df79956c4e061e412eb0d14e42231e3c6f2480078ad340cece1fb7cd09c3f4dd6cca405e252bf2235989c349ab55bf7e5b885e9fd5bc2c0911fc959b42a615bee7023c68515d30500bb452e1abad8d26038a04d7091b72b05a267ccd5830b1f9e6dce556d674b80b995f5d19b08cedcef1a6439c7a282827d758f34cf270e8c446dd7b536d7550bbe8a0443f9e57bc3fdffba00ce2645f10bf85964b5f9938fb4e23db1e86a291f88a35bf5c3a7158e93ede8f4acaae4125d960868e89c74968e9c67d5f397df8f98a86e1a68c26ae51b61a8ff5e5cffdec55a2f60ac96473ed4e231a585aec389d25b2a12288456b6c0d0ad6f44ffe94fe438bc1bf1f40bb5b6a35cf176bdf64cab2d0e700d3c2810b2c1ca06711b5fb81ebafa4992bb11dd92ac8fbe43c58c0904560c4fa4d34aca5afaac4f7156cfad43df0a9177e221dd5ee35c1f3fe73df11e9b600";
		let input = hex::decode(batch_all_extrinsic).unwrap();
		let mut input = input.as_slice();

		let op_xt = OpaqueExtrinsic::from_bytes(input).unwrap();
		let op_xt = fill_opaque_extrinsic_with_status(op_xt, true).unwrap();
		let op_xt_encode = op_xt.encode();
		let mut input = op_xt_encode.as_slice();

		let api = ParentchainApi::new(WsRpcClient::new("ws://host.docker.internal:9944")).unwrap();
		let metadata = api.get_metadata().unwrap();
		let metadata = Metadata::try_from(metadata).unwrap();
		let node_metadata = NodeMetadata::new(metadata, 0, 0);
		let metadata_repo = NodeMetadataRepository::new(node_metadata);

		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(&mut input).unwrap();
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(&mut input).unwrap();

		let version = input.read_byte().unwrap();

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != V4 {
			assert!(false, "Invalid transaction version");
		}

		let actual_xt = ParentchainUncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(&mut input).unwrap()) } else { None },
			function: decode_batch_call(&mut input, &metadata_repo).unwrap(),
		};

		for call in actual_xt.function.1 {
			match call {
				SupportedCall::SetUserShieldingKey(_i, Some((shard, encrypted_key))) => {
					assert_eq!(
						"e87a605c77f07948f72885e65ca6160a56cae62d7cd06948c0c649710c9c7062",
						hex::encode(shard)
					);
					assert_eq!(
					"279be123803321f5141db80c551af8cf7ed980d3126f5fca058a3be44d474dec9f733100c47bba5cca00fde0697c83a93f0e4df3e0c5a2f2c2105869bd53cecdb075f5f8abbbd17a53c4355a0a160e14a4f2bd327ca44546807a6f44a6ccb4e6822f2f4c01d73cf8f714333ee179d61c67f087d04af5165fd09496d1c687b866e3237df468717f730ffb72125b1b6aad04ddc31e879fbfeb4a48625f75813a45abb33d69dd44603cf7698112dde1aeac245edd5f7cabf2a37a7f9ebf5eb890230dcb8181a9a88efaeb48debcbdb5b3d7c89ca645f980c3da74779894089a1505a6f98f7cd0dcad8769dd1b4a3480f745dd453b94e7e3bcf6fc77fe5566feb6e08a294c644baf999ce4fce838fa0e5d07ad0e50bbf168464cb3306a95e1ff079a05cdbf9e5434a5e48a5b079af9554fe7664fbdc3292e6461c12c29939f5177534ce16b0be474d965216c22051488c723f5d3d6c4b5e1dc67f8f19961a71ab4f1fd16a0be94ffde5adf082d5ae5e3a376f798fd8e19b6c95c2e377658ec36412f",
					hex::encode(encrypted_key)
				);
				},
				SupportedCall::CreateIdentity(
					_i,
					Some((shard, account, encrypted_identity, _)),
				) => {
					assert_eq!(
						"17c6e5c576916d2757f79af6c211367a46bf3b0de6d0e91ff8c1a288ac067be9",
						hex::encode(shard)
					);
					assert_eq!(
						"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
						hex::encode(account)
					); // Alice
				},
				SupportedCall::RemoveIdentity(_, _) => {
					assert!(false, "failed");
				},
				SupportedCall::VerifyIdentity(_, _) => {
					assert!(false, "failed");
				},
				_ => {
					assert!(false, "failed");
				},
			}
		}
	}
}
