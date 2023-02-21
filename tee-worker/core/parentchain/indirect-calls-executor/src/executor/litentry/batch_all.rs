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
use crate::{error::Error, executor::Executor, IndirectCallsExecutor};
use codec::{Decode, Input};
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
		_context: &IndirectCallsExecutor<
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
		log::warn!("enter batch call, len:{}", calls.len());

		for mut call in calls.iter() {
			// let mut call = call();
			// match call {
			// itp_types::BatchSupportCall::SetUserShieldingKey(_) => todo!(),
			// 	itp_types::BatchSupportCall::CreateIdentity(_) => todo!(),
			// 	itp_types::BatchSupportCall::RemoveIdentity(_) => todo!(),
			// 	itp_types::BatchSupportCall::VerifyIdentity(_) => todo!(),
			// };
			// if let Ok(call) = SetUserShieldingKeyFn::decode(&mut call) {
			// 	let set_user_shielding_key = SetUserShieldingKey {};
			// 	if Executor::<
			// 		ShieldingKeyRepository,
			// 		StfEnclaveSigner,
			// 		TopPoolAuthor,
			// 		NodeMetadataProvider,
			// 	>::is_target_call(
			// 		&set_user_shielding_key, &call, context.node_meta_data_provider.as_ref()
			// 	) {
			// 		log::warn!("wowowo...yeah...");
			// 		let xt = ParentchainUncheckedExtrinsic {
			// 			function: call,
			// 			signature: extrinsic.signature.clone(),
			// 		};
			// 		set_user_shielding_key.execute(context, xt)?;
			// 	}
			// }

			// if let Ok(call) = CreateIdentityFn::decode(&mut call) {
			// 	let create_identity = CreateIdentity { block_number: self.block_number };
			// 	if Executor::<
			// 		ShieldingKeyRepository,
			// 		StfEnclaveSigner,
			// 		TopPoolAuthor,
			// 		NodeMetadataProvider,
			// 	>::is_target_call(
			// 		&create_identity, &call, context.node_meta_data_provider.as_ref()
			// 	) {
			// 		let xt = ParentchainUncheckedExtrinsic {
			// 			function: call,
			// 			signature: extrinsic.signature.clone(),
			// 		};
			// 		create_identity.execute(context, xt)?;
			// 	}
			// }
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
		_context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		input: &mut &[u8],
	) -> Result<
		itp_types::extrinsics::ParentchainUncheckedExtrinsicWithStatus<Self::Call>,
		codec::Error,
	> {
		// let decode_runtime_call = ||Vec<BatchAllFn>->{

		// };

		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != V4 {
			return Err("Invalid transaction version".into())
		}
		ParentchainUncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(input)?) } else { None },
			function: Decode::decode(input)?,
		};

		ParentchainUncheckedExtrinsicWithStatus::<Self::Call>::decode(input)
	}

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

pub(crate) fn decode_batch_call<NodeMetadataProvider>(
	input: &mut &[u8],
	metadata_repo: NodeMetadataProvider,
) -> BatchAllFn
where
	NodeMetadataProvider: AccessNodeMetadata,
	NodeMetadataProvider::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + UTILCallIndexes,
{
	// TODO handle unwrap/error return error
	let call_index: [u8; 2] = Decode::decode(input).unwrap();

	// TODO handle unwrap/error return error
	let vector: Vec<()> = Decode::decode(input).unwrap();

	let mut supported_call: Vec<SupportedCall> = vec![];

	// TODO handle unwrap/error return error
	supported_call.push(
		metadata_repo
			.get_from_metadata(|m| {
				SupportedCall::SetUserShieldingKey(
					IMPCallIndexes::set_user_shielding_key_call_indexes(m).unwrap(),
					None,
				)
			})
			.unwrap(),
	);
	supported_call.push(
		metadata_repo
			.get_from_metadata(|m| {
				SupportedCall::CreateIdentity(
					IMPCallIndexes::create_identity_call_indexes(m).unwrap(),
					None,
				)
			})
			.unwrap(),
	);

	// TODO handle unwrap/error return error
	for _i in 0..vector.len() {
		let call_index: CallIndex = Decode::decode(input).unwrap();
		supported_call.iter_mut().for_each(|call| match call {
			SupportedCall::SetUserShieldingKey(expected_call_idnex, params) =>
				if expected_call_idnex == &call_index {
					let actual_params = SetUserShieldingKeyParameters::decode(input).unwrap();
					*params = Some(actual_params);
				},
			SupportedCall::CreateIdentity(expected_call_idnex, params) =>
				if expected_call_idnex == &call_index {
					let actual_params = CreateIdentityParameters::decode(input).unwrap();
					*params = Some(actual_params);
				},
			SupportedCall::RemoveIdentity(expected_call_idnex, params) =>
				if expected_call_idnex == &call_index {
					let actual_params = RemoveIdentityParameters::decode(input).unwrap();
					*params = Some(actual_params);
				},
			SupportedCall::VerifyIdentity(expected_call_idnex, params) =>
				if expected_call_idnex == &call_index {
					let actual_params = VerifyIdentityParameters::decode(input).unwrap();
					*params = Some(actual_params);
				},
		});
	}
	(call_index, supported_call)
}

#[cfg(test)]
mod tests {

	use super::{decode_batch_call, V4};
	use codec::{Decode, Input};
	use itp_node_api::{
		api_client::{ParentchainApi, ParentchainUncheckedExtrinsic, WsRpcClient},
		metadata::{provider::NodeMetadataRepository, NodeMetadata},
	};
	use itp_types::SupportedCall;
	use substrate_api_client::Metadata;

