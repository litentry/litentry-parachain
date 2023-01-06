// Copyright 2020-2022 Litentry Technologies GmbH.
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
	assertion::AssertionHandler, format, identity_verification::IdentityVerificationHandler,
	AccessNodeMetadata, AuthorApi, CreateExtrinsics, Error, HandleState, Hash, IMPCallIndexes,
	SgxExternalitiesTrait, ShardIdentifier, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt,
	StfEnclaveSigning, StfTaskContext, TaskHandler,
};

use codec::Decode;
use ita_sgx_runtime::IdentityManagement;
use itp_node_api::metadata::pallet_vcmp::VCMPCallIndexes;
use itp_ocall_api::EnclaveOnChainOCallApi;
use lc_stf_task_sender::{stf_task_sender, RequestType};
use log::*;
use std::sync::Arc;

// lifetime elision: StfTaskContext is guaranteed to outlive the fn
pub fn run_stf_task_receiver<K, O, C, M, A, S, H>(
	context: Arc<StfTaskContext<K, O, C, M, A, S, H>>,
) -> Result<(), Error>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	M::MetadataType: IMPCallIndexes + VCMPCallIndexes,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	let receiver = stf_task_sender::init_stf_task_sender_storage()
		.map_err(|e| Error::OtherError(format!("read storage error:{:?}", e)))?;

	// TODO: When an error occurs, send the extrinsic (error message) to the parachain
	// TODO: error handling still incomplete, we only print logs but no error handling
	// TODO: we can further simplify the handling logic
	loop {
		let request_type = receiver
			.recv()
			.map_err(|e| Error::OtherError(format!("receiver error:{:?}", e)))?;

		match request_type.clone() {
			RequestType::Web2IdentityVerification(_) | RequestType::Web3IdentityVerification(_) => {
				IdentityVerificationHandler { req: request_type.clone(), context: context.clone() }
					.start();
			},
			RequestType::AssertionVerification(request) => {
				AssertionHandler { req: request.clone(), context: context.clone() }.start();
			},
			// only used for testing
			// demonstrate how to read the storage in the stf-task handling with the loaded state
			// in real cases we prefer to read the state ahead and sent the related storage as parameters in `Request`
			RequestType::SetUserShieldingKey(request) => {
				let shard = ShardIdentifier::decode(&mut request.encoded_shard.as_slice())
					.map_err(|e| {
						Error::OtherError(format!("error decoding ShardIdentifier {:?}", e))
					})?;

				let (mut state, _) = context
					.state_handler
					.load_cloned(&shard)
					.map_err(|e| Error::OtherError(format!("load state failed: {:?}", e)))?;

				let key =
					state.execute_with(|| IdentityManagement::user_shielding_keys(&request.who));

				debug!("in RequestType::SetUserShieldingKey read key is: {:?}", key);

				context.decode_and_submit_trusted_call(
					request.encoded_shard,
					request.encoded_callback,
				)?;
			},
		}
	}
}
