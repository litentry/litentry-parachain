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
	error::Result, pallet_imp::IMPCallIndexes, pallet_sidechain::SidechainCallIndexes,
	pallet_system::SystemSs58Prefix, pallet_teerex::TeerexCallIndexes,
	pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes, runtime_call::RuntimeCall,
};
use codec::{Decode, Encode};

use crate::pallet_enclave_bridge::EnclaveBridgeCallIndexes;
use itp_api_client_types::Metadata;

impl TryFrom<NodeMetadataMock> for Metadata {
	type Error = ();

	fn try_from(_: NodeMetadataMock) -> core::result::Result<Self, Self::Error> {
		Err(())
	}
}

#[derive(Default, Encode, Decode, Debug, Clone)]
pub struct NodeMetadataMock {
	teerex_module: u8,
	register_sgx_enclave: u8,
	unregister_sovereign_enclave: u8,
	unregister_proxied_enclave: u8,
	register_quoting_enclave: u8,
	register_tcb_info: u8,
	enclave_bridge_module: u8,
	invoke: u8,
	confirm_processed_parentchain_block: u8,
	shield_funds: u8,
	unshield_funds: u8,
	publish_hash: u8,
	update_shard_config: u8,
	sidechain_module: u8,
	// litentry
	update_scheduled_enclave: u8,
	remove_scheduled_enclave: u8,
	// IMP
	imp_module: u8,
	imp_set_user_shielding_key: u8,
	imp_link_identity: u8,
	imp_deactivate_identity: u8,
	imp_activate_identity: u8,
	imp_user_shielding_key_set: u8,
	imp_identity_linked: u8,
	imp_identity_deactivated: u8,
	imp_identity_activated: u8,
	imp_some_error: u8,
	// VCMP
	vcmp_module: u8,
	vcmp_request_vc: u8,
	vcmp_vc_issued: u8,
	vcmp_some_error: u8,

	utility_module: u8,
	utility_batch: u8,
	utility_as_derivative: u8,
	utility_batch_all: u8,
	utility_dispatch_as: u8,
	utility_force_batch: u8,

	imported_sidechain_block: u8,
	runtime_spec_version: u32,
	runtime_transaction_version: u32,
}

impl NodeMetadataMock {
	pub fn new() -> Self {
		NodeMetadataMock {
			teerex_module: 50u8,
			register_sgx_enclave: 0u8,
			unregister_sovereign_enclave: 1u8,
			unregister_proxied_enclave: 2u8,
			register_quoting_enclave: 3,
			register_tcb_info: 4,
			enclave_bridge_module: 54u8,
			invoke: 0u8,
			confirm_processed_parentchain_block: 1u8,
			shield_funds: 2u8,
			unshield_funds: 3u8,
			publish_hash: 4u8,
			update_shard_config: 5u8,
			sidechain_module: 53u8,
			// litentry
			update_scheduled_enclave: 10u8,
			remove_scheduled_enclave: 11u8,

			imp_module: 64u8,
			imp_set_user_shielding_key: 0u8,
			imp_link_identity: 1u8,
			imp_deactivate_identity: 2u8,
			imp_activate_identity: 3u8,
			imp_user_shielding_key_set: 4u8,
			imp_identity_linked: 6u8,
			imp_identity_deactivated: 7u8,
			imp_identity_activated: 7u8,
			imp_some_error: 9u8,

			vcmp_module: 66u8,
			vcmp_request_vc: 0u8,
			vcmp_vc_issued: 3u8,
			vcmp_some_error: 9u8,

			utility_module: 80u8,
			utility_batch: 0u8,
			utility_as_derivative: 1u8,
			utility_batch_all: 2u8,
			utility_dispatch_as: 3u8,
			utility_force_batch: 4u8,

			imported_sidechain_block: 0u8,
			runtime_spec_version: 25,
			runtime_transaction_version: 4,
		}
	}
}

impl TeerexCallIndexes for NodeMetadataMock {
	fn register_sgx_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_sgx_enclave])
	}

	fn unregister_sovereign_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.unregister_sovereign_enclave])
	}

	fn unregister_proxied_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.unregister_proxied_enclave])
	}

	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_quoting_enclave])
	}

	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.register_tcb_info])
	}
}

impl EnclaveBridgeCallIndexes for NodeMetadataMock {
	fn invoke_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.invoke])
	}

	fn confirm_processed_parentchain_block_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.confirm_processed_parentchain_block])
	}

	fn shield_funds_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.shield_funds])
	}

	fn unshield_funds_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.unshield_funds])
	}

	fn publish_hash_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.publish_hash])
	}

	fn update_shard_config_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.enclave_bridge_module, self.update_shard_config])
	}

	fn update_scheduled_enclave(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.update_scheduled_enclave])
	}

	fn remove_scheduled_enclave(&self) -> Result<[u8; 2]> {
		Ok([self.teerex_module, self.remove_scheduled_enclave])
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

	fn link_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_link_identity])
	}

	fn deactivate_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_deactivate_identity])
	}

	fn activate_identity_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_activate_identity])
	}

	fn user_shielding_key_set_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_user_shielding_key_set])
	}

	fn identity_linked_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_linked])
	}

	fn identity_deactivated_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_deactivated])
	}

	fn identity_activated_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_identity_activated])
	}

	fn imp_some_error_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.imp_module, self.imp_some_error])
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

impl UtilityCallIndexes for NodeMetadataMock {
	fn batch_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.utility_module, self.utility_batch])
	}

	fn as_derivative_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.utility_module, self.utility_as_derivative])
	}

	fn batch_all_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.utility_module, self.utility_batch_all])
	}

	fn dispatch_as_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.utility_module, self.utility_dispatch_as])
	}

	fn force_batch_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.utility_module, self.utility_force_batch])
	}
}

impl RuntimeCall for NodeMetadataMock {
	fn retrieve(&self) -> Result<u32> {
		Err(crate::Error::MetadataNotSet)
	}
}

impl SystemSs58Prefix for NodeMetadataMock {
	fn system_ss58_prefix(&self) -> Result<u16> {
		Ok(131)
	}
}
