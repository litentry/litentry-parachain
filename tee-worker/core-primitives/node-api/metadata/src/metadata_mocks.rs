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

use crate::{
	error::Result, pallet_imp::IMPCallIndexes, pallet_imp_mock::IMPMockCallIndexes,
	pallet_sidechain::SidechainCallIndexes, pallet_system::SystemSs58Prefix,
	pallet_teerex::TeerexCallIndexes, pallet_vcmp::VCMPCallIndexes,
};
use codec::{Decode, Encode};

#[derive(Default, Encode, Decode, Debug, Clone)]
pub struct NodeMetadataMock {
	teerex_module: u8,
	register_ias_enclave: u8,
	register_dcap_enclave: u8,
	unregister_enclave: u8,
	register_quoting_enclave: u8,
	register_tcb_info: u8,
	call_worker: u8,
	processed_parentchain_block: u8,
	shield_funds: u8,
	unshield_funds: u8,
	publish_hash: u8,
	sidechain_module: u8,
	// litentry
	// IMP
	imp_module: u8,
	imp_set_user_shielding_key: u8,
	imp_create_identity: u8,
	imp_remove_identity: u8,
	imp_verify_identity: u8,
	imp_user_shielding_key_set: u8,
	imp_identity_created: u8,
	imp_identity_removed: u8,
	imp_identity_verified: u8,
	imp_some_error: u8,
	// IMP mock
	imp_mock_module: u8,
	imp_mock_set_user_shielding_key: u8,
	imp_mock_create_identity: u8,
	imp_mock_remove_identity: u8,
	imp_mock_verify_identity: u8,
	imp_mock_user_shielding_key_set: u8,
	imp_mock_identity_created: u8,
	imp_mock_identity_removed: u8,
	imp_mock_identity_verified: u8,
	imp_mock_some_error: u8,
	// VCMP
	vcmp_module: u8,
	vcmp_request_vc: u8,
	vcmp_vc_issued: u8,
	vcmp_some_error: u8,

	imported_sidechain_block: u8,
	runtime_spec_version: u32,
	runtime_transaction_version: u32,
}

impl NodeMetadataMock {
	pub fn new() -> Self {
		NodeMetadataMock {
			teerex_module: 50u8,
			register_ias_enclave: 0u8,
			register_dcap_enclave: 6,
			unregister_enclave: 1u8,
			register_quoting_enclave: 7,
			register_tcb_info: 8,
			call_worker: 2u8,
			processed_parentchain_block: 3u8,
			shield_funds: 4u8,
			unshield_funds: 5u8,
			publish_hash: 9u8,
			sidechain_module: 53u8,
			// litentry
			imp_module: 64u8,
			imp_set_user_shielding_key: 0u8,
			imp_create_identity: 1u8,
			imp_remove_identity: 2u8,
			imp_verify_identity: 3u8,
			imp_user_shielding_key_set: 4u8,
			imp_identity_created: 6u8,
			imp_identity_removed: 7u8,
			imp_identity_verified: 8u8,
			imp_some_error: 9u8,

			vcmp_module: 66u8,
			vcmp_request_vc: 0u8,
			vcmp_vc_issued: 3u8,
			vcmp_some_error: 9u8,

			imp_mock_module: 100u8,
			imp_mock_set_user_shielding_key: 0u8,
			imp_mock_create_identity: 1u8,
			imp_mock_remove_identity: 2u8,
			imp_mock_verify_identity: 3u8,
			imp_mock_user_shielding_key_set: 4u8,
			imp_mock_identity_created: 6u8,
			imp_mock_identity_removed: 7u8,
			imp_mock_identity_verified: 8u8,
			imp_mock_some_error: 9u8,

			imported_sidechain_block: 0u8,
			runtime_spec_version: 25,
			runtime_transaction_version: 4,
		}
	}
}

impl TeerexCallIndexes for NodeMetadataMock {
	fn register_ias_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_ias_enclave])
	}

	fn register_dcap_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_dcap_enclave])
	}

	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.unregister_enclave])
	}

	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_quoting_enclave])
	}

	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_tcb_info])
	}

	fn call_worker_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.call_worker])
	}

	fn confirm_processed_parentchain_block_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.processed_parentchain_block])
	}

	fn shield_funds_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.shield_funds])
	}

	fn unshield_funds_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.unshield_funds])
	}

	fn publish_hash_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.unshield_funds])
	}
}

impl SidechainCallIndexes for NodeMetadataMock {
	fn confirm_imported_sidechain_block_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.sidechain_module, self.imported_sidechain_block])
	}
}

impl IMPCallIndexes for NodeMetadataMock {
	fn set_user_shielding_key_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_set_user_shielding_key])
	}

	fn create_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_create_identity])
	}

	fn remove_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_remove_identity])
	}

	fn verify_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_verify_identity])
	}

	fn user_shielding_key_set_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_user_shielding_key_set])
	}

	fn identity_created_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_created])
	}

	fn identity_removed_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_removed])
	}

	fn identity_verified_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_verified])
	}

	fn imp_some_error_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_some_error])
	}
}

impl IMPMockCallIndexes for NodeMetadataMock {
	fn set_user_shielding_key_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_set_user_shielding_key])
	}

	fn create_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_create_identity])
	}

	fn remove_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_remove_identity])
	}

	fn verify_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_verify_identity])
	}

	fn user_shielding_key_set_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_user_shielding_key_set])
	}

	fn identity_created_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_identity_created])
	}

	fn identity_removed_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_identity_removed])
	}

	fn identity_verified_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_identity_verified])
	}

	fn imp_some_error_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_mock_module, self.imp_mock_some_error])
	}
}

impl VCMPCallIndexes for NodeMetadataMock {
	fn request_vc_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.vcmp_module, self.vcmp_request_vc])
	}

	fn vc_issued_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.vcmp_module, self.vcmp_vc_issued])
	}

	fn vcmp_some_error_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.vcmp_module, self.vcmp_some_error])
	}
}

impl SystemSs58Prefix for NodeMetadataMock {
	fn system_ss58_prefix(&self) -> Result<u16> {
		Ok(131)
	}
}
