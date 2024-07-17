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
	error::Result, pallet_balances::BalancesCallIndexes, pallet_bitacross::BitAcrossCallIndexes,
	pallet_proxy::ProxyCallIndexes, pallet_system::SystemSs58Prefix,
	pallet_teebag::TeebagCallIndexes, pallet_timestamp::TimestampCallIndexes,
	pallet_utility::UtilityCallIndexes, runtime_call::RuntimeCall,
};
use codec::{Decode, Encode};

use itp_api_client_types::Metadata;

impl TryFrom<NodeMetadataMock> for Metadata {
	type Error = ();

	fn try_from(_: NodeMetadataMock) -> core::result::Result<Self, Self::Error> {
		Err(())
	}
}

#[derive(Default, Encode, Decode, Debug, Clone)]
pub struct NodeMetadataMock {
	// teebag
	teebag_module: u8,
	force_add_authorized_enclave: u8,
	force_remove_authorized_enclave: u8,
	register_enclave: u8,
	unregister_enclave: u8,
	register_quoting_enclave: u8,
	register_tcb_info: u8,
	post_opaque_task: u8,
	parentchain_block_processed: u8,
	sidechain_block_imported: u8,

	utility_module: u8,
	utility_batch: u8,
	utility_as_derivative: u8,
	utility_batch_all: u8,
	utility_dispatch_as: u8,
	utility_force_batch: u8,

	imported_sidechain_block: u8,
	proxy_module: u8,
	add_proxy: u8,
	proxy: u8,
	balances_module: u8,
	transfer: u8,
	transfer_keep_alive: u8,
	transfer_allow_death: u8,
	timestamp_module: u8,
	timestamp_set: u8,
	runtime_spec_version: u32,
	runtime_transaction_version: u32,

	bitacross_module: u8,
	bitacross_add_relayer: u8,
	bitacross_remove_relayer: u8,
	btc_wallet_generated: u8,
	eth_wallet_generated: u8,
}

impl NodeMetadataMock {
	pub fn new() -> Self {
		NodeMetadataMock {
			teebag_module: 50u8,
			force_add_authorized_enclave: 0u8,
			force_remove_authorized_enclave: 1u8,
			register_enclave: 2u8,
			unregister_enclave: 3u8,
			register_quoting_enclave: 4u8,
			register_tcb_info: 5u8,
			post_opaque_task: 6u8,
			parentchain_block_processed: 7u8,
			sidechain_block_imported: 8u8,

			utility_module: 80u8,
			utility_batch: 0u8,
			utility_as_derivative: 1u8,
			utility_batch_all: 2u8,
			utility_dispatch_as: 3u8,
			utility_force_batch: 4u8,

			imported_sidechain_block: 0u8,
			proxy_module: 7u8,
			add_proxy: 1u8,
			proxy: 0u8,
			balances_module: 10u8,
			transfer: 7u8,
			transfer_keep_alive: 3u8,
			transfer_allow_death: 0u8,
			timestamp_module: 3,
			timestamp_set: 0,
			runtime_spec_version: 25,
			runtime_transaction_version: 4,

			bitacross_module: 69u8,
			bitacross_add_relayer: 0u8,
			bitacross_remove_relayer: 1u8,
			btc_wallet_generated: 2u8,
			eth_wallet_generated: 3u8,
		}
	}
}

impl TeebagCallIndexes for NodeMetadataMock {
	fn force_add_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.force_add_authorized_enclave])
	}
	fn force_remove_authorized_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.force_remove_authorized_enclave])
	}
	fn register_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.register_enclave])
	}
	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.unregister_enclave])
	}
	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.register_quoting_enclave])
	}
	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.register_tcb_info])
	}
	fn post_opaque_task_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.post_opaque_task])
	}
	fn parentchain_block_processed_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.parentchain_block_processed])
	}
	fn sidechain_block_imported_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.teebag_module, self.sidechain_block_imported])
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

impl ProxyCallIndexes for NodeMetadataMock {
	fn add_proxy_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.proxy_module, self.add_proxy])
	}

	fn proxy_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.proxy_module, self.proxy])
	}
}

impl BalancesCallIndexes for NodeMetadataMock {
	fn transfer_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.balances_module, self.transfer])
	}

	fn transfer_keep_alive_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.balances_module, self.transfer_keep_alive])
	}

	fn transfer_allow_death_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.balances_module, self.transfer_allow_death])
	}
}

impl BitAcrossCallIndexes for NodeMetadataMock {
	fn add_relayer_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.bitacross_module, self.bitacross_add_relayer])
	}

	fn remove_relayer_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.bitacross_module, self.bitacross_remove_relayer])
	}

	fn btc_wallet_generated_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.bitacross_module, self.btc_wallet_generated])
	}

	fn eth_wallet_generated_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.bitacross_module, self.eth_wallet_generated])
	}
}

impl TimestampCallIndexes for NodeMetadataMock {
	fn timestamp_set_call_indexes(&self) -> Result<[u8; 2]> {
		Ok([self.timestamp_module, self.timestamp_set])
	}
}