	#[test]
	fn decoed_batch_call_extinsic() {
		let batch_all_extrinsic = "4d0f8400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01cc272312724f571f75be8af39569e6f68db3ca188fb2e5140f4e085fa078052be8d10c1ad83b7e6bb8ba042ea54b1d4ae593167465fefc097bacc79c30b7428d680b08000902083602e87a605c77f07948f72885e65ca6160a56cae62d7cd06948c0c649710c9c70620106279be123803321f5141db80c551af8cf7ed980d3126f5fca058a3be44d474dec9f733100c47bba5cca00fde0697c83a93f0e4df3e0c5a2f2c2105869bd53cecdb075f5f8abbbd17a53c4355a0a160e14a4f2bd327ca44546807a6f44a6ccb4e6822f2f4c01d73cf8f714333ee179d61c67f087d04af5165fd09496d1c687b866e3237df468717f730ffb72125b1b6aad04ddc31e879fbfeb4a48625f75813a45abb33d69dd44603cf7698112dde1aeac245edd5f7cabf2a37a7f9ebf5eb890230dcb8181a9a88efaeb48debcbdb5b3d7c89ca645f980c3da74779894089a1505a6f98f7cd0dcad8769dd1b4a3480f745dd453b94e7e3bcf6fc77fe5566feb6e08a294c644baf999ce4fce838fa0e5d07ad0e50bbf168464cb3306a95e1ff079a05cdbf9e5434a5e48a5b079af9554fe7664fbdc3292e6461c12c29939f5177534ce16b0be474d965216c22051488c723f5d3d6c4b5e1dc67f8f19961a71ab4f1fd16a0be94ffde5adf082d5ae5e3a376f798fd8e19b6c95c2e377658ec36412f3603e87a605c77f07948f72885e65ca6160a56cae62d7cd06948c0c649710c9c7062d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d01060e2b0deb7f4675ed085713d29088389f09891f57347ad9a017ec030478cfad82916e7ae1ba0d3e84c5e21b42fbac543d9d7c73094d00633f4d1cef7f59810bc85560cb9874d0a5b18fb9800836e0a4acd4213cfc89d53ce7ecc679d476d3ae9c5a329c3bf9409114f40c06ee674723841e4a61829f13c66ca4401f61b922b4d3163b29470d09686b16cc31e811a5df7af11328f04d6c18573d5e87df0b663595ddfbfd5cd01650ef260a91f0f0acb77c59bbccd0816f2c6b1605053c5019156f77cdbf4ad17afe998f22b1d2363d127d920caffc76e0f55d54036ad42b9f9851de6d7159af7af44c2e313459af0496a511ee29bea475a7818be5b87bb539088661ef3b4c092a996149bffaeeb8140f2287ce62f8f76d870f9298ce6fb6be071fc002b0fad53351ce2d7b4209833e2433d0961493bbe376d4eb0a00ba8e2cf30f0572a39c8d2a189574865a2e03cad6e2e41f5b9d22e3d1c1f9e24db7ca2e525626fab3d7b27f51e1dddac3432dccb9e9a1cfb2f1e7fffdbf85b2da8fc2b2ba7c00";
		let input = hex::decode(batch_all_extrinsic).unwrap();
		let mut input = input.as_slice();

		let api = ParentchainApi::new(WsRpcClient::new("ws://host.docker.internal:9944")).unwrap();
		let metadata = api.get_metadata().unwrap();
		let metadata = Metadata::try_from(metadata).unwrap();
		let node_metadata = NodeMetadata::new(metadata, 0, 0);
		let metadata_repo = NodeMetadataRepository::new(node_metadata);

		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(&mut input).unwrap();

		let version = input.read_byte().unwrap();

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != V4 {
			assert!(false, "Invalid transaction version");
		}

		let actual_xt = ParentchainUncheckedExtrinsic {
			signature: if is_signed { Some(Decode::decode(&mut input).unwrap()) } else { None },
			function: decode_batch_call(&mut input, metadata_repo),
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
						"e87a605c77f07948f72885e65ca6160a56cae62d7cd06948c0c649710c9c7062",
						hex::encode(shard)
					);
					assert_eq!(
						"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
						hex::encode(account)
					); // Alice
					assert_eq!(
					"0e2b0deb7f4675ed085713d29088389f09891f57347ad9a017ec030478cfad82916e7ae1ba0d3e84c5e21b42fbac543d9d7c73094d00633f4d1cef7f59810bc85560cb9874d0a5b18fb9800836e0a4acd4213cfc89d53ce7ecc679d476d3ae9c5a329c3bf9409114f40c06ee674723841e4a61829f13c66ca4401f61b922b4d3163b29470d09686b16cc31e811a5df7af11328f04d6c18573d5e87df0b663595ddfbfd5cd01650ef260a91f0f0acb77c59bbccd0816f2c6b1605053c5019156f77cdbf4ad17afe998f22b1d2363d127d920caffc76e0f55d54036ad42b9f9851de6d7159af7af44c2e313459af0496a511ee29bea475a7818be5b87bb539088661ef3b4c092a996149bffaeeb8140f2287ce62f8f76d870f9298ce6fb6be071fc002b0fad53351ce2d7b4209833e2433d0961493bbe376d4eb0a00ba8e2cf30f0572a39c8d2a189574865a2e03cad6e2e41f5b9d22e3d1c1f9e24db7ca2e525626fab3d7b27f51e1dddac3432dccb9e9a1cfb2f1e7fffdbf85b2da8fc2b2ba7c",
					hex::encode(encrypted_identity)
				);
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
